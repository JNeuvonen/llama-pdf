#!/bin/bash

export MODEL=llama-2-13b-chat.ggmlv3.q4_0.bin
export MODEL_PATH="./${MODEL}"

cd llama-pdf/server

python -m venv venv
source venv/bin/activate
pip3 install -r requirements.txt

if [ ! -f "$MODEL_PATH" ]; then
    wget "https://huggingface.co/TheBloke/Llama-2-13B-chat-GGML/resolve/main/${MODEL}"
fi

python3 inference.py &  

sleep 4

cd ..
cd ..

cd llama-pdf/cli
cargo run -- --file pdf_chatbot_example.pdf


