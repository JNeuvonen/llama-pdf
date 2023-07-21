@echo off

set MODEL=llama-2-13b-chat.ggmlv3.q4_0.bin
set MODEL_PATH=%CD%\%MODEL%

cd llama-pdf\server

python -m venv venv
call venv\Scripts\activate
pip3 install -r requirements.txt

if not exist "%MODEL_PATH%" (
    powershell.exe -Command "& {Invoke-WebRequest -Uri https://huggingface.co/TheBloke/Llama-2-13B-chat-GGML/resolve/main/%MODEL% -OutFile %MODEL%}"
)

start python3 inference.py

cd ..
cd ..

cd llama-pdf\cli
cargo run -- --file pdf_chatbot_example.pdf

timeout /t 2 /nobreak
