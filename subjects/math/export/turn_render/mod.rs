// Export functionality for converting theorem files to turn_render format
// This module is specifically designed to support the theorem_watcher
// and other conversion tools

use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::fs::{self, File, create_dir_all};
use std::io::Read;
use std::path::{Path, PathBuf};

use super::super::formalism::core::Theorem;
use super::dev::export::{export_to_json, get_theory_data_path};
use super::dev::json::generate_math_json_exports;
use crate::turn_render::MathNode;
use crate::turn_render::ToTurnMath;

/// Converts a single theorem file from JSON to turn_render format
pub fn convert_theorems_file(theorem_file: &Path, output_dir: Option<&Path>) -> Result<()> {
    println!("Converting theorem file: {}", theorem_file.display());

    // Read the theorem file
    let mut file_content = String::new();
    File::open(theorem_file)
        .context(format!("Failed to open {}", theorem_file.display()))?
        .read_to_string(&mut file_content)
        .context("Failed to read file")?;

    // Parse JSON
    let theorems: Value = serde_json::from_str(&file_content).context("Failed to parse JSON")?;

    // Determine the theory name from the file path
    let theory_name = determine_theory_name(theorem_file)?;

    // Determine output path - either in the same directory as the input file or in a specified output directory
    let output_path = match output_dir {
        Some(dir) => {
            // Create output directory if it doesn't exist
            if !dir.exists() {
                create_dir_all(dir).context(format!(
                    "Failed to create output directory: {}",
                    dir.display()
                ))?;
            }

            // Create the theory output directory
            let theory_output_dir = dir.join(&theory_name);
            if !theory_output_dir.exists() {
                create_dir_all(&theory_output_dir).context(format!(
                    "Failed to create theory output directory: {}",
                    theory_output_dir.display()
                ))?;
            }

            theory_output_dir.join("theorems.json")
        }
        None => {
            // In-place - use the same directory as the input file
            theorem_file.to_path_buf()
        }
    };

    // Convert to turn_render format
    let converted = convert_theorems_to_turn_render(theorems, &theory_name)?;

    // Write to output file
    fs::write(&output_path, serde_json::to_string_pretty(&converted)?)
        .context(format!("Failed to write to {}", output_path.display()))?;

    println!(
        "✅ Successfully processed theorems for theory: {}",
        theory_name
    );
    println!("   📄 Wrote theorems.json to {}", output_path.display());
    Ok(())
}

/// Converts all theorem files in a directory to turn_render format
pub fn convert_all_theorem_files(input_dir: &Path, output_dir: Option<&Path>) -> Result<()> {
    println!("Converting all theorem files from {}", input_dir.display(),);

    if let Some(out_dir) = output_dir {
        println!("Output directory: {}", out_dir.display());
    } else {
        println!("Writing in-place to original files");
    }

    // Find all theorems.json files
    let mut converted_count = 0;
    for entry in walkdir::WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let path = entry.path();
        if path
            .file_name()
            .map(|f| f.to_string_lossy() == "theorems.json")
            .unwrap_or(false)
        {
            match convert_theorems_file(path, output_dir) {
                Ok(_) => {
                    converted_count += 1;
                }
                Err(e) => {
                    eprintln!("⚠️ Error converting {}: {}", path.display(), e);
                }
            }
        }
    }

    println!(
        "✅ Successfully converted {} theorems.json files",
        converted_count
    );
    Ok(())
}

/// Generates theorem files for all supported theories by calling the theorem functions
pub fn generate_all_theorem_files(base_dir: &Path) -> Result<()> {
    println!("Generating theorem files for all supported theories");

    // Ensure the base directory exists
    if !base_dir.exists() {
        create_dir_all(base_dir).context(format!(
            "Failed to create base directory: {}",
            base_dir.display()
        ))?;
    }

    // Generate group theory theorems
    generate_theory_theorems("groups", &[
        ("inverse_uniqueness", super::super::theories::groups::theorems::prove_inverse_uniqueness),
        ("identity_uniqueness", super::super::theories::groups::theorems::prove_identity_uniqueness_with_syntax_trees),
        ("inverse_product_rule", super::super::theories::groups::theorems::prove_inverse_product_rule),
        ("abelian_squared_criterion", super::super::theories::groups::theorems::prove_abelian_squared_criterion),
        ("lagrange_theorem", super::super::theories::groups::theorems::prove_lagrange_theorem),
    ], base_dir)?;

    // Generate JSON exports of definitions (also ensure they're available)
    generate_math_json_exports()?;

    println!("✅ Successfully generated all theorem files");
    Ok(())
}

