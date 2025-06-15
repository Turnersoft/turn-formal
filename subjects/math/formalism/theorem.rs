// Module: src/formalize_v2/subjects/math/theorem/core.rs
// Defines core mathematical objects and context for the theorem system

use leptos::math::Math;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::mem::discriminant;
use std::rc::Rc;
use uuid::Uuid;

use crate::subjects::math::formalism::proof::ProofGoal;

use super::expressions::{Identifier, MathExpression, TheoryExpression};
use super::proof::tactics::Tactic;
use super::proof::{ProofForest, ProofNode, ProofStatus};
use super::relations::MathRelation;
use crate::turn_render::ToProofDisplay;
use crate::turn_render::{
    MathDocument, MathDocumentType, PaperType, ScientificPaperContent, ToMathDocument,
};

use super::super::theories::analysis::definition::functions::Function;
use super::super::theories::groups::definitions::{
    Group, GroupOperation, GroupProperty, GroupRelation, LieGroup, TopologicalGroup,
};
use super::super::theories::linear_algebra::definitions::VectorSpace;
use super::super::theories::rings::definitions::{
    Algebra, Field, Module, Ring, RingExpression, RingProperty,
};
use super::super::theories::topology::TopologicalSpace;
use super::super::theories::zfc::Set;

/// A unified wrapper for all mathematical objects across theories
/// This is just a reference to objects defined in their respective theory modules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathObject {
    // Group theory objects
    Group(Group),

    // Ring theory objects
    Ring(Ring),
    Field(Field),
    Module(Module),
    Algebra(Algebra),

    // Topology objects
    TopologicalSpace(TopologicalSpace),

    // Linear algebra objects
    VectorSpace(VectorSpace),

    // Set theory objects
    Set(Set),

    // Analysis objects
    Function(Function),

    // Basic number types
    Integer,
    Rational,
    Irrational,
    Real,
    Complex,

    // General types
    Element(Box<MathObject>),                   // Element of a given type
    Morphism(Box<MathObject>, Box<MathObject>), // Morphism between types

    // Type constructors
    Product(Vec<MathObject>),
    Coproduct(Vec<MathObject>),

    // Other
    // The standard way to address this in systems aiming for
    // HOL/HoTT compatibility is not typically by changing
    // the quantifier structure to directly take a MathRelation or MathExpression.
    // Instead, the MathObject (or a parallel "Type" system) is extended to include variants representing these higher-order concepts:
    // Prop,
    // Type(UnverseLevel),
    // FunctionType(FunctionType),
    Todo(String),
}

/// A unified representation of a mathematical theorem from any domain
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theorem {
    /// Unique identifier for the theorem
    pub id: String,

    /// Human-readable name of the theorem
    pub name: String,

    /// Human-readable description of the theorem
    pub description: String,

    /// The complete proof forest containing the structured proof.
    /// The forest is the single source of truth for the theorem's goal and proof.
    pub proofs: ProofForest,
}

impl Theorem {
    /// Register this theorem in the global registry
    pub fn register_self(&self) {
        println!("Registering theorem: {}", self.name);
        let registry =
            super::super::super::super::subjects::math::formalism::proof::get_theorem_registry();
        registry.lock().unwrap().register(self.clone());
    }

    pub fn get_all_nodes_in_tree(&self, root_id: &str) -> Vec<&ProofNode> {
        let mut result = Vec::new();
        let mut queue = vec![root_id];
        let mut visited = HashSet::new();

        while let Some(node_id) = queue.pop() {
            if visited.contains(node_id) {
                continue;
            }
            visited.insert(node_id);

            if let Some(node) = self.proofs.get_node(node_id) {
                result.push(node);
                for child_id in &node.children {
                    queue.push(child_id);
                }
            }
        }
        result
    }

    pub fn get_all_goals(&self) -> Vec<&ProofGoal> {
        self.proofs.node_values().map(|node| &node.state).collect()
    }

    pub fn get_all_tactics(&self) -> Vec<&Tactic> {
        self.proofs
            .node_values()
            .filter_map(|node| node.tactic.as_ref())
            .collect()
    }

    /// A theorem is proven if all its proof branches are complete.
    pub fn is_proven(&self) -> bool {
        !self.proofs.roots.is_empty()
            && self.proofs.roots.iter().all(|root_id| {
                if let Some(root_node) = self.proofs.get_node(root_id) {
                    self.is_branch_complete(root_node)
                } else {
                    false
                }
            })
    }

    /// Check if a proof has been started (i.e., has at least one node).
    pub fn has_proof_started(&self) -> bool {
        !self.proofs.is_empty()
    }

    /// Recursively checks if a branch is complete
    fn is_branch_complete(&self, node: &ProofNode) -> bool {
        matches!(node.status, ProofStatus::Complete)
    }

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Theorem: {}", self.name)
    }
}

// The ToMathDocument implementation has been moved to src/formalism/render/theorem.rs
// to centralize rendering logic.
