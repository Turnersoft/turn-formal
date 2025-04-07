//! Dependent product types (Π-types)
//! Generalizes function types to dependent types

use super::super::core::{Error, Result, Term};
use super::{TypeConstructor, TypeEliminator};

/// Dependent product type (Πx:A.B)
#[derive(Debug, Clone)]
pub struct Pi {
    /// Variable name
    var: String,
    /// Domain type
    domain: Box<Term>,
    /// Codomain type (may depend on var)
    codomain: Box<Term>,
}

impl Pi {
    /// Create a new dependent product type
    pub fn new(var: impl Into<String>, domain: Term, codomain: Term) -> Self {
        Pi {
            var: var.into(),
            domain: Box::new(domain),
            codomain: Box::new(codomain),
        }
    }

    /// Get the domain type
    pub fn domain(&self) -> &Term {
        &self.domain
    }

    /// Get the codomain type
    pub fn codomain(&self) -> &Term {
        &self.codomain
    }
}

impl TypeConstructor for Pi {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::Lambda { var, body } => {
                // Check that variable matches
                if var != &self.var {
                    return Err(Error::TypeError(format!(
                        "Expected variable {}, got {}",
                        self.var, var
                    )));
                }

                // Check body type
                // Note: This is simplified; should check in extended context
                Ok(())
            }
            _ => Err(Error::TypeError("Expected lambda abstraction".to_string())),
        }
    }

    fn universe_level(&self) -> usize {
        // Pi type lives in maximum of domain and codomain levels
        // This is simplified; should actually compute levels
        0
    }
}

/// Dependent product elimination
pub struct PiElim {
    /// The Pi type being eliminated
    pi: Pi,
}

impl PiElim {
    /// Create new eliminator
    pub fn new(pi: Pi) -> Self {
        PiElim { pi }
    }
}

impl TypeEliminator for PiElim {
    fn eliminate(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Apply { left, right } => {
                // Check function has Pi type
                self.pi.check_term(left)?;

                // Apply substitution
                match &**left {
                    Term::Lambda { var, body } => {
                        // Substitute argument in body
                        Ok(substitute(body, var, right))
                    }
                    _ => Err(Error::TypeError("Expected lambda".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected application".to_string())),
        }
    }
}

/// Helper function to substitute a term for a variable
fn substitute(term: &Term, var: &str, replacement: &Term) -> Term {
    match term {
        Term::Var(name) if name == var => replacement.clone(),
        Term::Var(_) => term.clone(),
        Term::Lambda { var: v, body } if v != var => Term::Lambda {
            var: v.clone(),
            body: Box::new(substitute(body, var, replacement)),
        },
        Term::Lambda { .. } => term.clone(),
        Term::Apply { left, right } => Term::Apply {
            left: Box::new(substitute(left, var, replacement)),
            right: Box::new(substitute(right, var, replacement)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi_type() {
        // Create Πx:A.B
        let pi = Pi::new("x", Term::Var("A".to_string()), Term::Var("B".to_string()));

        // Test lambda abstraction
        let lambda = Term::Lambda {
            var: "x".to_string(),
            body: Box::new(Term::Var("y".to_string())),
        };

        assert!(pi.check_term(&lambda).is_ok());
    }

    #[test]
    fn test_pi_elim() {
        // Create Πx:A.B
        let pi = Pi::new("x", Term::Var("A".to_string()), Term::Var("B".to_string()));

        let elim = PiElim::new(pi);

        // Test application (λx.y) z
        let app = Term::Apply {
            left: Box::new(Term::Lambda {
                var: "x".to_string(),
                body: Box::new(Term::Var("y".to_string())),
            }),
            right: Box::new(Term::Var("z".to_string())),
        };

        let result = elim.eliminate(&app).unwrap();
        assert_eq!(result, Term::Var("y".to_string()));
    }
}
