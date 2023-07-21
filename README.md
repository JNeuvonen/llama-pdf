# What is llama-pdf?

Extract information from or summarize PDF files using llama-v2 running locally on your own machine. Demo video is coming!

# Requirements

- Enough of RAM/VRAM to run llama-v2 locally
- Rust
- Python

# Run

The setup script downloads llama-v2 GGML from huggingface with 13b parameters using 4-bit quantization. That is around 7 GB of data. The setup script will take a while to complete.

## Mac/Linux

1. `chmod +x unix_setup && ./unix_setup.sh`

## Windows

1. `win_setup.bat`

I have not yet tested windows start script, so I am not completely certain it works.


# TODO

- Make streaming respones more pretty. Currently, tokens are printed as they arrive, which does not make for a very pleasant reading experience.
- Add support for follow-up questions. Currently, context is refreshed on every prompt.
  
