use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::subjects::math::export::utils::{
    extract_types_from_source, find_rust_files_with_derive,
};

/// Generate JSON from Rust type definitions
pub fn generate_math_json_exports() -> Result<()> {
    println!("Generating JSON for math domains from Rust type definitions...");

    // Get the common types directly
    let common_types_path = "subjects/math/export/common_types.rs";
    let mut rust_files = vec![common_types_path.to_string()];

    // Change back to using the types directory
    // Define the directory where theory-specific Rust types are defined
    let rust_types_dir = "subjects/math/export/types";
    let rust_files = find_rust_files_with_derive(rust_types_dir, "Serialize")?;

    println!(
        "Found {} Rust files with serializable types",
        rust_files.len()
    );

    // Create a mapping of theory names to their Rust types
    let mut theory_types = std::collections::HashMap::new();

    // Process each Rust file to extract type definitions
    for rust_file in &rust_files {
        let content = fs::read_to_string(rust_file)?;
        let file_name = Path::new(rust_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        // Skip the main types.rs file which contains the general types
        if file_name == "types" {
            continue;
        }

        // Theory name is derived from the file name (e.g., groups.rs -> groups)
        let theory_name = file_name.to_string();

        // Extract the types from this file
        let types = extract_types_from_source(&content)?;

        if !types.is_empty() {
            println!(
                "Found {} types in {}: {:?}",
                types.len(),
                theory_name,
                types.iter().map(|t| &t.name).collect::<Vec<_>>()
            );

            theory_types.insert(theory_name, types);
        }
    }

    // Also check in the main types.rs file for general types that might be used across theories
    let main_types_path = format!("{}/types.rs", "subjects/math/export/types");
    if Path::new(&main_types_path).exists() {
        let content = fs::read_to_string(&main_types_path)?;
        let types = extract_types_from_source(&content)?;

        // Categorize types to appropriate theories based on their name or documentation
        for type_def in types {
            let theory_name = determine_theory_from_type(&type_def.name, &type_def.docs);

            if let Some(types_list) = theory_types.get_mut(&theory_name) {
                types_list.push(type_def);
            } else {
                theory_types.insert(theory_name.clone(), vec![type_def]);
            }
        }
    }

    // If no types were extracted from Rust files, return an error
    if theory_types.is_empty() {
        println!("No types found in Rust files for JSON export");
        return Ok(());
    }

    // Process each theory and generate JSON files
    for (theory_name, types) in theory_types {
        // Create the directory for this theory
        let theory_dir = format!("subjects/math/theories/{}", theory_name);
        let theory_path = Path::new(&theory_dir);

        // Make sure the directory exists
        if !theory_path.exists() {
            println!("Theory directory doesn't exist, creating: {}", theory_dir);
            fs::create_dir_all(theory_path)?;
        }

        // Group types by category (definitions, axioms, theorems, etc.)
        let mut definitions = Vec::new();
        let mut axioms = Vec::new();
        let mut theorems = Vec::new();

        for type_def in &types {
            // Serialize the type definition to JSON
            let json_value = serde_json::to_value(type_def)?;

            // Categorize based on name and documentation
            if type_def.name.contains("Definition") || type_def.docs.contains("definition") {
                definitions.push(json_value);
            } else if type_def.name.contains("Axiom") || type_def.docs.contains("axiom") {
                axioms.push(json_value);
            } else if type_def.name.contains("Theorem") || type_def.docs.contains("theorem") {
                theorems.push(json_value);
            } else {
                // Default to definitions for other types
                definitions.push(json_value);
            }
        }

        // Export definitions
        if !definitions.is_empty() {
            let definitions_json = serde_json::to_string_pretty(&definitions)?;
            let definitions_path = format!("{}/definitions.json", theory_dir);
            fs::write(&definitions_path, &definitions_json)?;
            println!(
                "Generated definitions JSON for {} at {}",
                theory_name, definitions_path
            );
        }

        // Export axioms
        if !axioms.is_empty() {
            let axioms_json = serde_json::to_string_pretty(&axioms)?;
            let axioms_path = format!("{}/axioms.json", theory_dir);
            fs::write(&axioms_path, &axioms_json)?;
            println!(
                "Generated axioms JSON for {} at {}",
                theory_name, axioms_path
            );
        }

        // Export theorems
        if !theorems.is_empty() {
            let theorems_json = serde_json::to_string_pretty(&theorems)?;
            let theorems_path = format!("{}/theorems.json", theory_dir);
            fs::write(&theorems_path, &theorems_json)?;
            println!(
                "Generated theorems JSON for {} at {}",
                theory_name, theorems_path
            );
        }
    }

    println!("Successfully generated JSON files for all theories");
    Ok(())
}

/// Helper function to determine which theory a type belongs to based on its name and docs
fn determine_theory_from_type(type_name: &str, docs: &str) -> String {
    // Try to determine the theory from the type name
    if type_name.contains("Group") {
        return "groups".to_string();
    } else if type_name.contains("Set") || type_name.contains("ZFC") {
        return "zfc".to_string();
    } else if type_name.contains("Category") {
        return "category_theory".to_string();
    }

    // Try to determine from docs
    let docs_lower = docs.to_lowercase();
    if docs_lower.contains("group") {
        return "groups".to_string();
    } else if docs_lower.contains("set") || docs_lower.contains("zfc") {
        return "zfc".to_string();
    } else if docs_lower.contains("category") {
        return "category_theory".to_string();
    }

    // Default theory
    "general".to_string()
}
