use basic::{AssumptionTactic, OrIntroTactic, ProofStateTactic};
use builder::TacticBuilder;

use super::basic::{
    AndElimTactic, AndIntroTactic, DNEtactic, ImpliesElimTactic, ImpliesIntroTactic,
};
use super::proof_state::ProofState;
use super::*;

/// Tactic that combines introduction and elimination rules
pub struct IntroElimTactic;
impl ProofStateTactic for IntroElimTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        // Try each tactic in sequence without using trait objects
        AndIntroTactic::new(AssumptionTactic, AssumptionTactic)
            .apply(state)
            .or_else(|_| OrIntroTactic.apply(state))
            .or_else(|_| ImpliesIntroTactic.apply(state))
            .or_else(|_| AndElimTactic.apply(state))
            .or_else(|_| ImpliesElimTactic.apply(state))
            .or_else(|_| DNEtactic.apply(state))
            .or_else(|_| Err(TacticError::NoMatchingRule))
    }
}

/// Tactic that tries to decompose complex propositions
pub struct DecomposeTactic;
impl ProofStateTactic for DecomposeTactic {
    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        match goal {
            // Break down conjunctions
            Proposition::And(_, _) => AndElimTactic.apply(state),
            // Break down implications
            Proposition::Implies(_, _) => ImpliesElimTactic.apply(state),
            _ => Err(TacticError::NoMatchingRule),
        }
    }

    type Error = TacticError;
}
