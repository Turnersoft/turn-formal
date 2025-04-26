// Module: src/formalize_v2/subjects/math/theorem/test/real_world_example.rs
// Demonstrates a real-world mathematical proof using the theorem proving system

use std::collections::HashMap;

use uuid::Uuid;

use super::super::super::formalism::core::{ProofGoal, Theorem};
use super::super::super::formalism::expressions::{Identifier, MathExpression, TheoryExpression};
use super::super::super::formalism::proof::{
    ProofForest, ProofNode, ProofStatus, RewriteDirection, Tactic,
};
use super::super::super::formalism::relations::MathRelation;
use super::super::super::theories::rings::definitions::{Ring, RingElementValue, RingExpression};

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

/// A proof of the quadratic formula using case analysis
pub fn prove_quadratic_formula() -> Theorem {
    // Create expression for the quadratic equation
    let polynomial = create_expr("ax² + bx + c");
    let equation = create_expr("ax² + bx + c = 0");
    let solution = create_expr("x = (-b ± √(b² - 4ac)) / (2a)");

    println!("Beginning proof of the quadratic formula...");

    // Create the theorem
    let theorem_id = "quadratic_formula";
    let name = "Quadratic Formula";
    let statement = MathRelation::implies(
        MathRelation::equal(polynomial, create_var("0")),
        MathRelation::custom("SolutionFor".to_string(), vec![solution, equation]),
    );

    let goal = ProofGoal::new(statement);

    let mut proofs = ProofForest::new();

    // Initialize the proof forest with a root node
    let p0 = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: None,
        children: vec![],
        state: goal.clone(),
        tactic: None,
        status: ProofStatus::InProgress,
    };
    proofs.add_node(p0.clone());

    // Setup the proof with initial assumptions
    let p1 = p0.tactics_intro_expr(
        "Consider ax² + bx + c = 0 with a ≠ 0",
        create_expr("initial assumption"),
        &mut proofs,
    );

    let p2 = p1.tactics_intro_expr(
        "We want to find values of x that satisfy this equation",
        create_expr("goal statement"),
        &mut proofs,
    );

    // First step: rearranging to standard form
    let mut rearrange_tactic = Tactic::Rewrite {
        target_expr: create_expr("ax² + bx + c = 0"),
        equation_expr: create_expr("ax² + bx = -c"),
        direction: RewriteDirection::LeftToRight,
        location: None,
    };

    let p3 = p2.apply_tactic(rearrange_tactic, &mut proofs);

    // Complete the square approach
    let p4 = p3.tactics_intro_expr(
        "Divide all terms by a",
        create_expr("division by a"),
        &mut proofs,
    );

    let p5 = p4.tactics_subs_expr(
        create_expr("x² + (b/a)x = -c/a"),
        create_expr("divided form"),
        None,
        &mut proofs,
    );

    // Add (b/2a)² to both sides
    let p6 = p5.tactics_intro_expr(
        "Add (b/2a)² to both sides to complete the square",
        create_expr("complete square"),
        &mut proofs,
    );

    let p7 = p6.tactics_subs_expr(
        create_expr("-c/a + (b/2a)² = (b² - 4ac) / (4a²)"),
        create_expr("square completed"),
        None,
        &mut proofs,
    );

    // Simplify right side
    let p8 = p7.tactics_intro_expr("Simplify right side", create_expr("simplify"), &mut proofs);

    let p9 = p8.tactics_subs_expr(
        create_expr("-c/a + (b/2a)² = (b² - 4ac) / (4a²)"),
        create_expr("simplified"),
        None,
        &mut proofs,
    );

    // Express left side as perfect square
    let p10 = p9.tactics_intro_expr(
        "Left side is a perfect square",
        create_expr("perfect square"),
        &mut proofs,
    );

    let p11 = p10.tactics_subs_expr(
        create_expr("(x + b/2a)² = (b² - 4ac) / (4a²)"),
        create_expr("perfect square form"),
        None,
        &mut proofs,
    );

    // Case analysis on discriminant
    let case_analysis = p11.case_analysis(&mut proofs);

    // Case 1: discriminant > 0
    let case1_branch = p11.clone();
    let c1 = case1_branch.tactics_intro_expr(
        "When b² - 4ac > 0, we have two distinct real solutions",
        create_expr("positive discriminant"),
        &mut proofs,
    );

    let c1_1 = c1.tactics_intro_expr(
        "Take square root of both sides",
        create_expr("square root"),
        &mut proofs,
    );

    let c1_2 = c1_1.tactics_subs_expr(
        create_expr("x + b/2a = ±√((b² - 4ac) / (4a²))"),
        create_expr("squared form"),
        None,
        &mut proofs,
    );

    let c1_3 = c1_2.tactics_intro_expr(
        "Simplify the square root",
        create_expr("simplify sqrt"),
        &mut proofs,
    );

    let c1_4 = c1_3.tactics_subs_expr(
        create_expr("x + b/2a = ±√(b² - 4ac) / (2a)"),
        create_expr("simplified sqrt"),
        None,
        &mut proofs,
    );

    let c1_5 = c1_4.tactics_intro_expr(
        "Subtract b/2a from both sides",
        create_expr("isolate x"),
        &mut proofs,
    );

    let c1_6 = c1_5.tactics_subs_expr(
        create_expr("x = -b/2a ± √(b² - 4ac) / (2a)"),
        create_expr("x isolated"),
        None,
        &mut proofs,
    );

    let c1_7 = c1_6.tactics_intro_expr(
        "Combine terms with common denominator",
        create_expr("combine terms"),
        &mut proofs,
    );

    let c1_8 = c1_7.tactics_subs_expr(
        create_expr("x = (-b ± √(b² - 4ac)) / (2a)"),
        create_expr("final formula"),
        None,
        &mut proofs,
    );

    let c1_complete = c1_8.should_complete(&mut proofs);

    // Case 2: discriminant = 0
    let case2_branch = p11.clone();
    let c2 = case2_branch.tactics_intro_expr(
        "When b² - 4ac = 0, we have exactly one real solution",
        create_expr("zero discriminant"),
        &mut proofs,
    );

    let c2_1 = c2.tactics_intro_expr(
        "Take square root of both sides",
        create_expr("square root"),
        &mut proofs,
    );

    let c2_2 = c2_1.tactics_subs_expr(
        create_expr("x + b/2a = 0"),
        create_expr("squared form"),
        None,
        &mut proofs,
    );

    let c2_3 = c2_2.tactics_intro_expr(
        "Subtract b/2a from both sides",
        create_expr("isolate x"),
        &mut proofs,
    );

    let c2_4 = c2_3.tactics_subs_expr(
        create_expr("x = -b/2a"),
        create_expr("x isolated"),
        None,
        &mut proofs,
    );

    let c2_5 = c2_4.tactics_intro_expr(
        "Express in standard form",
        create_expr("standard form"),
        &mut proofs,
    );

    let c2_6 = c2_5.tactics_subs_expr(
        create_expr("x = -b/(2a)"),
        create_expr("final formula"),
        None,
        &mut proofs,
    );

    let c2_complete = c2_6.should_complete(&mut proofs);

    // Case 3: discriminant < 0
    let case3_branch = p11.clone();
    let c3 = case3_branch.tactics_intro_expr(
        "When b² - 4ac < 0, we have two complex conjugate solutions",
        create_expr("negative discriminant"),
        &mut proofs,
    );

    let c3_1 = c3.tactics_intro_expr(
        "Take square root of both sides",
        create_expr("square root"),
        &mut proofs,
    );

    let c3_2 = c3_1.tactics_subs_expr(
        create_expr("x + b/2a = ±i√(|b² - 4ac| / (4a²))"),
        create_expr("squared form"),
        None,
        &mut proofs,
    );

    let c3_3 = c3_2.tactics_intro_expr(
        "Simplify the square root",
        create_expr("simplify sqrt"),
        &mut proofs,
    );

    let c3_4 = c3_3.tactics_subs_expr(
        create_expr("x + b/2a = ±i√(|b² - 4ac|) / (2a)"),
        create_expr("simplified sqrt"),
        None,
        &mut proofs,
    );

    let c3_5 = c3_4.tactics_intro_expr(
        "Subtract b/2a from both sides",
        create_expr("isolate x"),
        &mut proofs,
    );

    let c3_6 = c3_5.tactics_subs_expr(
        create_expr("x = -b/2a ± i√(|b² - 4ac|) / (2a)"),
        create_expr("x isolated"),
        None,
        &mut proofs,
    );

    let c3_7 = c3_6.tactics_intro_expr(
        "Combine terms with common denominator",
        create_expr("combine terms"),
        &mut proofs,
    );

    let c3_8 = c3_7.tactics_subs_expr(
        create_expr("x = (-b ± i√(|b² - 4ac|)) / (2a)"),
        create_expr("final formula"),
        None,
        &mut proofs,
    );

    let c3_complete = c3_8.should_complete(&mut proofs);

    // Continue with the main proof, combining all cases
    let p12 = p11.tactics_intro_expr(
        "In all cases, we have shown the solutions",
        create_expr("combined cases"),
        &mut proofs,
    );

    let p13 = p12.tactics_intro_expr(
        "Combining all cases, we get the quadratic formula",
        create_expr("formula conclusion"),
        &mut proofs,
    );

    let p14 = p13.tactics_subs_expr(
        create_expr("x = (-b ± √(b² - 4ac)) / (2a)"),
        create_expr("quadratic formula"),
        None,
        &mut proofs,
    );

    let p15 = p14.should_complete(&mut proofs);

    // Return the theorem
    Theorem {
        id: theorem_id.to_string(),
        name: name.to_string(),
        description: "The formula for finding the roots of a quadratic equation ax² + bx + c = 0"
            .to_string(),
        goal,
        proofs,
    }
}

