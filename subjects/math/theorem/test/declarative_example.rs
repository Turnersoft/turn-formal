// Module: src/formalize_v2/subjects/math/theorem/test/declarative_example.rs
// Examples of using the declarative proof structure

use crate::subjects::math::theorem::core::Theorem;
use crate::subjects::math::theorem::declarative_proof::tactics::*;
use crate::subjects::math::theorem::declarative_proof::{
    Branch, DeclarativeProofBuilder, ProofTree, Step,
};
use crate::subjects::math::theorem::expressions::MathExpression;
use crate::subjects::math::theorem::proof::RewriteDirection;
use crate::subjects::math::theorem::relations::MathRelation;

/// Prove the group associativity theorem using a declarative structure
pub fn prove_group_associativity_declarative() -> Theorem {
    // Create expressions for the theorem
    let left = MathExpression::string_expr("a * (b * c)");
    let right = MathExpression::string_expr("(a * b) * c");

    // Build the proof tree - this is the key difference from the procedural approach!
    // Here we define the entire proof structure upfront, and it gets executed later.
    let main_branch = Branch::new(vec![
        // Step 1: Introduce variable 'a'
        Step::new(intro("a", 1))
            .with_description("Introduce variable 'a'")
            .with_branch(
                // Branch from Step 1 - a deep nested example
                Branch::new(vec![
                    Step::new(intro("x", 3))
                        .with_description("First step in branch")
                        .with_branch(Branch::new(vec![
                            Step::new(subs("y", 4))
                                .with_description("Deeper nesting")
                                .with_branch(Branch::new(vec![
                                    Step::new(theorem_app("some_lemma"))
                                        .with_description("Deepest level"),
                                ])),
                        ])),
                ])
                .with_description("Deep branch example"),
            ),
        // Step 2: Substitute with 'b+c'
        Step::new(subs("b+c", 2))
            .with_description("Apply substitution")
            .with_branch(
                // Alternative branch from Step 2
                Branch::new(vec![
                    Step::new(intro("alternative", 7)).with_description("Alternative approach"),
                ])
                .with_description("Alternative branch"),
            ),
        // Step 3: Apply the associativity theorem
        Step::new(theorem_app("group_axiom_associativity"))
            .with_description("Complete the proof using the axiom"),
    ]);

    // Create the proof tree
    let proof_tree = ProofTree::new(main_branch);

    // Build the theorem using the declarative builder
    let builder = DeclarativeProofBuilder::new(
        "Group Associativity (Declarative)",
        MathRelation::equal(left, right),
        vec![],
    )
    .with_proof(proof_tree);

    // Execute the proof and return the theorem
    builder.build()
}

/// Prove the absolute value theorem using case analysis
pub fn prove_absolute_value_declarative() -> Theorem {
    // Create expressions for the theorem
    let absolute_value = MathExpression::string_expr("|x|");
    let zero = MathExpression::string_expr("0");

    // Define the case branches
    let case_branches = vec![
        // Case 1: when x ≥ 0
        Branch::new(vec![
            Step::new(intro("When x ≥ 0, |x| = x", 1)),
            Step::new(subs("|x| = x", 2)),
            Step::new(theorem_app("x_geq_0_implies_x_geq_0")),
        ])
        .with_description("Case x ≥ 0"),
        // Case 2: when x < 0
        Branch::new(vec![
            Step::new(intro("When x < 0, |x| = -x", 1)),
            Step::new(subs("|x| = -x", 2)),
            Step::new(intro("For x < 0, -x > 0", 3)),
            Step::new(theorem_app("x_lt_0_implies_neg_x_gt_0")),
        ])
        .with_description("Case x < 0"),
    ];

    // Create the main proof
    let main_branch = Branch::new(vec![
        // Step 1: Set up the proof
        Step::new(intro("x as a real number", 1)).with_description("Set up the proof"),
        // Step 2: Case analysis on the sign of x
        Step::new(case_analysis(
            "x",
            vec!["x ≥ 0".to_string(), "x < 0".to_string()],
        ))
        .with_branches(case_branches)
        .with_description("Analyze cases based on sign of x"),
    ]);

    // Create the proof tree
    let proof_tree = ProofTree::new(main_branch);

    // Build the theorem
    let builder = DeclarativeProofBuilder::new(
        "Absolute Value Non-Negativity (Declarative)",
        MathRelation::greater_than_or_equal(absolute_value, zero),
        vec![],
    )
    .with_proof(proof_tree);

    builder.build()
}

