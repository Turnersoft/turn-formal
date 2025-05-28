#!/bin/bash

# 🚀 UNIFIED MATHEMATICAL CONTENT EXPORTER
# Exports ALL mathematical theories to organized JSON files
# This is the SINGLE COMMAND you want to use!

set -e

# Default output directory
OUTPUT_DIR="${1:-frontend/public/}"

# If argument ends with .json, extract the directory
if [[ "$OUTPUT_DIR" == *.json ]]; then
    OUTPUT_DIR=$(dirname "$OUTPUT_DIR")
fi

echo "🌍 Complete Multi-Theory Mathematical Content Exporter"
echo "====================================================="
echo "📁 Output directory: $OUTPUT_DIR"
echo ""

echo "🎯 Exporting ALL available mathematical theories..."
echo "   Discovering and exporting theories dynamically..."
echo ""

# Run the comprehensive Rust exporter with ALL theories
cargo run --bin export_math_content -- --all-theories "$OUTPUT_DIR"

echo ""
echo "✅ COMPLETE EXPORT FINISHED!"
echo ""
echo "📊 Check the following files in $OUTPUT_DIR:"
echo "   📄 manifest.json (shows what was actually exported)"
echo "   📄 [theory_id]_l1_definitions.json (for each available theory)"
echo "   📄 [theory_id]_l3_constructors.json (for theories with constructors)"
echo "   📄 [theory_id]_theorems.json (for theories with theorems)"
echo ""
echo "🚀 Frontend integration:"
echo "   Load content using the individual JSON files"
echo "   Use manifest.json to discover available content"
echo ""
echo "💡 Example frontend usage:"
echo "   const manifest = await fetch('./manifest.json');"
echo "   console.log('Available theories:', manifest.theories);" 