use super::*;
use basic::{AssumptionTactic, ImpliesElimTactic, ImpliesIntroTactic, ProofStateTactic};

/// Builder for composing tactics
pub struct TacticBuilder<T> {
    tactic: T,
}

impl<T> TacticBuilder<T> {
    pub fn new(tactic: T) -> Self {
        TacticBuilder { tactic }
    }

    pub fn build(self) -> T {
        self.tactic
    }
}

// Example usage:
pub fn auto_tactic() -> impl ProofStateTactic<Error = TacticError> {
    AutoTactic
}

/// A tactic that tries AssumptionTactic, ImpliesIntroTactic, and ImpliesElimTactic in sequence
pub struct AutoTactic;

impl ProofStateTactic for AutoTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        AssumptionTactic
            .apply(state)
            .or_else(|_| ImpliesIntroTactic.apply(state))
            .or_else(|_| ImpliesElimTactic.apply(state))
    }
}
