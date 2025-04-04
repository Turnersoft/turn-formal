use super::{
    context::TacticalContext,
    proof_state::{ProofNode, ProofState, TacticProofTerm},
    TacticError,
    TacticResult,

};
use crate::{formalize_v2::{foundational_theories::type_theory_v2::calculi::simply_typed::{
    goals::Context, 
    terms::{Term, SumSide},
    types::Type,
}, subjects::logic::propositional::{
    foundations::type_theory::TypeTheoryFoundation, Foundation, GenericProof, Proposition
}}, parse::{entities::Identifier, Parse}};

/// A tactic that can be applied to a proof state
pub trait ProofStateTactic {
    type Error;

    /// Apply the tactic to the current proof state
    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error>;
}

/// Assumption tactic - tries to prove goal directly from context
#[derive(Clone)]
pub struct AssumptionTactic;

impl ProofStateTactic for AssumptionTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        if context.contains(&goal) {
            // Create the assumption term
            let term = foundation.make_assumption(&goal)?;
            
            // Add the assumption to STLC context if needed
            let mut new_context = context.clone();
            let prop_type = foundation.convert_proposition_to_type(&goal);
            
            if let Term::Variable(var_name) = &term {
                new_context.stlc_context_mut().add_variable(var_name.clone(), prop_type);
            }
            
            // Update the state's context
            *state.current_context_mut() = new_context.clone();
            
            // Set the current term to complete
            state.set_current_term(TacticProofTerm::Complete(term.clone()));
            
            // Create the completed node with updated context
            let new_node = ProofNode::new(new_context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
            state.add_child(new_node)?;
            
            // Create and set the proof
            let proof = GenericProof::new(goal, term);
            state.set_proof(proof);
            
            Ok(state.clone())
        } else {
            Err(TacticError::NoMatchingRule)
        }
    }
}

/// And-introduction tactics
#[derive(Clone)]
pub struct AndIntroTactic<L, R>
where
    L: ProofStateTactic<Error = TacticError>,
    R: ProofStateTactic<Error = TacticError>,
{
    left: L,
    right: R,
}

impl<L, R> AndIntroTactic<L, R>
where
    L: ProofStateTactic<Error = TacticError>,
    R: ProofStateTactic<Error = TacticError>,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R> Default for AndIntroTactic<L, R>
where
    L: ProofStateTactic<Error = TacticError> + Default,
    R: ProofStateTactic<Error = TacticError> + Default,
{
    fn default() -> Self {
        Self {
            left: L::default(),
            right: R::default(),
        }
    }
}

impl<L, R> ProofStateTactic for AndIntroTactic<L, R>
where
    L: ProofStateTactic<Error = TacticError>,
    R: ProofStateTactic<Error = TacticError>,
{
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        if let Proposition::And(ref a, ref b) = goal {
            // Create separate states for left and right proofs with the same context
            let mut left_state = state.clone();
            let mut right_state = state.clone();
            
            // Set appropriate goals
            left_state.set_goal(*a.clone());
            right_state.set_goal(*b.clone());
            
            // Apply tactics to get both parts
            let left_result = self.left.apply(&mut left_state)?;
            let right_result = self.right.apply(&mut right_state)?;
            
            // Get the terms from both results
            let left_term = match left_result.current_node().term() {
                TacticProofTerm::Complete(t) => t.clone(),
                _ => return Err(TacticError::RuleApplicationFailed("Left premise incomplete".to_string())),
            };
            
            let right_term = match right_result.current_node().term() {
                TacticProofTerm::Complete(t) => t.clone(),
                _ => return Err(TacticError::RuleApplicationFailed("Right premise incomplete".to_string())),
            };
            
            // Create the conjunction using foundation
            let term = foundation.make_and_intro(left_term, right_term);
            
            // Merge contexts from both proofs
            let mut new_context = context.clone();
            new_context.merge_stlc_context(left_result.current_context().stlc_context().clone());
            new_context.merge_stlc_context(right_result.current_context().stlc_context().clone());
            
            // Update the state's context
            *state.current_context_mut() = new_context.clone();
            
            // Set the current term to complete
            state.set_current_term(TacticProofTerm::Complete(term.clone()));
            
            // Create new node with merged context
            let new_node = ProofNode::new(new_context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
            state.add_child(new_node)?;
            
            // Create and set the proof
            let proof = GenericProof::new(goal, term);
            state.set_proof(proof);
            
            Ok(state.clone())
        } else {
            Err(TacticError::GoalMismatch {
                expected: Proposition::And(
                    Box::new(Proposition::Atomic(Identifier::parse("A"))),
                    Box::new(Proposition::Atomic(Identifier::parse("B"))),
                ),
                actual: goal,
            })
        }
    }
}

