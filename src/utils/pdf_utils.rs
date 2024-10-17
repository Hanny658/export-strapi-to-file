use serde_json::Value;
use inflector::Inflector;
use std::error::Error;

pub fn generate_pdf(json_data: &Value, data_name: &str) -> Result<(), Box<dyn Error>> {
    // Save to a capitalised filename
    let file_name = format!("{}.pdf", data_name.to_title_case());

    println!("Calling generate_pdf() fn. Filename='{}'.", file_name);
    Ok(())
}
