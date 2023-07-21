@echo off
set MODEL=llama-2-13b-chat.ggmlv3.q4_0.bin
set MODEL_PATH="./%MODEL%"

cd llama-pdf
cd server

python3 -m venv venv
venv\Scripts\activate
pip3 install -r requirements.txt

if not exist "%MODEL_PATH%" (
    powershell -Command "Invoke-WebRequest -Uri 'https://huggingface.co/TheBloke/Llama-2-13B-chat-GGML/resolve/main/%MODEL%' -OutFile '%MODEL_PATH%'"
)

python3 inference.py

cd ..
cd ..
