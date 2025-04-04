// src/formalize_v2/subjects/math/theories/common/logical_connectives.rs
//! Provides logical connectives from logic module for use in mathematical theories.
//! This module bridges first-order logic with mathematical theories, enabling
//! rigorous theorem representation without making ZFC the foundation.

use crate::formalize_v2::subjects::logic::first_order::{Formula, Term};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a logical statement that can be used in mathematical theorems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TheoremStatement {
    /// The name of the theorem
    pub name: String,
    /// The logical formula representing the theorem
    pub formula: Formula,
    /// Optional metadata (tags, classification, etc.)
    pub metadata: TheoremMetadata,
}

/// Metadata for theorems
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TheoremMetadata {
    /// Classification of the theorem (e.g., "analysis", "algebra")
    pub classification: Vec<String>,
    /// Dependencies (other theorems this one depends on)
    pub dependencies: Vec<String>,
    /// Whether this theorem has a formal proof in the system
    pub has_formal_proof: bool,
    /// Difficulty level (1-5)
    pub difficulty: Option<u8>,
}

/// Represents a theorem with a proof
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TheoremWithProof {
    /// The theorem statement
    pub statement: TheoremStatement,
    /// The proof steps
    pub proof: Vec<ProofStep>,
}

/// A single step in a proof
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofStep {
    /// The formula representing this step
    pub formula: Formula,
    /// The justification for this step
    pub justification: Justification,
}

/// Justification for a proof step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Justification {
    /// Axiom reference
    Axiom(String),
    /// Theorem reference
    Theorem(String),
    /// Follows from previous steps via a rule of inference
    Inference(InferenceRule, Vec<usize>),
    /// Given as a hypothesis
    Hypothesis,
    /// An assertion that needs to be proven separately
    Assertion,
}

/// Rules of inference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InferenceRule {
    /// Modus Ponens: From P and P → Q, deduce Q
    ModusPonens,
    /// Universal Instantiation: From ∀x.P(x), deduce P(t)
    UniversalInstantiation,
    /// Existential Generalization: From P(t), deduce ∃x.P(x)
    ExistentialGeneralization,
    /// And Introduction: From P and Q, deduce P ∧ Q
    AndIntroduction,
    /// And Elimination: From P ∧ Q, deduce P (or Q)
    AndElimination,
    /// Or Introduction: From P, deduce P ∨ Q
    OrIntroduction,
    /// Or Elimination: From P ∨ Q, P → R, and Q → R, deduce R
    OrElimination,
    /// Custom inference rule with description
    Custom(String),
}

/// Helper functions for creating logical formulas in mathematical contexts
impl TheoremStatement {
    /// Create a new theorem statement
    pub fn new(name: &str, formula: Formula) -> Self {
        Self {
            name: name.to_string(),
            formula,
            metadata: TheoremMetadata::default(),
        }
    }

    /// Add a classification to the theorem
    pub fn with_classification(mut self, classification: &str) -> Self {
        self.metadata
            .classification
            .push(classification.to_string());
        self
    }

    /// Add a dependency to the theorem
    pub fn with_dependency(mut self, dependency: &str) -> Self {
        self.metadata.dependencies.push(dependency.to_string());
        self
    }

    /// Set whether the theorem has a formal proof
    pub fn with_formal_proof(mut self, has_formal_proof: bool) -> Self {
        self.metadata.has_formal_proof = has_formal_proof;
        self
    }

    /// Set the difficulty level of the theorem
    pub fn with_difficulty(mut self, difficulty: u8) -> Self {
        if difficulty > 0 && difficulty <= 5 {
            self.metadata.difficulty = Some(difficulty);
        }
        self
    }
}

/// Extension trait for Formula to make it easier to use in mathematical contexts
pub trait MathematicalFormula {
    /// Create a universally quantified formula with type annotation
    fn for_all_with_type(var: &str, var_type: &str, body: Formula) -> Formula;

    /// Create an existentially quantified formula with type annotation
    fn exists_with_type(var: &str, var_type: &str, body: Formula) -> Formula;

    /// Create a biconditional formula (if and only if)
    fn iff(left: Formula, right: Formula) -> Formula;

    /// Create a formula representing set membership
    fn in_set(element: Term, set: Term) -> Formula;

    /// Create a formula representing subset relationship
    fn subset(subset: Term, superset: Term) -> Formula;
}

impl MathematicalFormula for Formula {
    fn for_all_with_type(var: &str, var_type: &str, body: Formula) -> Formula {
        // We represent "∀x: T. P(x)" as "∀x. (x ∈ T → P(x))"
        let var_term = Term::Variable(var.to_string());
        let type_term = Term::Constant(var_type.to_string());
        let type_pred = Formula::predicate("element_of", vec![var_term, type_term]);
        Formula::ForAll(
            var.to_string(),
            Box::new(Formula::Implies(Box::new(type_pred), Box::new(body))),
        )
    }

    fn exists_with_type(var: &str, var_type: &str, body: Formula) -> Formula {
        // We represent "∃x: T. P(x)" as "∃x. (x ∈ T ∧ P(x))"
        let var_term = Term::Variable(var.to_string());
        let type_term = Term::Constant(var_type.to_string());
        let type_pred = Formula::predicate("element_of", vec![var_term, type_term]);
        Formula::Exists(
            var.to_string(),
            Box::new(Formula::And(Box::new(type_pred), Box::new(body))),
        )
    }

    fn iff(left: Formula, right: Formula) -> Formula {
        // Represents "P ↔ Q" as "(P → Q) ∧ (Q → P)"
        Formula::And(
            Box::new(Formula::Implies(
                Box::new(left.clone()),
                Box::new(right.clone()),
            )),
            Box::new(Formula::Implies(Box::new(right), Box::new(left))),
        )
    }

    fn in_set(element: Term, set: Term) -> Formula {
        Formula::predicate("element_of", vec![element, set])
    }

    fn subset(subset: Term, superset: Term) -> Formula {
        Formula::predicate("subset_of", vec![subset, superset])
    }
}

/// Conversion from mathematical structures to logical formulas and back
pub trait LogicalRepresentable {
    /// Convert to a logical formula
    fn to_formula(&self) -> Formula;

    /// Try to interpret a logical formula as this type
    fn from_formula(formula: &Formula) -> Option<Self>
    where
        Self: Sized;
}

/// Helper functions for creating common mathematical statements
pub fn forall_real(var: &str, body: Formula) -> Formula {
    Formula::for_all_with_type(var, "ℝ", body)
}

pub fn exists_real(var: &str, body: Formula) -> Formula {
    Formula::exists_with_type(var, "ℝ", body)
}

pub fn forall_natural(var: &str, body: Formula) -> Formula {
    Formula::for_all_with_type(var, "ℕ", body)
}

pub fn exists_natural(var: &str, body: Formula) -> Formula {
    Formula::exists_with_type(var, "ℕ", body)
}

pub fn forall_integer(var: &str, body: Formula) -> Formula {
    Formula::for_all_with_type(var, "ℤ", body)
}

pub fn exists_integer(var: &str, body: Formula) -> Formula {
    Formula::exists_with_type(var, "ℤ", body)
}
