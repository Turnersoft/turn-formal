// Module: src/formalize_v2/subjects/math/theorem/test/proof_example.rs
// Examples of using the proof builder to prove theorems with branching

use std::collections::HashMap;

use uuid::Uuid;

use super::super::super::formalism::core::{ProofGoal, Theorem};
use super::super::super::formalism::expressions::{Identifier, MathExpression, TheoryExpression};
use super::super::super::formalism::proof::{
    CaseAnalysisBuilder, CaseResult, ProofForest, ProofNode, ProofStatus, Tactic,
};
use super::super::super::formalism::relations::MathRelation;
use super::super::super::theories::groups::definitions::GroupProperty;
use super::super::super::theories::rings::definitions::{Ring, RingExpression};

/// Helper function to create a variable expression
fn create_var(name: &str) -> MathExpression {
    MathExpression::Var(Identifier::Name(name.to_string(), 0))
}

/// Helper function to create a simple ring-based expression
fn create_expr(expr_str: &str) -> MathExpression {
    // This is a simplification - in a real implementation, we would parse the expression
    // For now, we'll just create a variable with the same name
    let ring = Ring::default();
    let ring_var = RingExpression::variable(ring, expr_str);
    MathExpression::Expression(TheoryExpression::Ring(ring_var))
}

/// Example: Proving Group Associativity Theorem
///
/// This example shows how to use the proof builder to create a theorem
/// with multiple proof branches at different depths.
pub fn prove_group_associativity() -> Theorem {
    // Create a theorem about group associativity
    let left = create_expr("a * (b * c)");
    let right = create_expr("(a * b) * c");

    // Create the initial proof goal and theorem
    let theorem_id = "group_associativity_theorem";
    let name = "Group Associativity";
    let statement = MathRelation::equal(left, right);

    let goal = ProofGoal::new(statement);

    let mut proofs = ProofForest::new();

    // Initialize the proof forest with a root node
    let mut p0 = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: None,
        children: vec![],
        state: goal.clone(),
        tactic: None,
        status: ProofStatus::InProgress,
    };
    proofs.add_node(p0.clone());

    // Main proof path
    let p1 = p0.tactics_intro_expr("a", create_expr("a"), &mut proofs);
    let p2 = p1.tactics_subs_expr(
        create_expr("x + y"),
        create_expr("x + y"),
        None,
        &mut proofs,
    );

    // Apply theorem and mark as complete
    let mut p3 = p2.tactics_theorem_app_expr(
        "group_axiom_associativity",
        HashMap::new(),
        None,
        &mut proofs,
    );
    p3 = p3.should_complete(&mut proofs);

    // Alternative branch from p1
    let alt1 = p1.clone();
    let alt2 = alt1.tactics_intro_expr("alternative approach", create_expr("alt"), &mut proofs);

    // Mark as work in progress
    let mut alt3 = alt2.tactics_subs_expr(
        create_expr("different substitution"),
        create_expr("different result"),
        None,
        &mut proofs,
    );
    alt3.status = ProofStatus::Wip;
    proofs.add_node(alt3.clone());

    // Deep branching example with 5+ levels
    let b1 = p1.clone();
    let b2 = b1.tactics_intro_expr("deep approach", create_expr("deep"), &mut proofs);

    let b2_1 = b2.clone();
    let b2_2 = b2_1.tactics_subs_expr(
        create_expr("level 3"),
        create_expr("level 3 result"),
        None,
        &mut proofs,
    );

    let b2_2_1 = b2_2.clone();
    let b2_2_2 = b2_2_1.tactics_intro_expr("level 4", create_expr("level 4"), &mut proofs);

    let b2_2_2_1 = b2_2_2.clone();
    let b2_2_2_2 = b2_2_2_1.tactics_subs_expr(
        create_expr("level 5"),
        create_expr("level 5 result"),
        None,
        &mut proofs,
    );

    // Go even deeper
    let b3_1 = b2_2_2_2.clone();
    let b3_2 = b3_1.tactics_intro_expr("level 6", create_expr("level 6"), &mut proofs);

    let b3_2_1 = b3_2.clone();
    let mut b3_2_2 = b3_2_1.tactics_subs_expr(
        create_expr("deepest level 7"),
        create_expr("deepest level 7 result"),
        None,
        &mut proofs,
    );

    // Mark as todo
    b3_2_2.status = ProofStatus::Todo;
    proofs.add_node(b3_2_2.clone());

    // Build the theorem
    Theorem {
        id: theorem_id.to_string(),
        name: name.to_string(),
        description: "Demonstrates the associative property of group operations".to_string(),
        goal,
        proofs,
    }
}