/// Generates theorems for a specific theory by calling the theorem functions
fn generate_theory_theorems(
    theory_name: &str,
    theorem_fns: &[(&str, fn() -> Theorem)],
    base_dir: &Path,
) -> Result<()> {
    println!("Generating theorems for theory: {}", theory_name);

    // Create the theory directory
    let theory_dir = base_dir.join(theory_name);
    if !theory_dir.exists() {
        create_dir_all(&theory_dir).context(format!(
            "Failed to create theory directory: {}",
            theory_dir.display()
        ))?;
    }

    // Generate each theorem
    let mut theorems = Vec::new();

    for (id, theorem_fn) in theorem_fns {
        println!("  Generating theorem: {}", id);

        // Call the theorem function to get the theorem
        let theorem = theorem_fn();

        // Use the to_turn_math method to convert the theorem to a MathNode
        let math_node = theorem.to_turn_math(id.to_string());

        // Serialize to JSON - the to_turn_math method returns a struct that implements Serialize
        let theorem_json = serde_json::to_value(&math_node)?;

        theorems.push(theorem_json);
    }

    // Write the theorems to a JSON file
    let output_path = theory_dir.join("theorems.json");
    fs::write(&output_path, serde_json::to_string_pretty(&theorems)?)
        .context(format!("Failed to write to {}", output_path.display()))?;

    println!("✅ Successfully generated theorems for {}", theory_name);
    println!("   📄 Wrote theorems.json to {}", output_path.display());
    Ok(())
}

/// Extracts the theory name from a theorem file path
fn determine_theory_name(theorem_file: &Path) -> Result<String> {
    // Normalize the path to make extraction consistent
    let normalized_path = theorem_file.to_string_lossy().replace("\\", "/");

    // Pattern for extracting theory name from paths like:
    // subjects/math/theories/THEORY_NAME/theorems.json
    let segments: Vec<&str> = normalized_path.split('/').collect();

    // Find the "theories" segment and take the next one as the theory name
    for (i, &segment) in segments.iter().enumerate() {
        if segment == "theories" && i + 1 < segments.len() {
            return Ok(segments[i + 1].to_string());
        }
    }

    // If that doesn't work, try to extract from the parent directory
    if let Some(parent) = theorem_file.parent() {
        if let Some(dir_name) = parent.file_name() {
            return Ok(dir_name.to_string_lossy().to_string());
        }
    }

    // Default fallback
    Ok("unknown_theory".to_string())
}

/// Converts theorem JSON to turn_render format
fn convert_theorems_to_turn_render(theorems: Value, theory_name: &str) -> Result<Value> {
    // In this implementation, we'll just pass through the theorems with some metadata
    // In a more complex implementation, you might transform the data more significantly

    // Create a new array to hold our converted theorems
    let mut enriched_theorems = Vec::new();

    // Process each theorem in the array
    if let Value::Array(theorem_array) = theorems {
        for theorem in theorem_array {
            // Create a copy of the theorem
            let mut enriched_theorem = theorem.clone();

            // Try to add theory name metadata
            if enriched_theorem.is_object() {
                // Get the mutable object reference
                let obj = enriched_theorem.as_object_mut().unwrap();

                // Try to get and modify the content field
                if let Some(content) = obj.get_mut("content") {
                    if content.is_object() {
                        let content_obj = content.as_object_mut().unwrap();

                        // Try to get and modify the Theorem field
                        if let Some(theorem_obj) = content_obj.get_mut("Theorem") {
                            if theorem_obj.is_object() {
                                let theorem_inner = theorem_obj.as_object_mut().unwrap();

                                // Add theory name
                                theorem_inner.insert(
                                    "theory".to_string(),
                                    Value::String(theory_name.to_string()),
                                );
                            }
                        }
                    }
                }
            }

            enriched_theorems.push(enriched_theorem);
        }

        return Ok(Value::Array(enriched_theorems));
    }

    // If it's not an array, return as is
    Ok(theorems)
}
