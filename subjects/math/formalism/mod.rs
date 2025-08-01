// Module: src/formalize_v2/subjects/math/theorem/mod.rs
// Acts as a central hub for the theorem system in the project

// Note: counter_example is intentionally commented out to prevent compile errors
// Uncomment to see various examples of compile-time errors in action
// pub mod counter_example;

pub mod automation;
pub mod expressions;
pub mod extract;
pub mod foundational_axioms;
pub mod interpretation;
pub mod location;
pub mod objects;
pub mod proof;
pub mod relations;
pub mod render;
pub mod test;
pub mod theorem;
pub mod traits;

use std::sync::Arc;

use expressions::{MathExpression, TheoryExpression};
use extract::Parametrizable;
use location::Located;
use proof::{ProofForest, ProofGoal};
use relations::MathRelation;
use theorem::Theorem;
use traits::debug::ShortDebug;
use traits::detag::TryDetag;

use crate::turn_render::Identifier;

use super::theories::groups::definitions::{Group, GroupExpression};
pub fn group_identity_theorem_2() -> Theorem {
    let group = Located::new_concrete(Group::new_generic());
    let x_var = Identifier::new_simple("x".to_string());
    let identity_gexpr = GroupExpression::Identity(group.clone());

    // e * x
    let e_mult_x_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Located::new_concrete(identity_gexpr.clone()),
        right: Located::new_variable(x_var.clone()),
    };

    // x * e
    let x_mult_e_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Located::new_variable(x_var.clone()),
        right: Located::new_concrete(identity_gexpr),
    };

    let x_var_mex = Identifier::new_simple("x".to_string());

    let left_identity_rel = MathRelation::Equal {
        left: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
            e_mult_x_gexpr,
        ))),
        right: Located::new_variable(x_var_mex.clone()),
    };

    let right_identity_rel = MathRelation::Equal {
        left: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
            x_mult_e_gexpr,
        ))),
        right: Located::new_variable(x_var_mex.clone()),
    };

    let identity_relation = MathRelation::And(vec![
        Located::new_concrete(left_identity_rel),
        Located::new_concrete(right_identity_rel),
    ]);

    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new_concrete(identity_relation),
    };

    Theorem {
        id: "group_identity_axiom".to_string(),
        name: "Group Identity Axiom".to_string(),
        description: "There exists an identity element e in G such that for every element x in G, e ∘ x = x and x ∘ e = x.".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}
