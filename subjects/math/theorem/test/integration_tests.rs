// Module: src/formalize_v2/subjects/math/theorem/test/integration_tests.rs
// Integration tests for the theorem proving system, focusing on case analysis

use std::collections::HashMap;

use crate::subjects::math::theorem::core::ProofState;
use crate::subjects::math::theorem::declarative_proof::{
    self, DeclarativeProofBuilder, StepStatus, proof_builder,
};
use crate::subjects::math::theorem::expressions::MathExpression;
use crate::subjects::math::theorem::proof::{
    CaseAnalysisBuilder, ProofBranch, ProofForest, ProofStatus, RewriteDirection, Tactic,
    TheoremBuilder,
};
use crate::subjects::math::theorem::relations::MathRelation;

/// Test a basic case analysis using the builder pattern
#[test]
fn test_case_analysis_abs_value() {
    // Create a theorem about absolute value being non-negative
    let absolute_value = MathExpression::string_expr("|x|");
    let zero = MathExpression::string_expr("0");

    let builder = TheoremBuilder::new(
        "Absolute Value Non-Negativity",
        MathRelation::greater_than_or_equal(absolute_value, zero),
        vec![],
    );

    // Start with initial branch
    let p0 = builder.initial_branch();
    println!("Initial branch path: {}", p0.get_path_name());

    // Introduce x as a real number
    let p1 = p0.tactics_intro("x as a real number", 1);

    // Use the case analysis builder pattern
    let case_result = p1
        .case_analysis()
        .on_variable("x")
        .case("x ≥ 0", |branch| {
            // Inside case 1 scope
            let c1 = branch.tactics_intro("When x ≥ 0, |x| = x", 1);
            let c1_1 = c1.tactics_subs("|x| = x", 2);
            c1_1.tactics_theorem_app("x_geq_0_implies_x_geq_0", HashMap::new())
                .should_complete()
        })
        .case("x < 0", |branch| {
            // Inside case 2 scope
            let c2 = branch.tactics_intro("When x < 0, |x| = -x", 1);
            let c2_1 = c2.tactics_subs("|x| = -x", 2);
            let c2_2 = c2_1.tactics_intro("For x < 0, -x > 0", 3);
            c2_2.tactics_theorem_app("x_lt_0_implies_neg_x_gt_0", HashMap::new())
                .should_complete()
        })
        .build();

    // Continue with the main proof after cases
    let p2 = case_result
        .parent_branch
        .tactics_intro("Combining the cases", 2);
    let p3 = p2.should_complete();

    // Print the proof visualization
    println!("\nProof visualization:\n{}", p0.visualize_forest());

    // Build the theorem
    let theorem = builder.build();
    assert_eq!(theorem.name, "Absolute Value Non-Negativity");
}

/// Test a more complex case analysis with nested cases
#[test]
fn test_nested_case_analysis() {
    // Create a theorem statement
    let expr1 = MathExpression::string_expr("f(n)");
    let expr2 = MathExpression::string_expr("g(n)");

    let builder = TheoremBuilder::new(
        "Number Theory Property",
        MathRelation::equal(expr1, expr2),
        vec![],
    );

    // Start proof
    let p0 = builder.initial_branch();

    // Introduce n as integer
    let p1 = p0.tactics_intro("n as integer", 1);

    // Case analysis on modulo 3
    let case_result = p1
        .case_analysis()
        .on_expression("n mod 3")
        .case("n ≡ 0 (mod 3)", |branch| {
            let c1 = branch.tactics_intro("When n = 3k for some integer k", 1);

            // Nested case analysis on k
            let nested_cases = c1
                .case_analysis()
                .on_variable("k")
                .case("k is even", |subcase| {
                    let sc1 = subcase.tactics_intro("When k = 2m", 1);
                    let sc1_1 = sc1.tactics_subs("n = 3(2m) = 6m", 2);
                    sc1_1.should_complete()
                })
                .case("k is odd", |subcase| {
                    let sc2 = subcase.tactics_intro("When k = 2m+1", 1);
                    let sc2_1 = sc2.tactics_subs("n = 3(2m+1) = 6m+3", 2);
                    sc2_1.should_complete()
                })
                .build();

            nested_cases.parent_branch.should_complete()
        })
        .case("n ≡ 1 (mod 3)", |branch| {
            let c2 = branch.tactics_intro("When n = 3k + 1", 1);
            c2.should_complete()
        })
        .case("n ≡ 2 (mod 3)", |branch| {
            let c3 = branch.tactics_intro("When n = 3k + 2", 1);
            c3.should_complete()
        })
        .build();

    // Complete the proof
    let p2 = case_result.parent_branch.should_complete();

    // Print the proof
    println!("\nNested case analysis proof:\n{}", p0.visualize_forest());

    // Build the theorem
    let theorem = builder.build();
    assert_eq!(theorem.name, "Number Theory Property");
}

