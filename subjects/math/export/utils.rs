use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

/// Helper struct to hold extracted documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeWithDocs {
    /// Name of the type
    pub name: String,
    /// Documentation comments
    pub docs: String,
    /// Type of the definition (struct, enum)
    pub kind: String,
    /// Fields or variants with their documentation
    pub members: Vec<MemberWithDocs>,
}

/// Helper struct to hold extracted field/variant documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberWithDocs {
    /// Name of the field or variant
    pub name: String,
    /// Documentation comments
    pub docs: String,
    /// Type information (for struct fields)
    pub type_info: Option<String>,
}

/// Extracts all types from a Rust source file with their documentation
#[crabtime::function]
pub fn extract_types_with_docs(source_code: &str) -> String {
    #![dependency(syn = { version = "2.0", features = ["full", "extra-traits"] })]
    #![dependency(quote = "1.0")]
    #![dependency(proc-macro2 = "1.0")]
    #![dependency(serde = "1.0")]
    #![dependency(serde_json = "1.0")]

    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use syn::{Attribute, Fields, Item, ItemEnum, ItemStruct, Variant, parse_file};

    // Helper to extract doc comments from attributes
    fn extract_docs(attrs: &[Attribute]) -> String {
        attrs
            .iter()
            .filter_map(|attr| {
                if attr.path().is_ident("doc") {
                    attr.meta.require_name_value().ok().and_then(|meta| {
                        if let syn::Expr::Lit(expr_lit) = &meta.value {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                return Some(lit_str.value().trim().to_string());
                            }
                        }
                        None
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    let file = parse_file(source_code).expect("Failed to parse file");
    let mut types = Vec::new();

    for item in file.items {
        match item {
            Item::Struct(s) => {
                let docs = extract_docs(&s.attrs);
                let mut members = Vec::new();

                match &s.fields {
                    Fields::Named(fields) => {
                        for field in &fields.named {
                            if let Some(name) = &field.ident {
                                let field_docs = extract_docs(&field.attrs);
                                let type_str = format!("{:?}", field.ty);
                                members.push(MemberWithDocs {
                                    name: name.to_string(),
                                    docs: field_docs,
                                    type_info: Some(type_str),
                                });
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        for (i, field) in fields.unnamed.iter().enumerate() {
                            let field_docs = extract_docs(&field.attrs);
                            let type_str = format!("{:?}", field.ty);
                            members.push(MemberWithDocs {
                                name: format!("_{}", i),
                                docs: field_docs,
                                type_info: Some(type_str),
                            });
                        }
                    }
                    Fields::Unit => {}
                }

                types.push(TypeWithDocs {
                    name: s.ident.to_string(),
                    docs,
                    kind: "struct".to_string(),
                    members,
                });
            }
            Item::Enum(e) => {
                let docs = extract_docs(&e.attrs);
                let mut members = Vec::new();

                for variant in &e.variants {
                    let variant_docs = extract_docs(&variant.attrs);
                    let mut variant_desc = String::new();

                    match &variant.fields {
                        Fields::Named(fields) => {
                            write!(variant_desc, "{{ ").unwrap();
                            for field in &fields.named {
                                if let Some(name) = &field.ident {
                                    write!(variant_desc, "{}: {:?}, ", name, field.ty).unwrap();
                                }
                            }
                            write!(variant_desc, "}}").unwrap();
                        }
                        Fields::Unnamed(fields) => {
                            write!(variant_desc, "(").unwrap();
                            for field in &fields.unnamed {
                                write!(variant_desc, "{:?}, ", field.ty).unwrap();
                            }
                            write!(variant_desc, ")").unwrap();
                        }
                        Fields::Unit => {}
                    }

                    members.push(MemberWithDocs {
                        name: variant.ident.to_string(),
                        docs: variant_docs,
                        type_info: if !variant_desc.is_empty() {
                            Some(variant_desc)
                        } else {
                            None
                        },
                    });
                }

                types.push(TypeWithDocs {
                    name: e.ident.to_string(),
                    docs,
                    kind: "enum".to_string(),
                    members,
                });
            }
            _ => {}
        }
    }

    // Convert to JSON
    let json = serde_json::to_string_pretty(&types).expect("Failed to serialize to JSON");
    format!("r#\"{}\"#", json)
}

/// Utility to generate documentation-preserving JSON from a Rust source file
pub fn generate_math_json(file_path: &str) -> anyhow::Result<String> {
    use std::fs;
    use std::path::Path;

    // Read the source file
    let source_code = fs::read_to_string(file_path)?;

    // Extract definitions to JSON
    let json = extract_definitions_to_json(&source_code);

    Ok(json)
}

/// Extract Rust type definitions to JSON format
pub fn extract_definitions_to_json(source: &str) -> String {
    let mut types = Vec::new();

    // Type extraction state
    let mut current_docs = String::new();
    let mut current_type = None;
    let mut in_struct_or_enum = false;
    let mut brace_count = 0;
    let mut field_docs = String::new();
    let mut members = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();

        // Collect docs
        if trimmed.starts_with("///") {
            let doc_content = trimmed.trim_start_matches("///").trim();

            if in_struct_or_enum && brace_count == 1 {
                // Field documentation
                if !field_docs.is_empty() {
                    field_docs.push('\n');
                }
                field_docs.push_str(doc_content);
            } else {
                // Type documentation
                if !current_docs.is_empty() {
                    current_docs.push('\n');
                }
                current_docs.push_str(doc_content);
            }
            continue;
        }

        // Look for struct or enum definitions
        if trimmed.starts_with("pub struct") || trimmed.starts_with("pub enum") {
            let kind = if trimmed.starts_with("pub struct") {
                "struct"
            } else {
                "enum"
            };

            // Extract the name
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[2].trim_end_matches(" {");
                current_type = Some((name.to_string(), kind.to_string(), current_docs.clone()));
                current_docs.clear();
                in_struct_or_enum = true;
                brace_count = 1;
                members = Vec::new();
            }
            continue;
        }

        // Track braces
        if in_struct_or_enum {
            brace_count += trimmed.chars().filter(|&c| c == '{').count();
            brace_count -= trimmed.chars().filter(|&c| c == '}').count();

            // Process struct field
            if brace_count == 1 && trimmed.contains(':') && trimmed.starts_with("pub") {
                let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let field_name = parts[0].trim().trim_start_matches("pub").trim();
                    let field_type = parts[1].trim().trim_end_matches(',');

                    members.push(format!(
                        r#"{{"name":"{}","docs":"{}","type":"{}"}}"#,
                        field_name,
                        field_docs.replace("\"", "\\\"").replace("\n", "\\n"),
                        field_type
                    ));
                    field_docs.clear();
                }
            }

            // Process enum variant
            if brace_count == 1
                && !trimmed.contains(':')
                && trimmed.ends_with(',')
                && !trimmed.starts_with("//")
                && !trimmed.is_empty()
            {
                let variant_name = trimmed.trim_end_matches(',').trim();

                members.push(format!(
                    r#"{{"name":"{}","docs":"{}"}}"#,
                    variant_name,
                    field_docs.replace("\"", "\\\"").replace("\n", "\\n")
                ));
                field_docs.clear();
            }

            // End of type definition
            if brace_count == 0 {
                in_struct_or_enum = false;
                if let Some((name, kind, docs)) = current_type.take() {
                    let members_json = if members.is_empty() {
                        "[]".to_string()
                    } else {
                        format!("[\n      {}\n    ]", members.join(",\n      "))
                    };

                    types.push(format!(
                        r#"{{"name":"{}","kind":"{}","docs":"{}","members":{}}}"#,
                        name,
                        kind,
                        docs.replace("\"", "\\\"").replace("\n", "\\n"),
                        members_json
                    ));
                    members.clear();
                }
            }
        }
    }

    format!("[\n{}\n]", types.join(",\n"))
}

/// Creates a TypeScript type definition file from Rust definitions
pub fn generate_typescript_types(file_path: &str) -> anyhow::Result<String> {
    use std::collections::HashMap;
    use std::fs;

    // Read the source file
    let source_code = fs::read_to_string(file_path)?;

    // Extract definitions
    let type_info = extract_types(source_code)?;

    // Generate TypeScript code
    let mut ts_code = String::new();
    ts_code.push_str("// Auto-generated TypeScript types with documentation\n\n");

    for type_def in type_info {
        // Add JSDoc comment
        ts_code.push_str("/**\n");
        for line in type_def.docs.lines() {
            ts_code.push_str(&format!(" * {}\n", line));
        }
        ts_code.push_str(" */\n");

        if type_def.kind == "enum" {
            ts_code.push_str(&format!("export enum {} {{\n", type_def.name));

            for member in &type_def.members {
                if !member.docs.is_empty() {
                    ts_code.push_str("  /**\n");
                    for line in member.docs.lines() {
                        ts_code.push_str(&format!("   * {}\n", line));
                    }
                    ts_code.push_str("   */\n");
                }

                ts_code.push_str(&format!("  {} = \"{}\",\n", member.name, member.name));
            }

            ts_code.push_str("}\n\n");
        } else {
            ts_code.push_str(&format!("export interface {} {{\n", type_def.name));

            for member in &type_def.members {
                if !member.docs.is_empty() {
                    ts_code.push_str("  /**\n");
                    for line in member.docs.lines() {
                        ts_code.push_str(&format!("   * {}\n", line));
                    }
                    ts_code.push_str("   */\n");
                }

                let ts_type = member
                    .type_info
                    .as_ref()
                    .map(|t| map_rust_type_to_ts(t))
                    .unwrap_or_else(|| "any".to_string());

                ts_code.push_str(&format!("  {}: {};\n", member.name, ts_type));
            }

            ts_code.push_str("}\n\n");
        }
    }

    Ok(ts_code)
}

/// Simple parser to extract Rust types and their documentation
fn extract_types(source: String) -> anyhow::Result<Vec<TypeWithDocs>> {
    let mut types = Vec::new();
    let mut current_docs = String::new();
    let mut current_type: Option<(String, String, String)> = None;
    let mut current_members = Vec::new();
    let mut in_struct_or_enum = false;
    let mut brace_count = 0;
    let mut field_docs = String::new();

    for line in source.lines() {
        let trimmed = line.trim();

        // Collect doc comments
        if trimmed.starts_with("///") {
            let doc_content = trimmed.trim_start_matches("///").trim();

            if in_struct_or_enum && brace_count == 1 && !trimmed.contains("Unique identifier") {
                // Field documentation
                if !field_docs.is_empty() {
                    field_docs.push('\n');
                }
                field_docs.push_str(doc_content);
            } else {
                // Type documentation
                if !current_docs.is_empty() {
                    current_docs.push('\n');
                }
                current_docs.push_str(doc_content);
            }
            continue;
        }

        // Look for struct or enum definitions
        if trimmed.starts_with("pub struct") || trimmed.starts_with("pub enum") {
            let kind = if trimmed.starts_with("pub struct") {
                "struct"
            } else {
                "enum"
            };

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[2].trim_end_matches(" {");
                current_type = Some((name.to_string(), kind.to_string(), current_docs.clone()));
                current_docs.clear();
                in_struct_or_enum = true;
                brace_count = 1; // We've entered the first brace
                current_members = Vec::new();
            }
            continue;
        }

        // Track braces to know when we enter/exit struct bodies
        if in_struct_or_enum {
            brace_count += trimmed.chars().filter(|&c| c == '{').count() as i32;
            brace_count -= trimmed.chars().filter(|&c| c == '}').count() as i32;

            // Parse field
            if brace_count == 1 && trimmed.contains(':') && trimmed.starts_with("pub") {
                // This is likely a struct field
                let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let field_name = parts[0].trim().trim_start_matches("pub").trim();
                    let field_type = parts[1].trim().trim_end_matches(',');

                    current_members.push(MemberWithDocs {
                        name: field_name.to_string(),
                        docs: field_docs.clone(),
                        type_info: Some(field_type.to_string()),
                    });
                    field_docs.clear();
                }
            } else if brace_count == 1
                && trimmed.ends_with(',')
                && !trimmed.contains(':')
                && !trimmed.starts_with("//")
            {
                // This is likely an enum variant
                let variant_name = trimmed.trim_end_matches(',');
                current_members.push(MemberWithDocs {
                    name: variant_name.to_string(),
                    docs: field_docs.clone(),
                    type_info: None,
                });
                field_docs.clear();
            }

            // End of type definition
            if brace_count == 0 {
                in_struct_or_enum = false;
                if let Some((name, kind, docs)) = current_type.take() {
                    types.push(TypeWithDocs {
                        name,
                        docs,
                        kind,
                        members: current_members.clone(),
                    });
                    current_members.clear();
                }
            }
        }
    }

    Ok(types)
}

/// Helper function to map Rust types to TypeScript types
fn map_rust_type_to_ts(rust_type: &str) -> String {
    // This is a simplified mapping, a real implementation would be more robust
    if rust_type.contains("String") {
        "string".to_string()
    } else if rust_type.contains("i32")
        || rust_type.contains("u32")
        || rust_type.contains("f32")
        || rust_type.contains("i64")
        || rust_type.contains("u64")
        || rust_type.contains("f64")
    {
        "number".to_string()
    } else if rust_type.contains("bool") {
        "boolean".to_string()
    } else if rust_type.contains("Vec<") {
        let inner_type = rust_type
            .split('<')
            .nth(1)
            .unwrap_or("unknown")
            .trim_end_matches('>')
            .trim();
        format!("{}[]", map_rust_type_to_ts(inner_type))
    } else if rust_type.contains("Option<") {
        let inner_type = rust_type
            .split('<')
            .nth(1)
            .unwrap_or("unknown")
            .trim_end_matches('>')
            .trim();
        format!("{} | null", map_rust_type_to_ts(inner_type))
    } else if rust_type.contains("HashMap<") || rust_type.contains("BTreeMap<") {
        // Extract key and value types
        let parts: Vec<&str> = rust_type
            .split('<')
            .nth(1)
            .unwrap_or("String, unknown")
            .trim_end_matches('>')
            .split(',')
            .collect();

        if parts.len() >= 2 {
            let key_type = parts[0].trim();
            let value_type = parts[1].trim();

            // Only string keys are directly supported in TypeScript objects
            if key_type.contains("String") {
                format!("{{ [key: string]: {} }}", map_rust_type_to_ts(value_type))
            } else {
                "Record<string, any>".to_string()
            }
        } else {
            "Record<string, any>".to_string()
        }
    } else {
        // For custom types, preserve the name
        rust_type
            .trim_start_matches("&")
            .trim_start_matches("Box<dyn ")
            .trim_end_matches('>')
            .to_string()
    }
}

/// A struct to represent a Rust type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    /// The name of the type (struct or enum)
    pub name: String,
    /// Documentation comments for the type
    pub docs: String,
    /// The source code of the type definition
    pub source: String,
    /// The kind of type (struct, enum, etc.)
    pub kind: String,
}

/// Extract Rust types from source code as JSON
pub fn extract_rust_types_as_json(path: &str) -> Result<Vec<TypeDefinition>> {
    let source = fs::read_to_string(path)?;
    extract_types_from_source(&source)
}

/// Extract type definitions from Rust source code
pub fn extract_types_from_source(source: &str) -> Result<Vec<TypeDefinition>> {
    let mut types = Vec::new();
    let mut current_docs = String::new();
    let lines: Vec<&str> = source.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Collect documentation comments
        if line.starts_with("///") {
            current_docs.push_str(&line[3..].trim());
            current_docs.push('\n');
            i += 1;
            continue;
        }

        // Check for struct or enum definitions
        if (line.starts_with("pub struct")
            || line.starts_with("struct")
            || line.starts_with("pub enum")
            || line.starts_with("enum"))
            && !line.contains(";")
        {
            // Skip type aliases

            let kind = if line.contains("struct") {
                "struct"
            } else {
                "enum"
            };

            // Extract the type name
            let name_start = line.find(kind).unwrap() + kind.len();
            let mut name_end = line.len();

            // Find where the name ends (before generic params or opening brace)
            if let Some(pos) = line[name_start..].find('<') {
                name_end = name_start + pos;
            } else if let Some(pos) = line[name_start..].find('{') {
                name_end = name_start + pos;
            } else if let Some(pos) = line[name_start..].find("where") {
                name_end = name_start + pos;
            }

            let name = line[name_start..name_end].trim().to_string();

            // Capture the full type definition
            let mut source = String::new();
            source.push_str(line);

            let mut brace_count =
                line.matches('{').count() as i32 - line.matches('}').count() as i32;
            let mut j = i + 1;

            // Continue capturing until we've closed all braces
            while j < lines.len() && brace_count > 0 {
                let next_line = lines[j];
                source.push('\n');
                source.push_str(next_line);

                brace_count += next_line.matches('{').count() as i32;
                brace_count -= next_line.matches('}').count() as i32;

                j += 1;
            }

            types.push(TypeDefinition {
                name,
                docs: current_docs.clone(),
                source,
                kind: kind.to_string(),
            });

            // Reset docs for the next type
            current_docs.clear();

            // Skip ahead if we found the end of the type definition
            if j > i + 1 {
                i = j;
                continue;
            }
        }

        // If not a doc comment or type definition, reset collected docs
        if !line.is_empty() && !line.starts_with("//") {
            current_docs.clear();
        }

        i += 1;
    }

    Ok(types)
}

