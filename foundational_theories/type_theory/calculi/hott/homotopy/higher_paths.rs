//! Higher Path Operations
//! Implements operations for higher dimensional paths

use crate::foundational_theories::type_theory::{
    calculi::{hott::Term, Error}, types::path::PathOperations
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Higher path
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HigherPath {
    /// Source path
    pub source: Box<Term>,
    /// Target path
    pub target: Box<Term>,
    /// Path level
    pub level: usize,
}

impl HigherPath {
    /// Create new higher path
    pub fn new(source: Term, target: Term, level: usize) -> Self {
        HigherPath {
            source: Box::new(source),
            target: Box::new(target),
            level,
        }
    }

    /// Get path level
    pub fn level(&self) -> usize {
        self.level
    }

    /// Check if path is valid at its level
    pub fn is_valid(&self) -> Result<bool, String> {
        if self.level == 0 {
            return Err(Error::TypeError("Invalid path level".to_string()));
        }

        // Check source and target are valid at level - 1
        if self.level > 1 {
            let source_path =
                HigherPath::new(*self.source.clone(), *self.source.clone(), self.level - 1);
            let target_path =
                HigherPath::new(*self.target.clone(), *self.target.clone(), self.level - 1);

            source_path.is_valid()?;
            target_path.is_valid()?;
        }

        Ok(true)
    }
}

/// Higher path context with integrated coherence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HigherPathContext {
    /// Path witnesses by dimension
    witnesses: HashMap<usize, Vec<Term>>,
    /// Coherence witnesses
    coherence_witnesses: HashMap<(Term, Term), Term>,
}

impl HigherPathContext {
    /// Create new higher path context
    pub fn new() -> Self {
        HigherPathContext {
            witnesses: HashMap::new(),
            coherence_witnesses: HashMap::new(),
        }
    }

    /// Add witness for path at given dimension
    pub fn add_witness(&mut self, dim: usize, witness: Term) -> Result<()> {
        self.witnesses
            .entry(dim)
            .or_insert_with(Vec::new)
            .push(witness);
        Ok(())
    }

    /// Get witnesses at given dimension
    pub fn get_witnesses(&self, dim: usize) -> Option<&Vec<Term>> {
        self.witnesses.get(&dim)
    }

    /// Add coherence witness
    pub fn add_coherence_witness(&mut self, p: Term, q: Term, witness: Term) {
        self.coherence_witnesses.insert((p, q), witness);
    }

    /// Get coherence witness
    pub fn get_coherence_witness(&self, p: &Term, q: &Term) -> Option<&Term> {
        self.coherence_witnesses.get(&(p.clone(), q.clone()))
    }
}

impl Default for HigherPathContext {
    fn default() -> Self {
        Self::new()
    }
}

impl PathOperations for HigherPathContext {
    fn vcompose(&self, p: &Term, q: &Term) -> Result<Term> {
        match (p, q) {
            (
                Term::Path {
                    level: l1,
                    source: s1,
                    target: t1,
                    ty: ty1,
                },
                Term::Path {
                    level: l2,
                    source: s2,
                    target: t2,
                    ty: ty2,
                },
            ) => {
                if l1 != l2 {
                    return Err(Error::TypeError("Path levels do not match".to_string()));
                }

                // Check if endpoints match
                if t1 != s2 {
                    return Err(Error::TypeError("Path endpoints do not match".to_string()));
                }

                // Check if types match
                if ty1 != ty2 {
                    return Err(Error::TypeError("Path types do not match".to_string()));
                }

                if let Some(witnesses) = self.get_witnesses(*l1) {
                    // Create vertical composition
                    Ok(Term::Path {
                        level: *l1,
                        source: s1.clone(),
                        target: t2.clone(),
                        ty: ty1.clone(),
                    })
                } else {
                    Err(Error::TypeError(format!("No witnesses for level {}", l1)))
                }
            }
            _ => Err(Error::TypeError("Expected path terms".to_string())),
        }
    }

