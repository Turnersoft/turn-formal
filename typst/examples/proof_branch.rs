// Example of creating and manipulating proof branches in Turn-Formal

use turn_formal::math::formalism::{
    core::ProofState,
    expressions::{Identifier, MathExpression},
    proof::{ProofBranch, TheoremBuilder},
    relations::MathRelation,
};

fn create_var(name: &str) -> MathExpression {
    MathExpression::var(name)
}

fn create_relation() -> MathRelation {
    // Create a simple equality relation for testing
    let a = create_var("a");
    let b = create_var("b");
    MathRelation::equal(a, b)
}

fn main() {
    // Create a simple proof state
    let state = ProofState::new();

    // Create a branch and introduce two variables
    let branch1 = state
        .tactics_intro_expr("a", MathExpression::Var(Identifier::E(1)), 0)
        .tactics_intro_expr("b", MathExpression::Var(Identifier::E(2)), 1);

    // Add some proof steps
    let p1 = branch1.tactics_intro_expr("a", create_var("a"), 1);
    let p2 = p1.tactics_intro_expr("b", create_var("b"), 2);

    // Mark as complete
    let p3 = p2.should_complete();

    // Build a theorem using this proof
    let theorem = TheoremBuilder::new("Test Theorem", create_relation(), vec![]).build();

    println!("Theorem name: {}", theorem.name);
    println!("Proof complete: {:?}", p3.summary());
}
