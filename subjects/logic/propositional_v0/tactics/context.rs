use crate::{
    super::super::super::super::logic::propositional::Proposition,
    crate::foundational_theories::type_theory_v2::calculi::simply_typed::{
        goals::Context, terms::Term, types::Type,
    },
    parse::entities::Identifier,
};
use std::collections::HashSet;

/// A context that supports tactical theorem proving
/// Wraps the basic STLC context and adds proof-specific features
#[derive(Debug, Clone)]
pub struct TacticalContext {
    /// The underlying STLC context for type checking
    stlc_context: Context,
    /// Propositions in this context
    propositions: HashSet<Proposition>,
    /// Parent context in the proof tree
    parent: Option<Box<TacticalContext>>,
}

impl TacticalContext {
    /// Create a new empty tactical context
    pub fn new() -> Self {
        TacticalContext {
            stlc_context: Context::new(),
            propositions: HashSet::new(),
            parent: None,
        }
    }

    /// Create a new tactical context with a parent
    pub fn with_parent(parent: TacticalContext) -> Self {
        TacticalContext {
            stlc_context: Context::new(),
            propositions: HashSet::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Add a proposition to the context
    pub fn add_proposition(&mut self, prop: Proposition) {
        self.propositions.insert(prop);
    }

    /// Get the underlying STLC context
    pub fn stlc_context(&self) -> &Context {
        &self.stlc_context
    }

    /// Get a mutable reference to the underlying STLC context
    pub fn stlc_context_mut(&mut self) -> &mut Context {
        &mut self.stlc_context
    }

    /// Merge another STLC context into this one
    pub fn merge_stlc_context(&mut self, other: Context) {
        self.stlc_context.types.extend(other.types.into_iter());
    }

    /// Look up a type in the context chain
    pub fn get_type(&self, name: &Identifier) -> Option<&Type> {
        // First check current context
        if let Some(ty) = self.stlc_context.get_type(name) {
            return Some(ty);
        }
        // Then check parent context
        if let Some(parent) = &self.parent {
            return parent.get_type(name);
        }
        None
    }

    /// Check if a proposition exists in the context or its parent contexts
    pub fn contains(&self, prop: &Proposition) -> bool {
        // First check current context
        if self.propositions.contains(prop) {
            return true;
        }

        // Then check parent context recursively
        if let Some(parent) = &self.parent {
            return parent.contains(prop);
        }
        false
    }
}

impl IntoIterator for TacticalContext {
    type Item = Proposition;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // Collect propositions from the current context and all parent contexts
        let mut propositions = self.propositions.into_iter().collect::<Vec<_>>();

        let mut current_parent = self.parent;
        while let Some(parent) = current_parent {
            propositions.extend(parent.propositions.iter().cloned());
            current_parent = parent.parent;
        }

        propositions.into_iter()
    }
}

impl<'a> IntoIterator for &'a TacticalContext {
    type Item = &'a Proposition;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // Collect propositions from the current context and all parent contexts
        let mut propositions = self.propositions.iter().collect::<Vec<_>>();

        let mut current_parent = &self.parent;
        while let Some(parent) = current_parent {
            propositions.extend(parent.propositions.iter());
            current_parent = &parent.parent;
        }

        propositions.into_iter()
    }
}

impl Extend<Proposition> for TacticalContext {
    fn extend<T: IntoIterator<Item = Proposition>>(&mut self, iter: T) {
        for prop in iter {
            self.add_proposition(prop);
        }
    }
}
