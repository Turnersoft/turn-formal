//! Path Composition Operations
//! Implements composition operations for higher paths

use crate::foundational_theories::type_theory::{
    core::{Error, Result},
};

use super::{higher_paths::HigherPathContext, HigherPathOps};
use crate::foundational_theories::type_theory::calculi::hott::Term;
use std::collections::HashMap;

/// Composition system for higher paths
#[derive(Debug, Clone)]
pub struct CompositionSystem {
    /// Source type
    source: Term,
    /// Target type
    target: Term,
    /// Composition map
    map: Term,
    /// Path level
    level: usize,
}

impl CompositionSystem {
    /// Create new composition system
    pub fn new(source: Term, target: Term, map: Term, level: usize) -> Self {
        CompositionSystem {
            source,
            target,
            map,
            level,
        }
    }

    /// Apply composition
    pub fn apply(&self, terms: &[Term]) -> Result<Term> {
        // Check all terms are at correct level
        for term in terms {
            if let Term::Path { level, .. } = term {
                if *level != self.level {
                    return Err(Error::TypeError("Invalid path level".to_string()));
                }
            } else {
                return Err(Error::TypeError("Expected path".to_string()));
            }
        }

        Ok(Term::Compose {
            left: Box::new(self.map.clone()),
            right: Box::new(Term::Core(crate::foundational_theories::type_theory::core::Term::Tuple(terms.to_vec()))),
        })
    }
}

/// Context for composition operations
#[derive(Debug, Clone)]
pub struct CompositionContext {
    /// Composition systems by level
    systems: HashMap<usize, Vec<CompositionSystem>>,
    /// Path operations
    path_ops: HigherPathContext,
}

impl CompositionContext {
    /// Create new composition context
    pub fn new(path_ops: HigherPathContext) -> Self {
        CompositionContext {
            systems: HashMap::new(),
            path_ops,
        }
    }

    /// Add system at level
    pub fn add_system(&mut self, system: CompositionSystem, level: usize) {
        self.systems
            .entry(level)
            .or_insert_with(Vec::new)
            .push(system);
    }

    /// Find system at level
    pub fn find_system(
        &self,
        source: &Term,
        target: &Term,
        level: usize,
    ) -> Option<&CompositionSystem> {
        self.systems.get(&level).and_then(|systems| {
            systems
                .iter()
                .find(|s| s.source == *source && s.target == *target)
        })
    }

    /// Compose paths at level
    pub fn compose_paths(&self, paths: &[Term], level: usize) -> Result<Term> {
        if paths.is_empty() {
            return Err(Error::TypeError("Empty path composition".to_string()));
        }

        // Get first and last path
        let first = &paths[0];
        let last = &paths[paths.len() - 1];

        if let (Term::Path { source, .. }, Term::Path { target, .. }) = (first, last) {
            // Find appropriate system
            if let Some(system) = self.find_system(source, target, level) {
                system.apply(paths)
            } else {
                Err(Error::TypeError("No composition system found".to_string()))
            }
        } else {
            Err(Error::TypeError("Expected paths".to_string()))
        }
    }
}

impl super::CompositionOps for CompositionContext {
    fn hcomp(&self, ty: &Term, sides: Vec<Term>, cap: Term) -> Result<Term> {
        Ok(Term::HComp {
            ty: Box::new(ty.clone()),
            sides: sides.into_iter().map(Box::new).collect(),
            cap: Box::new(cap),
        })
    }

    fn comp(&self, ty: &Term, family: Term, sides: Vec<Term>, cap: Term) -> Result<Term> {
        Ok(Term::Comp {
            ty: Box::new(ty.clone()),
            family: Box::new(family),
            sides: sides.into_iter().map(Box::new).collect(),
            cap: Box::new(cap),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composition_system() {
        let system = CompositionSystem::new(
            Term::Unit,
            Term::Unit,
            Term::Core(crate::foundational_theories::type_theory::core::Term::Lambda {
                var: "x".to_string(),
                body: Box::new(crate::foundational_theories::type_theory::core::Term::Var("x".to_string())),
            }),
            1,
        );

        let path = Term::Path {
            source: Box::new(Term::Unit),
            target: Box::new(Term::Unit),
            level: 1,
        };

        assert!(system.apply(&[path]).is_ok());
    }

    #[test]
    fn test_composition_context() {
        let mut ctx = CompositionContext::new(HigherPathContext::new());

        let system = CompositionSystem::new(
            Term::Unit,
            Term::Unit,
            Term::Core(crate::foundational_theories::type_theory::core::Term::Lambda {
                var: "x".to_string(),
                body: Box::new(crate::foundational_theories::type_theory::core::Term::Var("x".to_string())),
            }),
            1,
        );

        ctx.add_system(system, 1);

        let path = Term::Path {
            source: Box::new(Term::Unit),
            target: Box::new(Term::Unit),
            level: 1,
        };

        assert!(ctx.compose_paths(&[path], 1).is_ok());
    }
}
