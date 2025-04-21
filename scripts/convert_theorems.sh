#!/bin/bash

# Script to convert theorem JSON files to turn_render format

echo "Converting theorem files to turn_render format..."

# Go to project root
cd "$(dirname "$0")/.."

# Define directories
THEORIES_DIR="subjects/math/theories"
OUTPUT_DIR="subjects/math/theories_turn_render"

# Check if directories exist
if [ ! -d "$THEORIES_DIR" ]; then
  echo "Error: Input directory '$THEORIES_DIR' not found!"
  exit 1
fi

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Run the converter
if [ -f "target/debug/convert_theorems" ]; then
  # Use the debug build if available
  ./target/debug/convert_theorems
elif [ -f "target/release/convert_theorems" ]; then
  # Use the release build if available
  ./target/release/convert_theorems
else
  # Build and run
  echo "Building converter tool..."
  cargo run --bin convert_theorems
fi

echo "Done!" 