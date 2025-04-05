//! Path algebra operations
//! Implements path operations for homotopy type theory

use crate::foundational_theories::type_theory::{
    core::{Error, Result, Term},
    types::hits::PathConstructor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Path level information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathLevel(pub usize);

impl PathLevel {
    /// Create new path level
    pub fn new(level: usize) -> Self {
        PathLevel(level)
    }

    /// Get next level
    pub fn next(&self) -> Self {
        PathLevel(self.0 + 1)
    }

    /// Get previous level
    pub fn prev(&self) -> Option<Self> {
        if self.0 > 0 {
            Some(PathLevel(self.0 - 1))
        } else {
            None
        }
    }
}

/// Path composition operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathComposition {
    /// First path
    first: Box<Term>,
    /// Second path
    second: Box<Term>,
    /// Path level
    level: PathLevel,
}

impl PathComposition {
    /// Create new path composition
    pub fn new(first: Term, second: Term, level: PathLevel) -> Self {
        PathComposition {
            first: Box::new(first),
            second: Box::new(second),
            level,
        }
    }

    /// Get composed path
    pub fn compose(&self) -> Term {
        Term::Compose {
            left: self.first.clone(),
            right: self.second.clone(),
        }
    }

    /// Get level
    pub fn level(&self) -> PathLevel {
        self.level
    }
}

/// Path inverse operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathInverse {
    /// Path to invert
    path: Box<Term>,
    /// Path level
    level: PathLevel,
}

impl PathInverse {
    /// Create new path inverse
    pub fn new(path: Term, level: PathLevel) -> Self {
        PathInverse {
            path: Box::new(path),
            level,
        }
    }

    /// Get inverted path
    pub fn invert(&self) -> Term {
        Term::Inverse {
            path: self.path.clone(),
        }
    }

    /// Get level
    pub fn level(&self) -> PathLevel {
        self.level
    }
}

/// Path operations trait
pub trait PathOperations {
    fn vcompose(&self, p: &Term, q: &Term) -> Result<Term>;
    fn hcompose(&self, p: &Term, q: &Term) -> Result<Term>;
    fn whisker(&self, p: &Term, q: &Term) -> Result<Term>;
}

/// Path operations state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathContext {
    /// Path constructors
    constructors: HashMap<String, PathConstructor>,
    /// Current path level
    current_level: PathLevel,
}

impl PathContext {
    pub fn new() -> Self {
        PathContext {
            constructors: HashMap::new(),
            current_level: PathLevel::new(0),
        }
    }

    pub fn add_constructor(&mut self, constructor: PathConstructor) {
        self.constructors
            .insert(constructor.name.clone(), constructor);
    }

    /// Set current path level
    pub fn set_level(&mut self, level: PathLevel) {
        self.current_level = level;
    }

    /// Get current path level
    pub fn current_level(&self) -> PathLevel {
        self.current_level
    }

    /// Compose paths
    pub fn compose(&self, p: Term, q: Term) -> Result<Term> {
        // Check endpoints match
        self.check_endpoints(&p, &q)?;

        // Get path level
        let level = self.get_path_level(&p)?;
        if level != self.get_path_level(&q)? {
            return Err(Error::TypeError("Path levels do not match".to_string()));
        }

        Ok(PathComposition::new(p, q, level).compose())
    }

    /// Get path level
    fn get_path_level(&self, term: &Term) -> Result<PathLevel> {
        match term {
            Term::Path { level, .. } => Ok(PathLevel(*level)),
            _ => Err(Error::TypeError("Expected path".to_string())),
        }
    }

    /// Invert path
    pub fn invert(&self, p: Term) -> Term {
        PathInverse::new(p, self.get_path_level(&p).unwrap()).invert()
    }

    /// Check if path endpoints match
    fn check_endpoints(&self, p: &Term, q: &Term) -> Result<()> {
        match (p, q) {
            (
                Term::Path {
                    source: s1,
                    target: t1,
                    ..
                },
                Term::Path {
                    source: s2,
                    target: t2,
                    ..
                },
            ) => {
                if t1 != s2 {
                    Err(Error::TypeError("Path endpoints do not match".to_string()))
                } else {
                    Ok(())
                }
            }
            _ => Err(Error::TypeError("Expected paths".to_string())),
        }
    }
}

impl PathOperations for PathContext {
    fn vcompose(&self, p: &Term, q: &Term) -> Result<Term> {
        // Check levels match
        let p_level = self.get_path_level(p)?;
        let q_level = self.get_path_level(q)?;
        if p_level != q_level {
            return Err(Error::TypeError("Path levels do not match".to_string()));
        }
        Ok(Term::VCompose {
            first: Box::new(p.clone()),
            second: Box::new(q.clone()),
        })
    }

    fn hcompose(&self, p: &Term, q: &Term) -> Result<Term> {
        // Similar to vcompose
        Ok(Term::HCompose {
            first: Box::new(p.clone()),
            second: Box::new(q.clone()),
        })
    }

    fn whisker(&self, p: &Term, q: &Term) -> Result<Term> {
        Ok(Term::Whisker {
            left: Box::new(p.clone()),
            right: Box::new(q.clone()),
            level: 1,
        })
    }
}

/// Path groupoid operations
pub trait PathGroupoid {
    /// Identity path
    fn refl(&self, x: Term) -> Term;

