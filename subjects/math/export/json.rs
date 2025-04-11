use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::subjects::math::export::utils::{
    extract_enum_variants, extract_types_from_source, find_rust_files_with_derive,
};

/// Generate JSON from Rust type definitions
pub fn generate_math_json_exports() -> Result<()> {
    println!("Exporting mathematical theories using the improved parser...");

    // Define the directory where theory-specific Rust types are defined
    let rust_types_dir = "subjects/math/theories";
    let rust_files = find_rust_files_with_derive(rust_types_dir, "Serialize")?;

    println!(
        "Found {} Rust files with serializable types",
        rust_files.len()
    );

    // Create a mapping of theory names to their Rust types
    let mut theory_types: std::collections::HashMap<
        String,
        Vec<crate::subjects::math::export::utils::TypeDefinition>,
    > = std::collections::HashMap::new();

    // Process each Rust file to extract type definitions
    for rust_file in &rust_files {
        let file_path = Path::new(rust_file);

        // Get the top-level theory folder name by extracting the first directory after theories/
        let file_path_str = file_path.to_string_lossy();
        let path_segments: Vec<&str> = file_path_str.split('/').collect();

        // Find the "theories" segment and take the next one as the theory name
        let mut theory_name = String::new();
        for (i, &segment) in path_segments.iter().enumerate() {
            if segment == "theories" && i + 1 < path_segments.len() {
                theory_name = path_segments[i + 1].to_string();
                break;
            }
        }

        // Skip non-theory files, export.rs, and mod.rs when they appear as direct children of theories/
        if theory_name.is_empty()
            || file_path_str.ends_with("export.rs")
            || file_path_str.ends_with("theories/mod.rs")
        {
            continue;
        }

        // Read the file and extract type definitions
        let source = fs::read_to_string(file_path)?;
        let types = extract_types_from_source(&source)?;

        if !types.is_empty() {
            println!(
                "Found {} types in {}: {:?}",
                types.len(),
                theory_name,
                types.iter().map(|t| &t.name).collect::<Vec<_>>()
            );

            // Add to the theory types collection
            if let Some(existing_types) = theory_types.get_mut(&theory_name) {
                existing_types.extend(types);
            } else {
                theory_types.insert(theory_name, types);
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

        // Convert type definitions to JSON with proper handling of enum variants
        let mut json_types = Vec::new();

        for type_def in &types {
            // Extract documentation text from source using improved extract method
            let docs = extract_doc_comments_from_source(&type_def.source);

            let mut json_type = serde_json::json!({
                "name": type_def.name,
                "docs": docs,
                "kind": type_def.kind,
                "members": []
            });

            // Process members differently based on type kind
            if type_def.kind == "enum" {
                // Use the special enum variant processor for enums
                let variants = extract_enum_variants(&type_def.source);
                json_type["members"] = serde_json::to_value(variants)?;
            } else {
                // For structs, use simpler field extraction
                let mut fields = Vec::new();

                // Extract struct fields
                for line in type_def.source.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("pub ") && trimmed.contains(':') {
                        let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
                        if parts.len() == 2 {
                            let name = parts[0].trim_start_matches("pub ").trim();
                            let type_info = parts[1].trim().trim_end_matches(',');

                            // Find doc comment for this field
                            let field_doc = find_field_doc(&type_def.source, name);

                            fields.push(serde_json::json!({
                                "name": name,
                                "type": type_info,
                                "docs": field_doc
                            }));
                        }
                    }
                }

                json_type["members"] = serde_json::to_value(fields)?;
            }

            json_types.push(json_type);
        }

        // Export all types
        let json = serde_json::to_string_pretty(&json_types)?;
        let definitions_path = format!("{}/definitions.json", theory_dir);
        fs::write(&definitions_path, &json)?;
        println!(
            "Generated definitions JSON for {} at {}",
            theory_name, definitions_path
        );
    }

    println!("Successfully generated JSON files for all theories");
    Ok(())
}

/// Extract documentation directly from the raw source code
fn extract_doc_comments_from_source(source: &str) -> String {
    let mut in_doc_block = false;
    let mut doc_lines = Vec::new();
    let mut lines = source.lines().collect::<Vec<_>>();

    // Process from the beginning of the file
    for i in 0..lines.len() {
        let line = lines[i].trim();

        // Found a doc comment
        if line.starts_with("///") {
            in_doc_block = true;
            let doc_text = line.trim_start_matches("///").trim();
            doc_lines.push(doc_text.to_string());

            // Check if next non-empty, non-comment, non-attribute line is a type definition
            let mut j = i + 1;
            while j < lines.len() {
                let next_line = lines[j].trim();

                if next_line.starts_with("///") {
                    // Continue collecting doc comments
                    let next_doc = next_line.trim_start_matches("///").trim();
                    doc_lines.push(next_doc.to_string());
                } else if next_line.is_empty()
                    || next_line.starts_with("//")
                    || next_line.starts_with("#[")
                {
                    // Skip empty lines, regular comments, attributes
                } else if next_line.starts_with("pub struct")
                    || next_line.starts_with("pub enum")
                    || next_line.starts_with("struct")
                    || next_line.starts_with("enum")
                {
                    // Found a type definition, this doc block belongs to it
                    return doc_lines.join("\n");
                } else {
                    // Found something else, not a type definition
                    // Reset and continue searching
                    in_doc_block = false;
                    doc_lines.clear();
                    break;
                }

                j += 1;
            }
        }
    }

    // If we collected docs but didn't find a type def, return them anyway
    if !doc_lines.is_empty() {
        return doc_lines.join("\n");
    }

    String::new()
}

/// Helper function to find the doc comment for a struct field
fn find_field_doc(source: &str, field_name: &str) -> String {
    let mut doc_lines = Vec::new();
    let lines: Vec<&str> = source.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        // Match field name more precisely
        if (trimmed.starts_with(&format!("pub {}", field_name))
            || trimmed == format!("pub {}", field_name)
            || trimmed.starts_with(&format!("pub(crate) {}", field_name)))
            && (trimmed.contains(':')
                || (i + 1 < lines.len() && lines[i + 1].trim().starts_with(':')))
        {
            // Look backwards for doc comments
            let mut j = i;
            while j > 0 {
                j -= 1;
                let prev_line = lines[j].trim();

                if prev_line.starts_with("///") {
                    // Add the doc line to our collection, removing the /// prefix
                    let doc_text = prev_line.trim_start_matches("///").trim();
                    doc_lines.insert(0, doc_text.to_string());
                } else if !prev_line.is_empty()
                    && !prev_line.starts_with("//")
                    && !prev_line.starts_with("#[")
                {
                    // Found a non-doc, non-attribute line, stop collecting
                    break;
                }
                // Continue through attributes, empty lines, and regular comments
            }

            break;
        }
    }

    doc_lines.join("\n")
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
