use anyhow::Result;
use std::path::Path;
use turn_formal::subjects::math::export::{
    dev::json::generate_math_json_exports, turn_render::convert_all_theorem_files,
};

fn main() -> Result<()> {
    println!("Compiling mathematical content to JSON...");

    // Export JSON data
    generate_math_json_exports()?;

    // Convert existing theorem files to turn_render format in-place
    println!("Converting theorem files to turn_render compatible format...");
    let theories_dir = Path::new("subjects/math/theories");
    let output_dir = None; // Pass None to write in-place

    convert_all_theorem_files(theories_dir, output_dir)?;
    println!("Conversion to turn_render format complete!");

    println!("Content compilation complete!");
    Ok(())
}
