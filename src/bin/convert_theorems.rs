use anyhow::Result;
use std::path::Path;
use turn_formal::subjects::math::export::turn_render::convert_all_theorem_files;

fn main() -> Result<()> {
    println!("Converting theorem JSON files to turn_render format...");

    // Default paths
    let theories_dir = Path::new("subjects/math/theories");
    let output_dir = Path::new("subjects/math/theories_turn_render");

    // Check if directories exist
    if !theories_dir.exists() {
        println!(
            "Warning: Input directory '{}' not found",
            theories_dir.display()
        );
    } else {
        println!("Reading from: {}", theories_dir.display());
    }

    // Convert the files
    match convert_all_theorem_files(theories_dir, output_dir) {
        Ok(_) => {
            println!("✅ Conversion successful!");
            println!("Output directory: {}", output_dir.display());
            println!("Converted files are now available in turn_render compatible format");
        }
        Err(e) => {
            println!("❌ Conversion failed: {}", e);
        }
    }

    Ok(())
}
