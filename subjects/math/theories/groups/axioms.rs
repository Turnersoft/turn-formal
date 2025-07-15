//! Defines the axioms of group theory as formal theorems.

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::proof::{ProofForest, ProofGoal};
use crate::subjects::math::formalism::relations::MathRelation;
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
            left: Box::new(a_var),
            right: Box::new(b_var),
        }));

    // Placeholder: Using an equality that is trivially true `a ∘ b = a ∘ b`.
    // The description field is the important part for now.
    let closure_relation = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(operation.clone())),
        right: Located::new(Parametrizable::Concrete(operation)),
    };

    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(closure_relation),
    };

    Axiom {
        id: "group_closure_axiom".to_string(),
        name: "Group Closure Axiom".to_string(),
        description: "For any elements a, b in G, their composition under the group operation is also in G (a ∘ b ∈ G).".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Returns the associativity axiom as a formal theorem.
/// Statement: ∀ a, b, c ∈ G, (a ∘ b) ∘ c = a ∘ (b ∘ c)
pub fn group_associativity_axiom() -> Axiom {
    let group = Parametrizable::Concrete(Group::new_generic());
    let x_var = Parametrizable::Variable(Identifier::new_simple("x".to_string()));
    let y_var = Parametrizable::Variable(Identifier::new_simple("y".to_string()));
    let z_var = Parametrizable::Variable(Identifier::new_simple("z".to_string()));

    // (x * y) * z
    let xy_mult_z =
        MathExpression::Expression(TheoryExpression::Group(GroupExpression::Operation {
            group: group.clone(),
            left: Box::new(Parametrizable::Concrete(GroupExpression::Operation {
                group: group.clone(),
                left: Box::new(x_var.clone()),
                right: Box::new(y_var.clone()),
            })),
            right: Box::new(z_var.clone()),
        }));

    // x * (y * z)
    let x_mult_yz =
        MathExpression::Expression(TheoryExpression::Group(GroupExpression::Operation {
            group: group.clone(),
            left: Box::new(x_var.clone()),
            right: Box::new(Parametrizable::Concrete(GroupExpression::Operation {
                group: group.clone(),
                left: Box::new(y_var.clone()),
                right: Box::new(z_var.clone()),
            })),
        }));

    let associativity_relation = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(xy_mult_z)),
        right: Located::new(Parametrizable::Concrete(x_mult_yz)),
    };

    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(associativity_relation),
    };

    Axiom {
        id: "group_associativity_axiom".to_string(),
        name: "Group Associativity Axiom".to_string(),
        description:
            "The group operation is associative: (x ∘ y) ∘ z = x ∘ (y ∘ z) for all x, y, z in G."
                .to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Returns the identity element axiom as a formal theorem.
/// Statement: ∃e ∈ G, ∀x ∈ G, e ∘ x = x ∧ x ∘ e = x
pub fn group_identity_axiom() -> Axiom {
    let group = Parametrizable::Concrete(Group::new_generic());
    let x_var: Parametrizable<GroupExpression> =
        Parametrizable::Variable(Identifier::new_simple("x".to_string()));
    let identity_gexpr = GroupExpression::Identity(group.clone());

    // e * x
    let e_mult_x_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Box::new(Parametrizable::Concrete(identity_gexpr.clone())),
        right: Box::new(x_var.clone()),
    };

    // x * e
    let x_mult_e_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Box::new(x_var.clone()),
        right: Box::new(Parametrizable::Concrete(identity_gexpr)),
    };

    let x_var_mex: Parametrizable<MathExpression> =
        Parametrizable::Variable(Identifier::new_simple("x".to_string()));

    let left_identity_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(MathExpression::Expression(
            TheoryExpression::Group(e_mult_x_gexpr),
        ))),
        right: Located::new(x_var_mex.clone()),
    };

    let right_identity_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(MathExpression::Expression(
            TheoryExpression::Group(x_mult_e_gexpr),
        ))),
        right: Located::new(x_var_mex),
    };

    let identity_relation = MathRelation::And(vec![
        Located::new(Parametrizable::Concrete(left_identity_rel)),
        Located::new(Parametrizable::Concrete(right_identity_rel)),
    ]);

    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(identity_relation),
    };

    Axiom {
        id: "group_identity_axiom".to_string(),
        name: "Group Identity Axiom".to_string(),
        description: "There exists an identity element e in G such that for every element x in G, e ∘ x = x and x ∘ e = x.".to_string(),
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
            statement: Located::new(MathRelation::False),
        }),
    }
}

