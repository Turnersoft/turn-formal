use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{fs, path::Path};

use crate::subjects::math::formalism::core::Theorem;
use crate::subjects::math::theories::groups::theorems::*;
use crate::turn_render::ToTurnMath;

/// Generate theorem files for all supported theories
pub fn generate_all_theorem_files(base_dir: &Path) -> Result<()> {
    // List of supported theories
    let theories = vec!["groups"];

    // Generate theorems.json for each supported theory
    for theory in theories {
        let theory_dir = base_dir.join(theory);
        if !theory_dir.exists() {
            println!("Creating directory: {}", theory_dir.display());
            fs::create_dir_all(&theory_dir)?;
        }

        let output_path = theory_dir.join("theorems.json");
        generate_theory_theorems(&output_path, theory)?;
    }

    Ok(())
}

/// Generate a theorems.json file for a specific theory
pub fn generate_theory_theorems(output_path: &Path, theory_name: &str) -> Result<()> {
    // Generate turn_render version of the theorems
    let theorems_json = generate_turn_render_theorems(theory_name)?;

    // Write to the output file
    fs::write(output_path, theorems_json)?;
    println!(
        "Generated theorems.json for theory '{}' at: {}",
        theory_name,
        output_path.display()
    );

    Ok(())
}

/// Generate turn_render compatible JSON for a theory
fn generate_turn_render_theorems(theory_name: &str) -> Result<String> {
    // Create array to hold the converted theorems
    let mut turn_render_theorems = Vec::<Value>::new();

    // Generate theorems based on the theory name
    match theory_name {
        "groups" => {
            // For groups theory, call each theorem function and convert using to_turn_math
            let mut theorems = Vec::<Theorem>::new();

            // Add all the theorem functions from groups/theorems.rs
            theorems.push(prove_inverse_uniqueness());
            theorems.push(prove_identity_uniqueness_with_syntax_trees());
            theorems.push(prove_inverse_product_rule());
            theorems.push(prove_abelian_squared_criterion());
            theorems.push(prove_lagrange_theorem());

            // Convert each theorem to turn_math representation
            for (i, theorem) in theorems.iter().enumerate() {
                let master_id = format!("theorem_{}", i);
                let math_node = theorem.to_turn_math(master_id);

                // Convert to serde_json::Value and preserve the id/index
                let json_value = json!({
                    "id": format!("theorem_{}", i),
                    "content": {
                        "Theorem": {
                            "name": theorem.name,
                            "description": format!("Theorem: {}", theorem.name),
                            "initial_proof_state": serde_json::to_value(&math_node).context(format!(
                                "Failed to convert theorem {} to JSON",
                                theorem.name
                            ))?
                        }
                    }
                });

                turn_render_theorems.push(json_value);
            }
        }
        // Add other theories here as needed
        _ => {
            println!(
                "Warning: No implementation for generating theorems for theory '{}'",
                theory_name
            );
            // Create an empty array for theories we don't have implementations for
            // This ensures the file is valid JSON even for theories we haven't implemented
        }
    }

    // Convert to pretty JSON string
    let json_string = serde_json::to_string_pretty(&turn_render_theorems)
        .context("Failed to convert theorems to JSON string")?;

    Ok(json_string)
}
