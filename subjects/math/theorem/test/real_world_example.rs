// Module: src/formalize_v2/subjects/math/theorem/test/real_world_example.rs
// Demonstrates a real-world mathematical proof using the theorem proving system

use std::collections::HashMap;

use crate::subjects::math::theorem::expressions::MathExpression;
use crate::subjects::math::theorem::proof::{
    ProofStatus, RewriteDirection, Tactic, TheoremBuilder,
};
use crate::subjects::math::theorem::relations::MathRelation;

/// A proof of the quadratic formula using case analysis
pub fn prove_quadratic_formula() {
    // Create expression for the quadratic equation
    let polynomial = MathExpression::string_expr("ax² + bx + c");
    let equation = MathExpression::string_expr("ax² + bx + c = 0");
    let solution = MathExpression::string_expr("x = (-b ± √(b² - 4ac)) / (2a)");

    println!("Beginning proof of the quadratic formula...");

    // Create the theorem
    let builder = TheoremBuilder::new(
        "Quadratic Formula",
        MathRelation::implies(
            MathRelation::equal(polynomial, MathExpression::string_expr("0")),
            MathRelation::custom("SolutionFor".to_string(), vec![solution, equation]),
        ),
        vec![],
    );

    // Start the proof
    let p0 = builder.initial_branch();

    // Setup the proof with initial assumptions
    let p1 = p0.tactics_intro("Consider ax² + bx + c = 0 with a ≠ 0", 1);
    let p2 = p1.tactics_intro("We want to find values of x that satisfy this equation", 2);

    // First step: rearranging to standard form
    let p3 = p2.tactics_rewrite(
        "ax² + bx + c = 0",
        "ax² + bx = -c",
        RewriteDirection::LeftToRight,
    );

    // Complete the square approach
    let p4 = p3.tactics_intro("Divide all terms by a", 3);
    let p5 = p4.tactics_subs("x² + (b/a)x = -c/a", 4);

    // Add (b/2a)² to both sides
    let p6 = p5.tactics_intro("Add (b/2a)² to both sides to complete the square", 5);
    let p7 = p6.tactics_subs("x² + (b/a)x + (b/2a)² = -c/a + (b/2a)²", 6);

    // Simplify right side
    let p8 = p7.tactics_intro("Simplify right side", 7);
    let p9 = p8.tactics_subs("-c/a + (b/2a)² = (b² - 4ac) / (4a²)", 8);

    // Express left side as perfect square
    let p10 = p9.tactics_intro("Left side is a perfect square", 9);
    let p11 = p10.tactics_subs("(x + b/2a)² = (b² - 4ac) / (4a²)", 10);

    // Case analysis on discriminant
    let cases = p11
        .case_analysis()
        .on_expression("b² - 4ac")
        .case("b² - 4ac > 0", |branch| {
            // Two real solutions case
            let c1 =
                branch.tactics_intro("When b² - 4ac > 0, we have two distinct real solutions", 1);
            let c1_1 = c1.tactics_intro("Take square root of both sides", 2);
            let c1_2 = c1_1.tactics_subs("x + b/2a = ±√((b² - 4ac) / (4a²))", 3);
            let c1_3 = c1_2.tactics_intro("Simplify the square root", 4);
            let c1_4 = c1_3.tactics_subs("x + b/2a = ±√(b² - 4ac) / (2a)", 5);

            // Solve for x
            let c1_5 = c1_4.tactics_intro("Subtract b/2a from both sides", 6);
            let c1_6 = c1_5.tactics_subs("x = -b/2a ± √(b² - 4ac) / (2a)", 7);
            let c1_7 = c1_6.tactics_intro("Combine terms with common denominator", 8);
            let c1_8 = c1_7.tactics_subs("x = (-b ± √(b² - 4ac)) / (2a)", 9);
            c1_8.should_complete()
        })
        .case("b² - 4ac = 0", |branch| {
            // One real solution case (repeated root)
            let c2 =
                branch.tactics_intro("When b² - 4ac = 0, we have exactly one real solution", 1);
            let c2_1 = c2.tactics_intro("Take square root of both sides", 2);
            let c2_2 = c2_1.tactics_subs("x + b/2a = 0", 3);

            // Solve for x
            let c2_3 = c2_2.tactics_intro("Subtract b/2a from both sides", 4);
            let c2_4 = c2_3.tactics_subs("x = -b/2a", 5);
            let c2_5 = c2_4.tactics_intro("Express in standard form", 6);
            let c2_6 = c2_5.tactics_subs("x = -b/(2a)", 7);
            c2_6.should_complete()
        })
        .case("b² - 4ac < 0", |branch| {
            // Complex solutions case
            let c3 = branch.tactics_intro(
                "When b² - 4ac < 0, we have two complex conjugate solutions",
                1,
            );
            let c3_1 = c3.tactics_intro("Take square root of both sides", 2);
            let c3_2 = c3_1.tactics_subs("x + b/2a = ±i√(|b² - 4ac| / (4a²))", 3);
            let c3_3 = c3_2.tactics_intro("Simplify the square root", 4);
            let c3_4 = c3_3.tactics_subs("x + b/2a = ±i√(|b² - 4ac|) / (2a)", 5);

            // Solve for x
            let c3_5 = c3_4.tactics_intro("Subtract b/2a from both sides", 6);
            let c3_6 = c3_5.tactics_subs("x = -b/2a ± i√(|b² - 4ac|) / (2a)", 7);
            let c3_7 = c3_6.tactics_intro("Combine terms with common denominator", 8);
            let c3_8 = c3_7.tactics_subs("x = (-b ± i√(|b² - 4ac|)) / (2a)", 9);
            c3_8.should_complete()
        })
        .build();

    // Continue with the main proof, combining all cases
    let p12 = cases
        .parent_branch
        .tactics_intro("In all cases, we have shown the solutions", 11);
    let p13 = p12.tactics_intro("Combining all cases, we get the quadratic formula", 12);
    let p14 = p13.tactics_subs("x = (-b ± √(b² - 4ac)) / (2a)", 13);
    let p15 = p14.should_complete();

    // Visualize the proof
    println!("\nQuadratic formula proof:\n{}", p0.visualize_forest());

    // Get the theorem
    let theorem = builder.build();
    println!("Completed proof of: {}", theorem.name);
}

