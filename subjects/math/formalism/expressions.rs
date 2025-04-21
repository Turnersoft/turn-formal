// Module: src/formalize_v2/subjects/math/theorem/expressions.rs
// Defines expressions used in mathematical statements and theorems

use serde::{Deserialize, Serialize};

use super::super::theories::{
    GroupExpression,
    rings::definitions::{FieldExpression, RingExpression},
};

use super::super::theories::{
    groups::Group,
    number_theory::definitions::Number,
    rings::{Ring, definitions::Field},
};

use super::super::formalism::interpretation::TypeViewOperator;
use super::{
    core::{MathObject, MathObjectType},
    relations::MathRelation,
};

/// Variables for use in expressions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Identifier {
    /// Object variables
    O(u8),

    /// Morphism variables
    M(u8),

    /// Element variables
    E(u8),

    /// Number variables
    N(u8),

    /// custom name that you really want to costomize
    /// Named variables with an identifier
    /// The string is the human-readable name
    /// The u32 is a unique identifier to distinguish variables with the same name
    Name(String, u32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TheoryExpression {
    Group(GroupExpression),
    Ring(RingExpression),
    Field(FieldExpression),
}

/// A unified mathematical expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MathExpression {
    /// Variable reference
    Var(Identifier),

    /// Reference to a mathematical object
    Object(MathObject),

    Expression(TheoryExpression),

    /// treating math relationships as first-flass citizens
    /// this makes relation a sub class of MathExpression, but it is not, we can choose which
    /// node is the root node at a given problem.
    Relation(Box<MathRelation>),

    /// Numeric value
    Number(Number),

    /// An expression with a specific type view
    ViewAs {
        /// The original expression
        expression: Box<MathExpression>,
        /// The view operator
        view: TypeViewOperator,
    },
}

/// Errors that can occur during type checking of views
#[derive(Debug, Clone)]
pub enum TypeViewError {
    /// Error when trying to view an expression as an incompatible type
    InvalidView {
        expression_type: MathObjectType,
        view_type: String,
        message: String,
    },
    /// Other error types can be added as needed
    Other(String),
}

impl std::fmt::Display for TypeViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeViewError::InvalidView {
                expression_type,
                view_type,
                message,
            } => {
                write!(
                    f,
                    "Invalid view: cannot view {:?} using {:?}. {}",
                    expression_type, view_type, message
                )
            }
            TypeViewError::Other(msg) => {
                write!(f, "Type view error: {}", msg)
            }
        }
    }
}

impl std::error::Error for TypeViewError {}

// Implement From to allow ? operator with TypeViewError
impl From<&str> for TypeViewError {
    fn from(message: &str) -> Self {
        TypeViewError::Other(message.to_string())
    }
}

impl From<String> for TypeViewError {
    fn from(message: String) -> Self {
        TypeViewError::Other(message)
    }
}

impl MathExpression {
    /// Create a variable expression from a name
    pub fn var(name: &str) -> Self {
        MathExpression::Var(Identifier::Name(
            name.to_string(),
            name.bytes().fold(0, |acc, b| acc + b as u32),
        ))
    }

    /// Create a variable expression with an explicit identifier
    pub fn var_with_id(name: &str, id: u32) -> Self {
        MathExpression::Var(Identifier::Name(name.to_string(), id))
    }

    /// Apply a type view to this expression
    pub fn with_view(self, view: TypeViewOperator) -> Self {
        MathExpression::ViewAs {
            expression: Box::new(self),
            view,
        }
    }

    /// View this expression as a specific type
    pub fn view_as(self, type_name: &str) -> Self {
        self.with_view(TypeViewOperator::simple_view(type_name))
    }

    /// Helper to check if an expression can be viewed in a particular way
    pub fn can_view_as(&self, view: &TypeViewOperator) -> bool {
        // Simplified implementation while the main view methods are removed
        true
    }

    /// Infer the type of an expression
    pub fn infer_type(&self) -> String {
        match self {
            MathExpression::Var(Identifier::Name(name, _)) => format!("Variable({})", name),
            MathExpression::Var(_) => "Variable".to_string(),
            MathExpression::Object(_) => "Object".to_string(),
            MathExpression::Number(_) => "Number".to_string(),
            MathExpression::Relation(_) => "Relation".to_string(),
            MathExpression::Expression(theory_expr) => match theory_expr {
                TheoryExpression::Group(_) => "GroupExpression".to_string(),
                TheoryExpression::Ring(_) => "RingExpression".to_string(),
                TheoryExpression::Field(_) => "FieldExpression".to_string(),
            },
            MathExpression::ViewAs { expression, view } => {
                format!("{} viewed as {:?}", expression.infer_type(), view)
            }
        }
    }

    /// Returns true if this expression is a view of another expression
    pub fn is_view(&self) -> bool {
        matches!(self, MathExpression::ViewAs { .. })
    }

    /// Get the variable name if this is a variable expression
    pub fn as_variable_name(&self) -> Option<String> {
        match self {
            MathExpression::Var(Identifier::Name(name, _)) => Some(name.clone()),
            _ => None,
        }
    }

    /// Check if this expression is a variable with the given name
    pub fn is_variable_named(&self, name: &str) -> bool {
        match self {
            MathExpression::Var(Identifier::Name(var_name, _)) => var_name == name,
            _ => false,
        }
    }
}

// Comment out the helper methods that are causing mismatch issues
/*
/// Create a view of this expression as a group element
pub fn as_group_element(&self, group: Option<MathExpression>) -> Self {
    MathExpression::ViewAs {
        view: TypeViewOperator::AsGroupElement {
            group: group.map(Box::new),
        },
        expression: Box::new(self.clone()),
    }
}

/// Create a view of this expression as a ring element
pub fn as_ring_element(&self, ring: Option<MathExpression>) -> Self {
    MathExpression::ViewAs {
        view: TypeViewOperator::AsRingElement {
            ring: ring.map(Box::new),
        },
        expression: Box::new(self.clone()),
    }
}

/// Create a view of this expression as a cyclic group (for integers)
pub fn as_cyclic_group(&self) -> Self {
    MathExpression::ViewAs {
        view: TypeViewOperator::AsCyclicGroup,
        expression: Box::new(self.clone()),
    }
}

/// Create a view of this expression as a field element
pub fn as_field_element(&self, field: Option<MathExpression>) -> Self {
    MathExpression::ViewAs {
        view: TypeViewOperator::AsFieldElement {
            field: field.map(Box::new),
        },
        expression: Box::new(self.clone()),
    }
}
*/
