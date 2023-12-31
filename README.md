# What is llama-pdf?

Extract information from or summarize PDF files using llama-v2 running locally on your own machine.

# Demo

[![GIF](https://i.imgur.com/LlViwmN.gif)](https://imgur.com/a/YP03JcR)

# Requirements

- Enough of RAM/VRAM to run llama-v2 locally
- Rust
- Python

# Run

The setup script downloads llama-v2 GGML model from huggingface with 13b parameters using 4-bit quantization. That is around 7 GB of data. Depending on your internet speed the setup can take a long time to complete. 

## Mac/Linux

1. `chmod +x unix_setup.sh && ./unix_setup.sh`

## Windows

1. `win_setup.bat`

Update: This doesn't work, and I don't have time to debug the Windows build currently.

## Troubleshooting

- The setup script can fail because of too low GPU VRAM. In that case, the inference must be run on the CPU (assuming enough RAM is available), and the tokens per second will be reduced by 100 or 1000x. In this case, try removing the line `n_gpu_layers=20000` inside inference.py that initiates the Flask web server. Here is a link to it: https://github.com/JNeuvonen/llama-pdf/blob/master/llama-pdf/server/inference.py#L8
  
- I have noticed increased performance in tokens per second if the application CLI and server are run on separate processes. The current quick setup script written in bash runs the server in the background and the CLI in the foreground. The slow performance could be explained by other reasons as well, but running two separate terminal sessions increases tokens/sec a lot.

# TODO

- Add support for follow-up questions. Currently, context is refreshed on every prompt.
- Add support for prompting the document using voice via whisper.cpp speech-to-text. Extra: Make the model respond using voice with text-to-speech.
- GUI instead of CLI
- Run inference directly in rust, instead of using python web server
  
