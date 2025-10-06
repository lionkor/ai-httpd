  # ai-httpd

This HTTP server responds to every request with an LLM-generated response, including images, css, etc.

Enjoy.

## How it works

Each request is fed into an OpenAI model, which is prompted to respond with a response. This response can be html, svg, css, or anything else you ask it. For example, visiting `/blog/2025-best-cats` will generate a website that looks like a blog with the best cats of 2025 - including (probably invalid) images.

## How to try it yourself

1. Clone this repository
2. Copy `example.env` and rename it to `.env`
3. Fill out the OPENAI_KEY in your `.env` (you can generate one on [platform.openai.com](https://platform.openai.com/settings/organization/api-keys)), and adjust other settings in the `.env` if you'd like
5. Run `cargo run` and visit the website it prints
