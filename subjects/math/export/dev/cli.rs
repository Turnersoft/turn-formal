use anyhow::Result;
use std::error::Error;

use super::json::generate_math_json_exports;

/// CLI command to export mathematical domain data
pub fn export_math_data_command() -> Result<()> {
    println!("Exporting mathematical domain data...");
    export_all_math_data()?;
    println!("Export completed successfully!");
    Ok(())
}

/// Function to export all math domain data
fn export_all_math_data() -> Result<()> {
    // Export JSON data
    generate_math_json_exports()?;

    Ok(())
}

/// Run export for linear algebra definitions
pub fn export_linear_algebra_defs() -> Result<()> {
    println!("Exporting linear algebra definitions to JSON...");
    generate_math_json_exports()?;
    Ok(())
}
