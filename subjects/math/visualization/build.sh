#!/bin/bash
# Build script for the Theorem Visualizer

# Navigate to the project root
cd "$(dirname "$0")/../../../../../"

# Build the WebAssembly package
echo "Building the Theorem Visualizer WebAssembly package..."
wasm-pack build --target web --out-dir ./src/formalize_v2/subjects/math/visualization/pkg --features theorem_visualizer

# Copy the index.html to the package directory
echo "Setting up web files..."
cp ./src/formalize_v2/subjects/math/visualization/index.html ./src/formalize_v2/subjects/math/visualization/pkg/

# Start a local web server
echo "Starting local web server..."
cd ./src/formalize_v2/subjects/math/visualization/pkg/
python3 -m http.server 8080

echo "Theorem Visualizer is running at http://localhost:8080" 