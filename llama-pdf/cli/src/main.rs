extern crate pdf_extract;

mod args;
use args::Opt;
use hyper::body::HttpBody;
use hyper::{header, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use regex::Regex;
use std::io::{self, Write};
use structopt::StructOpt;

fn generate_prompt_template(pdf_text: String, prompt: String) -> String {
    let re = Regex::new(r"[\n\t\r]").unwrap();

    let formatted_pdf_text = re.replace_all(&pdf_text, |caps: &regex::Captures| match &caps[0] {
        "\n" => "".to_string(),
        "\t" => "".to_string(),
        "\r" => "".to_string(),
        _ => caps[0].to_string(),
    });
    let formatted_prompt = format!(
        "This is text of a PDF: '{}'. \\n [ PDF_END ] {}",
        formatted_pdf_text, prompt
    );

    formatted_prompt
}

async fn user_input_loop(pdf_text: String) {
    loop {
        let mut input = String::new();

        print!("Prompt the PDF (type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "exit" {
                    break;
                } else {
                    println!("Creating completion...");
                    let prompt_template =
                        generate_prompt_template(pdf_text.to_string(), input.to_string());

                    println!("{}", prompt_template);
                    create_completion(prompt_template).await;
                }
            }
            Err(error) => println!("Failed to read line: {}", error),
        }
    }
}

async fn create_completion(prompt: String) {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let json_body = format!(r#"{{"prompt":"{}"}}"#, prompt);

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://127.0.0.1:5000/create_completion")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json_body))
        .unwrap();

    let mut res = client.request(req).await.unwrap();

    while let Some(next) = res.data().await {
        let chunk = next.unwrap();
        let chunk_string = std::str::from_utf8(&chunk).unwrap();
        println!("{}", chunk_string);
    }
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let file = opt.file;
    let file_bytes = std::fs::read(file.to_string()).unwrap();
    let extracted_text = pdf_extract::extract_text_from_mem(&file_bytes).unwrap();
    println!("Text of PDF {} extracted", file);

    user_input_loop(extracted_text.to_string()).await;
}
