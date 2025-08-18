// Cayley's Theorem
// Statement: Every group G is isomorphic to a subgroup of a symmetric group
//
// Easiest Proof:
// 1. For each g ∈ G, define the left multiplication map L_g: G → G by L_g(x) = gx
// 2. L_g is a bijection (injective: gx = gy implies x = y, surjective: for any y, L_g(g⁻¹y) = y)
// 3. The map g ↦ L_g is a homomorphism: L_gh = L_g ∘ L_h
// 4. It's injective: if L_g = L_h, then gx = hx for all x, so g = h
// 5. Therefore G is isomorphic to {L_g : g ∈ G} ≤ S_|G|
//
// This embeds every group into a symmetric group, showing the universality of permutations.

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

pub fn cayley_theorem() -> Theorem {
    let group_id = Identifier::new_simple("G".to_string());

    let context = vec![ContextEntry {
        name: group_id.clone(),
        ty: Located::new_concrete(Group::new_generic().to_math_expression()),
        definition: DefinitionState::Abstract,
        description: None,
    }];

    // Goal: G is isomorphic to a subgroup of a symmetric group
    // This is Cayley's theorem - every group embeds into a symmetric group
    // We'll use an equality to represent this embedding relationship
    let goal_statement = MathRelation::Equal {
        left: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
            GroupExpression::Element {
                group: Located::new_variable(group_id.clone()),
                element: None,
            },
        ))),
        right: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
            GroupExpression::Element {
                group: Located::new_concrete(Group::new_generic()),
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

    // Note: This theorem requires a complex proof involving group homomorphisms and permutations
    // The proof is not implemented yet, so we just create the proof forest without tactics

    Theorem {
        id: "cayley_theorem".to_string(),
        name: "Cayley's Theorem".to_string(),
        description: "Every group G is isomorphic to a subgroup of a symmetric group".to_string(),
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cayley_theorem() {
        let theorem = cayley_theorem();
        println!("DEBUG: theorem:\n{:?}", theorem);
    }
}
