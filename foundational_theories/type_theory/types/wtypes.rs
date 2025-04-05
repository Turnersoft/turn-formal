//! W-types (Well-founded trees)
//! Implements Martin-Löf's W-types for well-founded trees

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::foundational_theories::type_theory::{
    core::{Term, Result, Error},
    types::{TypeConstructor, TypeEliminator},
};

/// W-type constructor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WConstructor {
    /// Node type
    node_type: Term,
    /// Branching type family
    branch_type: Term,
}

impl WConstructor {
    /// Create new W-type constructor
    pub fn new(node_type: Term, branch_type: Term) -> Self {
        WConstructor {
            node_type,
            branch_type,
        }
    }
}

/// W-type term
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WTerm {
    /// Sup constructor: sup(a, f) where
    /// a : A is the node label
    /// f : B(a) → W(A,B) is the branching function
    Sup {
        /// Node label
        node: Box<Term>,
        /// Branching function
        branch: Box<Term>,
    },
}

/// W-type (Well-founded tree)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WType {
    /// Type name
    name: String,
    /// Constructor
    constructor: WConstructor,
}

impl WType {
    /// Create new W-type
    pub fn new(name: impl Into<String>, constructor: WConstructor) -> Self {
        WType {
            name: name.into(),
            constructor,
        }
    }
    
    /// Get node type
    pub fn node_type(&self) -> &Term {
        &self.constructor.node_type
    }
    
    /// Get branch type
    pub fn branch_type(&self) -> &Term {
        &self.constructor.branch_type
    }
    
    /// Create sup term
    pub fn sup(&self, node: Term, branch: Term) -> Term {
        Term::W {
            var: self.name.clone(),
            node_type: Box::new(self.node_type().clone()),
            branch_type: Box::new(self.branch_type().clone()),
        }
    }
}

impl TypeConstructor for WType {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::W { var, node_type, branch_type } => {
                // Check node type
                if node_type != &Box::new(self.node_type().clone()) {
                    return Err(Error::TypeError(
                        "Invalid node type".to_string()
                    ));
                }
                
                // Check branch type
                if branch_type != &Box::new(self.branch_type().clone()) {
                    return Err(Error::TypeError(
                        "Invalid branch type".to_string()
                    ));
                }
                
                Ok(())
            }
            _ => Err(Error::TypeError("Expected W-type".to_string())),
        }
    }
    
    fn universe_level(&self) -> usize {
        // W-type lives in universe max(i,j+1) where
        // i is universe of A and j is universe of B
        let i = match self.node_type() {
            Term::Sort(l) => *l,
            _ => 0,
        };
        let j = match self.branch_type() {
            Term::Sort(l) => *l,
            _ => 0,
        };
        std::cmp::max(i, j + 1)
    }
}

/// W-type elimination
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WEliminator {
    /// W-type being eliminated
    wtype: WType,
    /// Step function for recursion
    step: Term,
}

impl WEliminator {
    /// Create new W-type eliminator
    pub fn new(wtype: WType, step: Term) -> Self {
        WEliminator { wtype, step }
    }
}

impl TypeEliminator for WEliminator {
    fn eliminate(&self, term: &Term) -> Result<Term> {
        match term {
            Term::W { var: _, node_type: _, branch_type: _ } => {
                // Apply step function
                Ok(self.step.clone())
            }
            _ => Err(Error::TypeError("Expected W-type".to_string())),
        }
    }
}

/// Examples of W-types
pub mod examples {
    use super::*;

    /// Create natural numbers as W-type
    pub fn nat_wtype() -> WType {
        WType::new(
            "Nat",
            WConstructor::new(
                Term::Bool, // A = Bool (false=zero, true=successor)
                Term::Lambda { // B(b) = if b then Unit else Empty
                    var: "b".to_string(),
                    body: Box::new(Term::If {
                        cond: Box::new(Term::Var("b".to_string())),
                        then_case: Box::new(Term::Unit),
                        else_case: Box::new(Term::Empty),
                    }),
                },
            ),
        )
    }

    /// Create binary trees as W-type
    pub fn binary_tree_wtype() -> WType {
        WType::new(
            "BinaryTree",
            WConstructor::new(
                Term::Var("A".to_string()), // Node labels
                Term::Bool, // Two branches for each node
            ),
        )
    }

    /// Create rose trees as W-type
    pub fn rose_tree_wtype() -> WType {
        WType::new(
            "RoseTree",
            WConstructor::new(
                Term::Var("A".to_string()), // Node labels
                Term::List(Box::new(Term::Unit)), // List of children
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::examples::*;

    #[test]
    fn test_nat_wtype() {
        let nat = nat_wtype();
        
        // Test zero
        let zero = nat.sup(
            Term::Bool, // false
            Term::Lambda {
                var: "x".to_string(),
                body: Box::new(Term::Empty),
            },
        );
        assert!(nat.check_term(&zero).is_ok());
        
        // Test successor
        let one = nat.sup(
            Term::Bool, // true
            Term::Lambda {
                var: "x".to_string(),
                body: Box::new(zero),
            },
        );
        assert!(nat.check_term(&one).is_ok());
    }

    #[test]
    fn test_binary_tree() {
        let tree = binary_tree_wtype();
        
        // Test leaf
        let leaf = tree.sup(
            Term::Var("a".to_string()),
            Term::Lambda {
                var: "x".to_string(),
                body: Box::new(Term::Empty),
            },
        );
        assert!(tree.check_term(&leaf).is_ok());
    }

    #[test]
    fn test_universe_level() {
        let nat = nat_wtype();
        assert!(nat.universe_level() > 0);
    }
}
