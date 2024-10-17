// module declarations
mod utils;

// std n utils
use std::env;
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

// POST request handler
#[post("/export")]
async fn export_file(req: web::Json<ExportRequest>) -> impl Responder {
    // Create Endpoints from API and ENV
    let strapi_endpoint = env::var("STRAPI_ENDPOINT").unwrap_or_else(|_| "https://example.com/api".to_string());
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "Nope, API Key is NESSESARY!".to_string());

    // Create request URL
    let api_url = format!("{}{}?populate=*", strapi_endpoint, req.data_name);

    // Get data from backend
    let client = Client::new();
    let response = client.get(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await;

    // Handle response
    match response {
        Ok(res) => {
            if let Ok(json_data) = res.json::<Value>().await {
                match req.format.as_str() {
                    "excel" => {
                        match generate_excel(&json_data, &req.data_name) {
                            Ok(_) => HttpResponse::Ok().body(format!("Excel file for {} created successfully.", req.data_name)),
                            Err(e) => HttpResponse::InternalServerError().body(format!("Error generating Excel: {}", e)),
                        }
                    }
                    "pdf" => {
                        // Assuming you have a generate_pdf function
                        match generate_pdf(&json_data, &req.data_name) {
                            Ok(()) => HttpResponse::Ok().body(format!("PDF file for {} created successfully.", req.data_name)),
                            Err(e) => HttpResponse::InternalServerError().body(format!("Error generating PDF: {}", e)),
                        }
                    }
                    _ => HttpResponse::BadRequest().body("Invalid format. Supported values are 'excel' or 'pdf'."),
                }
            } else {
                HttpResponse::InternalServerError().body("Error parsing JSON data from API.")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching data from API: {}", e)),
    }
}


/* Entry point here */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); // Load .env

    // Read the port from the environment variable
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Invalid port number in LISTENING_PORT");

    println!("Starting service at port {}!", port);
    // Start the Actix web server
    HttpServer::new(|| {
        App::new()
            .service(export_file)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

/* Created by Hanny as a Project for FUN
   Last but not least ->>>>>> Algorithms are FUN!!! */
