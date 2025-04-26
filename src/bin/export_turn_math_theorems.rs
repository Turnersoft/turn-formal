use anyhow::Result;
use std::path::Path;
use turn_formal::subjects::math::export::turn_render::generate_all_theorem_files;

fn main() -> Result<()> {
    println!("🔄 Generating theorems in turn_math format...");

    // Path to theories directory
    let theories_dir = Path::new("subjects/math/theories");

    // Ensure the theories directory exists
    if !theories_dir.exists() {
        println!(
            "⚠️ Warning: Theories directory '{}' not found, will be created",
            theories_dir.display()
        );
        std::fs::create_dir_all(theories_dir)?;
    } else {
        println!(
            "📂 Using directory for theorem generation: {}",
            theories_dir.display()
        );
    }

    // Generate theorem files for all supported theories
    match generate_all_theorem_files(theories_dir) {
        Ok(_) => {
            println!("✅ Generation successful!");
            println!("🚀 Theorems have been generated in their respective theory directories");
        }
        Err(e) => {
            println!("❌ Generation failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
