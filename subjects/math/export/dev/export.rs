use anyhow::{Context, Result};
use serde::Serialize;
use std::error::Error;
use std::fs::{self, create_dir_all};
use std::path::Path;

/// Exports a serializable data structure to a JSON file
pub fn export_to_json<T>(data: &T, output_path: &str) -> Result<()>
where
    T: Serialize,
{
    // Ensure the directory exists
    if let Some(parent) = Path::new(output_path).parent() {
        create_dir_all(parent).context("Failed to create directory")?;
    }

    // Serialize to JSON
    let json = serde_json::to_string_pretty(data).context("Failed to serialize to JSON")?;

    // Write to file
    fs::write(output_path, json).context("Failed to write JSON to file")?;

    println!("Exported data to {}", output_path);
    Ok(())
}

/// Exports a collection of items to individual JSON files
pub fn export_collection<T>(
    items: &[T],
    base_path: &str,
    id_fn: impl Fn(&T) -> String,
) -> Result<()>
where
    T: Serialize,
{
    // Ensure the base directory exists
    create_dir_all(base_path).context("Failed to create directory")?;

    // Create a manifest of all items
    let manifest: Vec<String> = items.iter().map(|item| id_fn(item)).collect();

    // Export the manifest
    export_to_json(&manifest, &format!("{}/manifest.json", base_path))?;

    // Export each item
    for item in items {
        let id = id_fn(item);
        let file_path = format!("{}/{}.json", base_path, id);
        export_to_json(item, &file_path)?;
    }

    Ok(())
}

/// Writes data to a JSON file in the frontend public directory
pub fn write_to_json<T: Serialize>(theory: &str, file_name: &str, data: &T) -> Result<()> {
    // Get the theory data path using our consistent path function
    let path = get_theory_data_path(theory);
    let file_path = format!("{}/{}.json", path, file_name);

    // Create directories if they don't exist
    fs::create_dir_all(&path).context("Failed to create theory directory")?;

    // Serialize data to JSON
    let json = serde_json::to_string_pretty(data).context("Failed to serialize to JSON")?;

    // Write JSON to file
    fs::write(&file_path, json).context("Failed to write JSON to file")?;

    println!("Wrote {} data to {}", file_name, file_path);
    Ok(())
}

/// Returns the path to a theory's data directory
///
/// # Arguments
/// * `theory` - Name of the theory
///
/// # Returns
/// * `String` - Path to the theory's data directory
pub fn get_theory_data_path(theory: &str) -> String {
    // Return the path to the theory directory
    // Check the environment for the output path configuration
    if let Ok(output_dir) = std::env::var("MATH_OUTPUT_DIR") {
        format!("{}/{}", output_dir, theory)
    } else if std::env::var("DEMO_MODE").is_ok() {
        format!("demo_output/{}", theory)
    } else {
        // Default to public directory structure for the web app
        format!("frontend/public/data/math/{}", theory)
    }
}

/// Checks if a theory's data directory exists
///
/// # Arguments
/// * `theory` - Name of the theory
///
/// # Returns
/// * `bool` - True if the directory exists
pub fn theory_data_exists(theory: &str) -> bool {
    let path = get_theory_data_path(theory);
    println!("Checking if theory path exists: {}", path);
    Path::new(&path).exists()
}
