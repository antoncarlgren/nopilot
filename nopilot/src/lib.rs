extern crate proc_macro;
extern crate syn;

use std::env;
use proc_macro::TokenStream;
use serde_json::json;
use reqwest::blocking::Client;
use syn::{ parse_macro_input, LitStr, __private::Span, };

const MAX_TOKENS: u16 = 256;
const ENDPOINT: &str = "https://api.openai.com/v1/completions";
const MODEL: &str = "text-davinci-003";
const ENV_API_KEY: &str = "OPENAI_KEY";
const TEMPERATURE: f32 = 0.0;

#[proc_macro]
pub fn generate(description: TokenStream) -> proc_macro::TokenStream {
    let parsed_description = parse_macro_input!(description as LitStr).value();

    let api_key = match env::var(ENV_API_KEY) {
        Ok(value) => value,
        Err(_) => return syn::Error::new(
            Span::call_site(), 
            format!("No environment variable could be found for key: '{ENV_API_KEY}'"))
                .into_compile_error()
                .into() 
    };  

    let response = get_function_from_chatgpt(parsed_description, api_key);
    
    match response {
        Ok(value) => match value.parse() {
            Ok(parsed_value) => parsed_value,
            Err(value) => syn::Error::new(
                Span::call_site(), 
                format!("Could not parse response: '{value}'"))
                    .into_compile_error()
                    .into() 
        },
        Err(status_code) => syn::Error::new(
            Span::call_site(), 
            format!("Something went wrong while contacting the OpenAI API. Error/status code: {status_code}"))
                .into_compile_error()
                .into() 
    }
}

fn get_function_from_chatgpt(description: String, api_key: String) -> Result<String, String> {
    let client = Client::new();
    let request = client
        .post(ENDPOINT)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "prompt": description,
            "max_tokens": MAX_TOKENS,
            "model": MODEL,
            "temperature": TEMPERATURE,
        }));

    let response = match request.send() {
        Ok(value) => value,
        Err(error) => return Err(error.to_string())
    };

    let json = match response
        .json::<serde_json::Value>() {
            Ok(output) => output,
            Err(_) => return Err("Could not serialize response.".to_string())
        };
   
    match json["choices"][0]["text"].as_str() {
        Some(parsed_code) => { 
            println!("Prompt: {}", description); 
            println!("Code: {}", parsed_code);
            return Ok(parsed_code.to_string());
        },
        None => Err(json["error"]["message"].to_string())
    }
}
