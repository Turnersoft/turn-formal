use std::env;
use std::path::Path;
use turn_formal::subjects::math::export::UnifiedExporter;
use turn_formal::subjects::math::formalism::automation::registry::get_theorem_registry;

fn main() {
    // ========================================
    // CONFIGURATION - CHANGE THESE PARAMETERS
    // ========================================

    // Set to true to export ALL available mathematical theories
    let all_theories = true;

    // Set the output directory for exported files
    // Can be a directory path or a .json file path (will use its directory)
    let output_dir = "frontend/public/";

    // ========================================
    // END CONFIGURATION
    // ========================================

    // Process output directory (handle .json file paths)
    let final_output_dir = if output_dir.ends_with(".json") {
        Path::new(output_dir)
            .parent()
            .unwrap_or(Path::new("."))
            .to_str()
            .unwrap()
    } else {
        output_dir
    };

    println!("🚀 Unified Mathematical Content Exporter");
    println!("========================================");
    println!("📁 Output directory: {}", final_output_dir);
    println!("🌍 All theories: {}", all_theories);
    println!("");

    // Initialize the theorem registry by calling it once.
    // All axioms and theorems will be registered automatically.
    println!("📚 Initializing theorem registry...");
    get_theorem_registry();
    println!("✅ Theorem registry initialized.");

    // Export mathematical content using the unified system
    if all_theories {
        // Export ALL available theories using the unified exporter
        match UnifiedExporter::export_all_theories_to_directory(final_output_dir) {
            Ok(()) => {
                println!("🎉 COMPLETE Multi-Theory Export Finished!");
                println!(
                    "📁 All available theories exported to: {}",
                    final_output_dir
                );
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
