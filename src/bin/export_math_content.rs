use std::env;
use std::path::Path;
use turn_formal::subjects::math::export::UnifiedExporter;
use turn_formal::subjects::math::theories::groups::theorems::register_basic_group_axioms;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut output_dir = "."; // Default to current directory
    let mut all_theories = false;

    for i in 1..args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--all-theories" => all_theories = true,
            "--definitions-only" => {
                // This is now the default behavior - no separate theorem processing
            }
            arg if !arg.starts_with("--") => {
                // If the argument ends with .json, treat it as a file and use its directory
                if arg.ends_with(".json") {
                    output_dir = Path::new(arg)
                        .parent()
                        .unwrap_or(Path::new("."))
                        .to_str()
                        .unwrap();
                } else {
                    // Otherwise treat it as a directory
                    output_dir = arg;
                }
            }
            _ => {
                eprintln!("❌ Unknown argument: {}", args[i]);
                print_help();
                std::process::exit(1);
            }
        }
    }

    println!("🚀 Unified Mathematical Content Exporter");
    println!("========================================");
    println!("📁 Output directory: {}", output_dir);
    println!("🌍 All theories: {}", all_theories);
    println!("");

    // Register basic group axioms BEFORE exporting content
    println!("📚 Registering basic group axioms...");
    register_basic_group_axioms();
    println!("✅ Group axioms registered successfully");
    println!("");

    // Export mathematical content using the unified system
    if all_theories {
        // Export ALL available theories using the unified exporter
        match UnifiedExporter::export_all_theories_to_directory(output_dir) {
            Ok(()) => {
                println!("🎉 COMPLETE Multi-Theory Export Finished!");
                println!("📁 All available theories exported to: {}", output_dir);
            }
            Err(e) => {
                eprintln!("❌ Error exporting all theories: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("");
    println!("🚀 Ready for frontend consumption:");
    if all_theories {
        println!("   Load individual theory files as needed");
        println!("   Use manifest.json to discover available content");
        println!("");
        println!("💡 Pro tip: Check manifest.json to see what was actually exported!");
    } else {
        println!("   Load the single group theory definitions file");
    }
}

fn print_help() {
    println!("🚀 Unified Mathematical Content Exporter");
    println!("");
    println!("USAGE:");
    println!("    cargo run --bin export_math_content [OPTIONS] [OUTPUT_DIR]");
    println!("");
    println!("ARGUMENTS:");
    println!("    <OUTPUT_DIR>     Output directory [default: current directory]");
    println!("                     Can also be a .json file path (uses its directory)");
    println!("");
    println!("OPTIONS:");
    println!("    --all-theories     Export ALL available mathematical theories (recommended!)");
    println!(
        "    --definitions-only (Default behavior - exports definitions, constructors, theorems)"
    );
    println!("    -h, --help         Print this help message");
    println!("");
    println!("EXAMPLES:");
    println!("    # 🎯 RECOMMENDED: Export ALL available theories to organized files");
    println!("    cargo run --bin export_math_content -- --all-theories frontend/public/");
    println!("    # Creates files dynamically based on available theories:");
    println!("    #   frontend/public/manifest.json");
    println!("    #   frontend/public/[theory_id]_l1_definitions.json");
    println!("    #   frontend/public/[theory_id]_l3_constructors.json");
    println!("    #   frontend/public/[theory_id]_theorems.json");
    println!("");
    println!("    # Legacy: Only Group Theory definitions");
    println!("    cargo run --bin export_math_content output_dir/");
    println!("");
    println!("    # Using .json file path (uses the directory)");
    println!("    cargo run --bin export_math_content -- --all-theories frontend/public/math.json");
}
