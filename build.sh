#!/bin/bash
set -e

echo "Building Turn-Formal..."

# Compile Rust content to JSON
echo "Compiling content to JSON..."
cargo run --bin export_math_content

# Build React frontend
echo "Building React frontend..."
cd frontend && npm run build

# Exporting rust bindings to typescript
echo "Exporting rust bindings to typescript..."
cargo test export_bindings  

echo "Build complete! The application is ready in frontend/dist" 