// Module: src/formalize_v2/subjects/math/theorem/test/advanced_example.rs
// Advanced examples of using the proof builder for more complex proofs

use std::collections::HashMap;

use crate::subjects::math::theorem::expressions::MathExpression;
use crate::subjects::math::theorem::proof::{ProofForest, Tactic, TheoremBuilder};
use crate::subjects::math::theorem::relations::MathRelation;

/// Example: Proving the First Isomorphism Theorem for Groups
///
/// This example shows a more complex proof with deep branches
/// and different strategies.
pub fn prove_first_isomorphism_theorem()
-> crate::subjects::math::theorem::core::Theorem {
    // Create expressions for the isomorphism theorem
    let g_ker_phi = MathExpression::string_expr("G/Ker(φ)");
    let im_phi = MathExpression::string_expr("Im(φ)");

    // Create a theorem: G/Ker(φ) ≅ Im(φ)
    let builder = TheoremBuilder::new(
        "First Isomorphism Theorem",
        // Use proper MathRelation API instead of deprecated Equality
        MathRelation::equal(g_ker_phi, im_phi),
        vec![
            // Replace Property with custom relations
            MathRelation::custom(
                "IsHomomorphism".to_string(),
                vec![MathExpression::string_expr("φ")],
            ),
            MathRelation::custom(
                "AreGroups".to_string(),
                vec![
                    MathExpression::string_expr("G"),
                    MathExpression::string_expr("H"),
                ],
            ),
        ],
    );

    // Start with initial branch
    let p0 = builder.initial_branch();

    // Apply the Kernel definition
    let p1 = p0.tactics_intro("ker(φ) = {g ∈ G | φ(g) = e_H}", 1);

    // Set up definitions
    let p2 = p1.tactics_intro("Let π: G → G/Ker(φ) be the canonical projection", 2);
    let p3 = p2.tactics_intro("Define ψ: G/Ker(φ) → Im(φ) by ψ([g]) = φ(g)", 3);

    // Check that ψ is well-defined
    let p4 = p3.tactics_intro("ψ is well-defined: if [g] = [g'], then g⁻¹g' ∈ Ker(φ)", 4);
    let p5 = p4.tactics_intro("This implies φ(g⁻¹g') = e_H, so φ(g) = φ(g')", 5);

    // Show ψ is a homomorphism
    let p6 = p5.tactics_intro("ψ is a homomorphism: ψ([g][g']) = ψ([gg'])", 6);
    let p7 = p6.tactics_intro("ψ([g][g']) = ψ([gg']) = φ(gg') = φ(g)φ(g')", 7);

    // Show ψ is bijective
    let p8 = p7.tactics_intro("ψ is injective: if ψ([g]) = ψ([g']), then φ(g) = φ(g')", 8);
    let p9 = p8.tactics_intro("This means g⁻¹g' ∈ Ker(φ), so [g] = [g']", 9);

    // Show surjectivity
    let p10 = p9.tactics_intro(
        "ψ is surjective: for any h ∈ Im(φ), h = φ(g) for some g ∈ G",
        10,
    );
    let p11 = p10.tactics_intro("Then h = φ(g) = ψ([g])", 11);

    // Complete the proof
    let p12 = p11.tactics_intro("Therefore, ψ is an isomorphism", 12);
    let p13 = p12.tactics_intro("We have G/Ker(φ) ≅ Im(φ)", 13);
    let p14 = p13.should_complete();

    // Build the theorem
    builder.build()
}

/// Example: A proving approach using nested closure scoping
///
/// This example shows how you could structure proofs if using an alternative
/// closure-based API approach.
pub fn prove_with_closures() -> crate::subjects::math::theorem::core::Theorem {
    // Create a theorem statement
    let statement = MathExpression::string_expr("P(G, p)");

    // Create a theorem: Sylow's First Theorem
    let builder = TheoremBuilder::new(
        "Sylow's First Theorem",
        // Use custom relation instead of Property
        MathRelation::custom("HasSylowSubgroup".to_string(), vec![statement]),
        vec![
            // Replace Property with custom relations
            MathRelation::custom(
                "IsFiniteGroup".to_string(),
                vec![MathExpression::string_expr("G")],
            ),
            MathRelation::custom(
                "IsPrimeDividingOrder".to_string(),
                vec![
                    MathExpression::string_expr("p"),
                    MathExpression::string_expr("|G|"),
                ],
            ),
        ],
    );

    // Start with initial branch
    let p0 = builder.initial_branch();

    // Use a closure to create a complex proof step
    let p1 = p0.apply_tactic(
        Tactic::Intro("|G| = p^n * m".to_string(), 1),
        "Express |G| as p^n * m where p does not divide m".to_string(),
    );

    // Create a helper function for branching
    let create_case =
        |parent: &crate::subjects::math::theorem::proof::ProofBranch,
         desc: &str,
         index: usize| {
            parent.apply_tactic(
                Tactic::Intro(format!("Case {}: {}", index, desc), index as usize),
                format!("Analyzing case: {}", desc),
            )
        };

    // Create multiple branches for the different cases
    let case1 = create_case(&p1, "Direct construction", 1);
    let case2 = create_case(&p1, "Induction on |G|", 2);
    let case3 = create_case(&p1, "Action on cosets", 3);

    // Develop the first case
    let c1_1 = case1.apply_tactic(
        Tactic::Custom {
            name: "ConstructSylowSubgroup".to_string(),
            args: vec!["p".to_string(), "n".to_string()],
        },
        "Explicitly construct a subgroup of order p^n".to_string(),
    );
    let c1_2 = c1_1.should_complete();

    // Develop the second case
    let c2_1 = case2.apply_tactic(
        Tactic::Intro("If G has a normal subgroup N".to_string(), 2),
        "Consider a normal subgroup N".to_string(),
    );
    let c2_2 = c2_1.apply_tactic(
        Tactic::Intro("Apply induction to G/N".to_string(), 3),
        "Use the induction hypothesis on the quotient".to_string(),
    );
    let c2_3 = c2_2.should_complete();

    // Develop the third case
    let c3_1 = case3.apply_tactic(
        Tactic::Intro("Let G act on cosets of a subgroup".to_string(), 2),
        "Consider the action of G on cosets".to_string(),
    );
    let c3_2 = c3_1.apply_tactic(
        Tactic::Intro("Apply Sylow's lemma".to_string(), 3),
        "Apply Sylow's lemma on the orbit-stabilizer relationship".to_string(),
    );
    let c3_3 = c3_2.apply_tactic(
        Tactic::Intro("This gives us a p-subgroup of order p^n".to_string(), 4),
        "Construct the desired Sylow p-subgroup".to_string(),
    );
    let c3_4 = c3_3.should_complete();

    // Complete the proof
    let p2 = p1.apply_tactic(
        Tactic::Intro("Therefore, G has a Sylow p-subgroup".to_string(), 2),
        "Combining all cases, we've shown the existence of a Sylow p-subgroup".to_string(),
    );
    let p3 = p2.should_complete();

    // Build the theorem
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_isomorphism_theorem() {
        let theorem = prove_first_isomorphism_theorem();
        assert_eq!(theorem.name, "First Isomorphism Theorem");
    }

    #[test]
    fn test_with_closures() {
        let theorem = prove_with_closures();
        assert_eq!(theorem.name, "Sylow's First Theorem");
    }
}
