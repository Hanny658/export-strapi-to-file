// module declarations
mod utils;

// std n utils
use std::env;
use std::io::{Read, Cursor};
use std::fs::File;
use utils::pdf_utils::generate_pdf;
use utils::excel_utils::generate_excel;
// external libraries
use actix_web::{web, App, HttpServer, HttpResponse, Responder, post};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct ExportRequest {
    format: String,
    data_name: String,
}

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok(); // Load .env

    // Creat Endpoints from API and ENV
    let strapi_endpoint = env::var("STRAPI_ENDPOINT").unwrap_or_else(|_| "https://api.do360.com/api".to_string());
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "Nope, API Key is NESSESARY!".to_string());

    // Load Port number also from ENV
    let listeing_port: i32 = env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .unwrap_or(3001);


    // Get Test data for now ====================== TODO: Change to real request
    let mut file = File::open("TestData.json").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let test_data: Value = serde_json::from_str(&contents).expect("Invalid JSON format");


    // Create request URL
    let api_url = format!("{}{}?populate=*", strapi_endpoint, "advertisements");

    // Get data from backend
    let client = Client::new();
    let response = client.get(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Request failed");

    if response.status().is_success() { // double check never fails!
        let res_data: Value = response.json().await.expect("Failed to parse JSON");
        println!("{:#?}", res_data);
    } else {
        println!("Request failed with status: {}", response.status());
    }
    


    //println!("Endpoint is '{}' and API URL is '{}'.", strapi_endpoint, api_url);
    generate_pdf();
    generate_excel();
}


/* Created by Hanny as a Project for FUN | Last Edit 16/10/2024 */
/* Last but not least ->>>>>> Algorithms are FUN!!! */
