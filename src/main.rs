use std::env;

use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionMessage},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    _ = dotenv::dotenv();

    let model = env::var("OPENAI_MODEL").unwrap_or("gpt-4.1-nano".to_string());
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:8080".to_string());

    let creds = Credentials::from_env();
    let messages = vec![ChatCompletionMessage {
        role: openai::chat::ChatCompletionMessageRole::System,
        content: Some(include_str!("../system_prompt.txt").to_string()),
        name: None,
        function_call: None,
        tool_call_id: None,
        tool_calls: None,
    }];

    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());

    loop {
        // simple tcp http server
        let (mut socket, _) = listener.accept().await.unwrap();
        let creds = creds.clone();
        let model = model.clone();
        let messages = messages.clone();
        tokio::spawn(async move {
            let mut messages = messages.clone();
            loop {
                let mut buf = vec![];
                // read as much as we can form this network socket, not via read_to_end
                match socket.read_buf(&mut buf).await {
                    Ok(0) => return, // connection closed
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Failed to read from socket: {}", err);
                        return;
                    }
                }

                // Get the peer IP address for logging
                let peer_addr = socket.peer_addr().unwrap();
                let request = String::from_utf8_lossy(&buf);
                println!("[{:?}] Received request: \n{}", peer_addr, request);
                messages.push(ChatCompletionMessage {
                    role: openai::chat::ChatCompletionMessageRole::User,
                    content: Some(request.into_owned()),
                    name: None,
                    function_call: None,
                    tool_call_id: None,
                    tool_calls: None,
                });
                println!("[{:?}] Sending messages to model", peer_addr);
                let completion = match ChatCompletion::builder(&model, messages.clone())
                    .credentials(creds.clone())
                    .create()
                    .await
                {
                    Ok(val) => val,
                    Err(err) => {
                        eprintln!("Failed to create chat completion: {}", err);
                        return;
                    }
                };
                let returned_message = match completion.choices.first() {
                    Some(val) => val.message.content.clone().unwrap(),
                    None => {
                        "HTTP/1.1 500 Internal Server Error\r\n\r\nNo choices returned".to_string()
                    }
                };
                // find the \r\n\r\n
                let mut parts = returned_message.split("\r\n\r\n").collect::<Vec<&str>>();
                if parts.len() < 2 {
                    parts = returned_message.split("\n\n").collect::<Vec<&str>>();
                }
                let headers = parts.get(0).unwrap_or(&"");
                let body = parts.get(1).unwrap_or(&"");

                // remove lines Content-Length and the HTTP header from the string
                let headers = headers
                    .lines()
                    .filter(|line| {
                        !line.starts_with("Content-Length") && !line.starts_with("HTTP/1.1")
                    })
                    .collect::<Vec<&str>>()
                    .join("\r\n");
                // add http header and content length at the beginning again
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n{}\r\n\r\n{}",
                    body.len(),
                    headers,
                    body
                );
                println!(
                    "[{:?}] Responding with {} byte(s)",
                    peer_addr,
                    response.len()
                );
                println!("[{:?}] Response: \n{}", peer_addr, response);
                if let Err(err) = socket.write_all(response.as_bytes()).await {
                    eprintln!("Failed to write to socket: {}", err);
                    return;
                }
            }
        });
    }
}
