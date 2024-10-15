mod utils;

use std::env;
use utils::pdf_utils::generate_pdf;
use utils::excel_utils::generate_excel;

fn main() {
    dotenv::dotenv().ok(); // Load .env

    // Creat Endpoints from API and ENV
    let strapi_endpoint = env::var("STRAPI_ENDPOINT").unwrap_or_else(|_| "https://api.do360.com/api".to_string());
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "Nope, API Key is NESSESARY!".to_string());
    
    println!("Endpoint is '{}' and API Key is '{}'.", strapi_endpoint, api_key);
    generate_pdf();
    generate_excel();
}
