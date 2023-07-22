# What is llama-pdf?

Extract information from or summarize PDF files using llama-v2 running locally on your own machine. Demo video is coming!

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

I have not yet tested windows start script, so I am not completely certain it works.

## Troubleshooting

- If setup scripts fail, try removing the line `n_gpu_layers=20000` inside inference.py that initiates the Flask web server. Here is a link to it: https://github.com/JNeuvonen/llama-pdf/blob/master/llama-pdf/server/inference.py#L8

- For some unknown reasons, the Python venv doesn't seem to get GPU acceleration every time.


# TODO

- Make streaming respones more pretty. Currently, tokens are printed as they arrive, which does not make for a very pleasant reading experience.
- Add support for follow-up questions. Currently, context is refreshed on every prompt.
- Add support for prompting the document using voice via whisper.cpp speech-to-text. Extra: Make the model respond using voice with text-to-speech.
- GUI instead of CLI
  
