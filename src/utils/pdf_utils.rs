use serde_json::Value;
use inflector::Inflector;
use std::error::Error;
use std::fs;

pub fn generate_pdf(_json_data: &Value, data_name: &str) -> Result<String, Box<dyn Error>> {
    // Save to a capitalised filename
    let file_name = format!("{}.pdf", data_name.to_title_case());
    let file_path = format!("./exports/{}", file_name);
    // Create the exports directory if it doesn't exist
    fs::create_dir_all("./exports")?;

    println!("Calling generate_pdf() fn. Filename='{}'.", file_name);
    Ok(file_path)
}
