//! Higher Inductive Types (HITs)
//! Implements types with both point and path constructors

use super::super::core::{Term, Result, Error};
use super::{TypeConstructor, TypeEliminator};
use serde::{Deserialize, Serialize};

/// Path constructor declaration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathConstructor {
    /// Name of the constructor
    pub name: String,
    /// Source point
    pub source: Box<Term>,
    /// Target point
    pub target: Box<Term>,
    /// Additional parameters (name, type)
    pub params: Vec<(String, Term)>,
}

impl PathConstructor {
    /// Create new path constructor
    pub fn new(
        name: impl Into<String>,
        source: Term,
        target: Term,
        params: Vec<(String, Term)>,
    ) -> Self {
        PathConstructor {
            name: name.into(),
            source: Box::new(source),
            target: Box::new(target),
            params,
        }
    }
}

/// Higher path constructor (2-paths and above)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HigherPathConstructor {
    /// Name of the constructor
    name: String,
    /// Source path
    source: Box<Term>,
    /// Target path
    target: Box<Term>,
    /// Path level (2 for 2-paths, etc)
    level: usize,
    /// Additional parameters
    params: Vec<(String, Term)>,
}

impl HigherPathConstructor {
    /// Create new higher path constructor
    pub fn new(
        name: impl Into<String>,
        source: Term,
        target: Term,
        level: usize,
        params: Vec<(String, Term)>,
    ) -> Self {
        HigherPathConstructor {
            name: name.into(),
            source: Box::new(source),
            target: Box::new(target),
            level,
            params,
        }
    }
}

/// Coherence condition between paths
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoherenceCondition {
    /// Name of the condition
    name: String,
    /// Paths involved
    paths: Vec<String>,
    /// Equation to satisfy
    equation: Term,
}

impl CoherenceCondition {
    /// Create new coherence condition
    pub fn new(
        name: impl Into<String>,
        paths: Vec<String>,
        equation: Term,
    ) -> Self {
        CoherenceCondition {
            name: name.into(),
            paths,
            equation,
        }
    }
}

/// Higher inductive type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hit {
    /// Name of the type
    name: String,
    /// Parameters (name, type)
    params: Vec<(String, Term)>,
    /// Point constructors
    point_constructors: Vec<(String, Vec<(String, Term)>)>,
    /// Path constructors
    path_constructors: Vec<PathConstructor>,
    /// Higher path constructors
    higher_paths: Vec<HigherPathConstructor>,
    /// Coherence conditions
    coherence: Vec<CoherenceCondition>,
}

impl Hit {
    /// Create new higher inductive type
    pub fn new(
        name: impl Into<String>,
        params: Vec<(String, Term)>,
        point_constructors: Vec<(String, Vec<(String, Term)>)>,
        path_constructors: Vec<PathConstructor>,
    ) -> Self {
        Hit {
            name: name.into(),
            params,
            point_constructors,
            path_constructors,
            higher_paths: vec![],
            coherence: vec![],
        }
    }
    
    /// Get point constructor by name
    pub fn get_point_constructor(&self, name: &str) -> Option<&Vec<(String, Term)>> {
        self.point_constructors
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, args)| args)
    }
    
    /// Get path constructor by name
    pub fn get_path_constructor(&self, name: &str) -> Option<&PathConstructor> {
        self.path_constructors
            .iter()
            .find(|p| p.name == name)
    }
    
    /// Add higher path constructor
    pub fn add_higher_path(
        &mut self,
        constructor: HigherPathConstructor,
    ) {
        self.higher_paths.push(constructor);
    }
    
    /// Add coherence condition
    pub fn add_coherence(
        &mut self,
        condition: CoherenceCondition,
    ) {
        self.coherence.push(condition);
    }
}

