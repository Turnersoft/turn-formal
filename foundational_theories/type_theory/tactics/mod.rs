//! Tactics for proof automation
//! Provides a framework for automated proof construction

use std::collections::HashMap;
use super::core::{Term, Result, Error};

/// Proof state containing goals and context
#[derive(Debug, Clone)]
pub struct ProofState {
    /// Current proof goals
    goals: Vec<Term>,
    /// Local context (name -> type)
    context: HashMap<String, Term>,
    /// Generated proof term
    proof: Option<Term>,
}

impl ProofState {
    /// Create new proof state with single goal
    pub fn new(goal: Term) -> Self {
        ProofState {
            goals: vec![goal],
            context: HashMap::new(),
            proof: None,
        }
    }
    
    /// Add hypothesis to context
    pub fn add_hypothesis(&mut self, name: impl Into<String>, ty: Term) {
        self.context.insert(name.into(), ty);
    }
    
    /// Get current goal
    pub fn current_goal(&self) -> Option<&Term> {
        self.goals.first()
    }
    
    /// Replace current goal with new subgoals
    pub fn replace_goal(&mut self, subgoals: Vec<Term>) {
        if !self.goals.is_empty() {
            self.goals.remove(0);
            self.goals.splice(0..0, subgoals);
        }
    }
    
    /// Set proof term
    pub fn set_proof(&mut self, proof: Term) {
        self.proof = Some(proof);
    }
    
    /// Check if proof is complete
    pub fn is_complete(&self) -> bool {
        self.goals.is_empty() && self.proof.is_some()
    }
}

/// Trait for proof tactics
pub trait Tactic {
    /// Apply tactic to proof state
    fn apply(&self, state: &mut ProofState) -> Result<()>;
    
    /// Get tactic name
    fn name(&self) -> &str;
}

/// Exact tactic - provide exact proof term
pub struct Exact {
    /// The exact proof term
    term: Term,
}

impl Exact {
    /// Create new exact tactic
    pub fn new(term: Term) -> Self {
        Exact { term }
    }
}

impl Tactic for Exact {
    fn apply(&self, state: &mut ProofState) -> Result<()> {
        if let Some(goal) = state.current_goal() {
            // Check term has goal type
            // This is simplified; should do proper type checking
            state.replace_goal(vec![]);
            state.set_proof(self.term.clone());
            Ok(())
        } else {
            Err(Error::TacticError("No current goal".to_string()))
        }
    }
    
    fn name(&self) -> &str {
        "exact"
    }
}

/// Introduction tactic for implications
pub struct Intro {
    /// Name for introduced variable
    name: String,
}

impl Intro {
    /// Create new intro tactic
    pub fn new(name: impl Into<String>) -> Self {
        Intro { name: name.into() }
    }
}

impl Tactic for Intro {
    fn apply(&self, state: &mut ProofState) -> Result<()> {
        if let Some(goal) = state.current_goal() {
            match goal {
                Term::Apply { left, right } => {
                    // Add hypothesis to context
                    state.add_hypothesis(self.name.clone(), (**right).clone());
                    // Replace goal with body
                    state.replace_goal(vec![(**left).clone()]);
                    Ok(())
                }
                _ => Err(Error::TacticError(
                    "Goal is not an implication".to_string()
                )),
            }
        } else {
            Err(Error::TacticError("No current goal".to_string()))
        }
    }
    
    fn name(&self) -> &str {
        "intro"
    }
}

/// Apply tactic for function application
pub struct Apply {
    /// Term to apply
    term: Term,
}

impl Apply {
    /// Create new apply tactic
    pub fn new(term: Term) -> Self {
        Apply { term }
    }
}

impl Tactic for Apply {
    fn apply(&self, state: &mut ProofState) -> Result<()> {
        if let Some(goal) = state.current_goal() {
            // Generate subgoals from application
            // This is simplified; should properly handle dependent types
            state.replace_goal(vec![self.term.clone()]);
            Ok(())
        } else {
            Err(Error::TacticError("No current goal".to_string()))
        }
    }
    
    fn name(&self) -> &str {
        "apply"
    }
}

/// Tactic script for combining tactics
pub struct Script {
    /// List of tactics to apply
    tactics: Vec<Box<dyn Tactic>>,
}

impl Script {
    /// Create new tactic script
    pub fn new(tactics: Vec<Box<dyn Tactic>>) -> Self {
        Script { tactics }
    }
    
    /// Run all tactics in sequence
    pub fn run(&self, state: &mut ProofState) -> Result<()> {
        for tactic in &self.tactics {
            tactic.apply(state)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_tactic() {
        let mut state = ProofState::new(Term::Var("A".to_string()));
        let exact = Exact::new(Term::Var("a".to_string()));
        
        assert!(exact.apply(&mut state).is_ok());
        assert!(state.is_complete());
    }

    #[test]
    fn test_intro_tactic() {
        // Goal: A → B
        let goal = Term::Apply {
            left: Box::new(Term::Var("B".to_string())),
            right: Box::new(Term::Var("A".to_string())),
        };
        
        let mut state = ProofState::new(goal);
        let intro = Intro::new("H");
        
        assert!(intro.apply(&mut state).is_ok());
        assert_eq!(state.current_goal(), Some(&Term::Var("B".to_string())));
    }

    #[test]
    fn test_script() {
        // Goal: A → B
        let goal = Term::Apply {
            left: Box::new(Term::Var("B".to_string())),
            right: Box::new(Term::Var("A".to_string())),
        };
        
        let mut state = ProofState::new(goal);
        let script = Script::new(vec![
            Box::new(Intro::new("H")),
            Box::new(Exact::new(Term::Var("b".to_string()))),
        ]);
        
        assert!(script.run(&mut state).is_ok());
        assert!(state.is_complete());
    }
}
