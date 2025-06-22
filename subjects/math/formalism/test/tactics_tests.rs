use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::objects::MathObject;
use crate::subjects::math::formalism::proof::get_theorem_registry;
use crate::subjects::math::formalism::proof::tactics::{
    RewriteDirection, Tactic, TacticApplicationResult,
};
use crate::subjects::math::formalism::proof::{
    ContextEntry, NodeRole, ProofForest, ProofGoal, ProofStatus,
};
use crate::subjects::math::formalism::relations::{MathRelation, Quantification};
use crate::subjects::math::formalism::theorem::Theorem;
use crate::turn_render::Identifier;

use std::collections::HashMap;

// Helper to create a basic initial state for tests
fn create_initial_goal() -> ProofGoal {
    ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: MathRelation::equal(MathExpression::var("a"), MathExpression::var("c")),
    }
}

fn create_simple_equality_theorem() -> Theorem {
    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: MathRelation::equal(MathExpression::var("x"), MathExpression::var("y")),
    };
    Theorem {
        id: "rewrite_thm".to_string(),
        name: "Simple Equality".to_string(),
        description: "".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

fn create_test_goal() -> ProofGoal {
    let goal = ProofGoal::new_empty();
    let (goal, g_id) = goal.with_variable(
        "G",
        MathExpression::Var(Identifier::new_simple("Group".to_string())),
        None,
    );
    let (goal, x_id) = goal.with_variable(
        "x",
        MathExpression::Var(Identifier::new_simple("Element".to_string())),
        None,
    );
    let goal = goal
        .with_quantifier(&g_id, Quantification::Universal)
        .with_quantifier(&x_id, Quantification::Universal);
    goal.with_statement(MathRelation::True)
}

#[test]
fn test_introduce_tactic() {
    let goal = create_test_goal();
    let new_entry = ContextEntry {
        name: Identifier::new_simple("h".to_string()),
        ty: MathExpression::Var(Identifier::new_simple("Hypothesis".to_string())),
        definition: None,
        description: Some("A new hypothesis".to_string()),
    };

    let tactic = Tactic::Introduce {
        entry: new_entry.clone(),
        position: None,
    };

    let result = tactic.apply_to_goal(&goal);
    if let TacticApplicationResult::SingleGoal(new_goal) = result {
        assert_eq!(new_goal.context.len(), 3);
        assert_eq!(new_goal.context.last().unwrap(), &new_entry);
    } else {
        panic!("Tactic application failed or produced multiple goals.");
    }
}

#[test]
fn test_assume_implication_tactic() {
    let mut goal = create_test_goal();
    let antecedent = MathRelation::Equal {
        left: MathExpression::Var(Identifier::new_simple("a".to_string())),
        right: MathExpression::Var(Identifier::new_simple("b".to_string())),
        meta: Default::default(),
    };
    let consequent = MathRelation::True;
    goal.statement =
        MathRelation::Implies(Box::new(antecedent.clone()), Box::new(consequent.clone()));

    let tactic = Tactic::AssumeImplicationAntecedent {
        hypothesis_name: Identifier::new_simple("H".to_string()),
    };

    let result = tactic.apply_to_goal(&goal);
    if let TacticApplicationResult::SingleGoal(new_goal) = result {
        assert_eq!(new_goal.statement, consequent);
        assert_eq!(new_goal.context.len(), 3);
        let hypothesis = new_goal.context.last().unwrap();
        assert_eq!(hypothesis.name.body, "H");
        assert_eq!(
            hypothesis.ty,
            MathExpression::Relation(Box::new(antecedent))
        );
    } else {
        panic!("Tactic application failed.");
    }
}

// Test for ExactWith
#[test]
fn test_exact_with_tactic() {
    let hypothesis = MathRelation::equal(MathExpression::var("a"), MathExpression::var("b"));

    let mut goal = create_initial_goal();
    // Add hypothesis to context instead of assumptions
    let hypothesis_entry = ContextEntry {
        name: Identifier::new_simple("H".to_string()),
        ty: MathExpression::Relation(Box::new(hypothesis.clone())),
        definition: None,
        description: None,
    };
    goal.context.push(hypothesis_entry);
    goal.statement = hypothesis.clone();

    let mut forest = ProofForest::new_from_goal(goal);

    let tactic = Tactic::ExactWith {
        theorem_id: "H".to_string(),
        instantiation: HashMap::new(),
    };
    let _new_node = forest.apply_initial_tactic(tactic);

    // assert_eq!(new_node.status, ProofStatus::Complete);
}

// Test for Rewrite
#[test]
fn test_rewrite_tactic() {
    // Setup a mock theorem for the rewrite
    let registry = get_theorem_registry();
    let thm = create_simple_equality_theorem();
    registry.lock().unwrap().register(thm.id.clone(), thm);

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

    let new_node = forest.apply_initial_tactic(tactic);

    // Check that the rewrite worked by examining the structure
    match &new_node.get_goal().statement {
        MathRelation::Equal { left, right, .. } => {
            let expected_left = MathExpression::var("b");
            let expected_right = MathExpression::var("c");
            assert_eq!(*left, expected_left);
            assert_eq!(*right, expected_right);
        }
        _ => panic!(
            "Expected equality relation, got {:?}",
            new_node.get_goal().statement
        ),
    }
}
