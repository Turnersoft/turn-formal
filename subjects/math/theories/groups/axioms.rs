//! Defines the axioms of group theory as formal theorems.

use std::sync::Arc;

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::proof::{
    ContextEntry, DefinitionState, ProofForest, ProofGoal, Quantifier,
};
use crate::subjects::math::formalism::relations::{MathRelation, Quantification};
use crate::subjects::math::formalism::theorem::{Axiom, Theorem};
use crate::turn_render::Identifier;

use super::definitions::{Group, GroupExpression};

/// Returns the closure axiom as a formal theorem.
/// Statement: ∀ a, b ∈ G, a ∘ b ∈ G
pub fn group_closure_axiom() -> Axiom {
    // TODO: The current system cannot formally express the typing judgement `a ∘ b ∈ G`.
    // This is a placeholder theorem. The statement is informally described.
    // A proper implementation requires extending MathRelation to support membership.
    let group = Parametrizable::Concrete(Group::new_generic());
    let a_var = Parametrizable::Variable(Identifier::new_simple("a".to_string()));
    let b_var = Parametrizable::Variable(Identifier::new_simple("b".to_string()));

    let operation =
        MathExpression::Expression(TheoryExpression::Group(GroupExpression::Operation {
            group,
            left: a_var.clone(),
            right: b_var.clone(),
        }));

    // Placeholder: Using an equality that is trivially true `a ∘ b = a ∘ b`.
    // The description field is the important part for now.
    let closure_relation = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(Arc::new(operation.clone()))),
        right: Located::new(Parametrizable::Concrete(Arc::new(operation))),
    };

    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(Arc::new(closure_relation)),
    };

    Axiom {
        id: "group_closure_axiom".to_string(),
        name: "Group Closure Axiom".to_string(),
        description: "For any elements a, b in G, their composition under the group operation is also in G (a ∘ b ∈ G).".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Returns the associativity axiom as a formal theorem.
/// Statement: ∀ x, y, z ∈ G, (x ∘ y) ∘ z = x ∘ (y ∘ z)
pub fn group_associativity_axiom() -> Axiom {
    // create identifier and context entries
    let group = Parametrizable::Concrete(Group::new_generic());
    let x_id = Identifier::new_simple("x".to_string());
    let y_id = Identifier::new_simple("y".to_string());
    let z_id = Identifier::new_simple("z".to_string());

    let group_element_type =
        MathExpression::Expression(TheoryExpression::Group(GroupExpression::Element {
            group: group.clone(),
            element: None, // No specific element, this represents the type
        }));

    let x_context_entry = crate::subjects::math::formalism::proof::ContextEntry {
        name: x_id.clone(),
        ty: Located::new(group_element_type.clone()),
        definition: crate::subjects::math::formalism::proof::DefinitionState::Abstract,
        description: Some(crate::turn_render::RichText {
            segments: vec![crate::turn_render::RichTextSegment::Text(
                "Group element x".to_string(),
            )],
            alignment: None,
        }),
    };
    let y_context_entry = ContextEntry {
        name: y_id.clone(),
        ty: Located::new(group_element_type.clone()),
        definition: DefinitionState::Abstract,
        description: Some(crate::turn_render::RichText {
            segments: vec![crate::turn_render::RichTextSegment::Text(
                "Group element y".to_string(),
            )],
            alignment: None,
        }),
    };
    let z_context_entry = ContextEntry {
        name: z_id.clone(),
        ty: Located::new(group_element_type),
        definition: DefinitionState::Abstract,
        description: Some(crate::turn_render::RichText {
            segments: vec![crate::turn_render::RichTextSegment::Text(
                "Group element z".to_string(),
            )],
            alignment: None,
        }),
    };

    let x_quantifier = Quantifier {
        variable_name: x_id.clone(),
        quantification: Quantification::Universal,
    };
    let y_quantifier = Quantifier {
        variable_name: y_id.clone(),
        quantification: Quantification::Universal,
    };
    let z_quantifier = Quantifier {
        variable_name: z_id.clone(),
        quantification: Quantification::Universal,
    };

    // create a vec to push them all in
    let context = vec![x_context_entry, y_context_entry, z_context_entry];
    let quantifiers = vec![x_quantifier, y_quantifier, z_quantifier];

    // create statement using the identifiers
    let x_var = GroupExpression::Element {
        group: group.clone(),
        element: Some(Parametrizable::Variable(x_id)),
    };
    let y_var = GroupExpression::Element {
        group: group.clone(),
        element: Some(Parametrizable::Variable(y_id)),
    };
    let z_var = GroupExpression::Element {
        group: group.clone(),
        element: Some(Parametrizable::Variable(z_id)),
    };

    // (x * y) * z
    let xy_mult_z =
        MathExpression::Expression(TheoryExpression::Group(GroupExpression::Operation {
            group: group.clone(),
            left: Parametrizable::Concrete(Arc::new(GroupExpression::Operation {
                group: group.clone(),
                left: Parametrizable::Concrete(Arc::new(x_var.clone())),
                right: Parametrizable::Concrete(Arc::new(y_var.clone())),
            })),
            right: Parametrizable::Concrete(Arc::new(z_var.clone())),
        }));

    // x * (y * z)
    let x_mult_yz =
        MathExpression::Expression(TheoryExpression::Group(GroupExpression::Operation {
            group: group.clone(),
            left: Parametrizable::Concrete(Arc::new(x_var.clone())),
            right: Parametrizable::Concrete(Arc::new(GroupExpression::Operation {
                group: group.clone(),
                left: Parametrizable::Concrete(Arc::new(y_var.clone())),
                right: Parametrizable::Concrete(Arc::new(z_var.clone())),
            })),
        }));

    let associativity_relation = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(Arc::new(xy_mult_z))),
        right: Located::new(Parametrizable::Concrete(Arc::new(x_mult_yz))),
    };

    // create the goal with statement, context, and quantifiers
    let goal = ProofGoal {
        context,
        quantifiers,
        statement: Located::new(Arc::new(associativity_relation)),
    };

    // return the theorem
    Axiom {
        id: "group_associativity".to_string(),
        name: "Group Associativity".to_string(),
        description: "For all elements x, y, z in a group G, (x ∘ y) ∘ z = x ∘ (y ∘ z)".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Returns the identity axiom as a formal theorem.
/// Statement: ∀ x ∈ G, e ∘ x = x ∧ x ∘ e = x (where e is the identity element)
pub fn group_identity_axiom() -> Axiom {
    // create identifier and context entries
    let group = Parametrizable::Concrete(Group::new_generic());
    let x_id = Identifier::new_simple("x".to_string());
    let group_element_type =
        MathExpression::Expression(TheoryExpression::Group(GroupExpression::Element {
            group: group.clone(),
            element: None, // No specific element, this represents the type
        }));

    let x_context_entry = crate::subjects::math::formalism::proof::ContextEntry {
        name: x_id.clone(),
        ty: Located::new(group_element_type),
        definition: crate::subjects::math::formalism::proof::DefinitionState::Abstract,
        description: Some(crate::turn_render::RichText {
            segments: vec![crate::turn_render::RichTextSegment::Text(
                "Group element x".to_string(),
            )],
            alignment: None,
        }),
    };
    let x_quantifier = crate::subjects::math::formalism::proof::Quantifier {
        variable_name: x_id.clone(),
        quantification: Quantification::Universal,
    };

    // create a vec to push them all in
    let context = vec![x_context_entry];
    let quantifiers = vec![x_quantifier];

    // create statement using the identifiers
    let x_var = GroupExpression::Element {
        group: group.clone(),
        element: Some(Parametrizable::Variable(x_id)),
    };
    let identity_gexpr = GroupExpression::Identity(group.clone());

    // e * x
    let e_mult_x_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Parametrizable::Concrete(Arc::new(identity_gexpr.clone())),
        right: Parametrizable::Concrete(Arc::new(x_var.clone())),
    };

    // x * e
    let x_mult_e_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Parametrizable::Concrete(Arc::new(x_var.clone())),
        right: Parametrizable::Concrete(Arc::new(identity_gexpr)),
    };

    let x_var_mex = MathExpression::Expression(TheoryExpression::Group(x_var));

    // e * x = x
    let left_identity_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(Arc::new(
            MathExpression::Expression(TheoryExpression::Group(e_mult_x_gexpr)),
        ))),
        right: Located::new(Parametrizable::Concrete(Arc::new(x_var_mex.clone()))),
    };

    // x * e = x
    let right_identity_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(Arc::new(
            MathExpression::Expression(TheoryExpression::Group(x_mult_e_gexpr)),
        ))),
        right: Located::new(Parametrizable::Concrete(Arc::new(x_var_mex))),
    };

    let identity_relation = MathRelation::And(vec![
        Located::new(Parametrizable::Concrete(Arc::new(left_identity_rel))),
        Located::new(Parametrizable::Concrete(Arc::new(right_identity_rel))),
    ]);

    // create the goal with statement, context, and quantifiers
    let goal = ProofGoal {
        context,
        quantifiers,
        statement: Located::new(Arc::new(identity_relation)),
    };

    // return the theorem
    Axiom {
        id: "group_identity".to_string(),
        name: "Group Identity".to_string(),
        description:
            "For every element x in G, e ∘ x = x and x ∘ e = x (where e is the identity element)."
                .to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

pub fn test_theorem() -> () {}

pub fn test_theorem_2() -> Theorem {
    Theorem {
        id: "test_theorem_2".to_string(),
        name: "Test Theorem 2".to_string(),
        description: "This is a test theorem".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: Located::new(Arc::new(MathRelation::False)),
        }),
    }
}

