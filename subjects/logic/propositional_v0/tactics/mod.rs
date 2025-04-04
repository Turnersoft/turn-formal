use super::{Foundation, GenericProof, Proposition};
use std::marker::PhantomData;

/// Result of applying a tactic
pub type TacticResult<T> = Result<T, TacticError>;

/// Errors that can occur during tactic application
#[derive(Debug)]
pub enum TacticError {
    NoMatchingRule,
    InvalidPremises(String),
    RuleApplicationFailed(String),
    FoundationError(String),
    GoalMismatch {
        expected: Proposition,
        actual: Proposition,
    },
    PropositionMismatch {
        expected: String,
        actual: String,
    },
    VerificationError(String),
    TypeMismatch,
}

impl From<String> for TacticError {
    fn from(error: String) -> Self {
        TacticError::RuleApplicationFailed(error)
    }
}

impl std::error::Error for TacticError {}

impl std::fmt::Display for TacticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TacticError::NoMatchingRule => write!(f, "No matching rule found"),
            TacticError::InvalidPremises(msg) => write!(f, "Invalid premises: {}", msg),
            TacticError::RuleApplicationFailed(msg) => {
                write!(f, "Rule application failed: {}", msg)
            }
            TacticError::FoundationError(msg) => write!(f, "Foundation error: {}", msg),
            TacticError::GoalMismatch { expected, actual } => write!(
                f,
                "Goal mismatch: expected {:?}, got {:?}",
                expected, actual
            ),
            TacticError::PropositionMismatch { expected, actual } => write!(
                f,
                "Proposition mismatch: expected {}, got {}",
                expected, actual
            ),
            TacticError::VerificationError(msg) => write!(f, "Verification error: {}", msg),
            TacticError::TypeMismatch => write!(f, "Type mismatch"),
        }
    }
}

/// A tactic that can be applied to a proof state to produce a new proof state
pub trait ProofStateTactic {
    type Error;

    /// Apply the tactic to the current proof state
    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error>;
}
pub mod basic;
pub mod builder;
pub mod combinators;
pub mod composition;
pub mod context;
pub mod proof_state;
pub mod tests;

pub use basic::{
    AndElimTactic, AndIntroTactic, FalseElimTactic, ImpliesElimTactic, ImpliesIntroTactic,
    NotElimTactic, NotIntroTactic, OrElimTactic, OrIntroTactic, TrueIntroTactic,
};
use proof_state::ProofState;
