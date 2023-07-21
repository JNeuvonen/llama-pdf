# What is llama-pdf?

Extract information from or summarize PDF files using llama-v2 running locally on your own machine.

# Requirements

- Enough of RAM/VRAM to run llama-v2 locally
- Rust
- Python

# Run

## Mac/Linux

`chmod +x init_unix.sh && ./init_unix.sh`

## Windows

`init_win.bat`

I have not yet tested windows start script, so I am not completely certain it works.

# Features

- llama v2 running locally on python web-server
- parse text out of PDF
- send requests to py web-server and stream back the results

# TODO

- Make streaming respones more pretty. Currently, tokens are printed as they arrive, which does not make for a very pleasant reading experience.
- Add support for follow-up questions. Currently, context is refreshed on every prompt.
  
