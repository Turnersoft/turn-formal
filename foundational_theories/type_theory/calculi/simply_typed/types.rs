use serde::{Deserialize, Serialize};
use std::fmt;

/// Types in Simply Typed Lambda Calculus
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// Base types
    Bool,
    Number,
    Unit,
    /// Bottom type (⊥) - type with no values
    Bottom,
    /// Top type (⊤) - terminal object
    Top,

    /// Product type (A × B)
    Product {
        left: Box<Type>,
        right: Box<Type>,
    },

    /// Sum type (A + B)
    Sum {
        left: Box<Type>,
        right: Box<Type>,
    },

    /// Function type (A → B)
    Function {
        param_type: Box<Type>,
        return_type: Box<Type>,
    },
}

impl Type {
    /// Create a new function type
    pub fn function(param_type: Type, return_type: Type) -> Self {
        Type::Function {
            param_type: Box::new(param_type),
            return_type: Box::new(return_type),
        }
    }

    /// Create a new product type
    pub fn product(left: Type, right: Type) -> Self {
        Type::Product {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a new sum type
    pub fn sum(left: Type, right: Type) -> Self {
        Type::Sum {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Check if this type is a function type
    pub fn is_function(&self) -> bool {
        matches!(self, Type::Function { .. })
    }

    /// Get the parameter type if this is a function type
    pub fn param_type(&self) -> Option<&Type> {
        match self {
            Type::Function { param_type, .. } => Some(param_type),
            _ => None,
        }
    }

    /// Get the return type if this is a function type
    pub fn return_type(&self) -> Option<&Type> {
        match self {
            Type::Function { return_type, .. } => Some(return_type),
            _ => None,
        }
    }

    /// Check if this is the bottom type
    pub fn is_bottom(&self) -> bool {
        matches!(self, Type::Bottom)
    }

    /// Create function type from bottom (ex falso quodlibet)
    pub fn from_bottom(return_type: Type) -> Self {
        Type::Function {
            param_type: Box::new(Type::Bottom),
            return_type: Box::new(return_type),
        }
    }

    /// Find a common type between two sum types
    pub fn common(left: Box<Type>, right: Box<Type>) -> Type {
        // If types are exactly the same, return that type
        if *left == *right {
            return *left;
        }

        // If types are different, return the most general type that can represent both
        // In this case, we'll use the Top type as a fallback
        Type::Top
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "Bool"),
            Type::Number => write!(f, "Number"),
            Type::Unit => write!(f, "Unit"),
            Type::Bottom => write!(f, "⊥"),
            Type::Top => write!(f, "⊤"),
            Type::Product { left, right } => write!(f, "({} × {})", left, right),
            Type::Sum { left, right } => write!(f, "({} + {})", left, right),
            Type::Function {
                param_type,
                return_type,
            } => write!(f, "({} → {})", param_type, return_type),
        }
    }
}
