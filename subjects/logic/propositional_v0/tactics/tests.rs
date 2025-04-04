use crate::formalize_v2::foundational_theories::type_theory_v2::calculi::simply_typed::terms::Term;
use crate::formalize_v2::subjects::logic::propositional::foundations::TypeTheoryFoundation;
use crate::formalize_v2::subjects::logic::propositional::tactics::basic::{
    AssumptionTactic, ProofStateTactic,
};
use crate::formalize_v2::subjects::logic::propositional::tactics::proof_state::{
    ProofState, TacticProofTerm,
};
use crate::formalize_v2::subjects::logic::propositional::tactics::TacticError;
use crate::formalize_v2::subjects::logic::propositional::{Foundation, ProofTerm};
use crate::parse::Parse;
use crate::{
    formalize_v2::subjects::logic::propositional::Proposition, parse::entities::Identifier,
};

use super::basic::{
    AndIntroTactic, ConstructiveDilemmaTactic, ContrapositionTactic, DNEtactic,
    DisjunctiveSyllogismTactic, FalseElimTactic, HypotheticalSyllogismTactic, ImpliesElimTactic,
    ImpliesIntroTactic, OrIntroTactic, TrueIntroTactic,
};

#[test]
fn test_basic_tactics() -> Result<(), TacticError> {
    // Test True introduction
    let goal = Proposition::True;

    let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
    assert!(TrueIntroTactic.apply(&mut state).is_ok());

    // Test False elimination
    let goal = Proposition::Atomic(Identifier::parse("A"));
    let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());

    // Add False to the context
    let false_prop = Proposition::False;
    state
        .current_context_mut()
        .add_proposition(false_prop.clone());

    // Get what we need from foundation first
    let foundation = &state.foundation;
    let prop_type = foundation.convert_proposition_to_type(&false_prop);
    let false_term = foundation
        .make_assumption(&false_prop)
        .expect("Failed to make assumption");

    // Now get the context
    let mut context = state.current_context_mut();
    if let Term::Variable(var_name) = false_term {
        context.stlc_context_mut().add_variable(var_name, prop_type);
    }

    assert!(FalseElimTactic.apply(&mut state).is_ok());

    // Test And introduction
    println!("\n=== Testing And Introduction ===");
    let a = Proposition::Atomic(Identifier::parse("A"));
    let b = Proposition::Atomic(Identifier::parse("B"));
    let goal = Proposition::And(Box::new(a.clone()), Box::new(b.clone()));
    println!("Goal: {:?}", goal);
    let context = vec![a.clone(), b.clone()];
    let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
    println!("Initial state created");

    for prop in context {
        println!("Adding proposition to context: {:?}", prop);
        state.current_context_mut().add_proposition(prop.clone());

        // Add to STLC context with proper type
        let term = state
            .foundation
            .make_assumption(&prop)
            .expect("Failed to make assumption");
        if let Term::Variable(var_name) = term {
            let ty = state.foundation.convert_proposition_to_type(&prop);
            state
                .current_context_mut()
                .stlc_context_mut()
                .add_variable(var_name.clone(), ty.clone());
            println!("Added to STLC context: {:?} with type {:?}", var_name, ty);
        }
    }

    println!(
        "STLC Context before tactic: {:?}\n",
        state.current_context().stlc_context()
    );
    println!(
        "Logical Context before tactic: {:?}\n",
        state.current_context()
    );

    let tactic = AndIntroTactic::new(AssumptionTactic, AssumptionTactic);
    println!("Created AndIntroTactic with AssumptionTactic for both parts\n");

    println!("Applying tactic...\n");
    let result = tactic.apply(&mut state);
    println!("Tactic result: {:?}\n", result);

    if let Ok(proof) = &result {
        println!("Final term: {:?}\n", state.current_node().term());
        println!(
            "Final type: {:?}\n",
            state.foundation.convert_proposition_to_type(&goal)
        );
    }

    assert!(result.is_ok());

    // Test Or introduction
    let a = Proposition::Atomic(Identifier::parse("A"));
    let b = Proposition::Atomic(Identifier::parse("B"));
    let goal = Proposition::Or(Box::new(a.clone()), Box::new(b));
    let context = vec![a];
    let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
    for prop in context {
        state.current_context_mut().add_proposition(prop);
    }
    assert!(OrIntroTactic.apply(&mut state).is_ok());

    Ok(())
}

