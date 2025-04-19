// Module: src/formalize_v2/subjects/math/theorem/test/case_builder_example.rs
// Examples of using the case analysis builder pattern with ad hoc scoping

use std::collections::HashMap;

use super::super::super::formalism::core::Theorem;
use super::super::super::formalism::expressions::{Identifier, MathExpression, TheoryExpression};
use super::super::super::formalism::proof::{ProofBranch, TheoremBuilder};
use super::super::super::formalism::relations::MathRelation;
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

/// Example of proving absolute value properties using case analysis builder
pub fn prove_absolute_value_with_builder() -> Theorem {
    // Create a theorem about absolute value being non-negative
    let absolute_value = create_expr("|x|");
    let zero = create_var("0");

    let builder = TheoremBuilder::new(
        "Absolute Value Non-Negativity (Builder)",
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
            // Inside case 1 scope - has its own variable naming
            let c1 = branch.tactics_intro("When x ≥ 0, |x| = x", 1);
            let c1_1 = c1.tactics_subs("|x| = x", 2);

            // Can create branches within a case
            let c1_2 = c1_1.branch();
            let c1_2_1 = c1_2.tactics_intro("Alternative approach in case 1", 3);
            let c1_2_2 = c1_2_1.tactics_subs("x ≥ 0", 4);

            // Return the main path completion for this case
            c1_1.tactics_theorem_app("x_geq_0_implies_x_geq_0", HashMap::new())
                .should_complete()
        })
        .case("x < 0", |branch| {
            // Inside case 2 scope - separate variable naming
            let c2 = branch.tactics_intro("When x < 0, |x| = -x", 1);
            let c2_1 = c2.tactics_subs("|x| = -x", 2);
            let c2_2 = c2_1.tactics_intro("For x < 0, -x > 0", 3);

            // Complete this case
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
    println!("\nFull proof visualization:\n{}", p0.visualize_forest());

    // Build and return the theorem
    builder.build()
}

/// Example of proving a more complex theorem with nested case analysis
pub fn prove_complex_theorem_with_builder() -> Theorem {
    // Create a theorem statement
    let expr1 = create_expr("f(x)");
    let expr2 = create_expr("g(x)");

    let builder = TheoremBuilder::new(
        "Piecewise Function Properties (Builder)",
        MathRelation::equal(expr1, expr2),
        vec![],
    );

    // Start with initial branch
    let p0 = builder.initial_branch();

    // Setup the theorem
    let p1 = p0.tactics_intro("Let f(x) and g(x) be piecewise functions", 1);

    // First level case analysis
    let domain_cases = p1
        .case_analysis()
        .on_expression("x")
        .case("x < 0", |branch| {
            // Case where x < 0
            let c1 = branch.tactics_intro("For x < 0, f(x) = -x² and g(x) = -x²", 1);

            // Nested case analysis within this case
            let subcases = c1
                .case_analysis()
                .on_expression("x²")
                .case("x² = 0", |subcase| {
                    // Subcase where x² = 0
                    let sc1 = subcase.tactics_intro("When x² = 0, then x = 0", 1);
                    let sc1_1 = sc1.tactics_intro("But this contradicts x < 0", 2);
                    sc1_1.should_complete()
                })
                .case("x² > 0", |subcase| {
                    // Subcase where x² > 0
                    let sc2 = subcase.tactics_intro("When x² > 0, -x² < 0", 1);
                    let sc2_1 = sc2.tactics_subs("f(x) = -x²", 2);
                    let sc2_2 = sc2_1.tactics_subs("g(x) = -x²", 3);

                    // Branch within subcase
                    let sc2_alt = sc2_2.branch();
                    let sc2_alt_1 = sc2_alt.tactics_intro("Alternative approach", 4);

                    // Complete the main subcase path
                    sc2_2
                        .tactics_theorem_app("equality_reflexive", HashMap::new())
                        .should_complete()
                })
                .build();

            // Continue after nested case analysis
            let c1_after = subcases
                .parent_branch
                .tactics_intro("Therefore, when x < 0, f(x) = g(x)", 2);
            c1_after.should_complete()
        })
        .case("x = 0", |branch| {
            // Case where x = 0
            let c2 = branch.tactics_intro("For x = 0, f(0) = 0 and g(0) = 0", 1);
            let c2_1 = c2.tactics_theorem_app("equality_reflexive", HashMap::new());
            c2_1.should_complete()
        })
        .case("x > 0", |branch| {
            // Case where x > 0
            let c3 = branch.tactics_intro("For x > 0, f(x) = x²+1 and g(x) = x²+1", 1);
            let c3_1 = c3.tactics_theorem_app("equality_reflexive", HashMap::new());
            c3_1.should_complete()
        })
        .build();

    // Complete the overall theorem
    let p2 = domain_cases
        .parent_branch
        .tactics_intro("All cases show f(x) = g(x)", 2);
    let p3 = p2.should_complete();

    // Print the proof visualization
    println!("\nComplex proof visualization:\n{}", p0.visualize_forest());

    // Build and return the theorem
    builder.build()
}

/// Example of a proof where cases use different theorem applications
pub fn prove_number_theory_theorem_with_builder() -> Theorem {
    // Create a theorem about number properties
    let expr = create_expr("P(n)");

    let builder = TheoremBuilder::new(
        "Number Theory Property (Builder)",
        MathRelation::custom("P".to_string(), vec![expr]),
        vec![],
    );

    // Start with initial branch
    let p0 = builder.initial_branch();

    // Setup the theorem
    let p1 = p0.tactics_intro("Let n be a positive integer", 1);

    // Case analysis on the remainder when dividing by 3
    let mod_cases = p1
        .case_analysis()
        .on_expression("n mod 3")
        .case("n ≡ 0 (mod 3)", |branch| {
            // Case where n is divisible by 3
            let c1 = branch.tactics_intro("When n = 3k for some integer k", 1);
            let c1_1 = c1.tactics_subs("n = 3k", 2);
            let c1_2 = c1_1.tactics_theorem_app("divisible_by_3_theorem", HashMap::new());
            c1_2.should_complete()
        })
        .case("n ≡ 1 (mod 3)", |branch| {
            // Case where remainder is 1
            let c2 = branch.tactics_intro("When n = 3k + 1 for some integer k", 1);
            let c2_1 = c2.tactics_subs("n = 3k + 1", 2);
            let c2_2 = c2_1.tactics_theorem_app("remainder_1_theorem", HashMap::new());
            c2_2.should_complete()
        })
        .case("n ≡ 2 (mod 3)", |branch| {
            // Case where remainder is 2
            let c3 = branch.tactics_intro("When n = 3k + 2 for some integer k", 1);
            let c3_1 = c3.tactics_subs("n = 3k + 2", 2);

            // This case requires further analysis
            let subcase_result = c3_1
                .case_analysis()
                .on_expression("k mod 2")
                .case("k is even", |subcase| {
                    let sc1 = subcase.tactics_intro("When k = 2m for some integer m", 1);
                    let sc1_1 = sc1.tactics_subs("n = 3(2m) + 2 = 6m + 2", 2);
                    let sc1_2 = sc1_1.tactics_theorem_app("even_k_theorem", HashMap::new());
                    sc1_2.should_complete()
                })
                .case("k is odd", |subcase| {
                    let sc2 = subcase.tactics_intro("When k = 2m + 1 for some integer m", 1);
                    let sc2_1 = sc2.tactics_subs("n = 3(2m+1) + 2 = 6m + 5", 2);
                    let sc2_2 = sc2_1.tactics_theorem_app("odd_k_theorem", HashMap::new());
                    sc2_2.should_complete()
                })
                .build();

            subcase_result.parent_branch.should_complete()
        })
        .build();

    // Conclude the proof
    let p2 = mod_cases
        .parent_branch
        .tactics_intro("All cases establish P(n)", 2);
    let p3 = p2.should_complete();

    // Print the proof visualization
    println!(
        "\nNumber theory proof visualization:\n{}",
        p0.visualize_forest()
    );

    // Build and return the theorem
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_value_builder() {
        let theorem = prove_absolute_value_with_builder();
        assert_eq!(theorem.name, "Absolute Value Non-Negativity (Builder)");
    }

    #[test]
    fn test_complex_theorem_builder() {
        let theorem = prove_complex_theorem_with_builder();
        assert_eq!(theorem.name, "Piecewise Function Properties (Builder)");
    }

    #[test]
    fn test_number_theory_builder() {
        let theorem = prove_number_theory_theorem_with_builder();
        assert_eq!(theorem.name, "Number Theory Property (Builder)");
    }
}
