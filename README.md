# Turn-Formal

A unified formal framework for mathematics, logic, law, and algorithms verified using various foundational theories.

## Overview

Turn-Formal is a project for developing and exploring formal systems across multiple disciplines, with a foundation-agnostic approach. It allows formal reasoning to be verified across different foundational theories such as Type Theory, Set Theory, and Category Theory.

Key capabilities:

- **Foundation-Agnostic Formalism**: Work with math definitions and proofs that can be interpreted across multiple foundations
- **Multiple Subject Domains**: Support for mathematics, logic, legal reasoning, and algorithmic verification
- **Rigorous Verification**: All formalisms can be automatically verified and checked for consistency

## Architecture

Turn-Formal uses a hybrid architecture:

- **React Frontend**: Modern UI for interacting with formal systems
- **Rust Core**: Strong typing and rigorous verification for formal systems

## Development

### Prerequisites

- Rust (stable channel)
- Node.js (v16 or higher)
- npm (v7 or higher)

### Setup

1. Clone the repository
2. Install frontend dependencies: `cd frontend && npm install`

### Development Workflow

1. Process content files to JSON: `cargo run --bin content_compiler`
2. Start the development server: `cd frontend && npm run dev`

Changes to content in the Rust directories (foundational_theories/, subjects/) will need to be recompiled to JSON using the content compiler.

### Building for Production

```
./build.sh
```

This script will:

1. Compile content from Rust codebase to JSON
2. Build the React frontend
3. Output the final application to `frontend/dist`

## Content Structure

- `foundational_theories/`: Contains formal definitions of foundational theories like type theory and category theory
- `subjects/`: Contains formal definitions for subject domains (math, logic, law, algorithms)

## License

[LICENSE](LICENSE)

## Sub-Repository Setup

This is a standalone Git repository that can be used independently or as a component of a larger project.
To use it as a Git submodule in a larger project, run:

```bash
git submodule add https://github.com/yourusername/formalize_v2.git path/to/formalize_v2
git submodule update --init --recursive
```