/// Test combining the declarative proof builder with case analysis
#[test]
fn test_declarative_case_analysis() {
    use proof_builder::{branch, case_analysis, intro, proof_tree, theorem_app};

    // Create a theorem statement
    let absolute_value = MathExpression::string_expr("|x|");
    let zero = MathExpression::string_expr("0");

    // Create case branches
    let cases = vec![
        // Case x ≥ 0
        branch(vec![
            intro("When x ≥ 0, |x| = x", 1).with_description("Case: x is non-negative"),
            intro("Apply |x| = x substitution", 2),
            theorem_app("x_geq_0_implies_x_geq_0").with_description("Complete the first case"),
        ])
        .with_description("Case x ≥ 0"),
        // Case x < 0
        branch(vec![
            intro("When x < 0, |x| = -x", 1).with_description("Case: x is negative"),
            intro("Apply |x| = -x substitution", 2),
            intro("For x < 0, -x > 0", 3),
            theorem_app("x_lt_0_implies_neg_x_gt_0").with_description("Complete the second case"),
        ])
        .with_description("Case x < 0"),
    ];

    // Create the main proof branch
    let main_branch = branch(vec![
        intro("x as a real number", 1).with_description("Introduce variable x"),
        case_analysis("x", vec!["x ≥ 0".to_string(), "x < 0".to_string()])
            .with_description("Split into cases based on sign of x")
            .with_branches(cases),
        intro("Combining all cases", 2).with_description("Complete the proof by combining cases"),
    ]);

    // Create the proof tree
    let proof_tree = proof_tree(main_branch);

    // Build the theorem using declarative proof
    let builder = DeclarativeProofBuilder::new(
        "Absolute Value Non-Negativity (Declarative)",
        MathRelation::greater_than_or_equal(absolute_value, zero),
        vec![],
    )
    .with_proof(proof_tree);

    let theorem = builder.build();

    // Verify the theorem was created
    assert_eq!(theorem.name, "Absolute Value Non-Negativity (Declarative)");
}

/// Test more advanced features combining different proof styles
#[test]
fn test_advanced_case_analysis() {
    // Create a theorem statement about a mathematical property
    let expr = MathExpression::string_expr("P(x)");

    let builder = TheoremBuilder::new(
        "Advanced Mathematical Property",
        MathRelation::custom("P".to_string(), vec![expr]),
        vec![],
    );

    // Start with initial branch
    let p0 = builder.initial_branch();

    // Introduce x
    let p1 = p0.tactics_intro("Let x be arbitrary", 1);

    // Define a bookmark
    let bookmark = p1.bookmark("main_branch");

    // Create first branch
    let b1 = bookmark.branch();
    let b1_1 = b1.tactics_intro("First approach", 2);

    // Create case analysis on the first branch
    let cases1 = b1_1
        .case_analysis()
        .on_variable("x")
        .case("x in domain A", |branch| {
            let c1 = branch.tactics_intro("Using properties of domain A", 1);
            c1.should_complete()
        })
        .case("x in domain B", |branch| {
            let c2 = branch.tactics_intro("Using properties of domain B", 1);

            // Try alternative approaches in this case
            let c2_alt = c2.branch();
            let c2_alt_1 = c2_alt.tactics_intro("Alternative method", 2);
            let c2_alt_2 = c2_alt_1
                .tactics_intro("This approach fails", 3)
                .mark_abandoned();

            // Back to main approach
            c2.should_complete()
        })
        .build();

    // Create second branch from bookmark
    let b2 = bookmark.branch();
    let b2_1 = b2.tactics_intro("Second approach", 2);
    let b2_2 = b2_1.tactics_subs("Using key substitution", 3);
    let b2_3 = b2_2.should_complete();

    // Visualize the proof forest
    println!("\nAdvanced proof:\n{}", p0.visualize_forest());

    // Build the theorem
    let theorem = builder.build();
    assert_eq!(theorem.name, "Advanced Mathematical Property");
}
