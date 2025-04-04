use serde::{Deserialize, Serialize};

/// First-order logic terms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Term {
    /// A variable (x, y, z, etc.)
    Variable(String),
    /// A constant (0, 1, π, etc.)
    Constant(String),
    /// A function application (f(x), g(x,y), etc.)
    Function(String, Vec<Term>),
}

/// First-order logic formulas
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Formula {
    /// Predicate application (P(x), Q(x,y), etc.)
    Predicate(String, Vec<Term>),
    /// Equality between terms (x = y)
    Equality(Term, Term),
    /// Universal quantification (∀x.P(x))
    ForAll(String, Box<Formula>),
    /// Existential quantification (∃x.P(x))
    Exists(String, Box<Formula>),
    /// Conjunction (P ∧ Q)
    And(Box<Formula>, Box<Formula>),
    /// Disjunction (P ∨ Q)
    Or(Box<Formula>, Box<Formula>),
    /// Implication (P → Q)
    Implies(Box<Formula>, Box<Formula>),
    /// Negation (¬P)
    Not(Box<Formula>),
    /// Atomic formula (true, false)
    Atomic(bool),
}

impl Formula {
    /// Create a universally quantified formula
    pub fn for_all(var: &str, body: Formula) -> Self {
        Formula::ForAll(var.to_string(), Box::new(body))
    }

    /// Create an existentially quantified formula
    pub fn exists(var: &str, body: Formula) -> Self {
        Formula::Exists(var.to_string(), Box::new(body))
    }

    /// Create an equality formula
    pub fn equals(left: Term, right: Term) -> Self {
        Formula::Equality(left, right)
    }

    /// Create a predicate formula
    pub fn predicate(name: &str, args: Vec<Term>) -> Self {
        Formula::Predicate(name.to_string(), args)
    }
}
