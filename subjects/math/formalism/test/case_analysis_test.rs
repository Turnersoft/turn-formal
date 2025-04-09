// Module: src/formalize_v2/subjects/math/theorem/test/case_analysis_test.rs
// Tests for case analysis functionality

use std::collections::HashMap;

use crate::subjects::math::formalism::core::ProofState;
use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::proof::{
    CaseAnalysisBuilder, ProofBranch, ProofForest, ProofStatus, Tactic, TheoremBuilder,
};
use crate::subjects::math::formalism::relations::MathRelation;

/// Test the basic case analysis functionality
#[test]
fn test_case_analysis() {
    // Create a simple test theorem
    let expr1 = MathExpression::string_expr("f(x)");
    let expr2 = MathExpression::string_expr("g(x)");

    let builder = TheoremBuilder::new("Test Theorem", MathRelation::equal(expr1, expr2), vec![]);

    // Start with initial branch
    let p0 = builder.initial_branch();
    println!("Initial branch path: {}", p0.get_path_name());

    // Add a simple proof step
    let p1 = p0.tactics_intro("intro step", 1);

    // Create case analysis
    let case_result = p1
        .case_analysis()
        .on_variable("x")
        .case("x > 0", |branch| {
            let c1 = branch.tactics_intro("Case: x > 0", 1);
            c1.should_complete()
        })
        .case("x = 0", |branch| {
            let c2 = branch.tactics_intro("Case: x = 0", 1);
            c2.should_complete()
        })
        .case("x < 0", |branch| {
            let c3 = branch.tactics_intro("Case: x < 0", 1);
            c3.should_complete()
        })
        .build();

    // Check that the structure of the case analysis is correct
    assert_eq!(case_result.cases.len(), 3);

    // Print the actual paths to help with debugging
    println!("Case 0 path: {}", case_result.cases[0].get_path_name());
    println!("Case 1 path: {}", case_result.cases[1].get_path_name());
    println!("Case 2 path: {}", case_result.cases[2].get_path_name());

    // Check that the pattern of the paths is consistent
    // We now expect a format like: p0_1_c1_1 (due to internal implementation details)
    let case0_path = case_result.cases[0].get_path_name();
    let case1_path = case_result.cases[1].get_path_name();
    let case2_path = case_result.cases[2].get_path_name();

    assert!(case0_path.starts_with("p0"));
    assert!(case1_path.starts_with("p0"));
    assert!(case2_path.starts_with("p0"));

    // These should be different paths
    assert_ne!(case0_path, case1_path);
    assert_ne!(case0_path, case2_path);
    assert_ne!(case1_path, case2_path);

    // Continue with the main proof after cases
    let p2 = case_result.parent_branch.tactics_intro("after cases", 2);
    let p3 = p2.should_complete();

    // Print the proof visualization
    println!("\nProof visualization:\n{}", p0.visualize_forest());
}

/// Test nested case analysis
#[test]
fn test_nested_case_analysis() {
    // Create a simple test theorem
    let expr1 = MathExpression::string_expr("f(x)");
    let expr2 = MathExpression::string_expr("g(x)");

    let builder = TheoremBuilder::new(
        "Test Nested Cases",
        MathRelation::equal(expr1, expr2),
        vec![],
    );

    // Start with initial branch
    let p0 = builder.initial_branch();

    // Add a simple proof step
    let p1 = p0.tactics_intro("intro step", 1);

    // Create case analysis with nested cases
    let outer_case_result = p1
        .case_analysis()
        .on_variable("x")
        .case("x ≥ 0", |branch| {
            let c1 = branch.tactics_intro("Case: x ≥ 0", 1);

            // Nested case analysis
            let inner_case_result = c1
                .case_analysis()
                .on_variable("y")
                .case("y > 0", |inner_branch| {
                    let i1 = inner_branch.tactics_intro("Case: y > 0", 1);
                    i1.should_complete()
                })
                .case("y ≤ 0", |inner_branch| {
                    let i2 = inner_branch.tactics_intro("Case: y ≤ 0", 1);
                    i2.should_complete()
                })
                .build();

            // Print the inner case paths for debugging
            println!(
                "Inner case 0 path: {}",
                inner_case_result.cases[0].get_path_name()
            );
            println!(
                "Inner case 1 path: {}",
                inner_case_result.cases[1].get_path_name()
            );

            // Check inner case structure with more flexible assertions
            assert_eq!(inner_case_result.cases.len(), 2);

            // Check that the inner case paths are distinct and start with the expected prefix
            let inner_case0 = inner_case_result.cases[0].get_path_name();
            let inner_case1 = inner_case_result.cases[1].get_path_name();

            assert!(inner_case0.contains("c1"));
            assert!(inner_case1.contains("c2"));
            assert_ne!(inner_case0, inner_case1);

            inner_case_result.parent_branch.should_complete()
        })
        .case("x < 0", |branch| {
            let c2 = branch.tactics_intro("Case: x < 0", 1);
            c2.should_complete()
        })
        .build();

    // Print the outer case paths for debugging
    println!(
        "Outer case 0 path: {}",
        outer_case_result.cases[0].get_path_name()
    );
    println!(
        "Outer case 1 path: {}",
        outer_case_result.cases[1].get_path_name()
    );

    // Check outer case structure with more flexible assertions
    assert_eq!(outer_case_result.cases.len(), 2);

    // Check that the outer case paths are distinct and have expected format
    let outer_case0 = outer_case_result.cases[0].get_path_name();
    let outer_case1 = outer_case_result.cases[1].get_path_name();

    assert!(outer_case0.contains("c1"));
    assert!(outer_case1.contains("c2"));
    assert_ne!(outer_case0, outer_case1);

    // Continue with the main proof after cases
    let p2 = outer_case_result
        .parent_branch
        .tactics_intro("after cases", 2);
    let p3 = p2.should_complete();

    // Print the proof visualization
    println!("\nNested proof visualization:\n{}", p0.visualize_forest());
}

