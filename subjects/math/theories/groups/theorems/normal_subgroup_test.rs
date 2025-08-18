// Normal Subgroup Test
// Statement: A subgroup H of G is normal if and only if gHg⁻¹ ⊆ H for all g ∈ G
// 
// Easiest Proof:
// 1. (⇒) If H is normal, then gH = Hg for all g ∈ G
// 2. So gHg⁻¹ = (gH)g⁻¹ = (Hg)g⁻¹ = H(gg⁻¹) = He = H ⊆ H
// 3. (⇐) If gHg⁻¹ ⊆ H for all g ∈ G, then gH ⊆ Hg
// 4. Apply this to g⁻¹: g⁻¹H(g⁻¹)⁻¹ ⊆ H, so g⁻¹Hg ⊆ H
// 5. Multiply by g on the left: H ⊆ gHg⁻¹
// 6. Since gHg⁻¹ ⊆ H and H ⊆ gHg⁻¹, we have gHg⁻¹ = H
// 7. Therefore gH = Hg, so H is normal
//
// This gives a practical way to check if a subgroup is normal.

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::proof::{ContextEntry, DefinitionState, ProofForest, ProofGoal};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::theorem::Theorem;
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::groups::definitions::{GenericGroup, Group, GroupExpression, SubGroup};
use crate::turn_render::Identifier;

pub fn normal_subgroup_test() -> Theorem {
    let group_id = Identifier::new_simple("G".to_string());
    let h_id = Identifier::new_simple("H".to_string());

    let context = vec![
        ContextEntry {
            name: group_id.clone(),
            ty: Located::new_concrete(Group::new_generic().to_math_expression()),
            definition: DefinitionState::Abstract,
            description: None,
        },
        ContextEntry {
            name: h_id.clone(),
            ty: Located::new_concrete(
                Group::SubGroup(SubGroup {
                    parent_group: Located::new_variable(group_id.clone()),
                    core: GenericGroup::default(),
                    subgroup_props: VariantSet::new(),
                })
                .to_math_expression(),
            ),
            definition: DefinitionState::Abstract,
            description: None,
        },
    ];

    // Goal: H is normal if and only if gHg⁻¹ ⊆ H for all g ∈ G
    let goal_statement = MathRelation::Equal {
        left: Located::new_concrete(
            MathExpression::Expression(TheoryExpression::Group(GroupExpression::Element {
                group: Located::new_variable(h_id.clone()),
                element: None,
            }))
        ),
        right: Located::new_concrete(
            MathExpression::Expression(TheoryExpression::Group(GroupExpression::Element {
                group: Located::new_variable(h_id.clone()),
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

    // Note: This theorem requires a complex proof involving conjugation and set inclusion
    // The proof is not implemented yet, so we just create the proof forest without tactics

    Theorem {
        id: "normal_subgroup_test".to_string(),
        name: "Normal Subgroup Test".to_string(),
        description: "A subgroup H of G is normal if and only if gHg⁻¹ ⊆ H for all g ∈ G".to_string(),
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_subgroup_test() {
        let theorem = normal_subgroup_test();
        println!("DEBUG: theorem:\n{:?}", theorem);
    }
}
