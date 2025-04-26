use anyhow::{Context, Result};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, SystemTime};
use std::{fs, thread};
use turn_formal::subjects::math::export::turn_render::{
    convert_theorems_file, generate_all_theorem_files,
};

fn main() -> Result<()> {
    println!("🔍 Starting Turn Math Theorem Watcher");
    println!("This tool will watch for changes and export theorems to turnMath format");

    // Path to theories directory
    let theories_dir = Path::new("subjects/math/theories");

    // Ensure the theories directory exists
    if !theories_dir.exists() {
        println!(
            "⚠️ Warning: Theories directory '{}' not found, will be created",
            theories_dir.display()
        );
        fs::create_dir_all(theories_dir)?;
    } else {
        println!(
            "📂 Using directory for theorem generation: {}",
            theories_dir.display()
        );
    }

    // Initial generation of theorem files
    println!("🔄 Performing initial generation of theorems...");
    match generate_all_theorem_files(theories_dir) {
        Ok(_) => println!("✅ Initial theorem generation successful!"),
        Err(e) => println!("❌ Initial theorem generation failed: {}", e),
    }

    // Set up file watcher
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                tx.send(event).unwrap_or_else(|e| {
                    println!("⚠️ Error sending event: {}", e);
                });
            }
        },
        Config::default(),
    )?;

    // Watch the theories directory recursively
    watcher.watch(theories_dir, RecursiveMode::Recursive)?;
    println!("👀 Watching for changes in: {}", theories_dir.display());
    println!("Press Ctrl+C to stop");

    // Map to track last processed time for each file to prevent duplicate processing
    let mut last_processed: std::collections::HashMap<PathBuf, SystemTime> =
        std::collections::HashMap::new();

    // Process events
    handle_events(rx, theories_dir, &mut last_processed)?;

    Ok(())
}

fn handle_events(
    rx: Receiver<Event>,
    theories_dir: &Path,
    last_processed: &mut std::collections::HashMap<PathBuf, SystemTime>,
) -> Result<()> {
    // Debounce interval
    let debounce_duration = Duration::from_secs(1);

    loop {
        match rx.recv() {
            Ok(event) => {
                // Filter for file modification events
                if let Some(path) = get_affected_path(&event) {
                    // Only process Rust files - they can trigger theorem generation
                    if path.extension().map_or(false, |ext| ext == "rs") {
                        // Check if this file was recently processed
                        let now = SystemTime::now();
                        let should_process = last_processed.get(&path).map_or(true, |last_time| {
                            now.duration_since(*last_time).unwrap_or(debounce_duration)
                                > debounce_duration
                        });

                        if should_process {
                            println!("🔄 Change detected in: {}", path.display());
                            // Update the last processed time
                            last_processed.insert(path.clone(), now);

                            // Generate theorems
                            println!("🔄 Regenerating theorems...");
                            match generate_all_theorem_files(theories_dir) {
                                Ok(_) => {
                                    println!("✅ Theorem generation successful!");
                                    // Also convert the files to turnMath format
                                    convert_theorem_files(theories_dir)?;
                                }
                                Err(e) => println!("❌ Theorem generation failed: {}", e),
                            }
                        }
                    } else if path
                        .file_name()
                        .map_or(false, |name| name == "theorems.json")
                    {
                        // Direct modification of theorems.json file
                        println!("🔄 Change detected in theorems file: {}", path.display());
                        // Update the last processed time
                        last_processed.insert(path.clone(), SystemTime::now());

                        // Convert this specific file
                        match convert_theorems_file(&path, None) {
                            Ok(_) => println!("✅ Converted: {}", path.display()),
                            Err(e) => println!("❌ Error converting {}: {}", path.display(), e),
                        }
                    }
                }
            }
            Err(e) => {
                println!("⚠️ Watch error: {}", e);
                // Brief pause to prevent error spam
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn get_affected_path(event: &Event) -> Option<PathBuf> {
    // Extract path from event
    // Handles various event types by looking at paths field
    event.paths.first().cloned()
}

fn convert_theorem_files(theories_dir: &Path) -> Result<()> {
    println!("🔄 Converting theorem files to turnMath format...");

    // Find all theorems.json files
    let mut converted_count = 0;
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
            match convert_theorems_file(path, None) {
                Ok(_) => {
                    println!("✅ Converted: {}", path.display());
                    converted_count += 1;
                }
                Err(e) => {
                    println!("❌ Error converting {}: {}", path.display(), e);
                }
            }
        }
    }

    println!(
        "✅ Successfully converted {} theorems.json files",
        converted_count
    );
    Ok(())
}