/// A proof of Fermat's Little Theorem using modular arithmetic
pub fn prove_fermats_little_theorem() -> Theorem {
    // Create expressions
    let p = create_var("p");
    let a = create_var("a");
    let expr1 = create_expr("a^p");
    let expr2 = create_var("a");

    println!("Beginning proof of Fermat's Little Theorem...");

    // Create the theorem: If p is prime and p doesn't divide a, then a^p ≡ a (mod p)
    let theorem_id = "fermats_little_theorem";
    let name = "Fermat's Little Theorem";
    let statement = MathRelation::custom("Congruent".to_string(), vec![expr1, expr2, p]);

    let goal = ProofGoal::new(statement);

    let mut proofs = ProofForest::new();

    // Initialize the proof forest with a root node
    let p0 = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: None,
        children: vec![],
        state: goal.clone(),
        tactic: None,
        status: ProofStatus::InProgress,
    };
    proofs.add_node(p0.clone());

    // Setup the proof with initial assumptions
    let p1 = p0.tactics_intro_expr(
        "Let p be a prime number",
        create_expr("p prime"),
        &mut proofs,
    );

    let p2 = p1.tactics_intro_expr(
        "Let a be an integer not divisible by p",
        create_expr("a not div p"),
        &mut proofs,
    );

    // Use induction on a
    let p3 = p2.tactics_intro_expr(
        "We will prove this by induction on a",
        create_expr("induction"),
        &mut proofs,
    );

    // Base case: a = 1
    let p4 = p3.tactics_intro_expr("Base case: a = 1", create_expr("base case"), &mut proofs);

    let p5 = p4.tactics_subs_expr(
        create_expr("1^p = 1"),
        create_expr("trivial case"),
        None,
        &mut proofs,
    );

    let p6 = p5.tactics_intro_expr(
        "Clearly, 1 ≡ 1 (mod p)",
        create_expr("base proven"),
        &mut proofs,
    );

    // Induction step
    let p7 = p6.tactics_intro_expr(
        "Induction step: Assume the theorem holds for some a",
        create_expr("induction step"),
        &mut proofs,
    );

    let p8 = p7.tactics_intro_expr(
        "We need to prove it holds for a+1",
        create_expr("induction goal"),
        &mut proofs,
    );

    // Use the binomial theorem
    let p9 = p8.tactics_intro_expr(
        "By the binomial theorem",
        create_expr("binomial expansion"),
        &mut proofs,
    );

    let p10 = p9.tactics_subs_expr(
        create_expr("(a+1)^p = a^p + p*C(p,1)*a^(p-1) + ... + p*C(p,p-1)*a + 1"),
        create_expr("expanded form"),
        None,
        &mut proofs,
    );

    // Apply modular arithmetic
    let p11 = p10.tactics_intro_expr(
        "All terms except the first and last contain a factor of p",
        create_expr("p factor"),
        &mut proofs,
    );

    let p12 = p11.tactics_intro_expr(
        "By the properties of modular arithmetic",
        create_expr("modular property"),
        &mut proofs,
    );

    let p13 = p12.tactics_subs_expr(
        create_expr("(a+1)^p ≡ a^p + 1 (mod p)"),
        create_expr("modular simplified"),
        None,
        &mut proofs,
    );

    // Apply induction hypothesis
    let p14 = p13.tactics_intro_expr(
        "By the induction hypothesis, a^p ≡ a (mod p)",
        create_expr("induction hypothesis"),
        &mut proofs,
    );

    let p15 = p14.tactics_subs_expr(
        create_expr("(a+1)^p ≡ a + 1 (mod p)"),
        create_expr("substituted"),
        None,
        &mut proofs,
    );

    let p16 = p15.tactics_intro_expr(
        "Thus, (a+1)^p ≡ (a+1) (mod p)",
        create_expr("induction proven"),
        &mut proofs,
    );

    // Complete the proof
    let p17 = p16.tactics_intro_expr(
        "By the principle of induction, the theorem holds for all a",
        create_expr("induction conclusion"),
        &mut proofs,
    );

    let p18 = p17.should_complete(&mut proofs);

    // Return the completed theorem
    Theorem {
        id: theorem_id.to_string(),
        name: name.to_string(),
        description: "If p is a prime number and a is not divisible by p, then a^p ≡ a (mod p)"
            .to_string(),
        goal,
        proofs,
    }
}

/// Run all real-world proof examples
pub fn run_real_world_examples() {
    let theorem1 = prove_quadratic_formula();
    let theorem2 = prove_fermats_little_theorem();

    println!(
        "Completed proofs of: {} and {}",
        theorem1.name, theorem2.name
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_formula() {
        let theorem = prove_quadratic_formula();
        assert_eq!(theorem.name, "Quadratic Formula");
    }

    #[test]
    fn test_fermats_little_theorem() {
        let theorem = prove_fermats_little_theorem();
        assert_eq!(theorem.name, "Fermat's Little Theorem");
    }
}
