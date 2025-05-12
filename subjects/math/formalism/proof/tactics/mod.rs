pub mod case_analysis;
pub mod risch;
pub mod tactic_impl;
pub mod tactic_types;
pub mod theorem_applier;
pub mod unification;
pub mod utils;

// Re-export the items so external code can continue to use them
pub use case_analysis::{CaseAnalysisBuilder, CaseResult};
pub use tactic_impl::legacy_apply;
pub use tactic_types::{DecompositionMethod, InductionType, RewriteDirection, Tactic};
pub use theorem_applier::{TheoremApplicationError, TheoremApplicationResult, TheoremApplier};
pub use unification::{
    UnificationContext, UnificationError, apply_instantiations, apply_instantiations_to_relation,
    unify,
};
pub use utils::{create_expr, expression_summary, name_to_string};

// Re-export only public functions from parent
pub use super::ProofForest;
pub use super::ProofNode;
pub use super::TheoremRegistry;
pub use super::get_theorem_registry;
