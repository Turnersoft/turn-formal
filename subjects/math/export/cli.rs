use anyhow::Result;
use std::error::Error;

use crate::subjects::math::export::json::generate_math_json_exports;
use crate::subjects::math::export::typescript::generate_typescript_exports;

/// Exports all mathematical domain data
pub fn export_all_math_data() -> Result<()> {
    // Export JSON data
    generate_math_json_exports()?;

    // Export TypeScript types
    generate_typescript_exports()?;

    println!("All math domain data exported successfully!");
    Ok(())
}

/// CLI command to export mathematical domain data
pub fn export_math_data_command() -> Result<()> {
    println!("Exporting mathematical domain data...");
    export_all_math_data()?;
    println!("Export completed successfully!");
    Ok(())
}
