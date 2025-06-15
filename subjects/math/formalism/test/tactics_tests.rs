use crate::subjects::math::formalism::expressions::{Identifier, MathExpression};
use crate::subjects::math::formalism::proof::get_theorem_registry;
use crate::subjects::math::formalism::proof::tactics::{RewriteDirection, Tactic};
use crate::subjects::math::formalism::proof::{
    ProofForest, ProofGoal, ProofStatus, ValueBindedVariable,
};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::theorem::Theorem;

use std::collections::HashMap;

// Helper to create a basic initial state for tests
fn create_initial_goal() -> ProofGoal {
    ProofGoal {
        quantifiers: vec![],
        value_variables: vec![],
        statement: MathRelation::equal(MathExpression::var("a"), MathExpression::var("c")),
    }
}

fn create_simple_equality_theorem() -> Theorem {
    let goal = ProofGoal {
        quantifiers: vec![],
        value_variables: vec![],
        statement: MathRelation::equal(MathExpression::var("x"), MathExpression::var("y")),
    };
    Theorem {
        id: "rewrite_thm".to_string(),
        name: "Simple Equality".to_string(),
        description: "".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

// Test for AssumeImplicationAntecedent
#[test]
fn test_assume_implication_antecedent() {
    let antecedent = MathRelation::equal(MathExpression::var("p"), MathExpression::var("q"));
    let consequent = MathRelation::equal(MathExpression::var("q"), MathExpression::var("r"));
    let goal = ProofGoal {
        quantifiers: vec![],
        value_variables: vec![],
        statement: MathRelation::Implies(
            Box::new(antecedent.clone()),
            Box::new(consequent.clone()),
        ),
    };
    let mut forest = ProofForest::new_from_goal(goal);
    let tactic = Tactic::AssumeImplicationAntecedent {
        hypothesis_name: Identifier::Name("H".to_string(), 0),
    };
    let new_node = forest.apply_initial_tactic(tactic).clone();

    assert!(
        new_node
            .state
            .value_variables
            .iter()
            .any(|v| v.value == MathExpression::Relation(Box::new(antecedent.clone())))
    );
    assert_eq!(new_node.state.statement, consequent);
}

// Test for ExactWith
#[test]
fn test_exact_with_tactic() {
    let hypothesis = MathRelation::equal(MathExpression::var("a"), MathExpression::var("b"));

    let mut goal = create_initial_goal();
    goal.value_variables.push(ValueBindedVariable {
        name: Identifier::Name("H".to_string(), 0),
        value: MathExpression::Relation(Box::new(hypothesis.clone())),
    });
    goal.statement = hypothesis.clone();

    let mut forest = ProofForest::new_from_goal(goal);

    let tactic = Tactic::ExactWith {
        theorem_id: "H".to_string(),
        instantiation: HashMap::new(),
    };
    let _new_node = forest.apply_initial_tactic(tactic).clone();

    // assert_eq!(new_node.status, ProofStatus::Complete);
}

// Test for Rewrite
#[test]
fn test_rewrite_tactic() {
    // Setup a mock theorem for the rewrite
    let registry = get_theorem_registry();
    let thm = create_simple_equality_theorem();
    registry.lock().unwrap().register(thm);

    let goal = create_initial_goal();
    let mut forest = ProofForest::new_from_goal(goal);

    let tactic = Tactic::Rewrite {
        target: MathExpression::var("a"),
        theorem_id: "rewrite_thm".to_string(),
        instantiation: {
            let mut map = HashMap::new();
            map.insert("x".to_string(), MathExpression::var("a"));
            map.insert("y".to_string(), MathExpression::var("b"));
            map
        },
        direction: RewriteDirection::LeftToRight,
    };

    let new_node = forest.apply_initial_tactic(tactic).clone();

    assert_eq!(
        new_node.state.statement,
        MathRelation::equal(MathExpression::var("b"), MathExpression::var("c"))
    );
}
