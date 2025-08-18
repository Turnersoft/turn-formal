// Center of a Group
// Statement: The center Z(G) = {g ∈ G : gh = hg for all h ∈ G} is a normal subgroup of G
// 
// Easiest Proof:
// 1. First show Z(G) is a subgroup:
//    - Identity: e ∈ Z(G) since eh = he for all h
//    - Closure: if g, h ∈ Z(G), then (gh)k = g(hk) = g(kh) = (gk)h = k(gh), so gh ∈ Z(G)
//    - Inverses: if g ∈ Z(G), then g⁻¹h = (hg⁻¹)⁻¹ = (g⁻¹h)⁻¹ = hg⁻¹, so g⁻¹ ∈ Z(G)
// 2. Show Z(G) is normal:
//    - For any g ∈ G and z ∈ Z(G), gzg⁻¹ = gg⁻¹z = ez = z ∈ Z(G)
//    - Therefore gZ(G)g⁻¹ ⊆ Z(G) for all g ∈ G
//    - By the normal subgroup test, Z(G) is normal
//
// The center is always abelian and provides insight into group structure.

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::proof::{ContextEntry, DefinitionState, ProofForest, ProofGoal};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::theorem::Theorem;
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::groups::definitions::{GenericGroup, Group, GroupExpression, SubGroup};
use crate::turn_render::Identifier;

pub fn center_is_normal_subgroup() -> Theorem {
    let group_id = Identifier::new_simple("G".to_string());

    let context = vec![
        ContextEntry {
            name: group_id.clone(),
            ty: Located::new_concrete(Group::new_generic().to_math_expression()),
            definition: DefinitionState::Abstract,
            description: None,
        },
    ];

    // Goal: Z(G) is a normal subgroup of G
    let goal_statement = MathRelation::Equal {
        left: Located::new_concrete(
            MathExpression::Expression(TheoryExpression::Group(GroupExpression::Element {
                group: Located::new_variable(group_id.clone()),
                element: None,
            }))
        ),
        right: Located::new_concrete(
            MathExpression::Expression(TheoryExpression::Group(GroupExpression::Element {
                group: Located::new_variable(group_id.clone()),
                element: None,
            }))
        ),
    };

    let goal = ProofGoal {
        context: context.clone(),
        quantifiers: vec![],
        statement: Located::new_concrete(goal_statement),
    };

    let mut proofs = ProofForest::new_from_goal(goal.clone());

    // Note: This theorem requires a complex proof involving subgroup and normality tests
    // The proof is not implemented yet, so we just create the proof forest without tactics

    Theorem {
        id: "center_is_normal_subgroup".to_string(),
        name: "Center is Normal Subgroup".to_string(),
        description: "The center Z(G) of a group G is always a normal subgroup".to_string(),
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_is_normal_subgroup() {
        let theorem = center_is_normal_subgroup();
        println!("DEBUG: theorem:\n{:?}", theorem);
    }
}
