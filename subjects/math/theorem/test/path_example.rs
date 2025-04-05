// Module: src/formalize_v2/subjects/math/theorem/test/path_example.rs
// Example of using path-based naming for proof steps

use std::collections::HashMap;

use crate::subjects::math::theorem::core::{ProofState, Theorem};
use crate::subjects::math::theorem::expressions::MathExpression;
use crate::subjects::math::theorem::proof::{
    CaseResult, ProofBranch, ProofStatus, RewriteDirection, Tactic, TheoremBuilder,
};
use crate::subjects::math::theorem::relations::MathRelation;

/// Demonstrate path-based naming pattern (px_x_x_x) throughout a complex proof
pub fn prove_with_path_naming() -> Theorem {
    // Create a theorem about associativity
    let left = MathExpression::string_expr("a * (b * c)");
    let right = MathExpression::string_expr("(a * b) * c");

    let builder = TheoremBuilder::new(
        "Associativity with Path Naming",
        MathRelation::equal(left, right),
        vec![],
    );

    // Start with initial branch - automatically gets path "p0"
    let p0 = builder.initial_branch();
    println!("Initial branch path: {}", p0.get_path_name());

    // Main proof path - path will become p0_1
    let p0_1 = p0.tactics_intro("a", 1);
    println!("First step path: {}", p0_1.get_path_name());

    // Continue main path - path will become p0_1_1
    let p0_1_1 = p0_1.tactics_subs("x + y", 2);
    println!("Second step path: {}", p0_1_1.get_path_name());

    // Complete main path - path will become p0_1_1_1
    let p0_1_1_1 = p0_1_1
        .tactics_theorem_app("group_axiom_associativity", HashMap::new())
        .should_complete();
    println!("Completed main path: {}", p0_1_1_1.get_path_name());

    // Create first branch from p0_1 - path will be p0_1_2
    let p0_1_2 = p0_1.branch();
    println!("First branch from p0_1: {}", p0_1_2.get_path_name());

    // Continue on branch - path will be p0_1_2_1
    let p0_1_2_1 = p0_1_2.tactics_intro("alternative approach", 3);

    // Branch again explicitly with id - path will be p0_1_2_1_5 (skipping indices)
    let p0_1_2_1_5 = p0_1_2_1.branch_with_id(5);

    // Apply a substitution - path will be p0_1_2_1_5_1
    let p0_1_2_1_5_1 = p0_1_2_1_5.tactics_subs("different substitution", 2);

    // Mark as work in progress
    let p0_1_2_1_5_1_wip = p0_1_2_1_5_1.mark_wip();

    // Create another branch from p0 - path will be p0_2
    let p0_2 = p0.branch();

    // Apply a rewrite tactic - path will be p0_2_1
    let p0_2_1 = p0_2.tactics_rewrite(
        "a * (b * c)",
        "associativity",
        RewriteDirection::LeftToRight,
    );

    // Apply induction
    let p0_2_1_1 = p0_2_1.tactics_intro("induction setup", 1);

    // Now instead of creating separate branches, use case analysis to handle both cases together
    let induction_cases =
        p0_2_1_1.cases(vec!["Base case".to_string(), "Induction step".to_string()]);

    // Get references to each case
    let base_case = induction_cases.case(0).unwrap();
    let induction_step = induction_cases.case(1).unwrap();

    // Handle base case
    let base_case_done = base_case
        .tactics_theorem_app("base_case", HashMap::new())
        .should_complete();

    // Handle induction step
    let induction_step_done = induction_step
        .tactics_theorem_app("induction_step", HashMap::new())
        .should_complete();

    // Mark the entire case analysis as complete
    let p0_2_1_1_cases_done = induction_cases.should_complete();

    // Print the path structure
    println!("Paths in proof:\n");
    println!(
        "Main path: {} -> {} -> {} -> {}",
        p0.get_path_name(),
        p0_1.get_path_name(),
        p0_1_1.get_path_name(),
        p0_1_1_1.get_path_name()
    );

    println!(
        "Alternative path: {} -> {} -> {} -> {} -> {}",
        p0.get_path_name(),
        p0_1.get_path_name(),
        p0_1_2.get_path_name(),
        p0_1_2_1.get_path_name(),
        p0_1_2_1_5_1_wip.get_path_name()
    );

    println!(
        "Induction path with cases: {} -> {} -> {} -> cases: ({}, {})",
        p0.get_path_name(),
        p0_2.get_path_name(),
        p0_2_1.get_path_name(),
        base_case_done.get_path_name(),
        induction_step_done.get_path_name()
    );

    // Display the proof forest visualization
    println!("\nFull proof visualization:\n{}", p0.visualize_forest());

    // Build and return the theorem
    builder.build()
}