    /// Path composition
    fn compose(&self, p: Term, q: Term) -> Result<Term>;

    /// Path inverse
    fn invert(&self, p: Term) -> Term;
}

impl PathGroupoid for PathContext {
    fn refl(&self, x: Term) -> Term {
        Term::Path {
            source: Box::new(x.clone()),
            target: Box::new(x),
            level: 1,
        }
    }

    fn compose(&self, p: Term, q: Term) -> Result<Term> {
        self.compose(p, q)
    }

    fn invert(&self, p: Term) -> Term {
        self.invert(p)
    }
}

/// Path homotopy operations
pub trait PathHomotopy {
    /// Vertical composition of homotopies
    fn vcompose(&self, α: Term, β: Term) -> Result<Term>;

    /// Horizontal composition of homotopies
    fn hcompose(&self, α: Term, β: Term) -> Result<Term>;

    /// Coherence witness between paths
    fn coherence(&self, p: Term, q: Term) -> Result<Term>;

    /// Whisker operation
    fn whisker(&self, p: Term, q: Term) -> Result<Term>;

    /// Transport along path
    fn transport(&self, p: Term, ty: Term, x: Term) -> Result<Term>;
}

impl PathHomotopy for PathContext {
    fn vcompose(&self, α: Term, β: Term) -> Result<Term> {
        // Check levels match
        let α_level = self.get_path_level(&α)?;
        let β_level = self.get_path_level(&β)?;

        if α_level != β_level {
            return Err(Error::TypeError(
                "Path levels do not match for vertical composition".to_string(),
            ));
        }

        Ok(Term::VCompose {
            first: Box::new(α),
            second: Box::new(β),
        })
    }

    fn hcompose(&self, α: Term, β: Term) -> Result<Term> {
        // Check levels match
        let α_level = self.get_path_level(&α)?;
        let β_level = self.get_path_level(&β)?;

        if α_level != β_level {
            return Err(Error::TypeError(
                "Path levels do not match for horizontal composition".to_string(),
            ));
        }

        Ok(Term::HCompose {
            first: Box::new(α),
            second: Box::new(β),
        })
    }

    fn coherence(&self, p: Term, q: Term) -> Result<Term> {
        // Check levels match and are > 0
        let p_level = self.get_path_level(&p)?;
        let q_level = self.get_path_level(&q)?;

        if p_level != q_level {
            return Err(Error::TypeError(
                "Path levels do not match for coherence".to_string(),
            ));
        }

        if p_level.0 == 0 {
            return Err(Error::TypeError(
                "Cannot construct coherence at level 0".to_string(),
            ));
        }

        Ok(Term::Coherence {
            first: Box::new(p),
            second: Box::new(q),
        })
    }

    fn whisker(&self, p: Term, q: Term) -> Result<Term> {
        // Check levels
        let p_level = self.get_path_level(&p)?;
        let q_level = self.get_path_level(&q)?;

        Ok(Term::Whisker {
            left: Box::new(p),
            right: Box::new(q),
            level: p_level.0.max(q_level.0),
        })
    }

    fn transport(&self, p: Term, ty: Term, x: Term) -> Result<Term> {
        // Check p is a path
        self.get_path_level(&p)?;

        Ok(Term::Transport {
            path: Box::new(p),
            ty: Box::new(ty),
            term: Box::new(x),
        })
    }
}

/// Examples of path operations
pub mod examples {
    use super::*;

    /// Create circle path operations
    pub fn circle_paths() -> PathContext {
        let mut ops = PathContext::new();

        // Add loop constructor
        ops.add_constructor(PathConstructor::new(
            "loop",
            Term::Var("base".to_string()),
            Term::Var("base".to_string()),
            vec![],
        ));

        ops
    }

    /// Create interval path operations
    pub fn interval_paths() -> PathContext {
        let mut ops = PathContext::new();

        // Add segment constructor
        ops.add_constructor(PathConstructor::new(
            "seg",
            Term::Var("zero".to_string()),
            Term::Var("one".to_string()),
            vec![],
        ));

        ops
    }
}

#[cfg(test)]
mod tests {
    use super::examples::*;
    use super::*;

    #[test]
    fn test_path_composition() {
        let ops = circle_paths();

        let loop1 = Term::Path {
            source: Box::new(Term::Var("base".to_string())),
            target: Box::new(Term::Var("base".to_string())),
            level: 1,
        };

        let loop2 = loop1.clone();

        assert!(ops.compose(loop1, loop2).is_ok());
    }

    #[test]
    fn test_path_inverse() {
        let ops = interval_paths();

        let seg = Term::Path {
            source: Box::new(Term::Var("zero".to_string())),
            target: Box::new(Term::Var("one".to_string())),
            level: 1,
        };

        let inv = ops.invert(seg);
        assert!(matches!(inv, Term::Inverse { .. }));
    }

    #[test]
    fn test_path_homotopy() {
        let ops = circle_paths();

        let α = Term::Path {
            source: Box::new(Term::Var("loop".to_string())),
            target: Box::new(Term::Var("loop".to_string())),
            level: 2,
        };

        let β = α.clone();

        assert!(ops.vcompose(α.clone(), β.clone()).is_ok());
        assert!(ops.hcompose(α, β).is_ok());
    }
}
