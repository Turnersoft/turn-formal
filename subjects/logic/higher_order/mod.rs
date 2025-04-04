//! Higher-Order Logic Implementation
//! This module provides a foundation-independent implementation of higher-order logic.
//! While HOL can theoretically encode other logics, we keep it separate for clarity and usability.

/// Types in higher-order logic
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Base type for individuals
    Individual,
    /// Type of propositions
    Prop,
    /// Function type (α → β)
    Function(Box<Type>, Box<Type>),
    /// Type variable (for polymorphism)
    Var(String),
}

/// Terms in higher-order logic
#[derive(Debug, Clone)]
pub enum Term {
    /// Variables (can be of any type)
    Variable(String, Type),
    /// Constants (can be of any type)
    Constant(String, Type),
    /// Function application
    Application(Box<Term>, Box<Term>),
    /// Lambda abstraction (λx:α. t)
    Lambda(String, Type, Box<Term>),
}

/// Formulas in higher-order logic
#[derive(Debug, Clone)]
pub enum Formula {
    /// Atomic formula (term of type Prop)
    Atomic(Term),
    /// Logical conjunction
    And(Box<Formula>, Box<Formula>),
    /// Logical disjunction
    Or(Box<Formula>, Box<Formula>),
    /// Logical implication
    Implies(Box<Formula>, Box<Formula>),
    /// Logical negation
    Not(Box<Formula>),
    /// Universal quantification (can quantify over any type)
    ForAll(String, Type, Box<Formula>),
    /// Existential quantification (can quantify over any type)
    Exists(String, Type, Box<Formula>),
    /// Equality between terms of the same type
    Equals(Term, Term),
}

impl Type {
    /// Create a function type
    pub fn func(from: Type, to: Type) -> Self {
        Type::Function(Box::new(from), Box::new(to))
    }

    /// Create a type variable
    pub fn var(name: &str) -> Self {
        Type::Var(name.to_string())
    }
}

impl Term {
    /// Create a variable term
    pub fn var(name: &str, ty: Type) -> Self {
        Term::Variable(name.to_string(), ty)
    }

    /// Create a constant term
    pub fn const_(name: &str, ty: Type) -> Self {
        Term::Constant(name.to_string(), ty)
    }

    /// Create a function application
    pub fn app(func: Term, arg: Term) -> Self {
        Term::Application(Box::new(func), Box::new(arg))
    }

    /// Create a lambda abstraction
    pub fn lambda(var: &str, ty: Type, body: Term) -> Self {
        Term::Lambda(var.to_string(), ty, Box::new(body))
    }
}

impl Formula {
    /// Create a conjunction
    pub fn and(left: Formula, right: Formula) -> Self {
        Formula::And(Box::new(left), Box::new(right))
    }

    /// Create a disjunction
    pub fn or(left: Formula, right: Formula) -> Self {
        Formula::Or(Box::new(left), Box::new(right))
    }

    /// Create an implication
    pub fn implies(antecedent: Formula, consequent: Formula) -> Self {
        Formula::Implies(Box::new(antecedent), Box::new(consequent))
    }

    /// Create a negation
    pub fn not(formula: Formula) -> Self {
        Formula::Not(Box::new(formula))
    }

    /// Create a universal quantification
    pub fn for_all(var: &str, ty: Type, body: Formula) -> Self {
        Formula::ForAll(var.to_string(), ty, Box::new(body))
    }

    /// Create an existential quantification
    pub fn exists(var: &str, ty: Type, body: Formula) -> Self {
        Formula::Exists(var.to_string(), ty, Box::new(body))
    }

    /// Create an equality formula
    pub fn equals(left: Term, right: Term) -> Self {
        Formula::Equals(left, right)
    }
}

/// Interface for converting to and from higher-order logic
pub trait ToHOL {
    /// Convert to a higher-order logic formula
    fn to_hol(&self) -> Formula;
}

/// Interface for converting from higher-order logic
pub trait FromHOL<T> {
    /// Convert from a higher-order logic formula if possible
    fn from_hol(formula: &Formula) -> Option<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_formula() {
        // Create formula: ∀x:ι. ∃y:ι. P(x,y)
        let ind = Type::Individual;
        let p = Term::const_(
            "P",
            Type::func(ind.clone(), Type::func(ind.clone(), Type::Prop)),
        );
        let x = Term::var("x", ind.clone());
        let y = Term::var("y", ind.clone());
        let px = Term::app(p.clone(), x.clone());
        let pxy = Term::app(px, y.clone());

        let formula = Formula::for_all(
            "x",
            ind.clone(),
            Formula::exists("y", ind.clone(), Formula::Atomic(pxy)),
        );

        // Just verify it compiles and basic structure
        match formula {
            Formula::ForAll(var, ty, _) => {
                assert_eq!(var, "x");
                assert_eq!(ty, Type::Individual);
            }
            _ => panic!("Expected ForAll"),
        }
    }

    #[test]
    fn test_lambda_term() {
        // Create term: λx:ι. λy:ι. P(x,y)
        let ind = Type::Individual;
        let p = Term::const_(
            "P",
            Type::func(ind.clone(), Type::func(ind.clone(), Type::Prop)),
        );
        let x = Term::var("x", ind.clone());
        let y = Term::var("y", ind.clone());
        let px = Term::app(p.clone(), x.clone());
        let pxy = Term::app(px, y.clone());

        let term = Term::lambda("x", ind.clone(), Term::lambda("y", ind.clone(), pxy));

        // Verify basic structure
        match term {
            Term::Lambda(var, ty, _) => {
                assert_eq!(var, "x");
                assert_eq!(ty, Type::Individual);
            }
            _ => panic!("Expected Lambda"),
        }
    }
}
