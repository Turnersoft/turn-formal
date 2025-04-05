// Module: src/formalize_v2/subjects/math/theorem/core.rs
// Defines core mathematical objects and context for the theorem system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::subjects::math::theories::analysis::definition::functions::Function;
use crate::subjects::math::theories::groups::definitions::{
    Group, GroupOperation, GroupProperty, GroupRelation, LieGroup, TopologicalGroup,
};
use crate::subjects::math::theories::linear_algebra::definitions::VectorSpace;
use crate::subjects::math::theories::rings::definitions::{
    Algebra, Field, Module, Ring, RingProperty,
};
use crate::subjects::math::theories::topology::TopologicalSpace;
use crate::subjects::math::theories::zfc::Set;

use super::expressions::MathExpression;
// Centralized re-exports for convenient access from other modules
pub use super::properties::{MathProperty, PropertyRequirement};
use super::relations::MathRelation;

/// A mathematical theory context that groups related theorems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathContext {
    GroupTheory,
    RingTheory,
    FieldTheory,
    Topology,
    LinearAlgebra,
    Analysis,
    SetTheory,
    CategoryTheory,
    Custom(String),
}

/// A unified wrapper for all mathematical objects across theories
/// This is just a reference to objects defined in their respective theory modules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathObject {
    // Group theory objects
    Group(Group),
    TopologicalGroup(TopologicalGroup),
    LieGroup(LieGroup),

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
}

/// A unified wrapper for all mathematical operations across theories
/// This is just a reference to operations defined in their respective theory modules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathOperation {
    // Group theory operations
    GroupOperation(GroupOperation),
    // Additional operations can be added here as needed
}

/// Types of mathematical objects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MathObjectType {
    // Group theory types
    Group(Group),
    TopologicalGroup(TopologicalGroup),
    LieGroup(LieGroup),

    // Ring theory types
    Ring(Ring),
    Field(Field),
    Module(Module),
    Algebra(Algebra),

    // Topology types
    TopologicalSpace(TopologicalSpace),

    // Linear algebra types
    VectorSpace(VectorSpace),

    // Set theory types
    Set(Set),

    // Function types
    Function(Function),

    // Basic types
    Integer,
    Rational,
    Real,
    Complex,

    // General types
    Element(Box<MathObjectType>), // Element of a given type
    Morphism(Box<MathObjectType>, Box<MathObjectType>), // Morphism between types

    // Type constructors
    Product(Vec<MathObjectType>),
    Coproduct(Vec<MathObjectType>),

    // Other
    Custom(String),
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
    pub initial_proof_state: ProofState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofState {
    /// Quantified objects in this state
    pub quantifier: Vec<QuantifiedMathObject>,
    /// Variables with assigned values
    pub value_variables: Vec<ValueBindedVariable>,
    /// The main mathematical relation being proven
    pub statement: MathRelation,
    /// Path to this state in the proof (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Justification for reaching this state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
}

impl ProofState {
    /// Create a new proof state for a theorem
    pub fn new(statement: MathRelation) -> Self {
        Self {
            quantifier: vec![],
            value_variables: vec![],
            statement,
            path: Some("p0".to_string()),
            justification: None,
        }
    }

    /// Apply a transformation to this proof state, creating a new state
    pub fn transform(
        &self,
        transform_fn: impl FnOnce(&MathRelation) -> MathRelation,
        path: String,
        justification: String,
    ) -> Self {
        let new_statement = transform_fn(&self.statement);

        Self {
            quantifier: self.quantifier.clone(),
            value_variables: self.value_variables.clone(),
            statement: new_statement,
            path: Some(path),
            justification: Some(justification),
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
        let path_str = self.path.as_deref().unwrap_or("no path");
        let just_str = self.justification.as_deref().unwrap_or("no justification");
        format!("[{}] {} - {:?}", path_str, just_str, self.statement)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueBindedVariable {
    pub variable: String,
    pub value: MathExpression,
    pub math_type: MathObjectType,
}

/// A mathematical object with quantification information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantifiedMathObject {
    /// The variable representing this object
    pub variable: String,

    /// The type of object
    pub object_type: MathObjectType,

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
