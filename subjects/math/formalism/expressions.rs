// Module: src/formalize_v2/subjects/math/theorem/expressions.rs
// Defines expressions used in mathematical statements and theorems

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::turn_render::Identifier;

use super::super::theories::groups::definitions::GroupExpression;
use super::super::theories::rings::definitions::{FieldExpression, RingExpression};

use super::complexity::Complexity;

use super::super::theories::{
    number_theory::definitions::Number,
    rings::{Ring, definitions::Field},
};

use super::super::formalism::interpretation::TypeViewOperator;
use super::extract::Parametrizable;
use super::{location::Located, objects::MathObject, relations::MathRelation};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum TheoryExpression {
    Group(GroupExpression),
    Ring(RingExpression),
    Field(FieldExpression),
}

/// A unified mathematical expression
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MathExpression {
    /// Variable reference, not definition
    /// we don't use this anymore, we use Parametrizable such as Parametrizable<MathExpression>
    /// to represent variables everywhere!
    // Var(Identifier),

    /// Reference to a mathematical object
    Object(Arc<MathObject>),

    /// expression that have both value and type
    Expression(TheoryExpression),

    /// treating math relationships as first-flass citizens
    /// this makes relation a sub class of MathExpression, but it is not, we can choose which
    /// node is the root node at a given problem.
    Relation(Arc<MathRelation>),

    /// Numeric value
    Number(Number),

    /// An expression with a specific type view
    /// this is a central transit for all math theories
    ViewAs {
        /// The original expression
        expression: Located<Parametrizable<Arc<MathExpression>>>,
        /// The view operator
        view: Located<TypeViewOperator>,
    },
}

/// Errors that can occur during type checking of views
#[derive(Debug, Clone)]
pub enum TypeViewError {
    /// Error when trying to view an expression as an incompatible type
    InvalidView {
        expression_type: MathObject,
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
    // /// Create a variable expression from a name
    // pub fn var(name: &str) -> Self {
    //     MathExpression::Var(Identifier::new_simple(name.to_string()))
    // }

    // /// Create a variable expression with an explicit identifier
    // pub fn var_with_id(name: &str, id: u32) -> Self {
    //     MathExpression::Var(Identifier::new_simple(name.to_string()))
    // }

    // pub fn is_variable(&self) -> bool {
    //     matches!(self, MathExpression::Var(_))
    // }

    /// Apply a type view to this expression
    pub fn with_view(&self, view: TypeViewOperator) -> Self {
        MathExpression::ViewAs {
            expression: Located::new(Parametrizable::Concrete(Arc::new(self.clone()))),
            view: Located::new(view),
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

    /// Returns true if this expression is a view of another expression
    pub fn is_view(&self) -> bool {
        matches!(self, MathExpression::ViewAs { .. })
    }

    // /// Get the variable name if this is a variable expression
    // pub fn as_variable_name(&self) -> Option<Identifier> {
    //     match self {
    //         MathExpression::Var(id) => Some(id.clone()),
    //         _ => None,
    //     }
    // }

    // /// Check if this expression is a variable with the given name
    // pub fn is_variable_named(&self, name: &Identifier) -> bool {
    //     match self {
    //         MathExpression::Var(id) => id == name,
    //         _ => false,
    //     }
    // }

    pub fn get_variant_name(&self) -> &'static str {
        match self {
            // MathExpression::Var(_) => "Var",
            MathExpression::Object(_) => "Object",
            MathExpression::Expression(_) => "Expression",
            MathExpression::Relation(_) => "Relation",
            MathExpression::Number(_) => "Number",
            MathExpression::ViewAs { .. } => "ViewAs",
        }
    }
}

// Implementation to convert GroupExpression into MathExpression
impl From<GroupExpression> for MathExpression {
    fn from(group_expr: GroupExpression) -> Self {
        MathExpression::Expression(TheoryExpression::Group(group_expr))
    }
}

// Implementation to convert RingExpression into MathExpression
impl From<RingExpression> for MathExpression {
    fn from(ring_expr: RingExpression) -> Self {
        MathExpression::Expression(TheoryExpression::Ring(ring_expr))
    }
}

// Implementation to convert FieldExpression into MathExpression
impl From<FieldExpression> for MathExpression {
    fn from(field_expr: FieldExpression) -> Self {
        MathExpression::Expression(TheoryExpression::Field(field_expr))
    }
}

// Implementation to convert MathRelation to MathExpression
impl From<MathRelation> for MathExpression {
    fn from(relation: MathRelation) -> Self {
        MathExpression::Relation(Arc::new(relation))
    }
}

// Implementation to convert MathObject to MathExpression
impl From<MathObject> for MathExpression {
    fn from(object: MathObject) -> Self {
        MathExpression::Object(Arc::new(object))
    }
}

impl From<Number> for MathExpression {
    fn from(number: Number) -> Self {
        MathExpression::Number(number)
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
