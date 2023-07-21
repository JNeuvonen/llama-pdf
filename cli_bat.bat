@echo off
cd /D "full\path\to\llama-pdf"
cd cli
cargo run -- --file pdf_chatbot_example.pdf
