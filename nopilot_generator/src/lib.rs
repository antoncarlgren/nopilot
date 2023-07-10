extern crate proc_macro;
extern crate syn;

use std::env;
use proc_macro::TokenStream;
use serde_json::json;
use reqwest::{ blocking::Client, StatusCode, };
use syn::{ parse_macro_input, LitStr, __private::Span, };

const API_MAX_TOKENS: u16 = 512;
const API_ENDPOINT: &str = "https://api/openai.com/v1/completions";
const API_MODEL: &str = "";
const API_ENV_KEY: &str = "OPENAI_KEY";

#[proc_macro]
pub fn generate_function(description: TokenStream) -> proc_macro::TokenStream {
    let parsed_description = parse_macro_input!(description as LitStr).value();

    let api_key = match env::var(API_ENV_KEY) {
        Ok(value) => value,
        Err(_) => return syn::Error::new(
            Span::call_site(), 
            format!("No environment variable could be found for key: '{API_ENV_KEY}'"))
                .into_compile_error()
                .into() 
    };  

    let response = get_function_from_chatgpt(parsed_description, api_key);
    match response {
        Ok(value) => match value.parse() {
            Ok(parsed_value) => parsed_value,
            Err(value) => syn::Error::new(
                Span::call_site(), 
                format!("Could not parse API response: '{value}'"))
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
        .post(API_ENDPOINT)
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&json!({
            "prompt": description,
            "max_tokens": API_MAX_TOKENS,
            "model": API_MODEL
        }));

    let response = match request.send() {
        Ok(value) => value,
        Err(error) => return Err(get_error_code_with_default(error))
    };

    let json = match response
        .json::<serde_json::Value>() {
            Ok(output) => output,
            Err(error) => return Err(get_error_code_with_default(error))
        };
   
    if let Some(parsed_code) = json["choices"][0]["text"].as_str() {
        return Ok(parsed_code.to_string());
    }

    Err("Invalid API response format.".to_string())
}

fn get_error_code_with_default(response: reqwest::Error) -> String {
        let status_code = match response.status() {
            Some(code) => code,
            None => StatusCode::INTERNAL_SERVER_ERROR
        };

        return status_code.as_str().to_string();
}
