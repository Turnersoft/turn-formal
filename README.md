# Mathematical Proof Tactics System

This module provides a comprehensive tactic system for mathematical proof construction and manipulation.

## Overview

The system provides **one clean way** to apply tactics: directly on `ProofNode` instances using `apply_tactic()`.

## Core Components

### Single Application Method

```rust
// The ONLY way to apply tactics
let new_node = proof_node.apply_tactic(tactic, &mut forest);
```

### Key Components

- **`ProofNode`**: Main proof state container with single tactic application method
- **`Tactic`**: Enumeration of all available tactics (Intro, Apply, Substitution, etc.)
- **`TacticApplier`** and **`TacticMatcher`**: Clean trait-based implementation
- **`SearchReplace`**: Powerful search and replace engine for expressions
- **`TheoremApplier`**: Specialized theorem application with pattern matching

## Usage Examples

### Basic Tactic Application

```rust
use crate::subjects::math::formalism::proof::tactics::*;

// Create a tactic
let tactic = Tactic::Intro {
    name: Identifier::Name("x".to_string(), 0),
    expression: some_expression,
    view: None,
};

// Apply it (single method)
let result_node = node.apply_tactic(tactic, &mut forest);
```

### Available Tactics

- **Intro**: Introduce variables/hypotheses
- **Apply**: Apply theorems/hypotheses  
- **Substitution**: Replace expressions
- **TheoremApplication**: Apply registered theorems
- **Rewrite**: Rewrite using equations
- **ChangeView**: Change mathematical perspective
- **Decompose**: Break down complex expressions
- **CaseAnalysis**: Analyze different cases
- **Induction**: Mathematical induction
- **Simplify**: Simplify expressions

## Architecture

The system uses a clean trait-based design:

1. **`TacticMatcher`**: Finds applicable locations and patterns
2. **`TacticApplier`**: Transforms proof goals based on tactic logic  
3. **Single Application Point**: `ProofNode::apply_tactic()` handles all result types

## Result Handling

The system returns different result types through `TacticApplicationResult`:

- **SingleGoal**: Creates one new proof node
- **MultipleGoals**: Creates multiple child nodes (CaseAnalysis, Decompose, Induction)
- **NoChange**: Tactic not applicable
- **Error**: Application failed with error message

## Design Principles

- **Simplicity**: One method for all tactic applications
- **Elegance**: Clean trait-based architecture
- **Completeness**: Handles all mathematical proof scenarios
- **Extensibility**: Easy to add new tactics following the pattern

The system achieves the original goal of "naive and elegant search and replace logic" through a unified, trait-based approach that keeps the API simple while providing powerful functionality underneath.

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
