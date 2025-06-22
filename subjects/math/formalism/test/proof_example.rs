// This file is for demonstrating and testing the proof construction functionalities.
use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::objects::MathObject;
use crate::subjects::math::formalism::proof::{ProofGoal, ProofStatus};
use crate::subjects::math::formalism::relations::{MathRelation, Quantification};
use crate::turn_render::Identifier;
use std::collections::HashMap;

#[test]
fn test_new_proof_goal_construction() {
    let goal = ProofGoal::new_empty();

    let (goal, g_id) = goal.with_variable(
        "G",
        MathExpression::Var(Identifier::new_simple("Group".to_string())),
        None,
    );
    let goal = goal.with_quantifier(&g_id, Quantification::Universal);

    let final_goal = goal.with_statement(MathRelation::True);

    assert!(final_goal.verify().is_ok());
}
