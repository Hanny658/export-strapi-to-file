use rust_xlsxwriter::*;
use serde_json::Value;
use inflector::Inflector;
use std::error::Error;

pub fn generate_excel(json_data: &Value, data_name: &str) -> Result<(), Box<dyn Error>> {
    // Save to a capitalised file
    let file_name = format!("{}.xlsx", data_name.to_title_case());

    let mut workbook = Workbook::new();

    // Add a worksheet without specifying a name
    let worksheet = workbook.add_worksheet();

    if let Some(data_array) = json_data["data"].as_array() {
        if !data_array.is_empty() {
            let first_item = &data_array[0]["attributes"];
            let headers: Vec<String> = first_item.as_object()
                .unwrap()
                .keys()
                .cloned()
                .collect();

            // Write the header row
            for (col_num, header) in headers.iter().enumerate() {
                worksheet.write_string(0, col_num as u16, header)?;
            }

            // Write the data rows
            for (row_num, item) in data_array.iter().enumerate() {
                if let Some(attributes) = item["attributes"].as_object() {
                    for (col_num, key) in headers.iter().enumerate() {
                        if let Some(value) = attributes.get(key) {
                            // Write the value based on its type
                            let result: Result<_, XlsxError> = match value {
                                Value::String(s) => worksheet.write_string((row_num + 1) as u32, col_num as u16, s),
                                Value::Number(n) => {
                                    if let Some(num) = n.as_f64() {
                                        worksheet.write_number((row_num + 1) as u32, col_num as u16, num)
                                    } else {
                                        Ok(worksheet) // If the number is not a valid f64, return Ok(())
                                    }
                                }
                                Value::Bool(b) => worksheet.write_string((row_num + 1) as u32, col_num as u16, if *b { "true" } else { "false" }),
                                _ => Ok(worksheet), // Handle any other types as empty cells
                            };
            
                            // Check for errors and return early if any
                            if let Err(e) = result {
                                return Err(Box::new(e));
                            }
                        }
                    }
                }
            }

            workbook.save(&file_name)?; // Save the workbook to local
        } else {
            println!("JSON is Empty...");
        }
    } else {
        println!("Incorrect JSON formatting, need a 'data' array。");
    }

    println!("Excel Exported as: {}", file_name);
    Ok(())
}