/// Example of a proof with multiple nested branches and cases
pub fn prove_complex_theorem_declarative() -> Theorem {
    // Create expressions
    let expr1 = MathExpression::string_expr("P(x) → Q(x)");
    let expr2 = MathExpression::string_expr("∀x. (P(x) → Q(x))");

    // Define the first case analysis
    let first_cases = vec![
        // Case 1: P(x) is true
        Branch::new(vec![
            Step::new(intro("Assume P(x) is true", 1)),
            Step::new(intro("Then we need to show Q(x)", 2)),
            Step::new(theorem_app("theorem_a")).with_branch(Branch::new(vec![
                Step::new(intro("Alternative approach for Q(x)", 3)),
                Step::new(theorem_app("theorem_b")),
            ])),
        ])
        .with_description("Case P(x) is true"),
        // Case 2: P(x) is false
        Branch::new(vec![
            Step::new(intro("Assume P(x) is false", 1)),
            Step::new(intro("Then P(x) → Q(x) is vacuously true", 2)),
            Step::new(theorem_app("vacuous_truth")),
        ])
        .with_description("Case P(x) is false"),
    ];

    // Define the second branch with another case analysis
    let second_case_analysis = Branch::new(vec![
        Step::new(intro("Different approach using induction", 1)),
        Step::new(case_analysis(
            "method",
            vec!["Direct".to_string(), "Contrapositive".to_string()],
        ))
        .with_branches(vec![
            // Direct proof
            Branch::new(vec![
                Step::new(intro("Direct proof", 1)),
                Step::new(theorem_app("direct_method")),
            ])
            .with_description("Direct proof method"),
            // Contrapositive
            Branch::new(vec![
                Step::new(intro("Contrapositive: ¬Q(x) → ¬P(x)", 1)),
                Step::new(rewrite(
                    "¬Q(x) → ¬P(x)",
                    "contrapositive",
                    RewriteDirection::LeftToRight,
                )),
                Step::new(theorem_app("contrapositive_equiv")),
            ])
            .with_description("Contrapositive method"),
        ]),
    ]);

    // Create main branch with both approaches
    let main_branch = Branch::new(vec![
        // Step 1: Set up the theorem
        Step::new(intro("We want to prove P(x) → Q(x) for all x", 1))
            .with_description("Setup")
            .with_branch(second_case_analysis.clone()),
        // Step 2: Case analysis on P(x)
        Step::new(case_analysis(
            "P(x)",
            vec!["P(x) true".to_string(), "P(x) false".to_string()],
        ))
        .with_branches(first_cases)
        .with_description("Case analysis on P(x)"),
        // Step 3: Quantify the result
        Step::new(intro(
            "Having shown P(x) → Q(x) for arbitrary x, we have ∀x. (P(x) → Q(x))",
            2,
        ))
        .with_description("Quantify the result"),
    ]);

    // Create the proof tree
    let proof_tree = ProofTree::new(main_branch);

    // Build the theorem
    let builder = DeclarativeProofBuilder::new(
        "Complex Implication Theorem (Declarative)",
        MathRelation::equal(expr1, expr2),
        vec![],
    )
    .with_proof(proof_tree);

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_associativity_declarative() {
        let theorem = prove_group_associativity_declarative();
        assert_eq!(theorem.name, "Group Associativity (Declarative)");
    }

    #[test]
    fn test_absolute_value_declarative() {
        let theorem = prove_absolute_value_declarative();
        assert_eq!(theorem.name, "Absolute Value Non-Negativity (Declarative)");
    }

    #[test]
    fn test_complex_theorem_declarative() {
        let theorem = prove_complex_theorem_declarative();
        assert_eq!(theorem.name, "Complex Implication Theorem (Declarative)");
    }
}
