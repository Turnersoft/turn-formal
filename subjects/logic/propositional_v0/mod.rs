//! Propositional Logic Implementation
//! This module provides a foundation-independent implementation of propositional logic.

use crate::parse::{entities::Identifier, Parse};

/// A proposition in propositional logic
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Proposition {
    /// Atomic proposition (P, Q, etc.)
    Atomic(Identifier),
    /// Truth constant (⊤)
    True,
    /// Falsity constant (⊥)
    False,
    /// Conjunction (P ∧ Q)
    And(Box<Proposition>, Box<Proposition>),
    /// Disjunction (P ∨ Q)
    Or(Box<Proposition>, Box<Proposition>),
    /// Implication (P → Q)
    Implies(Box<Proposition>, Box<Proposition>),
    /// Negation (¬P)
    Not(Box<Proposition>),
}

impl Proposition {
    /// Create a new atomic proposition
    pub fn atomic(name: &str) -> Self {
        Proposition::Atomic(Identifier::parse(name))
    }

    /// Create a conjunction
    pub fn and(left: Proposition, right: Proposition) -> Self {
        Proposition::And(Box::new(left), Box::new(right))
    }

    /// Create a disjunction
    pub fn or(left: Proposition, right: Proposition) -> Self {
        Proposition::Or(Box::new(left), Box::new(right))
    }

    /// Create an implication
    pub fn implies(antecedent: Proposition, consequent: Proposition) -> Self {
        Proposition::Implies(Box::new(antecedent), Box::new(consequent))
    }

    /// Create a negation
    pub fn not(prop: Proposition) -> Self {
        Proposition::Not(Box::new(prop))
    }

    /// Check if this proposition is atomic
    pub fn is_atomic(&self) -> bool {
        matches!(self, Proposition::Atomic(_))
    }

    /// Check if this proposition is a conjunction
    pub fn is_conjunction(&self) -> bool {
        matches!(self, Proposition::And(_, _))
    }

    /// Check if this proposition is a disjunction
    pub fn is_disjunction(&self) -> bool {
        matches!(self, Proposition::Or(_, _))
    }

    /// Check if this proposition is an implication
    pub fn is_implication(&self) -> bool {
        matches!(self, Proposition::Implies(_, _))
    }

    /// Check if this proposition is a negation
    pub fn is_negation(&self) -> bool {
        matches!(self, Proposition::Not(_))
    }
}

/// A proof rule in propositional logic
#[derive(Debug, Clone)]
pub enum ProofRule {
    /// Assumption introduction
    Assumption,
    /// And-introduction (∧I)
    AndIntro,
    /// And-elimination-left (∧E₁)
    AndElimLeft,
    /// And-elimination-right (∧E₂)
    AndElimRight,
    /// Or-introduction-left (∨I₁)
    OrIntroLeft,
    /// Or-introduction-right (∨I₂)
    OrIntroRight,
    /// Or-elimination (∨E)
    OrElim,
    /// Implies-introduction (→I)
    ImpliesIntro,
    /// Implies-elimination (→E)
    ImpliesElim,
    /// Not-introduction (¬I)
    NotIntro,
    /// Not-elimination (¬E)
    NotElim,
    /// Truth introduction (⊤I)
    TrueIntro,
    /// Falsity elimination (⊥E)
    FalseElim,
    /// Double negation elimination (¬¬A ⊢ A)
    DoubleNegationElim,
}

/// A proof step in propositional logic
#[derive(Debug, Clone)]
pub struct ProofStep {
    /// The proposition being proven
    pub proposition: Proposition,
    /// The rule used to derive this step
    pub rule: ProofRule,
    /// The premises used in this step
    pub premises: Vec<usize>,
    /// Any assumptions discharged in this step
    pub discharged: Vec<usize>,
}

/// A proof in propositional logic
#[derive(Debug, Clone)]
pub struct Proof {
    /// The steps in the proof
    pub steps: Vec<ProofStep>,
    /// The final conclusion
    pub conclusion: Proposition,
}

impl Proof {
    /// Create a new proof with a given conclusion
    pub fn new(conclusion: Proposition) -> Self {
        Self {
            steps: Vec::new(),
            conclusion,
        }
    }

    /// Add a step to the proof
    pub fn add_step(&mut self, step: ProofStep) {
        self.steps.push(step);
    }

    /// Get the proposition at a given step
    pub fn get_proposition(&self, step: usize) -> Option<&Proposition> {
        self.steps.get(step).map(|s| &s.proposition)
    }

    /// Check if this proof is complete
    pub fn is_complete(&self) -> bool {
        if let Some(last_step) = self.steps.last() {
            last_step.proposition == self.conclusion
        } else {
            false
        }
    }
}

/// Error types for propositional logic operations
#[derive(Debug)]
pub enum PropositionalError {
    /// Invalid proof rule application
    InvalidRule(String),
    /// Invalid premise reference
    InvalidPremise(String),
    /// Invalid assumption discharge
    InvalidDischarge(String),
    /// Incomplete proof
    IncompleteProof(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposition_constructors() {
        let p = Proposition::atomic("P");
        let q = Proposition::atomic("Q");

        let and = Proposition::and(p.clone(), q.clone());
        let or = Proposition::or(p.clone(), q.clone());
        let implies = Proposition::implies(p.clone(), q.clone());
        let not = Proposition::not(p.clone());

        assert!(p.is_atomic());
        assert!(and.is_conjunction());
        assert!(or.is_disjunction());
        assert!(implies.is_implication());
        assert!(not.is_negation());
    }

    #[test]
    fn test_proof_construction() {
        let p = Proposition::atomic("P");
        let q = Proposition::atomic("Q");
        let goal = Proposition::and(p.clone(), q.clone());

        let mut proof = Proof::new(goal.clone());

        // Add assumption steps
        proof.add_step(ProofStep {
            proposition: p.clone(),
            rule: ProofRule::Assumption,
            premises: vec![],
            discharged: vec![],
        });

        proof.add_step(ProofStep {
            proposition: q.clone(),
            rule: ProofRule::Assumption,
            premises: vec![],
            discharged: vec![],
        });

        // Add conjunction introduction
        proof.add_step(ProofStep {
            proposition: goal,
            rule: ProofRule::AndIntro,
            premises: vec![0, 1],
            discharged: vec![],
        });

        assert!(proof.is_complete());
    }
}