/// Find Rust files in a directory that contain a specific derive pattern
pub fn find_rust_files_with_derive(dir_path: &str, derive_pattern: &str) -> Result<Vec<String>> {
    let mut rust_files = Vec::new();

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(path)?;

            // Check if the file contains the derive pattern
            if content.contains(&format!("derive({}", derive_pattern))
                || content.contains(&format!("derive({})", derive_pattern))
                || content.contains(&format!("derive( {}", derive_pattern))
                || content.contains(&format!("derive( {} )", derive_pattern))
            {
                rust_files.push(path.to_string_lossy().to_string());
            }
        }
    }

    Ok(rust_files)
}

/// Generate TypeScript definitions from a Rust file
pub fn generate_typescript_from_rust(rust_file: &str) -> Result<String> {
    let types = extract_rust_types_as_json(rust_file)?;

    let mut typescript = String::new();
    typescript.push_str("// Generated from Rust type definitions\n\n");

    for type_def in types {
        // Add JSDoc comment if there's documentation
        if !type_def.docs.trim().is_empty() {
            typescript.push_str("/**\n");
            for doc_line in type_def.docs.lines() {
                typescript.push_str(&format!(" * {}\n", doc_line.trim()));
            }
            typescript.push_str(" */\n");
        }

        if type_def.kind == "struct" {
            // Convert Rust struct to TypeScript interface
            typescript.push_str(&format!("export interface {} {{\n", type_def.name));

            // Parse the struct fields
            if let Some(fields_start) = type_def.source.find('{') {
                let fields_str = &type_def.source[fields_start + 1..];

                // Simple parsing of fields, can be improved
                for line in fields_str.lines() {
                    let line = line.trim();
                    if line.starts_with("pub ") && line.contains(':') {
                        let field_parts: Vec<&str> = line.splitn(2, ':').collect();
                        if field_parts.len() == 2 {
                            let field_name = field_parts[0].trim().replace("pub ", "");
                            let field_type_str = field_parts[1].trim().trim_end_matches(',');

                            // Convert Rust types to TypeScript types
                            let ts_type = match field_type_str {
                                "String" => "string".to_string(),
                                "bool" => "boolean".to_string(),
                                "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => {
                                    "number".to_string()
                                }
                                "Vec<String>" => "string[]".to_string(),
                                _ if field_type_str.starts_with("Vec<") => {
                                    format!(
                                        "{}[]",
                                        field_type_str
                                            .trim_start_matches("Vec<")
                                            .trim_end_matches('>')
                                    )
                                }
                                _ => field_type_str.to_string(),
                            };

                            typescript.push_str(&format!("  {}: {};\n", field_name, ts_type));
                        }
                    }
                }
            }

            typescript.push_str("}\n\n");
        } else if type_def.kind == "enum" {
            // Convert Rust enum to TypeScript enum or union type
            // Simple version - can be improved for more complex enum handling
            typescript.push_str(&format!("export enum {} {{\n", type_def.name));

            // Parse the enum variants
            if let Some(fields_start) = type_def.source.find('{') {
                let fields_str = &type_def.source[fields_start + 1..];

                for line in fields_str.lines() {
                    let line = line.trim();
                    if !line.is_empty() && !line.starts_with('}') {
                        let variant = line.split(',').next().unwrap_or("").trim();
                        if !variant.is_empty() {
                            // For simple enum variants
                            if !variant.contains('(') && !variant.contains('{') {
                                typescript.push_str(&format!("  {} = \"{}\",\n", variant, variant));
                            }
                            // For complex variants, this is simplified
                        }
                    }
                }
            }

            typescript.push_str("}\n\n");
        }
    }

    Ok(typescript)
}

/// Generate TypeScript definitions for all Rust files in a directory
pub fn generate_typescript_for_directory(dir_path: &str, output_file: &str) -> Result<()> {
    let rust_files = find_rust_files_with_derive(dir_path, "Serialize")?;

    let mut combined_typescript = String::new();
    combined_typescript.push_str("// Generated from Rust type definitions\n\n");

    for rust_file in rust_files {
        let typescript = generate_typescript_from_rust(&rust_file)?;
        combined_typescript.push_str(&typescript);
    }

    fs::write(output_file, combined_typescript)?;
    println!("Generated TypeScript definitions in {}", output_file);

    Ok(())
}

/// Generate a TypeScript file for all mathematical theories
pub fn generate_math_typescript() -> Result<()> {
    let rust_types_dir = "subjects/math/export/types";
    let output_file = "subjects/math/export/math-types.ts";

    generate_typescript_for_directory(rust_types_dir, output_file)
}
