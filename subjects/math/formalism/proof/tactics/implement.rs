use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::objects::MathObject;
use crate::subjects::math::formalism::proof::tactics::search_replace::SearchReplace;
use crate::subjects::math::formalism::proof::{
    ContextEntry, ProofGoal, Tactic, format_relation_safely,
};
use crate::subjects::math::formalism::relations::{MathRelation, Quantification};
use crate::subjects::math::formalism::theorem::Theorem;
use crate::turn_render::Identifier;
use std::collections::HashMap;

#[derive(Clone)]
pub enum TacticApplicationResult {
    SingleGoal(ProofGoal),
    MultiGoal(Vec<ProofGoal>),
    ProofComplete,
    NoChange,
    Error(String),
}

impl std::fmt::Debug for TacticApplicationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TacticApplicationResult::SingleGoal(_) => {
                write!(f, "TacticApplicationResult::SingleGoal(...)")
            }
            TacticApplicationResult::MultiGoal(goals) => write!(
                f,
                "TacticApplicationResult::MultiGoal({} goals)",
                goals.len()
            ),
            TacticApplicationResult::ProofComplete => {
                write!(f, "TacticApplicationResult::ProofComplete")
            }
            TacticApplicationResult::NoChange => write!(f, "TacticApplicationResult::NoChange"),
            TacticApplicationResult::Error(msg) => {
                write!(f, "TacticApplicationResult::Error({})", msg)
            }
        }
    }
}

impl Tactic {
    pub fn apply_to_goal(&self, goal: &ProofGoal) -> TacticApplicationResult {
        match self {
            Tactic::AssumeImplicationAntecedent { hypothesis_name } => {
                if let MathRelation::Implies(antecedent, consequent) = &goal.statement {
                    let mut new_goal = goal.clone();
                    new_goal.statement = *consequent.clone();

                    let (new_goal, _) = new_goal.with_hypothesis(
                        &hypothesis_name.to_string(),
                        *antecedent.clone(),
                        Some("Assumed antecedent".to_string()),
                    );

                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error("Goal is not an implication.".to_string())
                }
            }

            Tactic::Introduce { entry, position } => {
                let mut new_goal = goal.clone();

                // Create a safe version of the entry to avoid circular references
                let safe_entry = ContextEntry {
                    name: entry.name.clone(),
                    ty: match &entry.ty {
                        MathExpression::Relation(relation) => {
                            // Convert MathExpression::Relation to a safe variable type
                            MathExpression::Var(Identifier::new_simple("Proposition".to_string()))
                        }
                        other => other.clone(),
                    },
                    definition: entry.definition.clone(),
                    description: match &entry.ty {
                        MathExpression::Relation(relation) => {
                            let safe_description =
                                format!("Proposition: {}", format_relation_safely(relation));
                            match &entry.description {
                                Some(desc) => Some(format!("{} ({})", desc, safe_description)),
                                None => Some(safe_description),
                            }
                        }
                        _ => entry.description.clone(),
                    },
                };

                match position {
                    Some(index) => new_goal.context.insert(*index, safe_entry),
                    None => new_goal.context.push(safe_entry),
                }
                TacticApplicationResult::SingleGoal(new_goal)
            }

            Tactic::Rewrite {
                target,
                theorem_id,
                instantiation,
                direction,
                ..
            } => {
                // This tactic needs to be significantly re-written to work with a real theorem registry
                // and a context-aware substitution engine. The logic below is a placeholder.
                TacticApplicationResult::Error(
                    "Rewrite tactic not implemented for new architecture.".to_string(),
                )
            }

            Tactic::ExactWith {
                theorem_id,
                instantiation,
            } => {
                // This also needs a proper theorem lookup and instantiation engine.
                TacticApplicationResult::Error(
                    "ExactWith tactic not implemented for new architecture.".to_string(),
                )
            }

            Tactic::SplitConjunction => {
                if let MathRelation::And(conjuncts) = &goal.statement {
                    if conjuncts.len() < 2 {
                        return TacticApplicationResult::NoChange;
                    }
                    let goals = conjuncts
                        .iter()
                        .map(|conjunct| {
                            let mut sub_goal = goal.clone();
                            sub_goal.statement = conjunct.clone();
                            sub_goal
                        })
                        .collect();
                    TacticApplicationResult::MultiGoal(goals)
                } else {
                    TacticApplicationResult::NoChange
                }
            }

            _ => TacticApplicationResult::Error(format!(
                "Tactic {:?} not yet implemented for the new architecture.",
                self
            )),
        }
    }
}