/// And-elimination tactics
pub struct AndElimTactic;

impl ProofStateTactic for AndElimTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        // Look for conjunction in context that could give us our goal
        for premise in &context {
            if let Proposition::And(ref left, ref right) = premise {
                if **left == goal || **right == goal {
                    let mut new_context = context.clone();
                    
                    // Get the conjunction term and add it to STLC context
                    let conj_term = foundation.make_assumption(&premise)?;
                    if let Term::Variable(var_name) = &conj_term {
                        let conj_type = foundation.convert_proposition_to_type(&premise);
                        if new_context.stlc_context().get_type(var_name).is_none() {
                            new_context.stlc_context_mut().add_variable(var_name.clone(), conj_type);
                        }
                    }
                    
                    // Create the elimination term
                    let term = if **left == goal {
                        foundation.make_and_elim_left(conj_term)
                    } else {
                        foundation.make_and_elim_right(conj_term)
                    };
                    
                    // Create new node with updated context
                    let new_node = ProofNode::new(new_context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
                    state.add_child(new_node)?;
                    
                    let proof = GenericProof::new(goal, term);
                    state.set_proof(proof);
                    
                    return Ok(state.clone());
                }
            }
        }
        
        Err(TacticError::NoMatchingRule)
    }
}

/// Implication introduction tactic
pub struct ImpliesIntroTactic;

impl ProofStateTactic for ImpliesIntroTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        // If the goal is already in context, use assumption
        if context.contains(&goal) {
            let term = foundation.make_assumption(&goal)?;
            let new_node = ProofNode::new(context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
            state.add_child(new_node)?;
            let proof = GenericProof::new(goal, term);
            state.set_proof(proof);
            return Ok(state.clone());
        }
        
        if let Proposition::Implies(ref antecedent, ref consequent) = goal {
            // Create a new state with the consequent as the goal
            let mut consequent_state = state.clone();
            consequent_state.set_goal(*consequent.clone());
            
            // Add antecedent to context
            let mut new_context = context.clone();
            new_context.add_proposition(*antecedent.clone());
            
            // Add antecedent to STLC context
            let ant_term = foundation.make_assumption(antecedent)?;
            if let Term::Variable(var_name) = &ant_term {
                let ant_type = foundation.convert_proposition_to_type(antecedent);
                new_context.stlc_context_mut().add_variable(var_name.clone(), ant_type);
            }
            
            // Update the consequent state's context
            *consequent_state.current_context_mut() = new_context.clone();
            
            // Try to prove the consequent
            let consequent_proof = if new_context.contains(consequent) {
                // If consequent is already in context, use assumption
                AssumptionTactic.apply(&mut consequent_state)?
            } else {
                // For nested implications, recursively apply ImpliesIntroTactic
                match **consequent {
                    Proposition::Implies(_, _) => {
                        ImpliesIntroTactic.apply(&mut consequent_state)?
                    },
                    _ => {
                        // Try to prove the consequent using implication elimination
                        let mut elim_state = consequent_state.clone();
                        match ImpliesElimTactic.apply(&mut elim_state) {
                            Ok(proof) => proof,
                            Err(_) => {
                                // Try direct assumption
                                match AssumptionTactic.apply(&mut consequent_state) {
                                    Ok(proof) => proof,
                                    Err(_) => return Err(TacticError::NoMatchingRule),
                                }
                            }
                        }
                    }
                }
            };
            
            // Get the consequent term from the proof
            let consequent_term = match consequent_proof.current_node().term() {
                TacticProofTerm::Complete(term) => term.clone(),
                _ => return Err(TacticError::NoMatchingRule),
            };
            
            // Create the implication term
            let term = foundation.make_implies_intro(consequent_term);
            
            // Create new node with original context
            let new_node = ProofNode::new(context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
            
            // Update the state's context and term
            *state.current_context_mut() = context;
            state.set_current_term(TacticProofTerm::Complete(term.clone()));
            state.add_child(new_node)?;
            
            // Create and set the proof
            let proof = GenericProof::new(goal, term);
            state.set_proof(proof);
            
            Ok(state.clone())
        } else {
            Err(TacticError::NoMatchingRule)
        }
    }
}