pub fn group_identity_theorem() -> Theorem {
    let group = Parametrizable::Concrete(Group::new_generic());

    // Create identifier for the variable
    let x_id = Identifier::new_simple("x".to_string());

    // Create group element type for the variable
    let group_element_type =
        MathExpression::Expression(TheoryExpression::Group(GroupExpression::Element {
            group: group.clone(),
            element: None, // No specific element, this represents the type
        }));

    // Create the context entry for x
    let x_context_entry = crate::subjects::math::formalism::proof::ContextEntry {
        name: x_id.clone(),
        ty: Located::new(group_element_type),
        definition: crate::subjects::math::formalism::proof::DefinitionState::Abstract,
        description: Some(crate::turn_render::RichText {
            segments: vec![crate::turn_render::RichTextSegment::Text(
                "Group element x".to_string(),
            )],
            alignment: None,
        }),
    };

    // Create the quantifier for x
    let x_quantifier = crate::subjects::math::formalism::proof::Quantifier {
        variable_name: x_id.clone(),
        quantification: Quantification::Universal,
    };

    let x_var = GroupExpression::Element {
        group: group.clone(),
        element: Some(Parametrizable::Variable(x_id)),
    };
    let identity_gexpr = GroupExpression::Identity(group.clone());

    // e * x
    let e_mult_x_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Parametrizable::Concrete(Arc::new(identity_gexpr.clone())),
        right: Parametrizable::Concrete(Arc::new(x_var.clone())),
    };

    // x * e
    let x_mult_e_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Parametrizable::Concrete(Arc::new(x_var.clone())),
        right: Parametrizable::Concrete(Arc::new(identity_gexpr)),
    };

    let x_var_mex = MathExpression::Expression(TheoryExpression::Group(x_var));

    let left_identity_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(Arc::new(
            MathExpression::Expression(TheoryExpression::Group(e_mult_x_gexpr)),
        ))),
        right: Located::new(Parametrizable::Concrete(Arc::new(x_var_mex.clone()))),
    };

    let right_identity_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(Arc::new(
            MathExpression::Expression(TheoryExpression::Group(x_mult_e_gexpr)),
        ))),
        right: Located::new(Parametrizable::Concrete(Arc::new(x_var_mex))),
    };

    let identity_relation = MathRelation::And(vec![
        Located::new(Parametrizable::Concrete(Arc::new(left_identity_rel))),
        Located::new(Parametrizable::Concrete(Arc::new(right_identity_rel))),
    ]);

    Theorem {
        id: "group_identity_theorem".to_string(),
        name: "Group Identity Theorem".to_string(),
        description: "There exists an identity element e in G such that for every element x in G, e ∘ x = x and x ∘ e = x.".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            context: vec![x_context_entry],
            quantifiers: vec![x_quantifier],
            statement: Located::new(Arc::new(identity_relation)),
        }),
    }
}

