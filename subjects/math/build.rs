use anyhow::Result;
use std::fs::{self, create_dir_all};
use std::path::Path;
use std::process::Command;

fn main() -> Result<()> {
    // Create directories for exported data
    create_export_directories()?;

    // Generate TypeScript types
    generate_ts_types()?;

    // Export domain data
    export_domain_data()?;

    println!("cargo:rerun-if-changed=subjects/math/export/types.rs");
    println!("cargo:rerun-if-changed=subjects/math/theories/*/types.rs");

    Ok(())
}

/// Create the directory structure for exported data
fn create_export_directories() -> Result<()> {
    let base_dir = "frontend/public/data/math/theories";
    let theory_dirs = get_theory_directories();

    // Create base directory
    if !Path::new(base_dir).exists() {
        create_dir_all(base_dir)?;
        println!("Created directory: {}", base_dir);
    }

    // Create theory directories
    for theory in theory_dirs {
        let theory_dir = format!("{}/{}", base_dir, theory);
        if !Path::new(&theory_dir).exists() {
            create_dir_all(&theory_dir)?;
            println!("Created directory: {}", theory_dir);
        }
    }

    Ok(())
}

/// Generate TypeScript type definitions from Rust types
fn generate_ts_types() -> Result<()> {
    // Create output directory for TypeScript types
    let ts_output_dir = "frontend/src/types/generated";
    if !Path::new(ts_output_dir).exists() {
        create_dir_all(ts_output_dir)?;
    }

    // Run ts-rs to generate TypeScript types
    let status = Command::new("cargo")
        .args(&["run", "--bin", "ts-rs-export"])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to generate TypeScript types");
    }

    println!("Generated TypeScript types");
    Ok(())
}

/// Export domain data to JSON files
fn export_domain_data() -> Result<()> {
    // Use the export functions to write domain data
    println!("Exporting domain data...");

    let status = Command::new("cargo")
        .args(&["run", "--bin", "export-math-domains"])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to export domain data");
    }

    println!("Domain data exported successfully");
    Ok(())
}

/// Get available theory directories
fn get_theory_directories() -> Vec<String> {
    let theories_dir = Path::new("subjects/math/theories");

    if !theories_dir.exists() || !theories_dir.is_dir() {
        return Vec::new();
    }

    fs::read_dir(theories_dir)
        .map(|entries| {
            entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        if e.path().is_dir() {
                            e.file_name().to_str().map(String::from)
                        } else {
                            None
                        }
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}
