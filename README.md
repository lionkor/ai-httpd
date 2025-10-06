# ai-httpd

This HTTP server responds to every request with an LLM-generated response, including images, css, etc.

Enjoy.

## How it works

Each request is fed into an OpenAI model, which is prompted to respond with a response. This response can be html, svg, css, or anything else you ask it. For example, visiting `/blog/2025-best-cats` will generate a website that looks like a blog with the best cats of 2025 - including (probably invalid) images.
