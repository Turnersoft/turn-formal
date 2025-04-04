#!/bin/bash
# Development script for the formalize_v2 visualization system

set -e

# Navigate to the project root directory
cd "$(dirname "$0")/../../.."
ROOT_DIR=$(pwd)

# Visualization directory
VISUALIZATION_DIR="$ROOT_DIR/src/formalize_v2/visualization"

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

# Parse command line arguments
PORT=8080
CLEAN=false
RELEASE=false

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
    -r|--release)
      RELEASE=true
      shift
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: $0 [-p|--port PORT] [-c|--clean] [-r|--release]"
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

# Build additional arguments based on flags
TRUNK_ARGS="--features theorem_visualizer --port $PORT"

if [ "$RELEASE" = true ]; then
    TRUNK_ARGS="$TRUNK_ARGS --release"
fi

# Run trunk serve with the theorem_visualizer feature
echo "Starting Trunk development server with hot reload on port $PORT..."
echo "Press Ctrl+C to stop the server"

# Start Trunk with the appropriate arguments
trunk serve $TRUNK_ARGS

# Clean up the symbolic link when we're done
rm -f "$VISUALIZATION_DIR/Cargo.toml" 