#[test]
fn test_implication() {
    let a = Proposition::Atomic(Identifier::parse("A"));
    let b = Proposition::Atomic(Identifier::parse("B"));

    // Test implication introduction with A and B in context
    let goal = Proposition::Implies(Box::new(a.clone()), Box::new(b.clone()));
    let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());

    // Add A and B to context
    state.current_context_mut().add_proposition(a.clone());
    let term = state
        .foundation
        .make_assumption(&a)
        .expect("Failed to make assumption");
    if let Term::Variable(var_name) = term {
        let ty = state.foundation.convert_proposition_to_type(&a);
        state
            .current_context_mut()
            .stlc_context_mut()
            .add_variable(var_name, ty);
    }

    state.current_context_mut().add_proposition(b.clone());
    let term = state
        .foundation
        .make_assumption(&b)
        .expect("Failed to make assumption");
    if let Term::Variable(var_name) = term {
        let ty = state.foundation.convert_proposition_to_type(&b);
        state
            .current_context_mut()
            .stlc_context_mut()
            .add_variable(var_name, ty);
    }

    assert!(ImpliesIntroTactic.apply(&mut state).is_ok());

    // Test implication elimination (modus ponens)
    let impl_prop = Proposition::Implies(Box::new(a.clone()), Box::new(b.clone()));
    let context = vec![impl_prop, a];
    let mut state = ProofState::new(Foundation::TypeTheory, b);
    for prop in context {
        state.current_context_mut().add_proposition(prop.clone());

        // Add to STLC context with proper type
        let term = state
            .foundation
            .make_assumption(&prop)
            .expect("Failed to make assumption");
        if let Term::Variable(var_name) = term {
            let ty = state.foundation.convert_proposition_to_type(&prop);
            state
                .current_context_mut()
                .stlc_context_mut()
                .add_variable(var_name, ty);
        }
    }
    assert!(ImpliesElimTactic.apply(&mut state).is_ok());
}

#[test]
fn test_complex_implications() -> Result<(), TacticError> {
    let p = Proposition::Atomic(Identifier::parse("P"));
    let q = Proposition::Atomic(Identifier::parse("Q"));
    let r = Proposition::Atomic(Identifier::parse("R"));

    // Test transitivity: P→Q, Q→R ⊢ P→R
    let p_imp_q = Proposition::Implies(Box::new(p.clone()), Box::new(q.clone()));
    let q_imp_r = Proposition::Implies(Box::new(q.clone()), Box::new(r.clone()));
    let p_imp_r = Proposition::Implies(Box::new(p.clone()), Box::new(r.clone()));

    let mut state = ProofState::new(Foundation::TypeTheory, p_imp_r.clone());

    // Add both implications to context
    let context_props = vec![p_imp_q.clone(), q_imp_r.clone()];

    // Add propositions to context and their terms to STLC context
    for prop in context_props {
        // First add to logical context
        state.current_context_mut().add_proposition(prop.clone());

        // Then add to STLC context with proper type
        let term = state.foundation.make_assumption(&prop)?;
        if let Term::Variable(var_name) = term {
            let ty = state.foundation.convert_proposition_to_type(&prop);
            state
                .current_context_mut()
                .stlc_context_mut()
                .add_variable(var_name, ty);
        }
    }

    // Use hypothetical syllogism to get P→R
    assert!(HypotheticalSyllogismTactic.apply(&mut state).is_ok());

    // Print debug info
    println!("Context: {:?}", state.current_context().stlc_context());
    println!("Term: {:?}", state.current_node().term());
    println!(
        "Expected Type: {:?}",
        state.foundation.convert_proposition_to_type(&p_imp_r)
    );

    // Verify the final proof
    assert!(state.foundation.verify_proof(
        state.current_context().stlc_context(),
        match state.current_node().term() {
            TacticProofTerm::Complete(t) => t,
            _ => panic!("Expected complete proof term"),
        },
        &state.foundation.convert_proposition_to_type(&p_imp_r)
    ));

    Ok(())
}

