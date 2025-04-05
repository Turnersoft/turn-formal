// Module: src/formalize_v2/subjects/math/theorem/test/proof_example.rs
// Examples of using the proof builder to prove theorems with branching

use std::collections::HashMap;

use crate::subjects::math::theorem::core::{ProofState, Theorem};
use crate::subjects::math::theorem::expressions::MathExpression;
use crate::subjects::math::theorem::proof::{
    ProofBranch, ProofForest, ProofStatus, Tactic, TheoremBuilder,
};
use crate::subjects::math::theorem::relations::MathRelation;
use crate::subjects::math::theories::groups::definitions::GroupProperty;

/// Example: Proving Group Associativity Theorem
///
/// This example shows how to use the proof builder to create a theorem
/// with multiple proof branches at different depths.
pub fn prove_group_associativity() -> Theorem {
    // Create a theorem about group associativity
    let left = MathExpression::string_expr("a * (b * c)");
    let right = MathExpression::string_expr("(a * b) * c");

    let builder = TheoremBuilder::new(
        "Group Associativity",
        MathRelation::equal(left, right),
        vec![], // No assumptions for this simple example
    );

    // Start with initial branch
    let p0 = builder.initial_branch();

    // Main proof path
    let p1 = p0.tactics_intro("a", 1);
    let p2 = p1.tactics_subs("x + y", 2);
    let p3_done = p2
        .tactics_theorem_app("group_axiom_associativity", HashMap::new())
        .should_complete();

    // Alternative branch from p1
    let alt1 = p1.branch();
    let alt2 = alt1.tactics_intro("alternative approach", 3);
    let alt3_wip = alt2.tactics_subs("different substitution", 2).mark_wip();

    // Deep branching example with 5+ levels
    let b1 = p1.branch();
    let b2 = b1.tactics_intro("deep approach", 4);

    let b2_1 = b2.branch();
    let b2_2 = b2_1.tactics_subs("level 3", 5);

    let b2_2_1 = b2_2.branch();
    let b2_2_2 = b2_2_1.tactics_intro("level 4", 6);

    let b2_2_2_1 = b2_2_2.branch();
    let b2_2_2_2 = b2_2_2_1.tactics_subs("level 5", 7);

    // Go even deeper
    let b3_1 = b2_2_2_2.branch();
    let b3_2 = b3_1.tactics_intro("level 6", 8);

    let b3_2_1 = b3_2.branch();
    let b3_2_2_todo = b3_2_1.tactics_subs("deepest level 7", 9).mark_todo();

    // Print the proof forest
    println!("Proof Forest:\n{}", p0.visualize_forest());

    // Build the theorem
    builder.build()
}

/// Example: Using bookmarks for complex proofs
pub fn prove_with_bookmarks() -> Theorem {
    let left = MathExpression::string_expr("a * b");
    let right = MathExpression::string_expr("b * a");

    let builder = TheoremBuilder::new(
        "Commutative Group Properties",
        MathRelation::equal(left, right),
        vec![],
    );

    // Start initial branch and create a bookmark
    let start = builder.initial_branch();

    // First key step - bookmark it
    let key_step = start
        .tactics_intro("commutativity property", 1)
        .bookmark("key_point");

    // Main branch - complete this one
    let main_path = key_step
        .tactics_subs("element exchange", 2)
        .tactics_theorem_app("group_axiom_commutativity", HashMap::new())
        .mark_complete();

    // Get the bookmark directly from the forest
    let forest_ref = &key_step.forest;
    let bookmark_id;
    {
        let forest = forest_ref.borrow();
        bookmark_id = *forest.bookmarks.get("key_point").unwrap();
    }

    // Alternative approach 1 - get from bookmark
    let alt1 = builder
        .branch_at(bookmark_id)
        .tactics_intro("alternative approach", 3)
        .tactics_subs("direct application", 4)
        .mark_wip();

    // Alternative approach 2 - different strategy
    // Create a clone for the second branch
    let alt2_start = key_step.clone();
    let alt2 = alt2_start
        .tactics_intro("inverse-based approach", 5)
        .tactics_subs("use inverses", 6)
        .mark_todo();

    // Print the branch summary for the main complete path
    println!("Main proof path:\n{}", main_path.summary());

    builder.build()
}

/// Example: Named proof steps for better clarity
pub fn prove_with_named_steps() -> Theorem {
    let left = MathExpression::string_expr("a * a⁻¹");
    let right = MathExpression::string_expr("e");

    let builder = TheoremBuilder::new(
        "Inverse Element Theorem",
        MathRelation::equal(left, right),
        vec![],
    );

    // Main branch with meaningful variable names
    let initial = builder.initial_branch();
    let intro_step = initial.tactics_intro("inverse property", 1);
    let substitution = intro_step.tactics_subs("definition of inverse", 2);
    let theorem_app = substitution.tactics_theorem_app("group_axiom_inverse", HashMap::new());
    let completed = theorem_app.mark_complete();

    // First branch exploring alternate approach
    let inverse_branch_1 = intro_step.branch();
    let inverse_approach_2 = inverse_branch_1.tactics_intro("use identity first", 3);
    let inverse_identity = inverse_approach_2.tactics_subs("e * a * a⁻¹", 4);
    let inverse_assoc =
        inverse_identity.tactics_theorem_app("group_axiom_associativity", HashMap::new());
    let inverse_complete = inverse_assoc.mark_complete();

    // Second branch exploring yet another approach
    let identity_branch_1 = intro_step.branch();
    let identity_approach_1 = identity_branch_1.tactics_intro("right identity approach", 5);
    let identity_step_2 = identity_approach_1.tactics_subs("a * a⁻¹ * e", 6);
    let identity_wip = identity_step_2.mark_wip();

    // Visualize and build
    println!("Proof visualization:\n{}", initial.visualize_forest());

    builder.build()
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
