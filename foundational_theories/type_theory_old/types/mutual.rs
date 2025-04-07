//! Mutual and nested inductive types
//! Implements support for mutually recursive and nested inductive types

use std::collections::{HashMap, HashSet};
use crate::foundational_theories::type_theory_v2::{
    core::{Term, Result, Error},
    types::{Constructor, InductiveType, TypeConstructor},
};

/// Mutual inductive type block
#[derive(Debug, Clone)]
pub struct MutualBlock {
    /// Types in the block
    types: HashMap<String, InductiveType>,
    /// Dependencies between types
    dependencies: HashMap<String, HashSet<String>>,
}

impl MutualBlock {
    /// Create new mutual block
    pub fn new() -> Self {
        MutualBlock {
            types: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
    
    /// Add type to block
    pub fn add_type(&mut self, ty: InductiveType) -> Result<()> {
        let name = ty.name().to_string();
        if self.types.contains_key(&name) {
            Err(Error::TypeError(format!(
                "Type {} already exists in block",
                name
            )))
        } else {
            self.types.insert(name.clone(), ty);
            self.dependencies.insert(name, HashSet::new());
            Ok(())
        }
    }
    
    /// Add dependency between types
    pub fn add_dependency(&mut self, from: &str, to: &str) -> Result<()> {
        if !self.types.contains_key(from) || !self.types.contains_key(to) {
            Err(Error::TypeError(
                "Types must exist in block".to_string()
            ))
        } else {
            self.dependencies
                .get_mut(from)
                .unwrap()
                .insert(to.to_string());
            Ok(())
        }
    }
    
    /// Check if block is well-founded
    pub fn is_well_founded(&self) -> bool {
        // Check for cycles using DFS
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();
        
        for ty in self.types.keys() {
            if !visited.contains(ty) {
                if self.has_cycle(ty, &mut visited, &mut stack) {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Check for cycles using DFS
    fn has_cycle(
        &self,
        ty: &str,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(ty.to_string());
        stack.insert(ty.to_string());
        
        if let Some(deps) = self.dependencies.get(ty) {
            for dep in deps {
                if !visited.contains(dep) {
                    if self.has_cycle(dep, visited, stack) {
                        return true;
                    }
                } else if stack.contains(dep) {
                    return true;
                }
            }
        }
        
        stack.remove(ty);
        false
    }
    
    /// Get induction principle for block
    pub fn get_induction_principle(&self) -> Result<Term> {
        if !self.is_well_founded() {
            return Err(Error::TypeError(
                "Mutual block is not well-founded".to_string()
            ));
        }
        
        // Create combined motive
        let mut motive = Term::Sort(0);
        for ty in self.types.values() {
            motive = Term::Pi {
                var: format!("P_{}", ty.name()),
                arg_type: Box::new(ty.get_type()),
                body: Box::new(motive),
            };
        }
        
        Ok(motive)
    }
}

/// Nested inductive type
#[derive(Debug, Clone)]
pub struct NestedType {
    /// Base type
    base: InductiveType,
    /// Nested parameter types
    params: Vec<TypeConstructor>,
}

impl NestedType {
    /// Create new nested type
    pub fn new(base: InductiveType) -> Self {
        NestedType {
            base,
            params: Vec::new(),
        }
    }
    
    /// Add nested parameter
    pub fn add_param(&mut self, param: TypeConstructor) {
        self.params.push(param);
    }
    
    /// Get type with parameters applied
    pub fn get_type(&self) -> Term {
        let mut ty = self.base.get_type();
        
        // Apply parameters
        for param in &self.params {
            ty = Term::Apply {
                left: Box::new(ty),
                right: Box::new(param.get_type()),
            };
        }
        
        ty
    }
}

/// Examples of mutual and nested types
pub mod examples {
    use super::*;

    /// Create tree and forest types
    pub fn tree_forest() -> Result<MutualBlock> {
        let mut block = MutualBlock::new();
        
        // Tree type
        let mut tree = InductiveType::new("Tree");
        tree.add_constructor(Constructor::new(
            "node",
            vec![
                Term::Var("a".to_string()),     // Value
                Term::Var("Forest".to_string()), // Children
            ],
        ));
        block.add_type(tree)?;
        
        // Forest type
        let mut forest = InductiveType::new("Forest");
        forest.add_constructor(Constructor::new(
            "nil",
            vec![],
        ));
        forest.add_constructor(Constructor::new(
            "cons",
            vec![
                Term::Var("Tree".to_string()),   // Head
                Term::Var("Forest".to_string()), // Tail
            ],
        ));
        block.add_type(forest)?;
        
        // Add dependencies
        block.add_dependency("Tree", "Forest")?;
        block.add_dependency("Forest", "Tree")?;
        
        Ok(block)
    }

    /// Create nested list type
    pub fn nested_list() -> NestedType {
        let mut list = InductiveType::new("List");
        list.add_constructor(Constructor::new(
            "nil",
            vec![],
        ));
        list.add_constructor(Constructor::new(
            "cons",
            vec![
                Term::Var("a".to_string()),
                Term::Var("List".to_string()),
            ],
        ));
        
        let mut nested = NestedType::new(list);
        nested.add_param(TypeConstructor::new("a"));
        
        nested
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::examples::*;

    #[test]
    fn test_tree_forest() {
        let block = tree_forest().unwrap();
        assert!(block.is_well_founded());
        assert!(block.get_induction_principle().is_ok());
    }

    #[test]
    fn test_nested_list() {
        let nested = nested_list();
        assert!(nested.get_type().to_string().contains("List"));
    }

    #[test]
    fn test_cycle_detection() {
        let mut block = MutualBlock::new();
        
        // Add cyclic types
        let a = InductiveType::new("A");
        let b = InductiveType::new("B");
        let c = InductiveType::new("C");
        
        block.add_type(a).unwrap();
        block.add_type(b).unwrap();
        block.add_type(c).unwrap();
        
        block.add_dependency("A", "B").unwrap();
        block.add_dependency("B", "C").unwrap();
        block.add_dependency("C", "A").unwrap();
        
        assert!(!block.is_well_founded());
    }
}