/// Returns the inverse axiom as a formal theorem.
/// Statement: ∀ x ∈ G, x ∘ x⁻¹ = e ∧ x⁻¹ ∘ x = e (existence of inverse for every element)
pub fn group_inverse_axiom() -> Axiom {
    // create identifier and context entries
    let group = Parametrizable::Concrete(Group::new_generic());
    let x_id = Identifier::new_simple("x".to_string());

    let group_element_type =
        MathExpression::Expression(TheoryExpression::Group(GroupExpression::Element {
            group: group.clone(),
            element: None, // No specific element, this represents the type
        }));

    let x_context_entry = crate::subjects::math::formalism::proof::ContextEntry {
        name: x_id.clone(),
        ty: Located::new(group_element_type),
        definition: crate::subjects::math::formalism::proof::DefinitionState::Abstract,
        description: Some(crate::turn_render::RichText {
            segments: vec![crate::turn_render::RichTextSegment::Text(
                "Group element x".to_string(),
            )],
            alignment: None,
        }),
    };
    let x_quantifier = crate::subjects::math::formalism::proof::Quantifier {
        variable_name: x_id.clone(),
        quantification: Quantification::Universal,
    };

    // create a vec to push them all in
    let context = vec![x_context_entry];
    let quantifiers = vec![x_quantifier];

    // create statement using the identifiers
    let x_var = GroupExpression::Element {
        group: group.clone(),
        element: Some(Parametrizable::Variable(x_id)),
    };
    let identity_gexpr = GroupExpression::Identity(group.clone());

    // x⁻¹
    let inverse_gexpr = GroupExpression::Inverse {
        group: group.clone(),
        element: Parametrizable::Concrete(Arc::new(x_var.clone())),
    };

    // x * x⁻¹
    let x_mult_inv_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Parametrizable::Concrete(Arc::new(x_var.clone())),
        right: Parametrizable::Concrete(Arc::new(inverse_gexpr.clone())),
    };

    // x⁻¹ * x
    let inv_mult_x_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Parametrizable::Concrete(Arc::new(inverse_gexpr)),
        right: Parametrizable::Concrete(Arc::new(x_var)),
    };

    let identity_mex = MathExpression::Expression(TheoryExpression::Group(identity_gexpr));

    // x * x⁻¹ = e
    let right_inverse_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(Arc::new(
            MathExpression::Expression(TheoryExpression::Group(x_mult_inv_gexpr)),
        ))),
        right: Located::new(Parametrizable::Concrete(Arc::new(identity_mex.clone()))),
    };

    // x⁻¹ * x = e
    let left_inverse_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(Arc::new(
            MathExpression::Expression(TheoryExpression::Group(inv_mult_x_gexpr)),
        ))),
        right: Located::new(Parametrizable::Concrete(Arc::new(identity_mex))),
    };

    let inverse_relation = MathRelation::And(vec![
        Located::new(Parametrizable::Concrete(Arc::new(right_inverse_rel))),
        Located::new(Parametrizable::Concrete(Arc::new(left_inverse_rel))),
    ]);

    // create the goal with statement, context, and quantifiers
    let goal = ProofGoal {
        context,
        quantifiers,
        statement: Located::new(Arc::new(inverse_relation)),
    };

    // return the theorem
    Axiom {
        id: "group_inverse".to_string(),
        name: "Group Inverse".to_string(),
        description: "For every element x in G, there exists an inverse x⁻¹ such that x ∘ x⁻¹ = e and x⁻¹ ∘ x = e.".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}