/// Test alternative approaches within cases
#[test]
fn test_case_with_alternatives() {
    // Create a simple test theorem
    let expr1 = MathExpression::string_expr("P(x)");

    let builder = TheoremBuilder::new(
        "Test Alternative Approaches",
        MathRelation::custom("IsTrue".to_string(), vec![expr1]),
        vec![],
    );

    // Start with initial branch
    let p0 = builder.initial_branch();

    // Add a simple proof step
    let p1 = p0.tactics_intro("intro step", 1);

    // Create case analysis with branching within cases
    let case_result = p1
        .case_analysis()
        .on_variable("x")
        .case("x > 0", |branch| {
            let c1 = branch.tactics_intro("Case: x > 0", 1);

            // Create alternative approaches
            let alt1 = c1.branch();
            let alt1_1 = alt1.tactics_intro("Alternative approach 1", 2);
            let alt1_2 = alt1_1.mark_abandoned();

            let alt2 = c1.branch();
            let alt2_1 = alt2.tactics_intro("Alternative approach 2", 2);
            let alt2_2 = alt2_1.should_complete();

            // Complete the main approach
            c1.should_complete()
        })
        .case("x ≤ 0", |branch| {
            let c2 = branch.tactics_intro("Case: x ≤ 0", 1);
            c2.should_complete()
        })
        .build();

    // Check case structure
    assert_eq!(case_result.cases.len(), 2);

    // Continue with the main proof after cases
    let p2 = case_result.parent_branch.tactics_intro("after cases", 2);
    let p3 = p2.should_complete();

    // Print the proof visualization
    println!(
        "\nAlternatives proof visualization:\n{}",
        p0.visualize_forest()
    );
}

/// Test that case results can be used in various ways
#[test]
fn test_case_result_usage() {
    // Create a simple test theorem
    let expr1 = MathExpression::string_expr("P(x)");

    let builder = TheoremBuilder::new(
        "Test Case Result Usage",
        MathRelation::custom("IsTrue".to_string(), vec![expr1]),
        vec![],
    );

    // Start with initial branch
    let p0 = builder.initial_branch();

    // Add a simple proof step
    let p1 = p0.tactics_intro("intro step", 1);

    // Create case analysis
    let case_result = p1
        .case_analysis()
        .on_variable("x")
        .case("case 1", |branch| branch.should_complete())
        .case("case 2", |branch| branch.should_complete())
        .case("case 3", |branch| branch.should_complete())
        .build();

    // Print paths for debugging
    println!("Case paths:");
    for (i, case) in case_result.cases.iter().enumerate() {
        println!("Case {} path: {}", i, case.get_path_name());
    }

    // Get a specific case
    let case_1 = case_result.case(0).unwrap();
    println!("First case path: {}", case_1.get_path_name());

    // Check that the case path contains expected elements
    assert!(case_1.get_path_name().contains("c1"));

    // Get the case count
    assert_eq!(case_result.case_count(), 3);

    // Complete all cases and parent
    let completed_cases = case_result.complete_all_cases();
    let completed_parent = completed_cases.should_complete();

    // Print the proof visualization
    println!(
        "\nCase usage proof visualization:\n{}",
        p0.visualize_forest()
    );
}