    fn hcompose(&self, p: &Term, q: &Term) -> Result<Term> {
        match (p, q) {
            (
                Term::Path {
                    level: l1,
                    source: s1,
                    target: t1,
                    ty: ty1,
                },
                Term::Path {
                    level: l2,
                    source: s2,
                    target: t2,
                    ty: ty2,
                },
            ) => {
                if l1 != l2 {
                    return Err(Error::TypeError("Path levels do not match".to_string()));
                }

                // Check if sources and targets match
                if s1 != s2 || t1 != t2 {
                    return Err(Error::TypeError("Path endpoints do not match".to_string()));
                }

                // Check if types match
                if ty1 != ty2 {
                    return Err(Error::TypeError("Path types do not match".to_string()));
                }

                if let Some(witnesses) = self.get_witnesses(*l1) {
                    // Create horizontal composition
                    Ok(Term::Path {
                        level: l1 + 1,
                        source: Box::new(p.clone()),
                        target: Box::new(q.clone()),
                        ty: ty1.clone(),
                    })
                } else {
                    Err(Error::TypeError(format!("No witnesses for level {}", l1)))
                }
            }
            _ => Err(Error::TypeError("Expected path terms".to_string())),
        }
    }

    fn whisker(&self, p: &Term, q: &Term) -> Result<Term> {
        match (p, q) {
            (
                Term::Path {
                    level: l1,
                    source: s1,
                    target: t1,
                    ty: ty1,
                },
                Term::Path {
                    level: l2,
                    source: s2,
                    target: t2,
                    ty: ty2,
                },
            ) => {
                // Whisker operation increases the level
                if l1 + 1 != *l2 {
                    return Err(Error::TypeError(
                        "Second path must be one level higher".to_string(),
                    ));
                }

                // Check if types match
                if ty1 != ty2 {
                    return Err(Error::TypeError("Path types do not match".to_string()));
                }

                if let Some(witnesses) = self.get_witnesses(*l2) {
                    // Create whisker composition
                    Ok(Term::Path {
                        level: *l2,
                        source: Box::new(self.vcompose(p, &s2)?),
                        target: Box::new(self.vcompose(p, &t2)?),
                        ty: ty1.clone(),
                    })
                } else {
                    Err(Error::TypeError(format!("No witnesses for level {}", l2)))
                }
            }
            _ => Err(Error::TypeError("Expected path terms".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::foundational_theories::type_theory::core::Term;

    fn create_test_path(level: usize, source: Term, target: Term) -> Term {
        Term::Path {
            level,
            source: Box::new(source),
            target: Box::new(target),
            ty: Box::new(Term::Unit),
        }
    }

    #[test]
    fn test_higher_path() {
        let source = Term::Var("x".to_string());
        let target = Term::Var("y".to_string());
        let path = HigherPath::new(source, target, 2);

        assert!(path.is_valid().unwrap_or(false));
        assert_eq!(path.level(), 2);
    }

    #[test]
    fn test_vertical_composition() {
        let mut ctx = HigherPathContext::new();
        ctx.add_witness(1, Term::Unit).unwrap();

        let p = create_test_path(1, Term::Unit, Term::Unit);
        let q = create_test_path(1, Term::Unit, Term::Unit);

        let result = ctx.vcompose(&p, &q);
        assert!(result.is_ok());

        let composed = result.unwrap();
        match composed {
            Term::Path { level, .. } => {
                assert_eq!(level, 1);
            }
            _ => panic!("Expected path term"),
        }
    }

    #[test]
    fn test_horizontal_composition() {
        let mut ctx = HigherPathContext::new();
        ctx.add_witness(1, Term::Unit).unwrap();

        let p = create_test_path(1, Term::Unit, Term::Unit);
        let q = create_test_path(1, Term::Unit, Term::Unit);

        let result = ctx.hcompose(&p, &q);
        assert!(result.is_ok());

        let composed = result.unwrap();
        match composed {
            Term::Path { level, .. } => {
                assert_eq!(level, 2);
            }
            _ => panic!("Expected path term"),
        }
    }

    #[test]
    fn test_whisker() {
        let mut ctx = HigherPathContext::new();
        ctx.add_witness(1, Term::Unit).unwrap();
        ctx.add_witness(2, Term::Unit).unwrap();

        let p = create_test_path(1, Term::Unit, Term::Unit);
        let q = create_test_path(2, Term::Unit, Term::Unit);

        let result = ctx.whisker(&p, &q);
        assert!(result.is_ok());

        let whiskered = result.unwrap();
        match whiskered {
            Term::Path { level, .. } => {
                assert_eq!(level, 2);
            }
            _ => panic!("Expected path term"),
        }
    }

    #[test]
    fn test_invalid_composition() {
        let ctx = HigherPathContext::new();
        let p = create_test_path(1, Term::Unit, Term::Unit);
        let q = create_test_path(2, Term::Unit, Term::Unit);

        let result = ctx.vcompose(&p, &q);
        assert!(result.is_err());
    }
}
