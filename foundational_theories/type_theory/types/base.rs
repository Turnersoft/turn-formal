//! Base types and operations
//! Provides fundamental type constructors

use super::super::core::{Term, Result, Error};
use super::TypeConstructor;

/// Basic types in our theory
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseType {
    /// Unit type (⊤)
    Unit,
    
    /// Empty type (⊥)
    Empty,
    
    /// Boolean type
    Bool,
    
    /// Natural numbers
    Nat,
}

/// Type representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Base types
    Base(BaseType),
    
    /// Function type (non-dependent)
    Arrow(Box<Type>, Box<Type>),
    
    /// Product type (A × B)
    Product(Box<Type>, Box<Type>),
    
    /// Sum type (A + B)
    Sum(Box<Type>, Box<Type>),
}

impl Type {
    /// Create a new base type
    pub fn base(ty: BaseType) -> Self {
        Type::Base(ty)
    }
    
    /// Create a new function type
    pub fn arrow(from: Type, to: Type) -> Self {
        Type::Arrow(Box::new(from), Box::new(to))
    }
    
    /// Create a new product type
    pub fn product(left: Type, right: Type) -> Self {
        Type::Product(Box::new(left), Box::new(right))
    }
    
    /// Create a new sum type
    pub fn sum(left: Type, right: Type) -> Self {
        Type::Sum(Box::new(left), Box::new(right))
    }
}

/// Unit type implementation
pub struct UnitType;

impl TypeConstructor for UnitType {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::Var(name) if name == "tt" => Ok(()),
            _ => Err(Error::TypeError("Expected unit value (tt)".to_string())),
        }
    }
    
    fn universe_level(&self) -> usize {
        0 // Unit type lives in the lowest universe
    }
}

/// Empty type implementation
pub struct EmptyType;

impl TypeConstructor for EmptyType {
    fn check_term(&self, _: &Term) -> Result<()> {
        // No terms can inhabit the empty type
        Err(Error::TypeError("No terms can inhabit the empty type".to_string()))
    }
    
    fn universe_level(&self) -> usize {
        0 // Empty type lives in the lowest universe
    }
}

/// Boolean type implementation
pub struct BoolType;

impl TypeConstructor for BoolType {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::Var(name) if name == "true" || name == "false" => Ok(()),
            _ => Err(Error::TypeError("Expected boolean value".to_string())),
        }
    }
    
    fn universe_level(&self) -> usize {
        0 // Bool type lives in the lowest universe
    }
}

/// Natural number type implementation
pub struct NatType;

impl TypeConstructor for NatType {
    fn check_term(&self, term: &Term) -> Result<()> {
        match term {
            Term::Var(name) if name == "zero" => Ok(()),
            Term::Apply { left, right } => {
                match &**left {
                    Term::Var(name) if name == "succ" => self.check_term(right),
                    _ => Err(Error::TypeError("Expected natural number".to_string())),
                }
            }
            _ => Err(Error::TypeError("Expected natural number".to_string())),
        }
    }
    
    fn universe_level(&self) -> usize {
        0 // Nat type lives in the lowest universe
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_type() {
        let unit = UnitType;
        assert!(unit.check_term(&Term::Var("tt".to_string())).is_ok());
        assert!(unit.check_term(&Term::Var("other".to_string())).is_err());
    }

    #[test]
    fn test_empty_type() {
        let empty = EmptyType;
        assert!(empty.check_term(&Term::Var("anything".to_string())).is_err());
    }

    #[test]
    fn test_bool_type() {
        let bool_type = BoolType;
        assert!(bool_type.check_term(&Term::Var("true".to_string())).is_ok());
        assert!(bool_type.check_term(&Term::Var("false".to_string())).is_ok());
        assert!(bool_type.check_term(&Term::Var("other".to_string())).is_err());
    }

    #[test]
    fn test_nat_type() {
        let nat = NatType;
        
        // Test zero
        assert!(nat.check_term(&Term::Var("zero".to_string())).is_ok());
        
        // Test successor
        let one = Term::Apply {
            left: Box::new(Term::Var("succ".to_string())),
            right: Box::new(Term::Var("zero".to_string())),
        };
        assert!(nat.check_term(&one).is_ok());
    }
}