/// Implication elimination (modus ponens)
pub struct ImpliesElimTactic;

impl ProofStateTactic for ImpliesElimTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        // Look for implications in context that could give us our goal
        for premise in context.clone() {
            if let Proposition::Implies(ref antecedent, ref consequent) = premise {
                if **consequent == goal {
                    // We found an implication A → B where B is our goal
                    // Now we need to find A in the context
                    if context.contains(antecedent) {
                        let mut new_context = context.clone();
                        
                        // Get the implication term and add it to STLC context
                        let impl_term = foundation.make_assumption(&premise)?;
                        if let Term::Variable(var_name) = &impl_term {
                            let impl_type = foundation.convert_proposition_to_type(&premise);
                            if new_context.stlc_context().get_type(var_name).is_none() {
                                new_context.stlc_context_mut().add_variable(var_name.clone(), impl_type);
                            }
                        }
                        
                        // Get the antecedent term and add it to STLC context
                        let ant_term = foundation.make_assumption(antecedent)?;
                        if let Term::Variable(var_name) = &ant_term {
                            let ant_type = foundation.convert_proposition_to_type(antecedent);
                            if new_context.stlc_context().get_type(var_name).is_none() {
                                new_context.stlc_context_mut().add_variable(var_name.clone(), ant_type);
                            }
                        }
                        
                        // Create the elimination term
                        let term = foundation.make_implies_elim(impl_term, ant_term);
                        
                        // Create new node with updated context
                        let new_node = ProofNode::new(new_context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
                        state.add_child(new_node)?;
                        
                        let proof = GenericProof::new(goal, term);
                        state.set_proof(proof);
                        
                        return Ok(state.clone());
                    }
                }
            }
        }
        
        Err(TacticError::NoMatchingRule)
    }
}

/// Double negation elimination tactic
pub struct DNEtactic;

impl ProofStateTactic for DNEtactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        // Look for ¬¬A in context when trying to prove A
        for premise in &context {
            if let Proposition::Not(inner) = &premise {
                if let Proposition::Not(a) = *inner.clone() {
                    if *a == goal {
                        let proof = foundation.make_assumption(&premise)?;
                        let term = foundation.make_double_negation_elim(proof);
                        let new_node = ProofNode::new(context, TacticProofTerm::Complete(term.clone()), goal.clone());
                        state.add_child(new_node)?;
                        let proof = GenericProof::new(goal, term);
                        state.set_proof(proof);
                        return Ok(state.clone());
                    }
                }
            }
        }
        Err(TacticError::NoMatchingRule)
    }
}

/// Disjunction introduction tactics
pub struct OrIntroTactic;

impl ProofStateTactic for OrIntroTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        if let Proposition::Or(ref a, ref b) = goal {
            let mut new_context = context.clone();
            
            // Helper function to create a disjunction term
            let create_disj_term = |term: Term, is_left: bool| -> Term {
                if is_left {
                    foundation.make_or_intro_left(term)
                } else {
                    foundation.make_or_intro_right(term)
                }
            };
            
