use anyhow::Result;
use turn_formal::subjects::math::export::json::generate_math_json_exports;

fn main() -> Result<()> {
    println!("Exporting mathematical theories using the improved parser...");
    generate_math_json_exports()?;
    println!("Export completed successfully!");
    Ok(())
}
