// Module: src/formalize_v2/subjects/math/theorem/core.rs
// Defines core mathematical objects and context for the theorem system

use leptos::math::Math;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

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

use super::expressions::{Identifier, MathExpression, TheoryExpression};
// Centralized re-exports for convenient access from other modules

use super::proof::{ProofForest, ProofNode, ProofStatus};
use super::relations::MathRelation;

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

    /// the initial proof state of the theorem as the formal form of the theorem
    pub goal: ProofGoal,

    /// The complete proof forest containing the structured proof
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

    pub fn initialize_branch(&mut self) -> ProofNode {
        let node = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: None,
            children: vec![],
            state: self.goal.clone(),
            tactic: None,
            status: ProofStatus::InProgress,
        };
        self.proofs.add_node(node.clone());
        node
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofGoal {
    /// Quantified objects in this state
    pub quantifier: Vec<QuantifiedMathObject>,
    /// Variables with assigned values
    pub value_variables: Vec<ValueBindedVariable>,
    /// The main mathematical relation being proven
    pub statement: MathRelation,
}

impl ProofGoal {
    /// Create a new proof state for a theorem
    pub fn new(statement: MathRelation) -> Self {
        Self {
            quantifier: vec![],
            value_variables: vec![],
            statement,
        }
    }

    /// Add a quantified object to this state
    pub fn with_quantified_object(&self, object: QuantifiedMathObject) -> Self {
        let mut new_state = self.clone();
        new_state.quantifier.push(object);
        new_state
    }

    /// Add a variable binding to this state
    pub fn with_variable_binding(&self, binding: ValueBindedVariable) -> Self {
        let mut new_state = self.clone();
        new_state.value_variables.push(binding);
        new_state
    }

    /// Format the state for display
    pub fn format(&self) -> String {
        format!("Statement: {:?}", self.statement)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueBindedVariable {
    pub name: Identifier,
    pub value: MathExpression,
}

/// A mathematical object with quantification information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantifiedMathObject {
    /// The variable representing this object
    pub variable: String,

    /// The type of object
    pub object_type: MathObject,

    /// How this object is quantified
    pub quantification: Quantification,

    /// Human-readable description of the object
    pub description: Option<String>,
}

/// Types of quantification for mathematical objects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Quantification {
    /// Universal quantification (∀)
    Universal,

    /// Existential quantification (∃)
    Existential,

    /// Unique existential quantification (∃!)
    UniqueExistential,

    /// Object defined in terms of others
    Defined,

    /// Arbitrary but fixed object
    Fixed,
}

/// Domain-specific mathematical object property
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObjectProperty {
    /// Group theory property
    Group(GroupProperty),

    /// Ring theory property
    Ring(RingProperty),

    /// Set theory property
    Set(String),

    /// Topology property
    Topology(String),

    /// Generic property
    Generic(String),
}

/// Set theory relations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SetRelation {
    // Basic set relations
    ElementOf(String, String),
    SubsetOf(String, String),
    ProperSubsetOf(String, String),
    Equals(String, String),
    Disjoint(String, String),
    Union(String, String, String),
    Intersection(String, String, String),

    // Custom set relation
    Custom(String),
}

/// Topology relations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologyRelation {
    // Basic topology relations
    Open(String),
    Closed(String),
    Connected(String),
    Compact(String),
    Homeomorphic(String, String),

    // Custom topology relation
    Custom(String),
}

/// A step in a proof
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofStep {
    /// The assertion made at this step
    pub assertion: MathRelation,

    /// Justification for this assertion
    pub justification: String,

    /// Previous steps this step depends on
    pub depends_on: Vec<usize>,
}

// Helper trait for inspecting theorem state (useful for testing)
pub trait TheoremExt {
    fn is_complete(&self) -> bool;
    fn has_step_using_theorem(&self, theorem_name: &str) -> bool;
    fn get_case_count(&self) -> usize;
    fn get_step_count(&self) -> usize;
    fn all_proof_steps_finished(&self) -> bool;
    fn proof_tree_is_valid(&self) -> bool;
    fn has_proper_justifications(&self) -> bool;
}

impl TheoremExt for Theorem {
    /// Checks if the main goal of the theorem is marked as proven in the proof forest.
    fn is_complete(&self) -> bool {
        self.proofs.roots.iter().any(|root_id| {
            self.proofs.nodes.get(root_id).map_or(false, |node| {
                node.state == self.goal && matches!(node.status, ProofStatus::Complete)
            })
        })
    }

    /// Recursively checks if any proof step uses a justification referencing a specific theorem name.
    fn has_step_using_theorem(&self, theorem_name: &str) -> bool {
        fn check_node(node: &ProofNode, forest: &ProofForest, theorem_name: &str) -> bool {
            if let Some(tactic) = &node.tactic {
                if format!("{:?}", tactic).contains(theorem_name) {
                    return true;
                }
            }
            node.children.iter().any(|child_id| {
                forest.nodes.get(child_id).map_or(false, |child_node| {
                    check_node(child_node, forest, theorem_name)
                })
            })
        }

        self.proofs.roots.iter().any(|root_id| {
            self.proofs.nodes.get(root_id).map_or(false, |root_node| {
                check_node(root_node, &self.proofs, theorem_name)
            })
        })
    }

    /// Counts the number of distinct proof branches (cases) originating from the root.
    fn get_case_count(&self) -> usize {
        self.proofs.roots.len()
    }

    /// Counts the total number of proof steps (nodes) in the entire proof forest.
    fn get_step_count(&self) -> usize {
        fn count_nodes(node: &ProofNode, forest: &ProofForest) -> usize {
            1 + node
                .children
                .iter()
                .filter_map(|child_id| forest.nodes.get(child_id))
                .map(|child_node| count_nodes(child_node, forest))
                .sum::<usize>()
        }

        self.proofs
            .roots
            .iter()
            .filter_map(|root_id| self.proofs.nodes.get(root_id))
            .map(|root_node| count_nodes(root_node, &self.proofs))
            .sum()
    }

    /// Checks if all nodes in the proof forest are marked as finished/complete.
    fn all_proof_steps_finished(&self) -> bool {
        fn check_node_finished(node: &ProofNode, forest: &ProofForest) -> bool {
            let current_complete = matches!(node.status, ProofStatus::Complete);
            current_complete
                && node
                    .children
                    .iter()
                    .filter_map(|child_id| forest.nodes.get(child_id))
                    .all(|child_node| check_node_finished(child_node, forest))
        }
        self.proofs
            .roots
            .iter()
            .filter_map(|root_id| self.proofs.nodes.get(root_id))
            .all(|root_node| check_node_finished(root_node, &self.proofs))
    }

    /// Basic validation of the proof tree structure (e.g., no cycles, parent pointers okay).
    fn proof_tree_is_valid(&self) -> bool {
        let roots_valid = self
            .proofs
            .roots
            .iter()
            .filter_map(|root_id| self.proofs.nodes.get(root_id))
            .all(|root_node| root_node.parent.is_none());

        roots_valid
    }

    /// Checks if all proof steps have some form of justification provided.
    fn has_proper_justifications(&self) -> bool {
        fn check_node_justification(node: &ProofNode, forest: &ProofForest) -> bool {
            node.tactic.is_some()
                && node
                    .children
                    .iter()
                    .filter_map(|child_id| forest.nodes.get(child_id))
                    .all(|child_node| check_node_justification(child_node, forest))
        }
        self.proofs
            .roots
            .iter()
            .filter_map(|root_id| self.proofs.nodes.get(root_id))
            .all(|root_node| check_node_justification(root_node, &self.proofs))
    }
}
