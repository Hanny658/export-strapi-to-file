// module declarations
mod utils;

// std n utils
use std::env;
use std::path::PathBuf;
use utils::pdf_utils::generate_pdf;         // Function saves a PDF to export/
use utils::excel_utils::generate_excel;     // Function saves a Excel to export/
// libraries
use actix_files::NamedFile;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder, Error, http, post};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct ExportRequest {
    format: String,
    data_name: String,
}

async fn download_file(file_path: &str) -> Result<NamedFile, Error> {
    let path = PathBuf::from(file_path);

    // Check if the file exists and is a file
    if path.exists() && path.is_file() {
        // Return the file as a NamedFile response, which sets appropriate headers automatically
        Ok(NamedFile::open(path)?)
    } else {
        Err(actix_web::error::ErrorNotFound("File not found"))
    }
}


#[post("/export")]
async fn export_file(req: HttpRequest, payload: web::Json<ExportRequest>) -> impl Responder {
    let strapi_endpoint = env::var("STRAPI_ENDPOINT").unwrap_or_else(|_| "https://example.com/api".to_string());
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "Nope, API Key is NESSESARY!".to_string());

    let api_url = format!("{}{}?populate=*", strapi_endpoint, payload.data_name);
    let client = reqwest::Client::new();
    let response = client.get(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await;

    match response {
        Ok(res) => {
            if let Ok(json_data) = res.json::<Value>().await {
                match payload.format.as_str() {
                    "excel" => {
                        match generate_excel(&json_data, &payload.data_name) {
                            Ok(file_path) => {
                                println!("{}", file_path);
                                // Return the file using NamedFile directly
                                match download_file(&file_path).await {
                                    Ok(file) => file.into_response(&req),
                                    Err(_) => HttpResponse::InternalServerError().body("Error downloading file"),
                                }
                            },
                            Err(e) => {
                                HttpResponse::InternalServerError().body(format!("Error generating Excel: {}", e))
                            },
                        }
                    }
                    "pdf" => {
                        match generate_pdf(&json_data, &payload.data_name) {
                            Ok(file_path) => {
                                match download_file(&file_path).await {
                                    Ok(file) => file.into_response(&req),
                                    Err(_) => HttpResponse::InternalServerError().body("Error downloading file"),
                                }
                            },
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
