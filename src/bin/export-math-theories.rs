use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct Definition {
    name: String,
    description: String,
    type_name: String,
    variants: Option<Vec<String>>,
    fields: Option<Vec<Field>>,
}

#[derive(Serialize, Deserialize)]
struct Field {
    name: String,
    type_name: String,
    description: String,
}

fn main() -> Result<()> {
    println!("Exporting mathematical theories...");

    // Base path for theory directories
    let theories_path = PathBuf::from("subjects/math/theories");

    // First, gather the list of theory directories
    let entries = fs::read_dir(&theories_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(theory_name) = path.file_name().and_then(|n| n.to_str()) {
                // Skip directories that aren't actual theories
                if theory_name == "export_output" || theory_name == "common" {
                    continue;
                }

                println!("Processing theory: {}", theory_name);

                // Create frontend public directory path for this theory
                let frontend_dir = PathBuf::from(format!("subjects/math/theories/{}", theory_name));
                if !frontend_dir.exists() {
                    fs::create_dir_all(&frontend_dir)?;
                }

                // Process definitions.rs if it exists
                let definitions_path = path.join("definitions.rs");
                if definitions_path.exists() {
                    let definitions = extract_definitions_with_docs(&definitions_path)?;

                    // Write definitions to JSON in the theory's folder
                    let definitions_file = frontend_dir.join("definitions.json");
                    fs::write(
                        &definitions_file,
                        serde_json::to_string_pretty(&definitions)?,
                    )?;
                    println!("  - Exported definitions to {}", definitions_file.display());
                }

                // Process theorems.rs if it exists
                let theorems_path = path.join("theorems.rs");
                if theorems_path.exists() {
                    let theorems = extract_theorems_with_docs(&theorems_path)?;

                    // Write theorems to JSON in the theory's folder
                    let theorems_file = frontend_dir.join("theorems.json");
                    fs::write(&theorems_file, serde_json::to_string_pretty(&theorems)?)?;
                    println!("  - Exported theorems to {}", theorems_file.display());
                }
            }
        }
    }

    println!("Export completed successfully!");
    Ok(())
}

