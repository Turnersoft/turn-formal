use anyhow::Result;
use std::fs;
use std::path::Path;

/// List of TypeScript types to export from Rust
pub const TS_TYPES: &[&str] = &[
    "DomainReference",
    "MathObjectMetadata",
    "ProofStep",
    "AnimationData",
    "AnimationState",
    "Proof",
    "MathModule",
    "ReferenceType",
    "GroupDefinition",
    "GroupElement",
    "GroupHomomorphism",
];

/// Generates TypeScript types from Rust types
pub fn generate_typescript_exports() -> Result<()> {
    println!("Generating TypeScript types from Rust types...");

    // Path to the TypeScript output directory
    let output_dir = "frontend/src/types/generated";

    // Create output directory if it doesn't exist
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    // The ts-rs crate handles the export of annotated types
    // This function just ensures the files are copied to their final location

    // Copy generated types to their final location
    for type_name in TS_TYPES {
        let src_path = format!("./target/ts-rs/{}.ts", type_name);
        let dest_path = format!("{}/{}.ts", output_dir, type_name);

        if Path::new(&src_path).exists() {
            fs::copy(&src_path, &dest_path)?;
            println!("Copied {} to {}", src_path, dest_path);
        } else {
            println!("Warning: {} not found", src_path);
        }
    }

    println!("TypeScript type generation complete!");
    Ok(())
}