#[test]
fn test_logical_laws() {
    let p = Proposition::Atomic(Identifier::parse("P"));
    let q = Proposition::Atomic(Identifier::parse("Q"));

    // Test double negation: ¬¬P ⊢ P
    let not_not_p = Proposition::Not(Box::new(Proposition::Not(Box::new(p.clone()))));
    let context = vec![not_not_p.clone()];
    let mut state = ProofState::new(Foundation::TypeTheory, p.clone());
    for prop in context {
        state.current_context_mut().add_proposition(prop);
    }
    assert!(DNEtactic.apply(&mut state).is_ok());

    // Test contraposition: (P→Q) ⊢ (¬Q→¬P)
    let p_imp_q = Proposition::Implies(Box::new(p.clone()), Box::new(q.clone()));
    let not_q = Proposition::Not(Box::new(q.clone()));
    let not_p = Proposition::Not(Box::new(p.clone()));
    let contra = Proposition::Implies(Box::new(not_q.clone()), Box::new(not_p.clone()));
    let context = vec![p_imp_q.clone()];
    let mut state = ProofState::new(Foundation::TypeTheory, contra.clone());
    for prop in context {
        state.current_context_mut().add_proposition(prop);
    }
    assert!(ContrapositionTactic.apply(&mut state).is_ok());
}

#[test]
fn test_disjunctive_syllogism() {
    let p = Proposition::Atomic(Identifier::parse("P"));
    let q = Proposition::Atomic(Identifier::parse("Q"));

    // Test P∨Q, ¬P ⊢ Q
    let p_or_q = Proposition::Or(Box::new(p.clone()), Box::new(q.clone()));
    let not_p = Proposition::Not(Box::new(p.clone()));
    let context = vec![p_or_q.clone(), not_p.clone()];
    let mut state = ProofState::new(Foundation::TypeTheory, q.clone());
    for prop in context {
        state.current_context_mut().add_proposition(prop);
    }
    assert!(DisjunctiveSyllogismTactic.apply(&mut state).is_ok());
}

