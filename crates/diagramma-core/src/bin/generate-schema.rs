use diagramma_core::diagram_spec_schema;
use serde_json::to_string_pretty;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let schema = diagram_spec_schema();
    let json = to_string_pretty(&schema)?;

    match env::args().nth(1) {
        Some(path) if path != "-" => {
            if let Some(parent) = Path::new(&path).parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(&path, json)?;
        }
        _ => {
            println!("{json}");
        }
    }

    Ok(())
}