            // Helper function to update state with a new term
            let update_state = |state: &mut ProofState, term: Term, new_context: TacticalContext, goal: Proposition| -> Result<ProofState, TacticError> {
                *state.current_context_mut() = new_context.clone();
                state.set_current_term(TacticProofTerm::Complete(term.clone()));
                let new_node = ProofNode::new(new_context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
                state.add_child(new_node)?;
                let proof = GenericProof::new(goal.clone(), term);
                state.set_proof(proof);
                Ok(state.clone())
            };
            
            // Try direct proofs first
            if context.contains(a) {
                let left_term = foundation.make_assumption(a)?;
                
                // Add term to STLC context if needed
                if let Term::Variable(var_name) = &left_term {
                    let left_type = foundation.convert_proposition_to_type(a);
                    new_context.stlc_context_mut().add_variable(var_name.clone(), left_type);
                }
                
                let term = create_disj_term(left_term, true);
                return update_state(state, term, new_context, goal);
            }
            
            if context.contains(b) {
                let right_term = foundation.make_assumption(b)?;
                
                // Add term to STLC context if needed
                if let Term::Variable(var_name) = &right_term {
                    let right_type = foundation.convert_proposition_to_type(b);
                    new_context.stlc_context_mut().add_variable(var_name.clone(), right_type);
                }
                
                let term = create_disj_term(right_term, false);
                return update_state(state, term, new_context, goal);
            }
            
            // Handle nested disjunctions
            for premise in &context {
                if let Proposition::Or(ref p, ref q) = premise {
                    // Check if either disjunct matches what we need
                    let (matches_left, matches_right) = (
                        **p == **a || **q == **a,
                        **p == **b || **q == **b
                    );
                    
                    if matches_left || matches_right {
                        let premise_term = foundation.make_assumption(premise)?;
                        
                        // Add premise to STLC context if needed
                        if let Term::Variable(var_name) = &premise_term {
                            let premise_type = foundation.convert_proposition_to_type(premise);
                            new_context.stlc_context_mut().add_variable(var_name.clone(), premise_type);
                        }
                        
                        // Create appropriate disjunction term based on which disjunct matches
                        let term = if matches_left {
                            create_disj_term(premise_term, true)
                        } else {
                            create_disj_term(premise_term, false)
                        };
                        
                        return update_state(state, term, new_context, goal);
                    }
                }
            }
            
            // Handle nested disjunctions in the goal
            if let Proposition::Or(ref inner_a, ref inner_b) = **a {
                if context.contains(inner_a) || context.contains(inner_b) {
                    let mut inner_state = state.clone();
                    inner_state.set_goal(*a.clone());
                    let inner_proof = self.apply(&mut inner_state)?;
                    if let TacticProofTerm::Complete(inner_term) = inner_proof.current_node().term() {
                        let term = create_disj_term(inner_term.clone(), true);
                        return update_state(state, term, new_context, goal);
                    }
                }
            }
            
            if let Proposition::Or(ref inner_a, ref inner_b) = **b {
                if context.contains(inner_a) || context.contains(inner_b) {
                    let mut inner_state = state.clone();
                    inner_state.set_goal(*b.clone());
                    let inner_proof = self.apply(&mut inner_state)?;
                    if let TacticProofTerm::Complete(inner_term) = inner_proof.current_node().term() {
                        let term = create_disj_term(inner_term.clone(), false);
                        return update_state(state, term, new_context, goal);
                    }
                }
            }
        }
        
        Err(TacticError::NoMatchingRule)
    }
}

/// True introduction tactic (⊤I)
pub struct TrueIntroTactic;

impl ProofStateTactic for TrueIntroTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        if let Proposition::True = goal {
            // Create the True introduction term
            let term = foundation.make_true_intro();
            
            // Create new node with current context
            let new_node = ProofNode::new(context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
            state.add_child(new_node)?;
            
            let proof = GenericProof::new(goal, term);
            state.set_proof(proof);
            
            Ok(state.clone())
        } else {
            Err(TacticError::GoalMismatch {
                expected: Proposition::True,
                actual: goal,
            })
        }
    }
}

/// False elimination tactic (E)
pub struct FalseElimTactic;

impl ProofStateTactic for FalseElimTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;

        // Look for False in the context
        if context.contains(&Proposition::False) {
            // Get the false term
            let false_term = foundation.make_assumption(&Proposition::False)?;
            
            // Convert goal proposition to a type
            let goal_type = foundation.convert_proposition_to_type(&goal);
            
            // Create the false elimination term
            let term = foundation.make_false_elim(false_term, goal_type);

            // Create new proof node and update state
            let new_node = ProofNode::new(context, TacticProofTerm::Complete(term.clone()), goal.clone());
            state.add_child(new_node)?;
            let proof = GenericProof::new(goal, term);
            state.set_proof(proof);
            return Ok(state.clone());
        } else {
            Err(TacticError::NoMatchingRule)
        }
    }
}

/// Or elimination tactic
pub struct OrElimTactic;