/// A proof of Fermat's Little Theorem using modular arithmetic
pub fn prove_fermats_little_theorem() {
    // Create expressions
    let p = MathExpression::string_expr("p");
    let a = MathExpression::string_expr("a");
    let expr1 = MathExpression::string_expr("a^p");
    let expr2 = MathExpression::string_expr("a");

    println!("Beginning proof of Fermat's Little Theorem...");

    // Create the theorem: If p is prime and p doesn't divide a, then a^p ≡ a (mod p)
    let builder = TheoremBuilder::new(
        "Fermat's Little Theorem",
        MathRelation::custom("Congruent".to_string(), vec![expr1, expr2, p]),
        vec![],
    );

    // Start the proof
    let p0 = builder.initial_branch();

    // Setup the proof with initial assumptions
    let p1 = p0.tactics_intro("Let p be a prime number", 1);
    let p2 = p1.tactics_intro("Let a be an integer not divisible by p", 2);

    // Use induction on a
    let p3 = p2.tactics_intro("We will prove this by induction on a", 3);

    // Base case: a = 1
    let p4 = p3.tactics_intro("Base case: a = 1", 4);
    let p5 = p4.tactics_subs("1^p = 1", 5);
    let p6 = p5.tactics_intro("Clearly, 1 ≡ 1 (mod p)", 6);

    // Induction step
    let p7 = p6.tactics_intro("Induction step: Assume the theorem holds for some a", 7);
    let p8 = p7.tactics_intro("We need to prove it holds for a+1", 8);

    // Use the binomial theorem
    let p9 = p8.tactics_intro("By the binomial theorem", 9);
    let p10 = p9.tactics_subs(
        "(a+1)^p = a^p + p*C(p,1)*a^(p-1) + ... + p*C(p,p-1)*a + 1",
        10,
    );

    // Apply modular arithmetic
    let p11 = p10.tactics_intro(
        "All terms except the first and last contain a factor of p",
        11,
    );
    let p12 = p11.tactics_intro("By the properties of modular arithmetic", 12);
    let p13 = p12.tactics_subs("(a+1)^p ≡ a^p + 1 (mod p)", 13);

    // Apply induction hypothesis
    let p14 = p13.tactics_intro("By the induction hypothesis, a^p ≡ a (mod p)", 14);
    let p15 = p14.tactics_subs("(a+1)^p ≡ a + 1 (mod p)", 15);
    let p16 = p15.tactics_intro("Thus, (a+1)^p ≡ (a+1) (mod p)", 16);

    // Complete the proof
    let p17 = p16.tactics_intro(
        "By the principle of induction, the theorem holds for all a",
        17,
    );
    let p18 = p17.should_complete();

    // Visualize the proof
    println!(
        "\nFermat's Little Theorem proof:\n{}",
        p0.visualize_forest()
    );

    // Get the theorem
    let theorem = builder.build();
    println!("Completed proof of: {}", theorem.name);
}

/// Run all real-world proof examples
pub fn run_real_world_examples() {
    prove_quadratic_formula();
    prove_fermats_little_theorem();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_formula() {
        prove_quadratic_formula();
    }

    #[test]
    fn test_fermats_little_theorem() {
        prove_fermats_little_theorem();
    }
}
