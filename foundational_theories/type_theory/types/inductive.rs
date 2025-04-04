//! Inductive types
//! Implements general inductive type families

use std::collections::HashMap;
use super::super::core::{Term, Result, Error};
use super::{TypeConstructor, TypeEliminator};

/// Constructor declaration
#[derive(Debug, Clone)]
pub struct Constructor {
    /// Name of the constructor
    name: String,
    /// Arguments to the constructor (name, type)
    args: Vec<(String, Term)>,
    /// Indices for the constructed value
    indices: Vec<Term>,
}

impl Constructor {
    /// Create new constructor
    pub fn new(
        name: impl Into<String>,
        args: Vec<(String, Term)>,
        indices: Vec<Term>,
    ) -> Self {
        Constructor {
            name: name.into(),
            args,
            indices,
        }
    }
}

/// Inductive type family
#[derive(Debug, Clone)]
pub struct Inductive {
    /// Name of the type
    name: String,
    /// Parameters (name, type)
    params: Vec<(String, Term)>,
    /// Indices (name, type)
    indices: Vec<(String, Term)>,
    /// Sort (Type or Prop)
    sort: Term,
    /// Constructors
    constructors: Vec<Constructor>,
}

impl Inductive {
    /// Create new inductive type
    pub fn new(
        name: impl Into<String>,
        params: Vec<(String, Term)>,
        indices: Vec<(String, Term)>,
        sort: Term,
        constructors: Vec<Constructor>,
    ) -> Self {
        Inductive {
            name: name.into(),
            params,
            indices,
            sort,
            constructors,
        }
    }
    
    /// Get constructor by name
    pub fn get_constructor(&self, name: &str) -> Option<&Constructor> {
        self.constructors.iter().find(|c| c.name == name)
    }
}

impl TypeConstructor for Inductive {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::Apply { left, right: _ } => {
                match &**left {
                    Term::Var(name) => {
                        // Check if it's a valid constructor
                        if self.get_constructor(name).is_some() {
                            // Should check arguments and indices
                            Ok(())
                        } else {
                            Err(Error::TypeError(format!(
                                "Unknown constructor: {}", name
                            )))
                        }
                    }
                    _ => Err(Error::TypeError("Expected constructor".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected inductive term".to_string())),
        }
    }
    
    fn universe_level(&self) -> usize {
        // Inductive type lives in universe specified by sort
        // This is simplified; should compute actual level
        0
    }
}

/// Recursor (simple elimination)
pub struct Rec {
    /// The inductive type being eliminated
    ind: Inductive,
    /// Return type
    motive: Box<Term>,
    /// Methods for each constructor
    methods: HashMap<String, Term>,
}

impl Rec {
    /// Create new recursor
    pub fn new(
        ind: Inductive,
        motive: Term,
        methods: HashMap<String, Term>,
    ) -> Self {
        Rec {
            ind,
            motive: Box::new(motive),
            methods,
        }
    }
}

impl TypeEliminator for Rec {
    fn eliminate(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Apply { left, right: _ } => {
                match &**left {
                    Term::Var(name) => {
                        // Get method for this constructor
                        self.methods.get(name)
                            .cloned()
                            .ok_or_else(|| Error::TypeError(
                                format!("No method for constructor: {}", name)
                            ))
                    }
                    _ => Err(Error::TypeError("Expected constructor".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected inductive term".to_string())),
        }
    }
}

/// Example: Natural numbers as an inductive type
pub fn nat_type() -> Inductive {
    Inductive::new(
        "Nat",
        vec![],
        vec![],
        Term::Var("Type".to_string()),
        vec![
            // zero : Nat
            Constructor::new("zero", vec![], vec![]),
            // succ : Nat → Nat
            Constructor::new(
                "succ",
                vec![("n".to_string(), Term::Var("Nat".to_string()))],
                vec![],
            ),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nat_type() {
        let nat = nat_type();
        
        // Test zero
        let zero = Term::Apply {
            left: Box::new(Term::Var("zero".to_string())),
            right: Box::new(Term::Var("".to_string())),
        };
        assert!(nat.check_term(&zero).is_ok());
        
        // Test succ
        let one = Term::Apply {
            left: Box::new(Term::Var("succ".to_string())),
            right: Box::new(zero),
        };
        assert!(nat.check_term(&one).is_ok());
    }

    #[test]
    fn test_nat_rec() {
        let nat = nat_type();
        
        // Recursor for addition
        let mut methods = HashMap::new();
        methods.insert("zero".to_string(), Term::Var("m".to_string()));
        methods.insert("succ".to_string(), Term::Var("succ_case".to_string()));
        
        let rec = Rec::new(
            nat,
            Term::Var("Nat".to_string()),
            methods,
        );
        
        // Test elimination on zero
        let zero = Term::Apply {
            left: Box::new(Term::Var("zero".to_string())),
            right: Box::new(Term::Var("".to_string())),
        };
        
        let result = rec.eliminate(&zero).unwrap();
        assert_eq!(result, Term::Var("m".to_string()));
    }
}
