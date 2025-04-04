#!/bin/bash
# Check script for the Theorem Visualizer

# Navigate to the project root
cd "$(dirname "$0")/../../../../../"

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed. Please install it with 'cargo install wasm-pack'"
    exit 1
fi

# Verify that the features compile correctly
echo "Checking Theorem Visualizer compilation..."
cargo check --features theorem_visualizer

# Build the WebAssembly package
echo "Building WebAssembly package (this may take a moment)..."
wasm-pack build --target web --out-dir ./src/formalize_v2/subjects/math/visualization/pkg --features theorem_visualizer

echo "Build check completed successfully!"
echo "To run the visualizer, use: ./src/formalize_v2/subjects/math/visualization/build.sh" 