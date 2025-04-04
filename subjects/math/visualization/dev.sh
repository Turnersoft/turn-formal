#!/bin/bash
# Development script for the Theorem Visualizer

set -e

# Navigate to the project root directory
cd "$(dirname "$0")/../../../../../"
ROOT_DIR=$(pwd)

# Visualization directory
VISUALIZATION_DIR="$ROOT_DIR/src/formalize_v2/subjects/math/visualization"

# Check if trunk is installed
if ! command -v trunk &> /dev/null
then
    echo "Trunk is not installed. Installing now..."
    cargo install trunk
fi

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null
then
    echo "wasm-bindgen-cli is not installed. Installing now..."
    cargo install wasm-bindgen-cli
fi

# Check for required dependencies in Cargo.toml
if ! grep -q "console_log" Cargo.toml || ! grep -q "console_error_panic_hook" Cargo.toml
then
    echo "Warning: Some required dependencies might be missing in Cargo.toml"
    echo "Make sure console_log and console_error_panic_hook are enabled in the theorem_visualizer feature"
fi

# Parse command line arguments
PORT=8080
CLEAN=false

while [[ "$#" -gt 0 ]]; do
  case $1 in
    -p|--port)
      PORT="$2"
      shift 2
      ;;
    -c|--clean)
      CLEAN=true
      shift
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Clean the dist directory if requested
if [ "$CLEAN" = true ]; then
    echo "Cleaning dist directory..."
    rm -rf "$VISUALIZATION_DIR/dist"
fi

# Create a symbolic link to Cargo.toml in the visualization directory
echo "Setting up Trunk environment..."
ln -sf "$ROOT_DIR/Cargo.toml" "$VISUALIZATION_DIR/Cargo.toml"

# Go to the visualization directory
cd "$VISUALIZATION_DIR"

# Run trunk serve with the theorem_visualizer feature
echo "Starting Trunk development server with hot reload on port $PORT..."
echo "Press Ctrl+C to stop the server"

# Start Trunk with the theorem_visualizer feature
trunk serve --features theorem_visualizer --port "$PORT" --watch-path="$ROOT_DIR/src"

# Clean up the symbolic link when we're done
rm -f "$VISUALIZATION_DIR/Cargo.toml"

# Note: If you're seeing any errors, you might need to install trunk:
# cargo install trunk 