impl ProofStateTactic for OrElimTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        for premise in &context {
            if let Proposition::Or(a, b) = &premise {
                // Try to prove goal from both cases
                let proof_a = foundation.make_assumption(&a)?;
                let proof_b = foundation.make_assumption(&b)?;
                let proof_or = foundation.make_assumption(&premise)?;
                let term = foundation.make_or_elim(proof_or, proof_a, proof_b);
                let new_node = ProofNode::new(context, TacticProofTerm::Complete(term.clone()), goal.clone());
                state.add_child(new_node)?;
                let proof = GenericProof::new(goal, term);
                state.set_proof(proof);
                return Ok(state.clone());
            }
        }
        Err(TacticError::NoMatchingRule)
    }
}

/// Not introduction tactic
pub struct NotIntroTactic;

impl ProofStateTactic for NotIntroTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = state.foundation.clone();
        
        if let Proposition::Not(a) = &goal {
            // Try to derive contradiction from assumption
            let mut extended_context = context.clone();
            extended_context.add_proposition(*a.clone());
            if let Ok(contradiction) = <FalseElimTactic as ProofStateTactic>::apply(&FalseElimTactic, state) {
                let term = foundation.make_not_intro(match contradiction.conclusion {
                    TacticProofTerm::Complete(t) => t,
                    _ => return Err(TacticError::RuleApplicationFailed("Incomplete proof term".to_string())),
                });
                let new_node = ProofNode::new(context, TacticProofTerm::Complete(term.clone()), goal.clone());
                state.add_child(new_node)?;
                let proof = GenericProof::new(goal, term);
                state.set_proof(proof);
                return Ok(state.clone());
            }
        }
        Err(TacticError::NoMatchingRule)
    }
}

/// Not elimination tactic
pub struct NotElimTactic;

impl ProofStateTactic for NotElimTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        // Look for both A and ¬A in context
        for premise in &context {
            if let Proposition::Not(a) = premise {
                if context.contains(&*a) {
                    let proof_a = foundation.make_assumption(&*a)?;
                    let proof_not_a = foundation.make_assumption(&premise)?;
                    let term = foundation.make_not_elim(proof_a, proof_not_a);
                    let new_node = ProofNode::new(context, TacticProofTerm::Complete(term.clone()), goal.clone());
                    state.add_child(new_node)?;
                    let proof = GenericProof::new(goal, term);
                    state.set_proof(proof);
                    return Ok(state.clone());
                }
            }
        }
        Err(TacticError::NoMatchingRule)
    }
}

/// Contraposition tactic: (P→Q) ⊢ (¬Q→¬P)
pub struct ContrapositionTactic;

impl ProofStateTactic for ContrapositionTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        if let Proposition::Implies(not_q, not_p) = &goal {
            if let (Proposition::Not(q), Proposition::Not(p)) = (&**not_q, &**not_p) {
                // Look for P→Q in context
                for premise in &context {
                    if let Proposition::Implies(p2, q2) = premise {
                        if *p2 == *p && *q2 == *q {
                            let proof_impl = foundation.make_assumption(&premise)?;
                            let term = foundation.make_contraposition(proof_impl);
                            let new_node = ProofNode::new(context, TacticProofTerm::Complete(term.clone()), goal.clone());
                            state.add_child(new_node)?;
                            let proof = GenericProof::new(goal, term);
                            state.set_proof(proof);
                            return Ok(state.clone());
                        }
                    }
                }
            }
        }
        Err(TacticError::NoMatchingRule)
    }
}

/// Hypothetical Syllogism tactic: P→Q, Q→R ⊢ P→R
pub struct HypotheticalSyllogismTactic;

