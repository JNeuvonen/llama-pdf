@echo off
set MODEL=llama-2-13b-chat.ggmlv3.q4_0.bin
set MODEL_PATH="./%MODEL%"

cd pdf_chat
cd server

wsl -- python3 -m venv venv
wsl -- source venv/bin/activate
wsl -- pip3 install -r requirements.txt

if not exist "%MODEL_PATH%" (
    wsl -- wget "https://huggingface.co/TheBloke/Llama-2-13B-chat-GGML/resolve/main/%MODEL%"
)

wsl -- python3 inference.py

cd ..
cd ..
