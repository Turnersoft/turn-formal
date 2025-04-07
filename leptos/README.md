# Turn-Formal - Leptos Frontend

This is a visualization interface for the Turn-Formal project, built with Leptos.

## Running the App

Make sure you have Trunk and Rust installed, then:

```bash
# Navigate to the project root
cd /path/to/formalize_v2

# Install trunk if you don't have it
cargo install trunk

# Run the development server
trunk serve --config Trunk.toml
```

This will start the development server at http://127.0.0.1:8081.

## Project Structure

- `src/main.rs` - Entry point for development mode
- `src/lib.rs` - WASM entry point for production builds
- `src/components/` - Reusable UI components
- `src/pages/` - Page components
- `index.html` - HTML template
- `styles.css` - Global styles