/// Demonstrate more complex case analysis with a full proof
pub fn prove_by_cases() -> Theorem {
    // A simple theorem requiring multiple cases (for example, proving |x| ≥ 0 for all real x)
    let absolute_value = MathExpression::string_expr("|x|");
    let zero = MathExpression::string_expr("0");

    let builder = TheoremBuilder::new(
        "Absolute Value Non-Negativity",
        MathRelation::greater_than_or_equal(absolute_value, zero),
        vec![],
    );

    // Start proof
    let p0 = builder.initial_branch();

    // Set up the proof by considering cases
    let p1 = p0.tactics_intro("x as a real number", 1);

    // Create cases for x ≥ 0 and x < 0
    let cases = p1.cases(vec!["Case x ≥ 0".to_string(), "Case x < 0".to_string()]);

    // Handle first case: when x ≥ 0
    let case1 = cases.case(0).unwrap();
    let case1_step1 = case1.tactics_intro("When x ≥ 0, |x| = x", 1);
    let case1_step2 = case1_step1.tactics_subs("|x| = x", 2);
    let case1_done = case1_step2
        .tactics_theorem_app("x_geq_0_implies_x_geq_0", HashMap::new())
        .should_complete();

    // Handle second case: when x < 0
    let case2 = cases.case(1).unwrap();
    let case2_step1 = case2.tactics_intro("When x < 0, |x| = -x", 1);
    let case2_step2 = case2_step1.tactics_subs("|x| = -x", 2);
    let case2_step3 = case2_step2.tactics_intro("For x < 0, -x > 0", 3);
    let case2_done = case2_step3
        .tactics_theorem_app("x_lt_0_implies_neg_x_gt_0", HashMap::new())
        .should_complete();

    // Complete the entire case analysis
    let cases_done = cases.should_complete();

    // Complete the overall proof
    let proof_done = cases_done.should_complete();

    // Print the proof structure
    println!("\nAbsolute value proof by cases:\n");
    println!("Case 1 (x ≥ 0): {}", case1_done.get_path_name());
    println!("Case 2 (x < 0): {}", case2_done.get_path_name());
    println!("Overall proof: {}", proof_done.get_path_name());

    // Visualize the full proof
    println!(
        "\nVisualization of the absolute value proof:\n{}",
        p0.visualize_forest()
    );

    // Build the theorem
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_naming_pattern() {
        let theorem = prove_with_path_naming();

        // Verify theorem was created
        assert_eq!(theorem.name, "Associativity with Path Naming");

        // In a real test, we would also verify the path structure is correct
        assert!(theorem.initial_proof_state.path.is_some());
        if let Some(path) = &theorem.initial_proof_state.path {
            assert_eq!(path, "p0");
        }
    }

    #[test]
    fn test_case_analysis_proof() {
        let theorem = prove_by_cases();

        // Verify theorem was created
        assert_eq!(theorem.name, "Absolute Value Non-Negativity");

        // Check path structure
        assert!(theorem.initial_proof_state.path.is_some());
    }
}
