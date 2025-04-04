//! Quotient types
//! Implements types modulo equivalence relations

use super::super::core::{Error, Result, Term};
use super::{TypeConstructor, TypeEliminator};

/// Equivalence relation
#[derive(Debug, Clone)]
pub struct Relation {
    /// Type being related
    type_a: Box<Term>,
    /// Relation term (should be an equivalence relation)
    rel: Box<Term>,
}

impl Relation {
    /// Create new relation
    pub fn new(type_a: Term, rel: Term) -> Self {
        Relation {
            type_a: Box::new(type_a),
            rel: Box::new(rel),
        }
    }

    /// Get base type
    pub fn type_a(&self) -> &Term {
        &self.type_a
    }

    /// Get relation
    pub fn rel(&self) -> &Term {
        &self.rel
    }
}

/// Quotient type A/R
#[derive(Debug, Clone)]
pub struct Quotient {
    /// Base type
    type_a: Box<Term>,
    /// Equivalence relation
    relation: Relation,
}

impl Quotient {
    /// Create new quotient type
    pub fn new(type_a: Term, relation: Relation) -> Self {
        Quotient {
            type_a: Box::new(type_a),
            relation,
        }
    }
}

impl TypeConstructor for Quotient {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::Apply { left, right } => {
                match &**left {
                    Term::Var(name) if name == "class_of" => {
                        // Check that term is from base type
                        // This is simplified; should check against type_a
                        Ok(())
                    }
                    _ => Err(Error::TypeError(
                        "Expected class_of constructor".to_string(),
                    )),
                }
            }
            _ => Err(Error::TypeError("Expected quotient term".to_string())),
        }
    }

    fn universe_level(&self) -> usize {
        // Quotient type lives in same universe as base type
        // This is simplified; should compute actual level
        0
    }
}

/// Quotient elimination principle
pub struct QuotientElim {
    /// The quotient type being eliminated
    quot: Quotient,
    /// Target type family
    target: Box<Term>,
    /// Function to lift
    func: Box<Term>,
    /// Proof that function respects relation
    resp_proof: Box<Term>,
}

impl QuotientElim {
    /// Create new quotient eliminator
    pub fn new(quot: Quotient, target: Term, func: Term, resp_proof: Term) -> Self {
        QuotientElim {
            quot,
            target: Box::new(target),
            func: Box::new(func),
            resp_proof: Box::new(resp_proof),
        }
    }
}

impl TypeEliminator for QuotientElim {
    fn eliminate(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Apply { left, right } => {
                match &**left {
                    Term::Var(name) if name == "class_of" => {
                        // Apply the lifted function
                        Ok(Term::Apply {
                            left: self.func.clone(),
                            right: (**right).clone(),
                        })
                    }
                    _ => Err(Error::TypeError("Expected class_of".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected quotient term".to_string())),
        }
    }
}

/// Example: Integers as quotient of pairs of naturals
pub fn integer_quotient() -> Quotient {
    // Type: Nat × Nat
    let nat_pair = Term::Apply {
        left: Box::new(Term::Var("Pair".to_string())),
        right: Box::new(Term::Apply {
            left: Box::new(Term::Var("Nat".to_string())),
            right: Box::new(Term::Var("Nat".to_string())),
        }),
    };

    // Relation: (a,b) ~ (c,d) iff a+d = b+c
    let relation = Term::Lambda {
        var: "p1".to_string(),
        body: Box::new(Term::Lambda {
            var: "p2".to_string(),
            body: Box::new(Term::Var("eq_nat".to_string())),
        }),
    };

    Quotient::new(nat_pair, Relation::new(nat_pair.clone(), relation))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotient_type() {
        let int = integer_quotient();

        // Test class_of constructor
        let pair = Term::Apply {
            left: Box::new(Term::Var("Pair".to_string())),
            right: Box::new(Term::Apply {
                left: Box::new(Term::Var("0".to_string())),
                right: Box::new(Term::Var("0".to_string())),
            }),
        };

        let zero_class = Term::Apply {
            left: Box::new(Term::Var("class_of".to_string())),
            right: Box::new(pair),
        };

        assert!(int.check_term(&zero_class).is_ok());
    }

    #[test]
    fn test_quotient_elim() {
        let int = integer_quotient();

        // Eliminator for absolute value function
        let elim = QuotientElim::new(
            int,
            Term::Var("Nat".to_string()),
            Term::Var("abs".to_string()),
            Term::Var("abs_resp".to_string()),
        );

        // Test elimination on zero class
        let pair = Term::Apply {
            left: Box::new(Term::Var("Pair".to_string())),
            right: Box::new(Term::Apply {
                left: Box::new(Term::Var("0".to_string())),
                right: Box::new(Term::Var("0".to_string())),
            }),
        };

        let zero_class = Term::Apply {
            left: Box::new(Term::Var("class_of".to_string())),
            right: Box::new(pair),
        };

        let result = elim.eliminate(&zero_class).unwrap();
        assert_eq!(
            result,
            Term::Apply {
                left: Box::new(Term::Var("abs".to_string())),
                right: Box::new(pair),
            }
        );
    }
}
