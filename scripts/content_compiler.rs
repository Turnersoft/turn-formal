use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct ContentItem {
    id: String,
    title: String,
    content: String,
    category: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentCollection {
    items: Vec<ContentItem>,
}

fn main() -> io::Result<()> {
    let content_dirs = vec![
        "foundational_theories/type_theory",
        "foundational_theories/category_theory",
        "subjects/math",
        "subjects/logic",
    ];

    // Create output directory if it doesn't exist
    let output_dir = PathBuf::from("frontend/public/content");
    fs::create_dir_all(&output_dir)?;

    // Process each content directory
    for &dir_path in &content_dirs {
        let category = dir_path.split('/').last().unwrap_or("unknown");
        let items = process_directory(Path::new(dir_path), category)?;

        // Write to JSON file
        let output_path = output_dir.join(format!("{}.json", category));
        let mut file = File::create(output_path)?;
        let content = ContentCollection { items };
        let json = serde_json::to_string_pretty(&content)?;
        file.write_all(json.as_bytes())?;

        println!(
            "Generated {} content items for category: {}",
            content.items.len(),
            category
        );
    }

    // Create an index file listing all available content categories
    let categories = content_dirs
        .iter()
        .map(|path| path.split('/').last().unwrap_or("unknown"))
        .collect::<Vec<_>>();

    let index_map: HashMap<String, Vec<String>> = [(
        "categories".to_string(),
        categories.iter().map(|&s| s.to_string()).collect(),
    )]
    .iter()
    .cloned()
    .collect();

    let index_path = output_dir.join("index.json");
    let mut index_file = File::create(index_path)?;
    let index_json = serde_json::to_string_pretty(&index_map)?;
    index_file.write_all(index_json.as_bytes())?;

    println!("Content compilation complete.");
    Ok(())
}

fn process_directory(dir_path: &Path, category: &str) -> io::Result<Vec<ContentItem>> {
    let mut items = Vec::new();

    // For demo purposes, create placeholder content if directory doesn't exist
    if !dir_path.exists() {
        println!(
            "Warning: Directory {} does not exist. Creating placeholder.",
            dir_path.display()
        );
        return Ok(vec![ContentItem {
            id: format!("{}-placeholder", category),
            title: format!("Placeholder for {}", category),
            content: format!("This is placeholder content for the {} category.", category),
            category: category.to_string(),
            path: dir_path.to_string_lossy().to_string(),
        }]);
    }

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && path
                .extension()
                .map_or(false, |ext| ext == "md" || ext == "rs")
        {
            let file_name = path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");

            let id = format!("{}-{}", category, file_name);

            // Read file content
            let mut content = String::new();
            File::open(&path)?.read_to_string(&mut content)?;

            // Extract title from first line or use filename
            let title = content
                .lines()
                .next()
                .map(|line| line.trim_start_matches('#').trim().to_string())
                .unwrap_or_else(|| file_name.to_string());

            items.push(ContentItem {
                id,
                title,
                content,
                category: category.to_string(),
                path: path.to_string_lossy().to_string(),
            });
        }
    }

    Ok(items)
}
