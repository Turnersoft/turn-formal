# Theorem Proof Naming Conventions

## Variable Naming for Proof Steps

Throughout the codebase, proof steps follow a specific naming convention:

- `p0` - The initial branch from a theorem builder
- `p1`, `p2`, etc. - Sequential primary proof steps 
- `p1s1`, `p1s2`, etc. - Optional sub-steps within a primary step

Example in [subjects/math/theories/groups/theorems.rs](mdc:subjects/math/theories/groups/theorems.rs):
```rust
// Initial branch
let p0 = builder.initial_branch();

// First main step
let p1 = p0.tactics_intro_expr("assumptions", MathExpression::Var(Identifier::E(51)), 1);

// Second step
let p2 = p1.tactics_subs_expr(g_h1_expr, e_expr.clone(), None, 2);
```

## Proof Structure Components

The proof system employs several key components:

1. **TheoremBuilder** - Creates the initial structure for a theorem proof
2. **ProofBranch** - Represents a specific branch in a proof tree
3. **CaseAnalysisBuilder** - Handles case analysis with multiple branches
4. **Tactic** - Different proof tactics (Intro, Substitution, etc.)

## Branching and Case Analysis

Both branching and case analysis should use a chained method approach:

### Case Analysis

```rust
let result = p0
    .case_analysis()
    .on_expression("direction")
    .case("case 1", |branch| {
        let p1 = branch.tactics_intro_expr(...);
        // Steps for case 1
        p9.should_complete()
    })
    .case("case 2", |branch| {
        let p1 = branch.tactics_intro_expr(...);
        // Steps for case 2
        p7.should_complete()
    })
    .build();
```

### Regular Branching

Branching should be implemented with a similar chained method approach:

```rust
// Proposed improvement to the API
let result = p0
    .branch_analysis()
    .branch("main approach", |branch| {
        let p1 = branch.tactics_intro_expr(...);
        // Steps for main branch
        p5.should_complete()
    })
    .branch("alternative approach", |branch| {
        let p1 = branch.tactics_intro_expr(...);
        // Steps for alternative branch
        p3.should_complete()
    })
    .build();
```

This chained approach creates a consistent pattern between case analysis and branching, making the proof structure clearer and more maintainable.

## Path IDs and Justifications

Each proof step maintains an internal path ID that tracks its position in the proof tree. Path IDs are critical for proper case analysis and branching, and need to be preserved when manipulating the proof state.