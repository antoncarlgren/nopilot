extern crate proc_macro;
extern crate syn;

use std::env;
use reqwest::blocking::Client;
use proc_macro::TokenStream;
use syn::{
    parse_macro_input, 
    LitStr,
    Error,
    __private::Span
};

const API_MAX_TOKENS: u16 = 512;
const API_ENDPOINT: &str = "https://api/openai.com/v1/completions";
const API_MODEL: &str = "";
const API_ENV_KEY: &str = "OPENAI_KEY";

#[proc_macro]
pub fn generate_function(description: TokenStream) -> proc_macro::TokenStream {
    let parsed_description = parse_macro_input!(description as LitStr)
        .value();

    let api_key = match env::var(API_ENV_KEY) {
        Ok(value) => value,
        Err(_) => return Error::new(
            Span::call_site(), 
            format!("No environment variable could be found for key: '{}'", API_ENV_KEY))
                .into_compile_error()
                .into() 
    };  

    let response = get_function_from_chatgpt(parsed_description, api_key);
    match response.parse() {
        Ok(value) => value,
        Err(value) => Error::new(
            Span::call_site(), 
            format!("Could not parse API response: '{}'", value))
                .into_compile_error()
                .into() 
    }
}

fn get_function_from_chatgpt(description: String, api_key: String) -> Response<String, String> {
    let client = Client::new();

    let response = client
        .post("");

    "".to_string()
}
