//! Unification system for constraint solving
//! Implements higher-order unification algorithms

pub mod higher_order;

use std::collections::{HashMap, HashSet};
use super::core::{Term, Result, Error};

/// Substitution mapping variables to terms
#[derive(Debug, Clone)]
pub struct Substitution {
    /// Variable mappings
    mappings: HashMap<String, Term>,
}

impl Substitution {
    /// Create empty substitution
    pub fn new() -> Self {
        Substitution {
            mappings: HashMap::new(),
        }
    }
    
    /// Add mapping to substitution
    pub fn add(&mut self, var: impl Into<String>, term: Term) -> Result<()> {
        let var = var.into();
        if self.mappings.contains_key(&var) {
            Err(Error::UnificationError(format!(
                "Variable {} already mapped", var
            )))
        } else {
            self.mappings.insert(var, term);
            Ok(())
        }
    }
    
    /// Apply substitution to term
    pub fn apply(&self, term: &Term) -> Term {
        match term {
            Term::Var(name) => {
                self.mappings.get(name)
                    .cloned()
                    .unwrap_or_else(|| term.clone())
            }
            Term::Lambda { var, body } => Term::Lambda {
                var: var.clone(),
                body: Box::new(self.apply(body)),
            },
            Term::Apply { left, right } => Term::Apply {
                left: Box::new(self.apply(left)),
                right: Box::new(self.apply(right)),
            },
        }
    }
    
    /// Compose with another substitution
    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = self.clone();
        for (var, term) in &other.mappings {
            result.mappings.insert(var.clone(), self.apply(term));
        }
        result
    }
}

/// Unification constraint between terms
#[derive(Debug, Clone)]
pub struct Constraint {
    /// Left term
    left: Term,
    /// Right term
    right: Term,
}

impl Constraint {
    /// Create new constraint
    pub fn new(left: Term, right: Term) -> Self {
        Constraint { left, right }
    }
}

/// Unification problem
#[derive(Debug, Clone)]
pub struct UnificationProblem {
    /// Constraints to solve
    constraints: Vec<Constraint>,
    /// Current substitution
    substitution: Substitution,
}

impl UnificationProblem {
    /// Create new unification problem
    pub fn new(constraints: Vec<Constraint>) -> Self {
        UnificationProblem {
            constraints,
            substitution: Substitution::new(),
        }
    }
    
    /// Get free variables in term
    fn free_vars(&self, term: &Term) -> HashSet<String> {
        match term {
            Term::Var(name) => {
                let mut vars = HashSet::new();
                vars.insert(name.clone());
                vars
            }
            Term::Lambda { var, body } => {
                let mut vars = self.free_vars(body);
                vars.remove(var);
                vars
            }
            Term::Apply { left, right } => {
                let mut vars = self.free_vars(left);
                vars.extend(self.free_vars(right));
                vars
            }
        }
    }
    
    /// Check if variable occurs in term
    fn occurs_check(&self, var: &str, term: &Term) -> bool {
        self.free_vars(term).contains(var)
    }
    
    /// Solve single constraint
    fn solve_constraint(&mut self, constraint: Constraint) -> Result<()> {
        let left = self.substitution.apply(&constraint.left);
        let right = self.substitution.apply(&constraint.right);
        
        match (left, right) {
            // Variable case
            (Term::Var(x), term) | (term, Term::Var(x)) => {
                if self.occurs_check(&x, &term) {
                    Err(Error::UnificationError(
                        "Occurs check failed".to_string()
                    ))
                } else {
                    self.substitution.add(x, term)
                }
            }
            
            // Lambda case
            (Term::Lambda { var: v1, body: b1 },
             Term::Lambda { var: v2, body: b2 }) => {
                // Rename bound variables to match
                let mut subst = Substitution::new();
                subst.add(v2, Term::Var(v1.clone()))?;
                let b2 = subst.apply(&b2);
                self.solve_constraint(Constraint::new(*b1, b2))
            }
            
            // Application case
            (Term::Apply { left: l1, right: r1 },
             Term::Apply { left: l2, right: r2 }) => {
                self.solve_constraint(Constraint::new(*l1, *l2))?;
                self.solve_constraint(Constraint::new(*r1, *r2))
            }
            
            // Rigid-rigid failure
            _ => Err(Error::UnificationError(
                "Terms cannot be unified".to_string()
            )),
        }
    }
    
    /// Solve all constraints
    pub fn solve(&mut self) -> Result<Substitution> {
        while let Some(constraint) = self.constraints.pop() {
            self.solve_constraint(constraint)?;
        }
        Ok(self.substitution.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_unification() {
        // x = a
        let constraint = Constraint::new(
            Term::Var("x".to_string()),
            Term::Var("a".to_string()),
        );
        
        let mut problem = UnificationProblem::new(vec![constraint]);
        let solution = problem.solve().unwrap();
        
        assert_eq!(
            solution.apply(&Term::Var("x".to_string())),
            Term::Var("a".to_string()),
        );
    }

    #[test]
    fn test_function_unification() {
        // f(x) = f(a)
        let constraint = Constraint::new(
            Term::Apply {
                left: Box::new(Term::Var("f".to_string())),
                right: Box::new(Term::Var("x".to_string())),
            },
            Term::Apply {
                left: Box::new(Term::Var("f".to_string())),
                right: Box::new(Term::Var("a".to_string())),
            },
        );
        
        let mut problem = UnificationProblem::new(vec![constraint]);
        let solution = problem.solve().unwrap();
        
        assert_eq!(
            solution.apply(&Term::Var("x".to_string())),
            Term::Var("a".to_string()),
        );
    }

    #[test]
    fn test_occurs_check() {
        // x = f(x) should fail
        let constraint = Constraint::new(
            Term::Var("x".to_string()),
            Term::Apply {
                left: Box::new(Term::Var("f".to_string())),
                right: Box::new(Term::Var("x".to_string())),
            },
        );
        
        let mut problem = UnificationProblem::new(vec![constraint]);
        assert!(problem.solve().is_err());
    }
}
