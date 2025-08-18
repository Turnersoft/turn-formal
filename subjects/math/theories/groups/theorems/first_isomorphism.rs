// First Isomorphism Theorem
use std::collections::HashMap;
use std::sync::Arc;

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::objects::MathObject;
use crate::subjects::math::formalism::proof::{
    ContextEntry, DefinitionState, ProofForest, ProofGoal,
};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::theorem::Theorem;
use crate::subjects::math::formalism::traits::debug::ShortDebug;
use crate::turn_render::Identifier;

use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::groups::definitions::{
    GenericGroup, Group, GroupExpression, GroupHomomorphism, GroupRelation, ImageGroup,
    KernelGroup, QuotientGroup,
};

/// Prove the First Isomorphism Theorem
///
/// Formal statement: If φ is a homomorphism from G to H, then G/Ker(φ) ≅ Im(φ)
///
/// This is one of the most fundamental theorems in group theory, connecting
/// homomorphisms, kernels, images, and quotient groups.
pub fn first_isomorphism_theorem() -> Theorem {
    // Create groups and variables
    let group_g = Group::new_generic();
    let group_h = Group::new_generic();
    let group_id = Identifier::new_simple("G".to_string());
    let h_id = Identifier::new_simple("H".to_string());
    let phi_id = Identifier::new_simple("φ".to_string());

    // Create the goal statement: G/Ker(φ) ≅ Im(φ)
    // Build the mathematical expressions step by step

    // Create the quotient group G/Ker(φ)
    let quotient_group = Group::Quotient(QuotientGroup {
        core: GenericGroup::default(),
        group: Located::new_variable(group_id.clone()),
        normal_subgroup: Located::new_concrete(Group::Kernel(KernelGroup {
            core: GenericGroup::default(),
            defining_homomorphism: Located::new_variable(phi_id.clone()),
        })),
        quotient_props: VariantSet::new(),
    });

    // Create the image group Im(φ)
    let image_group = Group::Image(ImageGroup {
        core: GenericGroup::default(),
        defining_homomorphism: Located::new_variable(phi_id.clone()),
    });

    // Create the isomorphism relation
    let goal_statement = MathRelation::GroupTheory(GroupRelation::IsIsomorphicTo {
        first: Located::new_concrete(quotient_group),
        second: Located::new_concrete(image_group),
    });

    let context = vec![
        ContextEntry {
            name: group_id.clone(),
            ty: Located::new_concrete(MathExpression::Object(Arc::new(MathObject::Group(
                group_g.clone(),
            )))),
            definition: DefinitionState::Abstract,
            description: None,
        },
        ContextEntry {
            name: h_id.clone(),
            ty: Located::new_concrete(MathExpression::Object(Arc::new(MathObject::Group(
                group_h.clone(),
            )))),
            definition: DefinitionState::Abstract,
            description: None,
        },
        ContextEntry {
            name: phi_id.clone(),
            ty: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
                GroupExpression::Homomorphism(Located::new_concrete(
                    crate::subjects::math::theories::groups::definitions::GroupHomomorphism {
                        domain: Located::new_variable(group_id.clone()),
                        codomain: Located::new_variable(h_id.clone()),
                    },
                )),
            ))),
            definition: DefinitionState::Abstract,
            description: None,
        },
    ];

    let goal = ProofGoal {
        context: context.clone(),
        quantifiers: vec![],
        statement: Located::new_concrete(goal_statement.clone()),
    };

    let mut proofs = ProofForest::new_from_goal(goal.clone());

    println!(
        "DEBUG: Created proof forest with goal:\n{}",
        goal.short_debug()
    );

    Theorem {
        id: "first_isomorphism_theorem".to_string(),
        name: "First Isomorphism Theorem".to_string(),
        description: "If φ is a homomorphism from G to H, then G/Ker(φ) ≅ Im(φ)".to_string(),
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        subjects::math::formalism::automation::registry::get_theorem_registry,
        subjects::math::formalism::traits::debug::ShortDebug, turn_render::ToMathDocument,
    };

    use super::*;

    #[test]
    fn test_prove_first_isomorphism() {
        let theorem = first_isomorphism_theorem();
        println!("DEBUG: theorem:\n{:?}", theorem);
    }
}
