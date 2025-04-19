use anyhow::Result;
use turn_formal::subjects::math::export::dev::json::generate_math_json_exports;

fn main() -> Result<()> {
    println!("Compiling mathematical content to JSON...");

    // Export JSON data
    generate_math_json_exports()?;

    println!("Content compilation complete!");
    Ok(())
}