impl TypeConstructor for Hit {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::Apply { left, right: _ } => {
                match &**left {
                    Term::Var(name) => {
                        // Check if it's a valid point or path constructor
                        if self.get_point_constructor(name).is_some() ||
                           self.get_path_constructor(name).is_some() {
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
            _ => Err(Error::TypeError("Expected HIT term".to_string())),
        }
    }
    
    fn universe_level(&self) -> usize {
        // HIT lives in universe specified by constructors
        // This is simplified; should compute actual level
        1 // HITs typically live in at least universe 1
    }
}

/// HIT elimination principle
pub struct HitElim {
    /// The HIT being eliminated
    hit: Hit,
    /// Methods for point constructors
    point_methods: Vec<(String, Term)>,
    /// Methods for path constructors
    path_methods: Vec<(String, Term)>,
}

impl HitElim {
    /// Create new HIT eliminator
    pub fn new(
        hit: Hit,
        point_methods: Vec<(String, Term)>,
        path_methods: Vec<(String, Term)>,
    ) -> Self {
        HitElim {
            hit,
            point_methods,
            path_methods,
        }
    }
}

impl TypeEliminator for HitElim {
    fn eliminate(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Apply { left, right: _ } => {
                match &**left {
                    Term::Var(name) => {
                        // Try point methods first
                        if let Some(method) = self.point_methods
                            .iter()
                            .find(|(n, _)| n == name)
                            .map(|(_, t)| t.clone())
                        {
                            return Ok(method);
                        }
                        
                        // Then try path methods
                        if let Some(method) = self.path_methods
                            .iter()
                            .find(|(n, _)| n == name)
                            .map(|(_, t)| t.clone())
                        {
                            return Ok(method);
                        }
                        
                        Err(Error::TypeError(format!(
                            "No method for constructor: {}", name
                        )))
                    }
                    _ => Err(Error::TypeError("Expected constructor".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected HIT term".to_string())),
        }
    }
}

/// Example: Circle as a HIT
pub fn circle_hit() -> Hit {
    Hit::new(
        "Circle",
        vec![],
        // Point constructor: base : Circle
        vec![("base".to_string(), vec![])],
        // Path constructor: loop : base = base
        vec![PathConstructor::new(
            "loop",
            Term::Var("base".to_string()),
            Term::Var("base".to_string()),
            vec![],
        )],
    )
}

/// Example: Torus as a HIT with higher paths
pub fn torus_hit() -> Hit {
    let mut torus = Hit::new(
        "Torus",
        vec![],
        // Point constructor: base : Torus
        vec![("base".to_string(), vec![])],
        // Path constructors: p, q : base = base
        vec![
            PathConstructor::new(
                "p",
                Term::Var("base".to_string()),
                Term::Var("base".to_string()),
                vec![],
            ),
            PathConstructor::new(
                "q",
                Term::Var("base".to_string()),
                Term::Var("base".to_string()),
                vec![],
            ),
        ],
    );
    
    // Add 2-path: t : p ∘ q = q ∘ p
    torus.add_higher_path(HigherPathConstructor::new(
        "t",
        Term::Compose {
            left: Box::new(Term::Var("p".to_string())),
            right: Box::new(Term::Var("q".to_string())),
        },
        Term::Compose {
            left: Box::new(Term::Var("q".to_string())),
            right: Box::new(Term::Var("p".to_string())),
        },
        2,
        vec![],
    ));
    
    torus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_hit() {
        let circle = circle_hit();
        
        // Test base point
        let base = Term::Apply {
            left: Box::new(Term::Var("base".to_string())),
            right: Box::new(Term::Var("".to_string())),
        };
        assert!(circle.check_term(&base).is_ok());
        
        // Test loop path
        let loop_path = Term::Apply {
            left: Box::new(Term::Var("loop".to_string())),
            right: Box::new(Term::Var("".to_string())),
        };
        assert!(circle.check_term(&loop_path).is_ok());
    }

    #[test]
    fn test_circle_elim() {
        let circle = circle_hit();
        
        // Eliminator for computing winding number
        let elim = HitElim::new(
            circle,
            vec![("base".to_string(), Term::Var("0".to_string()))],
            vec![("loop".to_string(), Term::Var("succ".to_string()))],
        );
        
        // Test elimination on base point
        let base = Term::Apply {
            left: Box::new(Term::Var("base".to_string())),
            right: Box::new(Term::Var("".to_string())),
        };
        
        let result = elim.eliminate(&base).unwrap();
        assert_eq!(result, Term::Var("0".to_string()));
    }

    #[test]
    fn test_torus_hit() {
        let torus = torus_hit();
        
        // Test paths
        assert!(torus.get_path_constructor("p").is_some());
        assert!(torus.get_path_constructor("q").is_some());
        
        // Test higher path
        assert!(torus.higher_paths.iter().any(|p| p.name == "t"));
    }
}