impl ProofStateTactic for HypotheticalSyllogismTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        if let Proposition::Implies(p, r) = &goal {
            // Look for implications that can be chained
            for p1 in &context {
                if let Proposition::Implies(p_inner, q1) = p1 {
                    if p_inner == p {  // Found P → Q
                        for p2 in &context {
                            if let Proposition::Implies(q2, r_inner) = p2 {
                                if q1 == q2 && r_inner == r {  // Found Q → R
                                    // Get the terms for both implications
                                    let proof_impl1 = foundation.make_assumption(&p1)?;
                                    let proof_impl2 = foundation.make_assumption(&p2)?;
                                    
                                    // Add both terms to STLC context
                                    let mut new_context = context.clone();
                                    let type1 = foundation.convert_proposition_to_type(&p1);
                                    let type2 = foundation.convert_proposition_to_type(&p2);
                                    
                                    if let Term::Variable(var1) = &proof_impl1 {
                                        if new_context.stlc_context().get_type(var1).is_none() {
                                            new_context.stlc_context_mut().add_variable(var1.clone(), type1);
                                        }
                                    }
                                    
                                    if let Term::Variable(var2) = &proof_impl2 {
                                        if new_context.stlc_context().get_type(var2).is_none() {
                                            new_context.stlc_context_mut().add_variable(var2.clone(), type2);
                                        }
                                    }
                                    
                                    // Create the composed implication using the current context
                                    let term = foundation.make_hypothetical_syllogism_with_context(
                                        proof_impl1.clone(),
                                        proof_impl2.clone(),
                                        new_context.stlc_context()
                                    );
                                    
                                    // Update the state's context
                                    *state.current_context_mut() = new_context.clone();
                                    
                                    // Set the current term to complete
                                    state.set_current_term(TacticProofTerm::Complete(term.clone()));
                                    
                                    // Create new node with updated context
                                    let new_node = ProofNode::new(new_context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
                                    state.add_child(new_node)?;
                                    
                                    // Create and set the proof
                                    let proof = GenericProof::new(goal.clone(), term);
                                    state.set_proof(proof);
                                    
                                    return Ok(state.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Err(TacticError::NoMatchingRule)
    }
}

/// Disjunctive Syllogism tactic: P∨Q, ¬P ⊢ Q
pub struct DisjunctiveSyllogismTactic;

impl ProofStateTactic for DisjunctiveSyllogismTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        // Look for P∨Q and ¬P in context
        for p_or_q in &context {
            if let Proposition::Or(p, q) = p_or_q {
                if let Some(not_p) = (&context).into_iter().find(|prop| {
                    matches!(prop, Proposition::Not(inner) if **inner == **p)
                }) {
                    if **q == goal {
                        let proof_or = foundation.make_assumption(&p_or_q)?;
                        let proof_not_p = foundation.make_assumption(not_p)?;
                        let term = foundation.make_disjunctive_syllogism(proof_or, proof_not_p);
                        let new_node = ProofNode::new(context.clone(), TacticProofTerm::Complete(term.clone()), goal.clone());
                        state.add_child(new_node)?;
                        let proof = GenericProof::new(goal, term);
                        state.set_proof(proof);
                        return Ok(state.clone());
                    }
                }
            }
        }
        Err(TacticError::NoMatchingRule)
    }
}

/// Constructive Dilemma tactic: (P→Q)∧(R→S), P∨R ⊢ Q∨S
pub struct ConstructiveDilemmaTactic;

impl ProofStateTactic for ConstructiveDilemmaTactic {
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let goal = state.current_goal().clone();
        let context = state.current_context().clone();
        let foundation = &state.foundation;
        
        if let Proposition::Or(q, s) = &goal {
            // Look for (P→Q)∧(R→S) and P∨R in context
            for and_prop in &context {
                if let Proposition::And(imp1, imp2) = and_prop {
                    if let (
                        Proposition::Implies(ref p1, ref q1),
                        Proposition::Implies(ref r1, ref s1)
                    ) = (&**imp1, &**imp2) {
                        // Look for P∨R
                        if let Some(or_prop) = (&context).into_iter().find(|prop| {
                            matches!(prop, Proposition::Or(p2, r2) 
                                if p2 == p1 && r2 == r1)
                        }) {
                            if *q1 == *q && *s1 == *s {
                                let proof_and = foundation.make_assumption(&and_prop)?;
                                let proof_or = foundation.make_assumption(&or_prop)?;
                                let term = foundation.make_constructive_dilemma(proof_and, proof_or);
                                let new_node = ProofNode::new(context, TacticProofTerm::Complete(term.clone()), goal.clone());
                                state.add_child(new_node)?;
                                let proof = GenericProof::new(goal, term);
                                state.set_proof(proof);
                                return Ok(state.clone());
                            }
                        }
                    }
                }
            }
        }
        Err(TacticError::NoMatchingRule)
    }
}
