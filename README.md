# What is llama-pdf?

Extract information from or summarize PDF files using llama-v2 running locally on your own machine. Demo video is coming!

# Requirements

- Enough of RAM/VRAM to run llama-v2 locally
- Rust
- Python

# Run

## Mac/Linux

1. `chmod +x server_unix.sh`
2. `./server_unix.sh`
3. `chmod +x cli_unix.sh`
4. `./cli_unix.sh`

## Windows

`server_win.bat`
`cli_win.bat`

I have not yet tested windows start script, so I am not completely certain it works.

# Features

- llama v2 running locally on python web-server
- parse text out of PDF
- send requests to py web-server and stream back the results

# TODO

- Make streaming respones more pretty. Currently, tokens are printed as they arrive, which does not make for a very pleasant reading experience.
- Add support for follow-up questions. Currently, context is refreshed on every prompt.
  
