//! Dependent sum types (Σ-types)
//! Generalizes product types to dependent types

use super::super::core::{Term, Result, Error};
use super::{TypeConstructor, TypeEliminator};

/// Dependent sum type (Σx:A.B)
#[derive(Debug, Clone)]
pub struct Sigma {
    /// Variable name
    var: String,
    /// First type
    first: Box<Term>,
    /// Second type (may depend on var)
    second: Box<Term>,
}

impl Sigma {
    /// Create a new dependent sum type
    pub fn new(var: impl Into<String>, first: Term, second: Term) -> Self {
        Sigma {
            var: var.into(),
            first: Box::new(first),
            second: Box::new(second),
        }
    }
    
    /// Get the first type
    pub fn first(&self) -> &Term {
        &self.first
    }
    
    /// Get the second type
    pub fn second(&self) -> &Term {
        &self.second
    }
}

impl TypeConstructor for Sigma {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::Apply { left, right } => {
                match &**left {
                    Term::Var(name) if name == "pair" => {
                        // Check first component
                        // Note: This is simplified; should check against first type
                        
                        // Check second component
                        // Note: This is simplified; should check against second type
                        // with first component substituted
                        Ok(())
                    }
                    _ => Err(Error::TypeError("Expected pair constructor".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected pair".to_string())),
        }
    }
    
    fn universe_level(&self) -> usize {
        // Sigma type lives in maximum of component levels
        // This is simplified; should actually compute levels
        0
    }
}

/// First projection from dependent sum
pub struct First {
    /// The Sigma type being projected from
    sigma: Sigma,
}

impl First {
    /// Create new first projection
    pub fn new(sigma: Sigma) -> Self {
        First { sigma }
    }
}

impl TypeEliminator for First {
    fn eliminate(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Apply { left, right } => {
                match &**left {
                    Term::Var(name) if name == "pair" => {
                        // Extract first component
                        Ok((**right).clone())
                    }
                    _ => Err(Error::TypeError("Expected pair".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected pair".to_string())),
        }
    }
}

/// Second projection from dependent sum
pub struct Second {
    /// The Sigma type being projected from
    sigma: Sigma,
}

impl Second {
    /// Create new second projection
    pub fn new(sigma: Sigma) -> Self {
        Second { sigma }
    }
}

impl TypeEliminator for Second {
    fn eliminate(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Apply { left, right: first } => {
                match &**left {
                    Term::Apply { left, right: second } => {
                        match &**left {
                            Term::Var(name) if name == "pair" => {
                                // Extract second component
                                Ok((**second).clone())
                            }
                            _ => Err(Error::TypeError("Expected pair".to_string())),
                        }
                    }
                    _ => Err(Error::TypeError("Expected pair".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected pair".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigma_type() {
        // Create Σx:A.B
        let sigma = Sigma::new(
            "x",
            Term::Var("A".to_string()),
            Term::Var("B".to_string()),
        );
        
        // Test pair construction
        let pair = Term::Apply {
            left: Box::new(Term::Apply {
                left: Box::new(Term::Var("pair".to_string())),
                right: Box::new(Term::Var("a".to_string())),
            }),
            right: Box::new(Term::Var("b".to_string())),
        };
        
        assert!(sigma.check_term(&pair).is_ok());
    }

    #[test]
    fn test_projections() {
        // Create Σx:A.B
        let sigma = Sigma::new(
            "x",
            Term::Var("A".to_string()),
            Term::Var("B".to_string()),
        );
        
        // Create pair (a,b)
        let pair = Term::Apply {
            left: Box::new(Term::Apply {
                left: Box::new(Term::Var("pair".to_string())),
                right: Box::new(Term::Var("a".to_string())),
            }),
            right: Box::new(Term::Var("b".to_string())),
        };
        
        // Test first projection
        let first = First::new(sigma.clone());
        let first_result = first.eliminate(&pair).unwrap();
        assert_eq!(first_result, Term::Var("a".to_string()));
        
        // Test second projection
        let second = Second::new(sigma);
        let second_result = second.eliminate(&pair).unwrap();
        assert_eq!(second_result, Term::Var("b".to_string()));
    }
}