#[test]
fn test_constructive_dilemma() {
    let p = Proposition::Atomic(Identifier::parse("P"));
    let q = Proposition::Atomic(Identifier::parse("Q"));
    let r = Proposition::Atomic(Identifier::parse("R"));
    let s = Proposition::Atomic(Identifier::parse("S"));

    // Test (P→Q)∧(R→S), P∨R ⊢ Q∨S
    let p_imp_q = Proposition::Implies(Box::new(p.clone()), Box::new(q.clone()));
    let r_imp_s = Proposition::Implies(Box::new(r.clone()), Box::new(s.clone()));
    let implications = Proposition::And(Box::new(p_imp_q), Box::new(r_imp_s));
    let p_or_r = Proposition::Or(Box::new(p), Box::new(r));
    let q_or_s = Proposition::Or(Box::new(q), Box::new(s));

    let context = vec![implications.clone(), p_or_r.clone()];
    let mut state = ProofState::new(Foundation::TypeTheory, q_or_s.clone());
    for prop in context {
        state.current_context_mut().add_proposition(prop);
    }
    assert!(ConstructiveDilemmaTactic.apply(&mut state).is_ok());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formalize_v2::subjects::logic::propositional::{Foundation, Proposition};
    use crate::parse::Parse;

    #[test]
    fn test_assumption_tactic_edge_cases() -> Result<(), TacticError> {
        // Test 1: Empty context
        let goal = Proposition::Atomic(Identifier::parse("A"));
        let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
        assert!(AssumptionTactic.apply(&mut state).is_err());

        // Test 2: Goal not in context but similar proposition is
        let goal = Proposition::Atomic(Identifier::parse("A"));
        let similar = Proposition::Not(Box::new(Proposition::Atomic(Identifier::parse("A"))));
        let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
        state.current_context_mut().add_proposition(similar);
        assert!(AssumptionTactic.apply(&mut state).is_err());

        // Test 3: Multiple identical propositions in context
        let goal = Proposition::Atomic(Identifier::parse("A"));
        let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
        state.current_context_mut().add_proposition(goal.clone());
        state.current_context_mut().add_proposition(goal.clone());
        assert!(AssumptionTactic.apply(&mut state).is_ok());

        Ok(())
    }

    #[test]
    fn test_and_intro_tactic_edge_cases() -> Result<(), TacticError> {
        // Test 1: Trying to prove A∧B with only A in context
        let a = Proposition::Atomic(Identifier::parse("A"));
        let b = Proposition::Atomic(Identifier::parse("B"));
        let goal = Proposition::And(Box::new(a.clone()), Box::new(b.clone()));
        let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
        state.current_context_mut().add_proposition(a.clone());
        let tactic = AndIntroTactic::new(AssumptionTactic, AssumptionTactic);
        assert!(tactic.apply(&mut state).is_err());

        // Test 2: Nested conjunction (A∧B)∧C
        let c = Proposition::Atomic(Identifier::parse("C"));
        let ab = Proposition::And(Box::new(a.clone()), Box::new(b.clone()));
        let goal = Proposition::And(Box::new(ab.clone()), Box::new(c.clone()));
        let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
        state.current_context_mut().add_proposition(a.clone());
        state.current_context_mut().add_proposition(b.clone());
        state.current_context_mut().add_proposition(c.clone());
        let inner_tactic = AndIntroTactic::new(AssumptionTactic, AssumptionTactic);
        let outer_tactic = AndIntroTactic::new(inner_tactic, AssumptionTactic);
        assert!(outer_tactic.apply(&mut state).is_ok());

        Ok(())
    }

    #[test]
    fn test_implies_intro_tactic_edge_cases() -> Result<(), TacticError> {
        // Test 1: Trying to prove A→B with A→B already in context
        let a = Proposition::Atomic(Identifier::parse("A"));
        let b = Proposition::Atomic(Identifier::parse("B"));
        let goal = Proposition::Implies(Box::new(a.clone()), Box::new(b.clone()));
        let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
        state.current_context_mut().add_proposition(goal.clone());

        // Add to STLC context with proper type
        let term = state.foundation.make_assumption(&goal)?;
        if let Term::Variable(var_name) = term {
            let ty = state.foundation.convert_proposition_to_type(&goal);
            state
                .current_context_mut()
                .stlc_context_mut()
                .add_variable(var_name, ty);
        }

        assert!(ImpliesIntroTactic.apply(&mut state).is_ok());

        // Test 2: Nested implication A→(B→C)
        let c = Proposition::Atomic(Identifier::parse("C"));
        let inner = Proposition::Implies(Box::new(b.clone()), Box::new(c.clone()));
        let goal = Proposition::Implies(Box::new(a.clone()), Box::new(inner.clone()));
        let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());

        // Add inner implication to context
        state.current_context_mut().add_proposition(inner.clone());

        // Add to STLC context with proper type
        let term = state.foundation.make_assumption(&inner)?;
        if let Term::Variable(var_name) = term {
            let ty = state.foundation.convert_proposition_to_type(&inner);
            state
                .current_context_mut()
                .stlc_context_mut()
                .add_variable(var_name, ty);
        }

        assert!(ImpliesIntroTactic.apply(&mut state).is_ok());

        Ok(())
    }

    #[test]
    fn test_implies_elim_tactic_edge_cases() -> Result<(), TacticError> {
        // Test 1: Multiple applicable implications in context
        let a = Proposition::Atomic(Identifier::parse("A"));
        let b = Proposition::Atomic(Identifier::parse("B"));
        let c = Proposition::Atomic(Identifier::parse("C"));
        let impl1 = Proposition::Implies(Box::new(a.clone()), Box::new(b.clone()));
        let impl2 = Proposition::Implies(Box::new(a.clone()), Box::new(c.clone()));

        let mut state = ProofState::new(Foundation::TypeTheory, b.clone());
        state.current_context_mut().add_proposition(impl1.clone());
        state.current_context_mut().add_proposition(impl2.clone());
        state.current_context_mut().add_proposition(a.clone());
        assert!(ImpliesElimTactic.apply(&mut state).is_ok());

        // Test 2: Chained implications (A→B)→(B→C)→(A→C)
        let impl_b_c = Proposition::Implies(Box::new(b.clone()), Box::new(c.clone()));
        let impl_a_c = Proposition::Implies(Box::new(a.clone()), Box::new(c.clone()));
        let mut state = ProofState::new(Foundation::TypeTheory, impl_a_c.clone());
        state.current_context_mut().add_proposition(impl1.clone());
        state
            .current_context_mut()
            .add_proposition(impl_b_c.clone());
        assert!(HypotheticalSyllogismTactic.apply(&mut state).is_ok());

        Ok(())
    }

    #[test]
    fn test_or_intro_tactic_edge_cases() -> Result<(), TacticError> {
        // Test 1: Introducing disjunction with neither disjunct in context
        let a = Proposition::Atomic(Identifier::parse("A"));
        let b = Proposition::Atomic(Identifier::parse("B"));
        let goal = Proposition::Or(Box::new(a.clone()), Box::new(b.clone()));
        let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
        assert!(OrIntroTactic.apply(&mut state).is_err());

        // Test 2: Nested disjunction (A∨B)∨C
        let c = Proposition::Atomic(Identifier::parse("C"));
        let ab = Proposition::Or(Box::new(a.clone()), Box::new(b.clone()));
        let goal = Proposition::Or(Box::new(ab.clone()), Box::new(c.clone()));
        let mut state = ProofState::new(Foundation::TypeTheory, goal.clone());
        state.current_context_mut().add_proposition(a.clone());
        assert!(OrIntroTactic.apply(&mut state).is_ok());

        Ok(())
    }

    #[test]
    fn test_false_elim_tactic_edge_cases() -> Result<(), TacticError> {
        // Test 1: False elimination to prove any proposition
        let complex_goal = Proposition::And(
            Box::new(Proposition::Atomic(Identifier::parse("A"))),
            Box::new(Proposition::Or(
                Box::new(Proposition::Atomic(Identifier::parse("B"))),
                Box::new(Proposition::Atomic(Identifier::parse("C"))),
            )),
        );
        let mut state = ProofState::new(Foundation::TypeTheory, complex_goal.clone());
        state
            .current_context_mut()
            .add_proposition(Proposition::False);
        assert!(FalseElimTactic.apply(&mut state).is_ok());

        // Test 2: False not directly in context but derivable
        let not_a = Proposition::Not(Box::new(Proposition::Atomic(Identifier::parse("A"))));
        let mut state = ProofState::new(Foundation::TypeTheory, Proposition::False);
        state
            .current_context_mut()
            .add_proposition(Proposition::Atomic(Identifier::parse("A")));
        state.current_context_mut().add_proposition(not_a);
        // This should fail as FalseElimTactic expects False directly in context
        assert!(FalseElimTactic.apply(&mut state).is_err());

        Ok(())
    }

    #[test]
    fn test_double_negation_tactic_edge_cases() -> Result<(), TacticError> {
        // Test 1: Triple negation
        let a = Proposition::Atomic(Identifier::parse("A"));
        let not_a = Proposition::Not(Box::new(a.clone()));
        let not_not_a = Proposition::Not(Box::new(not_a.clone()));
        let not_not_not_a = Proposition::Not(Box::new(not_not_a.clone()));

        let mut state = ProofState::new(Foundation::TypeTheory, not_a.clone());
        state.current_context_mut().add_proposition(not_not_not_a);
        assert!(DNEtactic.apply(&mut state).is_ok());

        // Test 2: Double negation of complex formula
        let complex = Proposition::And(
            Box::new(a.clone()),
            Box::new(Proposition::Atomic(Identifier::parse("B"))),
        );
        let not_not_complex =
            Proposition::Not(Box::new(Proposition::Not(Box::new(complex.clone()))));
        let mut state = ProofState::new(Foundation::TypeTheory, complex.clone());
        state.current_context_mut().add_proposition(not_not_complex);
        assert!(DNEtactic.apply(&mut state).is_ok());

        Ok(())
    }
}