pub fn group_identity_theorem() -> Theorem {
    let group = Parametrizable::Concrete(Group::new_generic());
    let x_var: Parametrizable<GroupExpression> =
        Parametrizable::Variable(Identifier::new_simple("x".to_string()));
    let identity_gexpr = GroupExpression::Identity(group.clone());

    // e * x
    let e_mult_x_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Box::new(Parametrizable::Concrete(identity_gexpr.clone())),
        right: Box::new(x_var.clone()),
    };

    // x * e
    let x_mult_e_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Box::new(x_var.clone()),
        right: Box::new(Parametrizable::Concrete(identity_gexpr)),
    };

    let x_var_mex: Parametrizable<MathExpression> =
        Parametrizable::Variable(Identifier::new_simple("x".to_string()));

    let left_identity_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(MathExpression::Expression(
            TheoryExpression::Group(e_mult_x_gexpr),
        ))),
        right: Located::new(x_var_mex.clone()),
    };

    let right_identity_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(MathExpression::Expression(
            TheoryExpression::Group(x_mult_e_gexpr),
        ))),
        right: Located::new(x_var_mex),
    };

    let identity_relation = MathRelation::And(vec![
        Located::new(Parametrizable::Concrete(left_identity_rel)),
        Located::new(Parametrizable::Concrete(right_identity_rel)),
    ]);

    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(identity_relation),
    };

    Theorem {
        id: "group_identity_axiom".to_string(),
        name: "Group Identity Axiom".to_string(),
        description: "There exists an identity element e in G such that for every element x in G, e ∘ x = x and x ∘ e = x.".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Returns the inverse element axiom as a formal theorem.
/// Statement: ∀x ∈ G, ∃x⁻¹ ∈ G, x ∘ x⁻¹ = e ∧ x⁻¹ ∘ x = e
pub fn group_inverse_axiom() -> Axiom {
    let group = Parametrizable::Concrete(Group::new_generic());
    let x_var: Parametrizable<GroupExpression> =
        Parametrizable::Variable(Identifier::new_simple("x".to_string()));
    let identity_gexpr = GroupExpression::Identity(group.clone());
    let inverse_gexpr = GroupExpression::Inverse {
        group: group.clone(),
        element: Box::new(x_var.clone()),
    };

    // x * x⁻¹
    let x_mult_inv_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Box::new(x_var.clone()),
        right: Box::new(Parametrizable::Concrete(inverse_gexpr.clone())),
    };

    // x⁻¹ * x
    let inv_mult_x_gexpr = GroupExpression::Operation {
        group: group.clone(),
        left: Box::new(Parametrizable::Concrete(inverse_gexpr)),
        right: Box::new(x_var),
    };

    let identity_mex = MathExpression::Expression(TheoryExpression::Group(identity_gexpr));

    let right_inverse_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(MathExpression::Expression(
            TheoryExpression::Group(x_mult_inv_gexpr),
        ))),
        right: Located::new(Parametrizable::Concrete(identity_mex.clone())),
    };

    let left_inverse_rel = MathRelation::Equal {
        left: Located::new(Parametrizable::Concrete(MathExpression::Expression(
            TheoryExpression::Group(inv_mult_x_gexpr),
        ))),
        right: Located::new(Parametrizable::Concrete(identity_mex)),
    };

    let inverse_relation = MathRelation::And(vec![
        Located::new(Parametrizable::Concrete(right_inverse_rel)),
        Located::new(Parametrizable::Concrete(left_inverse_rel)),
    ]);

    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(inverse_relation),
    };

    Axiom {
        id: "group_inverse_axiom".to_string(),
        name: "Group Inverse Axiom".to_string(),
        description: "For each element x in G, there exists an inverse element x⁻¹ in G such that x ∘ x⁻¹ = e and x⁻¹ ∘ x = e.".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}