fn extract_definitions_with_docs(file_path: &Path) -> Result<Vec<Value>> {
    let content = fs::read_to_string(file_path)?;
    let mut definitions = Vec::new();

    // Find struct and enum definitions with docs
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        // Look for doc comments
        let mut doc_comments = Vec::new();
        while i < lines.len() && lines[i].trim().starts_with("///") {
            let doc_line = lines[i].trim().trim_start_matches("///").trim();
            doc_comments.push(doc_line);
            i += 1;
        }

        // Check if the next line is a struct or enum definition
        if i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("pub struct") || line.starts_with("pub enum") {
                let is_struct = line.starts_with("pub struct");
                let kind = if is_struct { "struct" } else { "enum" };

                // Extract the name
                let name_start = line.find(kind).unwrap() + kind.len();
                let name_end = line[name_start..]
                    .find(|c: char| {
                        c == '{'
                            || c == '<'
                            || c == ' ' && line[name_start..].trim_start().starts_with("where")
                    })
                    .unwrap_or_else(|| line.len() - name_start);
                let name = line[name_start..name_start + name_end].trim().to_string();

                // Join doc comments
                let docs = doc_comments.join("\n");

                // Extract members (fields or variants)
                let mut members = Vec::new();
                let mut j = i + 1;
                let mut brace_count = if line.contains('{') { 1 } else { 0 };

                while j < lines.len() {
                    let current_line = lines[j].trim();

                    // Update brace count
                    brace_count += current_line.matches('{').count();
                    brace_count -= current_line.matches('}').count();

                    // If we've closed all braces, we're done with this definition
                    if brace_count == 0 && current_line.contains('}') {
                        break;
                    }

                    // For structs, look for fields
                    if is_struct && current_line.starts_with("pub ") && current_line.contains(':') {
                        // Get the field docs before this line
                        let mut field_docs = Vec::new();
                        let mut k = j - 1;
                        while k > i && lines[k].trim().starts_with("///") {
                            field_docs.insert(0, lines[k].trim().trim_start_matches("///").trim());
                            k -= 1;
                        }

                        // Extract field name and type
                        let parts: Vec<&str> = current_line.splitn(2, ':').collect();
                        if parts.len() == 2 {
                            let field_name = parts[0].trim().trim_start_matches("pub").trim();
                            let field_type = parts[1].trim().trim_end_matches(',');

                            // Create field JSON with name field first
                            members.push(serde_json::json!({
                                "name": field_name,
                                "type": field_type,
                                "docs": field_docs.join("\n")
                            }));
                        }
                    }
                    // For enums, look for variants
                    else if !is_struct
                        && !current_line.starts_with("///")
                        && !current_line.is_empty()
                        && !current_line.starts_with('}')
                    {
                        // Get the variant docs before this line
                        let mut variant_docs = Vec::new();
                        let mut k = j - 1;
                        while k > i && lines[k].trim().starts_with("///") {
                            variant_docs
                                .insert(0, lines[k].trim().trim_start_matches("///").trim());
                            k -= 1;
                        }

                        // Extract variant name and type
                        let variant_line = current_line.split(',').next().unwrap_or("").trim();
                        if !variant_line.is_empty() {
                            let variant_parts: Vec<&str> = variant_line.splitn(2, '(').collect();
                            let variant_name = variant_parts[0].trim();

                            // For variants like "Abelian(AbelianPropertyVariant)" we want to extract
                            // name = "Abelian", type = "AbelianPropertyVariant"
                            let (clean_name, param_type) = if variant_parts.len() > 1 {
                                // For tuple variants, extract the parameter type
                                let param_type = variant_parts[1].trim_end_matches(')').trim();
                                (variant_name, Some(param_type))
                            } else {
                                (variant_name, None)
                            };

                            // Create variant JSON with name field first
                            members.push(serde_json::json!({
                                "name": clean_name,
                                "type": if let Some(t) = param_type { t } else { "Unit" },
                                "docs": variant_docs.join("\n")
                            }));
                        }
                    }

                    j += 1;
                }

                // Create the definition with name field first
                let definition = serde_json::json!({
                    "name": name,
                    "docs": docs,
                    "kind": kind,
                    "members": members
                });

                definitions.push(definition);

                // Skip to after the closing brace
                i = j;
            }
        }

        i += 1;
    }

    Ok(definitions)
}

fn extract_theorems_with_docs(file_path: &Path) -> Result<Vec<Value>> {
    let content = fs::read_to_string(file_path)?;
    let mut theorems = Vec::new();

    // Find function definitions that look like proofs/theorems
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        // Look for doc comments
        let mut doc_comments = Vec::new();
        while i < lines.len() && lines[i].trim().starts_with("///") {
            let doc_line = lines[i].trim().trim_start_matches("///").trim();
            doc_comments.push(doc_line);
            i += 1;
        }

        // Check if the next line is a function that might be a theorem
        if i < lines.len() {
            let line = lines[i].trim();
            if (line.starts_with("pub fn prove_") || line.starts_with("fn prove_"))
                && line.contains("(")
            {
                // Extract the theorem name
                let name_start = line.find("fn").unwrap() + 2;
                let name_end = line[name_start..]
                    .find("(")
                    .unwrap_or_else(|| line.len() - name_start);
                let raw_name = line[name_start..name_start + name_end].trim();

                // Clean up the name (remove prove_ prefix)
                let name = raw_name.trim_start_matches("prove_").replace("_", " ");

                // Description from docs
                let docs = doc_comments.join("\n");

                // Create the theorem with a specific field order (name first)
                let theorem = serde_json::json!({
                    "name": name.clone(),
                    "id": raw_name,
                    "statement": name,
                    "description": docs,
                    "proof_steps": [],
                    "tags": ["auto-generated"]
                });

                theorems.push(theorem);
            }
        }

        i += 1;
    }

    Ok(theorems)
}