#[cfg(test)]
mod intensive_tests {
    use super::*;
    use crate::formalize_v2::subjects::logic::propositional::{Foundation, Proposition};
    use crate::parse::Parse;

    #[test]
    fn test_assumption_tactic_intensive() -> Result<(), TacticError> {
        println!("\n=== Intensive Test: AssumptionTactic ===");

        // Setup complex propositions
        let a = Proposition::Atomic(Identifier::parse("A"));
        let b = Proposition::Atomic(Identifier::parse("B"));
        let c = Proposition::Atomic(Identifier::parse("C"));

        let not_a = Proposition::Not(Box::new(a.clone()));
        let a_and_b = Proposition::And(Box::new(a.clone()), Box::new(b.clone()));
        let b_or_c = Proposition::Or(Box::new(b.clone()), Box::new(c.clone()));
        let complex_prop =
            Proposition::Implies(Box::new(a_and_b.clone()), Box::new(b_or_c.clone()));

        // Test 1: Basic assumption with multiple similar propositions
        println!("Test 1: Basic assumption with multiple similar propositions");
        let mut state = ProofState::new(Foundation::TypeTheory, a.clone());
        state.current_context_mut().add_proposition(not_a.clone()); // Add ¬A
        state.current_context_mut().add_proposition(a.clone()); // Add A
        state.current_context_mut().add_proposition(a_and_b.clone()); // Add A∧B
        let result = AssumptionTactic.apply(&mut state);
        println!("Result: {:?}", result);
        assert!(result.is_ok());

        // Test 2: Complex proposition in complex context
        println!("Test 2: Complex proposition in complex context");
        let mut state = ProofState::new(Foundation::TypeTheory, complex_prop.clone());
        state.current_context_mut().add_proposition(a.clone());
        state.current_context_mut().add_proposition(b.clone());
        state
            .current_context_mut()
            .add_proposition(complex_prop.clone());
        let result = AssumptionTactic.apply(&mut state);
        println!("Result: {:?}", result);
        assert!(result.is_ok());

        // Test 3: Assumption with type checking
        println!("Test 3: Assumption with type checking");
        let mut state = ProofState::new(Foundation::TypeTheory, a.clone());
        state.current_context_mut().add_proposition(a.clone());
        let result = AssumptionTactic.apply(&mut state);
        if let Ok(state) = result {
            let term = match state.current_node().term() {
                TacticProofTerm::Complete(t) => t,
                _ => panic!("Expected complete term"),
            };
            let ty = state.foundation.convert_proposition_to_type(&a);
            assert!(state.foundation.verify_proof(
                state.current_context().stlc_context(),
                term,
                &ty
            ));
        }

        // Test 4: Assumption in presence of contradictions
        println!("Test 4: Assumption in presence of contradictions");
        let mut state = ProofState::new(Foundation::TypeTheory, a.clone());
        state.current_context_mut().add_proposition(a.clone());
        state.current_context_mut().add_proposition(not_a.clone());
        state
            .current_context_mut()
            .add_proposition(Proposition::False);
        let result = AssumptionTactic.apply(&mut state);
        println!("Result: {:?}", result);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_and_intro_tactic_intensive() -> Result<(), TacticError> {
        println!("\n=== Intensive Test: AndIntroTactic ===");

        // Setup propositions
        let a = Proposition::Atomic(Identifier::parse("A"));
        let b = Proposition::Atomic(Identifier::parse("B"));
        let c = Proposition::Atomic(Identifier::parse("C"));
        let d = Proposition::Atomic(Identifier::parse("D"));

        // Test 1: Deeply nested conjunction
        println!("Test 1: Deeply nested conjunction");
        let inner_and = Proposition::And(Box::new(a.clone()), Box::new(b.clone()));
        let middle_and = Proposition::And(Box::new(inner_and.clone()), Box::new(c.clone()));
        let outer_and = Proposition::And(Box::new(middle_and.clone()), Box::new(d.clone()));

        let mut state = ProofState::new(Foundation::TypeTheory, outer_and.clone());
        state.current_context_mut().add_proposition(a.clone());
        state.current_context_mut().add_proposition(b.clone());
        state.current_context_mut().add_proposition(c.clone());
        state.current_context_mut().add_proposition(d.clone());

        let inner_tactic = AndIntroTactic::new(AssumptionTactic, AssumptionTactic);
        let middle_tactic = AndIntroTactic::new(inner_tactic.clone(), AssumptionTactic);
        let outer_tactic = AndIntroTactic::new(middle_tactic, AssumptionTactic);

        let result = outer_tactic.apply(&mut state);
        println!("Result: {:?}", result);
        assert!(result.is_ok());

        // Test 2: Conjunction with complex subformulas
        println!("Test 2: Conjunction with complex subformulas");
        let not_a = Proposition::Not(Box::new(a.clone()));
        let b_or_c = Proposition::Or(Box::new(b.clone()), Box::new(c.clone()));
        let complex_and = Proposition::And(Box::new(not_a.clone()), Box::new(b_or_c.clone()));

        let mut state = ProofState::new(Foundation::TypeTheory, complex_and.clone());
        state.current_context_mut().add_proposition(not_a.clone());
        state.current_context_mut().add_proposition(b.clone()); // For b_or_c

        let tactic = AndIntroTactic::new(AssumptionTactic, OrIntroTactic);
        let result = tactic.apply(&mut state);
        println!("Result: {:?}", result);
        assert!(result.is_ok());

        // Test 3: Conjunction with shared subformulas
        println!("Test 3: Conjunction with shared subformulas");
        let shared_and = Proposition::And(Box::new(a.clone()), Box::new(a.clone()));
        let mut state = ProofState::new(Foundation::TypeTheory, shared_and.clone());
        state.current_context_mut().add_proposition(a.clone());

        let tactic = AndIntroTactic::new(AssumptionTactic, AssumptionTactic);
        let result = tactic.apply(&mut state);
        println!("Result: {:?}", result);
        assert!(result.is_ok());

        // Test 4: Type checking the final term
        println!("Test 4: Type checking the final term");
        let simple_and = Proposition::And(Box::new(a.clone()), Box::new(b.clone()));
        let mut state = ProofState::new(Foundation::TypeTheory, simple_and.clone());
        state.current_context_mut().add_proposition(a.clone());
        state.current_context_mut().add_proposition(b.clone());

        let tactic = AndIntroTactic::new(AssumptionTactic, AssumptionTactic);
        let result = tactic.apply(&mut state)?;

        let term = match result.current_node().term() {
            TacticProofTerm::Complete(t) => t,
            _ => panic!("Expected complete term"),
        };
        let ty = result.foundation.convert_proposition_to_type(&simple_and);
        assert!(result
            .foundation
            .verify_proof(result.current_context().stlc_context(), term, &ty));

        Ok(())
    }
}
