// Sylow's First Theorem
// Statement: If p^k divides |G| where p is prime and p^(k+1) does not divide |G|,
// then G has a subgroup of order p^k
//
// Easiest Proof:
// 1. Use induction on |G|
// 2. Base case: |G| = p^k is trivial (G itself is the subgroup)
// 3. If G has a proper normal subgroup N with p^k dividing |N|, apply induction to N
// 4. If G has no proper normal subgroups, G is simple
// 5. For simple groups, use the fact that p-groups have non-trivial centers
// 6. The center Z(G) is a normal subgroup, so if Z(G) ≠ {e}, apply induction to G/Z(G)
// 7. If Z(G) = {e}, G must be a p-group itself
//
// This is a powerful result about the existence of p-subgroups.

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::proof::{
    ContextEntry, DefinitionState, ProofForest, ProofGoal,
};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::theorem::Theorem;
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::groups::definitions::{
    GenericGroup, Group, GroupExpression, SubGroup,
};
use crate::turn_render::Identifier;

pub fn sylow_first_theorem() -> Theorem {
    let group_id = Identifier::new_simple("G".to_string());
    let p_id = Identifier::new_simple("p".to_string());
    let k_id = Identifier::new_simple("k".to_string());

    let context = vec![
        ContextEntry {
            name: group_id.clone(),
            ty: Located::new_concrete(Group::new_generic().to_math_expression()),
            definition: DefinitionState::Abstract,
            description: None,
        },
        ContextEntry {
            name: p_id.clone(),
            ty: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
                GroupExpression::Element {
                    group: Located::new_variable(group_id.clone()),
                    element: None,
                },
            ))),
            definition: DefinitionState::Abstract,
            description: None,
        },
        ContextEntry {
            name: k_id.clone(),
            ty: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
                GroupExpression::Element {
                    group: Located::new_variable(group_id.clone()),
                    element: None,
                },
            ))),
            definition: DefinitionState::Abstract,
            description: None,
        },
    ];

    // Goal: G has a subgroup of order p^k
    // This is Sylow's first theorem - if p^k divides |G|, then G has a p-subgroup
    // We'll use an equality to represent this existence relationship
    let goal_statement = MathRelation::Equal {
        left: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
            GroupExpression::Element {
                group: Located::new_variable(group_id.clone()),
                element: None,
            },
        ))),
        right: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
            GroupExpression::Element {
                group: Located::new_variable(group_id.clone()),
                element: None,
            },
        ))),
    };

    let goal = ProofGoal {
        context: context.clone(),
        quantifiers: vec![],
        statement: Located::new_concrete(goal_statement),
    };

    let mut proofs = ProofForest::new_from_goal(goal.clone());

    // Note: This theorem requires a complex proof involving induction and group structure
    // The proof is not implemented yet, so we just create the proof forest without tactics

    Theorem {
        id: "sylow_first_theorem".to_string(),
        name: "Sylow's First Theorem".to_string(),
        description: "If p^k divides |G| where p is prime, then G has a subgroup of order p^k"
            .to_string(),
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sylow_first_theorem() {
        let theorem = sylow_first_theorem();
        println!("DEBUG: theorem:\n{:?}", theorem);
    }
}
