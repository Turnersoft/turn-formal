//! Type Theory Framework (v2)
//!
//! A comprehensive implementation of dependent type theory with support for:
//! - Multiple logical calculi (λ→, λ2, λω, λP, λC)
//! - Inductive types and families
//! - Universe polymorphism
//! - Higher-order unification
//! - Proof automation
//!
//! # Module Structure
//!
//! ```text
//! type_theory_v2/
//! ├── core/               # Core type theory components
//! │   ├── term.rs        # Term representation
//! │   ├── context.rs     # Typing contexts
//! │   ├── judgement.rs   # Type judgements
//! │   ├── reduction.rs   # Reduction strategies
//! │   ├── substitution.rs # Substitution operations
//! │   └── universe.rs    # Universe hierarchy
//! ├── calculi/           # Specific logical calculi
//! │   ├── simply_typed/  # Simply typed lambda calculus (λ→)
//! │   ├── system_f/      # Polymorphic lambda calculus (λ2)
//! │   ├── system_omega/  # Higher-order lambda calculus (λω)
//! │   ├── dependent/     # Dependent type calculus (λP)
//! │   └── constructions/ # Calculus of Constructions (λC)
//! ├── types/             # Type constructors
//! │   ├── base.rs        # Base type operations
//! │   ├── product.rs     # Dependent products (Π)
//! │   ├── sum.rs         # Dependent sums (Σ)
//! │   ├── identity.rs    # Identity types
//! │   ├── inductive.rs   # Inductive families
//! │   ├── coinductive.rs # Coinductive types
//! │   ├── quotient.rs    # Quotient types
//! │   └── hits.rs        # Higher inductive types
//! ├── tactics/           # Proof tactics
//! │   ├── auto.rs        # Automation
//! │   ├── rewrite.rs     # Rewriting
//! │   └── inversion.rs   # Inversion lemmas
//! └── unification/       # Unification engine
//!     ├── constraint.rs  # Constraint solving
//!     └── matching.rs    # Pattern matching
//! ```
//!
//! # Design Principles
//!
//! 1. **Modularity**: Each component is self-contained with clear interfaces
//! 2. **Extensibility**: Easy to add new calculi and type constructors
//! 3. **Safety**: Strong type safety guarantees through Rust's type system
//! 4. **Performance**: Efficient term representation and reduction
//!
//! # Type Hierarchy
//!
//! ```text
//! λC (Calculus of Constructions)
//!  ├── λω (Type-level computation)
//!  │    └── λ2 (Polymorphic types)
//!  │         └── λ→ (Simple types)
//!  └── λP (Term dependencies)
//!       └── λ→ (Simple types)
//! ```

use serde::{Deserialize, Serialize};

pub mod calculi;

pub mod tactics;
pub mod types;
pub mod unification;