/// Example: Using case analysis for complex proofs
pub fn prove_with_bookmarks() -> Theorem {
    let left = create_expr("a * b");
    let right = create_expr("b * a");

    // Create the initial proof goal and theorem
    let theorem_id = "commutative_group_properties";
    let name = "Commutative Group Properties";
    let statement = MathRelation::equal(left, right);

    let goal = ProofGoal::new(statement);

    let mut proofs = ProofForest::new();

    // Initialize the proof forest with a root node
    let start = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: None,
        children: vec![],
        state: goal.clone(),
        tactic: None,
        status: ProofStatus::InProgress,
    };
    proofs.add_node(start.clone());

    // First key step
    let key_step = start.tactics_intro_expr(
        "commutativity property",
        create_expr("commutativity"),
        &mut proofs,
    );

    // Main branch - complete this one
    let step1 = key_step.tactics_subs_expr(
        create_expr("element exchange"),
        create_expr("exchanged"),
        None,
        &mut proofs,
    );

    let mut main_path = step1.tactics_theorem_app_expr(
        "group_axiom_commutativity",
        HashMap::new(),
        None,
        &mut proofs,
    );

    main_path.status = ProofStatus::Complete;
    proofs.add_node(main_path.clone());

    // Alternative approach 1
    let mut alt1 = key_step.clone();
    let alt1_step1 = alt1.tactics_intro_expr(
        "alternative approach",
        create_expr("alternative"),
        &mut proofs,
    );

    let mut alt1_step2 = alt1_step1.tactics_subs_expr(
        create_expr("direct application"),
        create_expr("direct result"),
        None,
        &mut proofs,
    );

    alt1_step2.status = ProofStatus::Wip;
    proofs.add_node(alt1_step2.clone());

    // Alternative approach 2 - different strategy
    let alt2_start = key_step.clone();
    let alt2_step1 = alt2_start.tactics_intro_expr(
        "inverse-based approach",
        create_expr("inverse"),
        &mut proofs,
    );

    let mut alt2_step2 = alt2_step1.tactics_subs_expr(
        create_expr("use inverses"),
        create_expr("inverse result"),
        None,
        &mut proofs,
    );

    alt2_step2.status = ProofStatus::Todo;
    proofs.add_node(alt2_step2.clone());

    // Build the theorem
    Theorem {
        id: theorem_id.to_string(),
        name: name.to_string(),
        description: "Explores properties of commutative groups".to_string(),
        goal,
        proofs,
    }
}

/// Example: Named proof steps for better clarity
pub fn prove_with_named_steps() -> Theorem {
    let left = create_expr("a * a⁻¹");
    let right = create_expr("e");

    // Create the initial proof goal and theorem
    let theorem_id = "inverse_element_theorem";
    let name = "Inverse Element Theorem";
    let statement = MathRelation::equal(left, right);

    let goal = ProofGoal::new(statement);

    let mut proofs = ProofForest::new();

    // Initialize the proof forest with a root node
    let initial = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: None,
        children: vec![],
        state: goal.clone(),
        tactic: None,
        status: ProofStatus::InProgress,
    };
    proofs.add_node(initial.clone());

    // Main branch with meaningful variable names
    let intro_step =
        initial.tactics_intro_expr("inverse property", create_expr("inverse"), &mut proofs);

    let substitution = intro_step.tactics_subs_expr(
        create_expr("definition of inverse"),
        create_expr("inverse definition"),
        None,
        &mut proofs,
    );

    let theorem_app = substitution.tactics_theorem_app_expr(
        "group_axiom_inverse",
        HashMap::new(),
        None,
        &mut proofs,
    );

    let mut completed = theorem_app.clone();
    completed.status = ProofStatus::Complete;
    proofs.add_node(completed.clone());

    // First branch exploring alternate approach
    let inverse_branch_1 = intro_step.clone();
    let inverse_approach_2 = inverse_branch_1.tactics_intro_expr(
        "use identity first",
        create_expr("identity first"),
        &mut proofs,
    );

    let inverse_identity = inverse_approach_2.tactics_subs_expr(
        create_expr("e * a * a⁻¹"),
        create_expr("identity applied"),
        None,
        &mut proofs,
    );

    let inverse_assoc = inverse_identity.tactics_theorem_app_expr(
        "group_axiom_associativity",
        HashMap::new(),
        None,
        &mut proofs,
    );

    let mut inverse_complete = inverse_assoc.clone();
    inverse_complete.status = ProofStatus::Complete;
    proofs.add_node(inverse_complete.clone());

    // Second branch exploring yet another approach
    let identity_branch_1 = intro_step.clone();
    let identity_approach_1 = identity_branch_1.tactics_intro_expr(
        "right identity approach",
        create_expr("right identity"),
        &mut proofs,
    );

    let identity_step_2 = identity_approach_1.tactics_subs_expr(
        create_expr("a * a⁻¹ * e"),
        create_expr("right identity applied"),
        None,
        &mut proofs,
    );

    let mut identity_wip = identity_step_2.clone();
    identity_wip.status = ProofStatus::Wip;
    proofs.add_node(identity_wip.clone());

    // Build the theorem
    Theorem {
        id: theorem_id.to_string(),
        name: name.to_string(),
        description: "Proves that g * g⁻¹ = e in a group".to_string(),
        goal,
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_associativity_proof() {
        let theorem = prove_group_associativity();
        // Assert theorem name and basic properties
        assert_eq!(theorem.name, "Group Associativity");
    }

    #[test]
    fn test_bookmarked_proof() {
        let theorem = prove_with_bookmarks();
        assert_eq!(theorem.name, "Commutative Group Properties");
    }

    #[test]
    fn test_named_steps_proof() {
        let theorem = prove_with_named_steps();
        assert_eq!(theorem.name, "Inverse Element Theorem");
    }
}
