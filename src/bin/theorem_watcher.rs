use anyhow::Result;
use std::{path::Path, thread, time::Duration};
use turn_formal::subjects::math::export::turn_render::convert_theorems_file;

fn main() -> Result<()> {
    println!("Starting theorem file watcher...");
    println!("This tool will monitor theorems.json files and update them in-place");

    // Default paths
    let theories_dir = Path::new("subjects/math/theories");
    let output_dir = None; // Pass None to write in-place

    // Initial conversion of all files
    println!("Performing initial conversion of existing files...");

    // Find all theorems.json files
    let mut processed_files = Vec::new();
    for entry in walkdir::WalkDir::new(theories_dir)
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
            println!("Found: {}", path.display());

            // Convert this file
            match convert_theorems_file(path, output_dir) {
                Ok(_) => {
                    println!("✅ Converted: {}", path.display());
                    processed_files.push(path.to_path_buf());
                }
                Err(e) => {
                    println!("❌ Error converting {}: {}", path.display(), e);
                }
            }
        }
    }

    println!("\n🔍 Watching for changes in theorem files...");
    println!("Press Ctrl+C to stop");

    // Main watch loop
    loop {
        thread::sleep(Duration::from_secs(2));

        for entry in walkdir::WalkDir::new(theories_dir)
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
                // Get modification time
                if let Ok(metadata) = path.metadata() {
                    if let Ok(modified_time) = metadata.modified() {
                        // Check if this is a new or modified file
                        let needs_processing = !processed_files.contains(&path.to_path_buf())
                            || processed_files
                                .iter()
                                .find(|p| **p == path.to_path_buf())
                                .and_then(|p| p.metadata().ok())
                                .and_then(|m| m.modified().ok())
                                .map(|last_mod| last_mod < modified_time)
                                .unwrap_or(true);

                        if needs_processing {
                            println!("➡️ Change detected in: {}", path.display());

                            // Convert this file
                            match convert_theorems_file(path, output_dir) {
                                Ok(_) => {
                                    println!("✅ Converted: {}", path.display());

                                    // Update processed files list
                                    if let Some(idx) = processed_files
                                        .iter()
                                        .position(|p| *p == path.to_path_buf())
                                    {
                                        processed_files[idx] = path.to_path_buf();
                                    } else {
                                        processed_files.push(path.to_path_buf());
                                    }
                                }
                                Err(e) => {
                                    println!("❌ Error converting {}: {}", path.display(), e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
