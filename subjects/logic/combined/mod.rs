use crate::formalize_v2::subjects::logic::{first_order, modal, temporal};

use super::propositional::Proposition;

/// A logical formula that can combine different aspects of logic in a nested way
#[derive(Debug, Clone)]
pub enum LogicalFormula {
    /// First-order formula
    FirstOrder(first_order::Formula),
    /// Modal operator applied to a formula
    Modal(modal::ModalOperator, Box<LogicalFormula>),
    /// Temporal operator applied to a formula
    Temporal(temporal::TemporalOperator, Box<LogicalFormula>),
    /// Propositional formula
    Propositional(Proposition),
}

impl LogicalFormula {
    /// Create a first-order formula
    pub fn first_order(formula: first_order::Formula) -> Self {
        LogicalFormula::FirstOrder(formula)
    }

    /// Apply a modal operator to a formula
    pub fn modal(operator: modal::ModalOperator, inner: LogicalFormula) -> Self {
        LogicalFormula::Modal(operator, Box::new(inner))
    }

    /// Apply a temporal operator to a formula
    pub fn temporal(operator: temporal::TemporalOperator, inner: LogicalFormula) -> Self {
        LogicalFormula::Temporal(operator, Box::new(inner))
    }

    /// Create a propositional formula
    pub fn propositional(prop: Proposition) -> Self {
        LogicalFormula::Propositional(prop)
    }

    /// Convert this formula to a proposition if possible
    pub fn into_proposition(&self) -> Option<Proposition> {
        match self {
            LogicalFormula::Propositional(p) => Some(p.clone()),
            LogicalFormula::Modal(op, inner) => {
                // Convert modal operators to propositions
                let inner_prop = inner.into_proposition()?;
                match op {
                    modal::ModalOperator::Necessary(_) => {
                        Some(Proposition::and(Proposition::atomic("□"), inner_prop))
                    }
                    modal::ModalOperator::Possible(_) => {
                        Some(Proposition::and(Proposition::atomic("◇"), inner_prop))
                    }
                    modal::ModalOperator::None => Some(inner_prop),
                }
            }
            LogicalFormula::Temporal(op, inner) => {
                // Convert temporal operators to propositions
                let inner_prop = inner.into_proposition()?;
                match op {
                    temporal::TemporalOperator::Next(_) => {
                        Some(Proposition::and(Proposition::atomic("X"), inner_prop))
                    }
                    temporal::TemporalOperator::Always(_) => {
                        Some(Proposition::and(Proposition::atomic("G"), inner_prop))
                    }
                    temporal::TemporalOperator::Eventually(_) => {
                        Some(Proposition::and(Proposition::atomic("F"), inner_prop))
                    }
                    temporal::TemporalOperator::Until(left, right) => {
                        // Create a proposition for the until operator: U(p, q)
                        Some(Proposition::atomic("U"))
                    }
                    temporal::TemporalOperator::None => Some(inner_prop),
                }
            }
            LogicalFormula::FirstOrder(f) => {
                // Convert first-order formulas to propositions when possible
                match f {
                    first_order::Formula::Atomic(b) => Some(if *b {
                        Proposition::True
                    } else {
                        Proposition::False
                    }),
                    first_order::Formula::And(left, right) => {
                        // Recursively convert the parts
                        let left_prop =
                            LogicalFormula::FirstOrder(*left.clone()).into_proposition()?;
                        let right_prop =
                            LogicalFormula::FirstOrder(*right.clone()).into_proposition()?;
                        Some(Proposition::and(left_prop, right_prop))
                    }
                    first_order::Formula::Or(left, right) => {
                        let left_prop =
                            LogicalFormula::FirstOrder(*left.clone()).into_proposition()?;
                        let right_prop =
                            LogicalFormula::FirstOrder(*right.clone()).into_proposition()?;
                        Some(Proposition::or(left_prop, right_prop))
                    }
                    first_order::Formula::Implies(left, right) => {
                        let left_prop =
                            LogicalFormula::FirstOrder(*left.clone()).into_proposition()?;
                        let right_prop =
                            LogicalFormula::FirstOrder(*right.clone()).into_proposition()?;
                        Some(Proposition::implies(left_prop, right_prop))
                    }
                    first_order::Formula::Not(inner) => {
                        let inner_prop =
                            LogicalFormula::FirstOrder(*inner.clone()).into_proposition()?;
                        Some(Proposition::not(inner_prop))
                    }
                    // For quantified formulas and predicates, we create atomic propositions
                    first_order::Formula::ForAll(var, _) => {
                        Some(Proposition::atomic(&format!("∀{}", var)))
                    }
                    first_order::Formula::Exists(var, _) => {
                        Some(Proposition::atomic(&format!("∃{}", var)))
                    }
                    first_order::Formula::Predicate(name, _) => Some(Proposition::atomic(name)),
                    first_order::Formula::Equality(_, _) => Some(Proposition::atomic("=")),
                }
            }
        }
    }
}

/// Trait for converting logical formulas to a foundational theory
pub trait LogicalFoundation {
    /// The term type in this foundation
    type Term;
    /// The error type for conversion failures
    type Error;

    /// Convert a logical formula to this foundation
    fn convert_formula(&self, formula: &LogicalFormula) -> Result<Self::Term, Self::Error>;
}

/// Error types for logical operations
#[derive(Debug)]
pub enum LogicError {
    /// The formula is not well-formed
    MalformedFormula(String),
    /// The formula cannot be converted to the target foundation
    ConversionError(String),
    /// The proof is invalid
    InvalidProof(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use first_order::{Formula, Term};
    use modal::ModalOperator;
    use temporal::TemporalOperator;

    #[test]
    fn test_nested_formula() {
        // Create a formula: "Necessarily, for all x, eventually P(x)"
        let formula = LogicalFormula::modal(
            ModalOperator::necessary(None),
            LogicalFormula::first_order(Formula::for_all(
                "x",
                Formula::predicate("P", vec![Term::Variable("x".to_string())]),
            )),
        );

        // Add temporal operator
        let formula = LogicalFormula::temporal(TemporalOperator::eventually(None), formula);

        // Verify the structure
        match formula {
            LogicalFormula::Temporal(temp, inner) => {
                assert!(matches!(temp, TemporalOperator::Eventually(_)));
                match *inner {
                    LogicalFormula::Modal(modal, _) => {
                        assert!(matches!(modal, ModalOperator::Necessary(_)));
                    }
                    _ => panic!("Expected modal operator"),
                }
            }
            _ => panic!("Expected temporal operator"),
        }
    }

    #[test]
    fn test_mixed_formula() {
        // Create a formula: "Eventually (P ∧ Q) → Necessarily R"
        let p = Proposition::atomic("P");
        let q = Proposition::atomic("Q");
        let r = Proposition::atomic("R");

        let temporal_part = LogicalFormula::temporal(
            TemporalOperator::eventually(None),
            LogicalFormula::propositional(Proposition::and(p, q)),
        );

        let modal_part = LogicalFormula::modal(
            ModalOperator::necessary(None),
            LogicalFormula::propositional(r),
        );

        let formula = LogicalFormula::propositional(Proposition::implies(
            temporal_part.into_proposition().unwrap(),
            modal_part.into_proposition().unwrap(),
        ));

        // The structure verification would depend on how we implement into_proposition
        assert!(matches!(formula, LogicalFormula::Propositional(_)));
    }
}
