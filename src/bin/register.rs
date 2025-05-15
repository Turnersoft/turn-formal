use std::{fs::File, io::Write};

use turn_formal::subjects::math::formalism::registry::get_serializable_theorem_metas;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let serializable_metas = get_serializable_theorem_metas();
    let json = serde_json::to_string_pretty(&serializable_metas).map_err(|e| {
        eprintln!("Serialization error: {}", e);
        eprintln!("Serializable metas: {:?}", serializable_metas);
        e
    })?;

    let mut file = File::create("theorems.json").map_err(|e| {
        eprintln!("Failed to create theorems.json: {}", e);
        e
    })?;
    file.write_all(json.as_bytes()).map_err(|e| {
        eprintln!("Failed to write to theorems.json: {}", e);
        e
    })?;

    println!("Successfully wrote JSON to theorems.json");
    Ok(())
}
