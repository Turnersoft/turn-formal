# Theorem Visualizer

A web-based UI for visualizing mathematical theorems and their proofs using Leptos.

## Development Setup

The Theorem Visualizer uses [Trunk](https://trunkrs.dev/) for development, which provides hot reloading for a better developer experience.

### Prerequisites

- Rust and Cargo
- Trunk: `cargo install trunk`
- wasm-bindgen-cli: `cargo install wasm-bindgen-cli`

### Running the Development Server

To start the development server with hot reloading:

```bash
# Navigate to the visualization directory
cd src/formalize_v2/subjects/math/visualization

# Run the development script
./dev.sh
```

This will start a local development server at http://localhost:8080 with hot reloading.

### Development Script Options

The `dev.sh` script accepts several options:

- `-p, --port PORT`: Specify the port to run the server on (default: 8080)
- `-w, --watch DIRS`: Specify directories to watch for changes (default: current directory)
- `-c, --clean`: Clean the dist directory before starting

Example:
```bash
./dev.sh -p 3000 -w "src,assets" -c
```

## Component Structure

The UI is built using the Leptos framework and consists of the following components:

- `TheoremVisualizerApp`: The main application container
- `TheorySelector`: Component for selecting a mathematical theory
- `TheoremList`: Component for displaying theorems in a selected theory
- `TheoremDetail`: Component for displaying detailed information about a theorem
- `ProofStep`: Component for displaying a step in a proof
- `ProofBranch`: Component for displaying a branch in a proof

## Building for Production

To build the Theorem Visualizer for production:

```bash
# Navigate to the visualization directory
cd src/formalize_v2/subjects/math/visualization

# Build with Trunk
trunk build --release --features theorem_visualizer
```

This will generate optimized files in the `dist` directory.

## Implementation Notes

1. The application uses feature flags to ensure that the Theorem Visualizer code is only compiled when the `theorem_visualizer` feature is enabled.
2. All components are defined within modules that are conditionally compiled with the `#[cfg(feature = "theorem_visualizer")]` attribute.
3. The app imports mathematical theorems and proofs from the core math library.

## Next Steps

1. Implement the actual visualization of theorems and proofs.
2. Add interactive features for exploring proofs.
3. Connect to the theorem database for loading and visualizing content.
4. Improve the styling and layout of the UI. 