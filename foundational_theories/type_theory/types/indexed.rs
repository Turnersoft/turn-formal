//! Indexed inductive types and W-types
//! Implements support for indexed inductive types and well-founded trees

use std::collections::{HashMap, HashSet};
use crate::formalize_v2::foundational_theories::type_theory_v2::{
    core::{Term, Result, Error},
    types::{Constructor, TypeConstructor},
};

/// Index for inductive type
#[derive(Debug, Clone)]
pub struct Index {
    /// Index name
    name: String,
    /// Index type
    ty: Term,
}

impl Index {
    /// Create new index
    pub fn new(name: impl Into<String>, ty: Term) -> Self {
        Index {
            name: name.into(),
            ty,
        }
    }
}

/// Indexed inductive type
#[derive(Debug, Clone)]
pub struct IndexedType {
    /// Type name
    name: String,
    /// Type indices
    indices: Vec<Index>,
    /// Type constructors
    constructors: Vec<Constructor>,
}

impl IndexedType {
    /// Create new indexed type
    pub fn new(name: impl Into<String>) -> Self {
        IndexedType {
            name: name.into(),
            indices: Vec::new(),
            constructors: Vec::new(),
        }
    }
    
    /// Add index
    pub fn add_index(&mut self, index: Index) {
        self.indices.push(index);
    }
    
    /// Add constructor
    pub fn add_constructor(&mut self, constructor: Constructor) {
        self.constructors.push(constructor);
    }
    
    /// Get type with indices
    pub fn get_type(&self) -> Term {
        let mut ty = Term::Var(self.name.clone());
        
        // Apply indices
        for index in &self.indices {
            ty = Term::Apply {
                left: Box::new(ty),
                right: Box::new(index.ty.clone()),
            };
        }
        
        ty
    }
}

/// W-type (well-founded tree)
#[derive(Debug, Clone)]
pub struct WType {
    /// Node type
    node_type: Term,
    /// Branching type family
    branch_type: Term,
}

impl WType {
    /// Create new W-type
    pub fn new(node_type: Term, branch_type: Term) -> Self {
        WType {
            node_type,
            branch_type,
        }
    }
    
    /// Get W-type formation rule
    pub fn get_type(&self) -> Term {
        Term::W {
            var: "x".to_string(),
            node_type: Box::new(self.node_type.clone()),
            branch_type: Box::new(self.branch_type.clone()),
        }
    }
    
    /// Get introduction rule (sup constructor)
    pub fn get_intro(&self) -> Term {
        Term::Lambda {
            var: "a".to_string(),
            body: Box::new(Term::Lambda {
                var: "f".to_string(),
                body: Box::new(Term::Sup {
                    node: Box::new(Term::Var("a".to_string())),
                    branch: Box::new(Term::Var("f".to_string())),
                }),
            }),
        }
    }
    
    /// Get elimination rule
    pub fn get_elim(&self) -> Term {
        // W-rec elimination principle
        Term::Lambda {
            var: "P".to_string(), // Motive
            body: Box::new(Term::Lambda {
                var: "step".to_string(), // Step function
                body: Box::new(Term::Lambda {
                    var: "w".to_string(), // W-type value
                    body: Box::new(Term::WRec {
                        motive: Box::new(Term::Var("P".to_string())),
                        step: Box::new(Term::Var("step".to_string())),
                        value: Box::new(Term::Var("w".to_string())),
                    }),
                }),
            }),
        }
    }
}

/// Examples of indexed types and W-types
pub mod examples {
    use super::*;

    /// Create vector type (length-indexed list)
    pub fn vector() -> IndexedType {
        let mut vec = IndexedType::new("Vec");
        
        // Add length index
        vec.add_index(Index::new(
            "n",
            Term::Var("Nat".to_string()),
        ));
        
        // nil constructor: Vec A 0
        vec.add_constructor(Constructor::new(
            "nil",
            vec![],
        ));
        
        // cons constructor: ∀ n, A → Vec A n → Vec A (S n)
        vec.add_constructor(Constructor::new(
            "cons",
            vec![
                Term::Var("n".to_string()),
                Term::Var("A".to_string()),
                Term::Apply {
                    left: Box::new(Term::Apply {
                        left: Box::new(Term::Var("Vec".to_string())),
                        right: Box::new(Term::Var("A".to_string())),
                    }),
                    right: Box::new(Term::Var("n".to_string())),
                },
            ],
        ));
        
        vec
    }

    /// Create binary tree W-type
    pub fn binary_tree() -> WType {
        WType::new(
            Term::Var("A".to_string()), // Node labels
            Term::Bool,                  // Two branches
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::examples::*;

    #[test]
    fn test_vector_type() {
        let vec = vector();
        let ty = vec.get_type();
        assert!(ty.to_string().contains("Vec"));
    }

    #[test]
    fn test_binary_tree() {
        let tree = binary_tree();
        let ty = tree.get_type();
        assert!(ty.to_string().contains("W"));
        
        // Test introduction rule
        let intro = tree.get_intro();
        assert!(intro.to_string().contains("sup"));
        
        // Test elimination rule
        let elim = tree.get_elim();
        assert!(elim.to_string().contains("rec"));
    }

    #[test]
    fn test_indexed_constructors() {
        let mut vec = vector();
        
        // Add element type parameter
        let a_type = Term::Var("A".to_string());
        
        // Test nil constructor
        assert!(vec.constructors[0].args().is_empty());
        
        // Test cons constructor
        let cons = &vec.constructors[1];
        assert_eq!(cons.args().len(), 3);
    }
}
