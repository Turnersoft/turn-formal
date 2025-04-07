//! Identity types (equality types)
//! Implements Martin-Löf's identity types Id_A(a,b)

use serde::{Deserialize, Serialize};
use super::super::core::{Term, Result, Error};
use super::{TypeConstructor, TypeEliminator};

/// Identity type Id_A(a,b)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Id {
    /// Type of the terms being equated
    type_a: Box<Term>,
    /// First term
    term_a: Box<Term>,
    /// Second term
    term_b: Box<Term>,
}

impl Id {
    /// Create new identity type
    pub fn new(type_a: Term, term_a: Term, term_b: Term) -> Self {
        Id {
            type_a: Box::new(type_a),
            term_a: Box::new(term_a),
            term_b: Box::new(term_b),
        }
    }
    
    /// Get the type
    pub fn type_a(&self) -> &Term {
        &self.type_a
    }
    
    /// Get first term
    pub fn term_a(&self) -> &Term {
        &self.term_a
    }
    
    /// Get second term
    pub fn term_b(&self) -> &Term {
        &self.term_b
    }
}

impl TypeConstructor for Id {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::Apply { left, right: _ } => {
                match &**left {
                    Term::Var(name) if name == "refl" => {
                        // For reflexivity, terms must be definitionally equal
                        if self.term_a == self.term_b {
                            Ok(())
                        } else {
                            Err(Error::TypeError(format!(
                                "Terms {} and {} are not definitionally equal",
                                self.term_a, self.term_b
                            )))
                        }
                    }
                    _ => Err(Error::TypeError("Expected refl constructor".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected identity proof".to_string())),
        }
    }
    
    fn universe_level(&self) -> usize {
        // Identity type lives in same universe as type A
        // This is simplified; should compute actual level
        0
    }
}

/// J eliminator (path induction)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct J {
    /// The identity type being eliminated
    id: Id,
    /// Motive of elimination (C)
    motive: Box<Term>,
    /// Base case (refl case)
    base: Box<Term>,
}

impl J {
    /// Create new J eliminator
    pub fn new(id: Id, motive: Term, base: Term) -> Self {
        J {
            id,
            motive: Box::new(motive),
            base: Box::new(base),
        }
    }
}

impl TypeEliminator for J {
    fn eliminate(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Apply { left, right: _ } => {
                match &**left {
                    Term::Var(name) if name == "refl" => {
                        // For refl, return base case
                        Ok((*self.base).clone())
                    }
                    _ => Err(Error::TypeError("Expected refl".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected identity proof".to_string())),
        }
    }
}

/// K eliminator (uniqueness of identity proofs)
/// Note: This is only valid in some type theories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct K {
    /// The identity type being eliminated
    id: Id,
    /// Motive of elimination
    motive: Box<Term>,
    /// Base case (refl case)
    base: Box<Term>,
}

impl K {
    /// Create new K eliminator
    pub fn new(id: Id, motive: Term, base: Term) -> Self {
        K {
            id,
            motive: Box::new(motive),
            base: Box::new(base),
        }
    }
}

impl TypeEliminator for K {
    fn eliminate(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Apply { left, right: _ } => {
                match &**left {
                    Term::Var(name) if name == "refl" => {
                        // For refl, return base case
                        Ok((*self.base).clone())
                    }
                    _ => Err(Error::TypeError("Expected refl".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected identity proof".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_type() {
        // Create Id_A(a,a)
        let id = Id::new(
            Term::Var("A".to_string()),
            Term::Var("a".to_string()),
            Term::Var("a".to_string()),
        );
        
        // Test reflexivity proof
        let refl = Term::Apply {
            left: Box::new(Term::Var("refl".to_string())),
            right: Box::new(Term::Var("a".to_string())),
        };
        
        assert!(id.check_term(&refl).is_ok());
    }

    #[test]
    fn test_j_eliminator() {
        // Create Id_A(a,a)
        let id = Id::new(
            Term::Var("A".to_string()),
            Term::Var("a".to_string()),
            Term::Var("a".to_string()),
        );
        
        // Create J eliminator
        let j = J::new(
            id,
            Term::Var("C".to_string()),
            Term::Var("c".to_string()),
        );
        
        // Test elimination on refl
        let refl = Term::Apply {
            left: Box::new(Term::Var("refl".to_string())),
            right: Box::new(Term::Var("a".to_string())),
        };
        
        let result = j.eliminate(&refl).unwrap();
        assert_eq!(result, Term::Var("c".to_string()));
    }
}
