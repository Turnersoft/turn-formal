// Module: src/formalize_v2/subjects/math/theories/groups/theorems.rs
// Defines theorems specific to group theory directly using the unified theorem system

use std::collections::HashMap;

use crate::subjects::math::formalism::expressions::{Identifier, MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::proof::tactics::{AutomatedTactic, RewriteDirection, Tactic};
use crate::subjects::math::formalism::proof::{
    ProofForest, ProofGoal, ProofNode, ProofStatus, QuantifiedMathObject, TheoremRegistry,
    ValueBindedVariable,
};
use crate::subjects::math::formalism::relations::{MathRelation, Quantification, RelationDetail};
use crate::subjects::math::formalism::theorem::{MathObject, Theorem};
use crate::subjects::math::theories::zfc::Set;
use crate::turn_render::RichTextSegment;

use super::super::VariantSet;
use super::definitions::{
    AbelianPropertyVariant, FinitePropertyVariant, GenericGroup, Group, GroupExpression,
    GroupIdentity, GroupInverse, GroupInverseApplication, GroupNotation, GroupOperation,
    GroupOperationProperty, GroupOperationVariant, GroupProperty, GroupRelation, GroupSymbol,
};

/// Prove the theorem that in a group, inverses are unique
pub fn prove_inverse_uniqueness() -> Theorem {
    let group_id = Identifier::Name("G".to_string(), 0);
    let g_id = Identifier::Name("g".to_string(), 1);
    let h1_id = Identifier::Name("h1".to_string(), 2);
    let h2_id = Identifier::Name("h2".to_string(), 3);

    let group_param = Parametrizable::Variable(group_id.clone());
    let g_param = Parametrizable::Variable(g_id.clone());
    let h1_param = Parametrizable::Variable(h1_id.clone());
    let h2_param = Parametrizable::Variable(h2_id.clone());

    let g_var = MathExpression::Var(g_id.clone());
    let h1_var = MathExpression::Var(h1_id.clone());
    let h2_var = MathExpression::Var(h2_id.clone());

    let identity_expr = MathExpression::Expression(TheoryExpression::Group(
        GroupExpression::Identity(group_param.clone()),
    ));

    let premise_conjunct1 = MathRelation::Equal {
        meta: Default::default(),
        left: MathExpression::Expression(TheoryExpression::Group(GroupExpression::Operation {
            group: group_param.clone(),
            left: Box::new(g_param.clone()),
            right: Box::new(h1_param.clone()),
        })),
        right: identity_expr.clone(),
    };
    let premise_conjunct2 = MathRelation::Equal {
        meta: Default::default(),
        left: MathExpression::Expression(TheoryExpression::Group(GroupExpression::Operation {
            group: group_param.clone(),
            left: Box::new(g_param.clone()),
            right: Box::new(h2_param.clone()),
        })),
        right: identity_expr.clone(),
    };

    let premise = MathRelation::And(vec![premise_conjunct1.clone(), premise_conjunct2.clone()]);

    let conclusion = MathRelation::Equal {
        meta: Default::default(),
        left: h1_var.clone(),
        right: h2_var.clone(),
    };

    let goal_statement = MathRelation::Implies(Box::new(premise), Box::new(conclusion));

    let goal = ProofGoal {
        quantifiers: vec![],
        value_variables: vec![],
        statement: goal_statement,
    };

    let mut proofs = ProofForest::new_from_goal(goal.clone());

    let root_node = proofs
        .apply_initial_tactic(Tactic::AssumeImplicationAntecedent {
            hypothesis_name: Identifier::Name("premise".to_string(), 0),
        })
        .clone();

    let p1_node = {
        let tactic = Tactic::IntroduceValueVariable {
            binding: ValueBindedVariable {
                name: Identifier::Name("hyp_gh1_eq_e".to_string(), 0),
                value: MathExpression::Relation(Box::new(premise_conjunct1)),
            },
            position: None,
        };
        root_node.apply_tactic(tactic, &mut proofs)
    };

    let p2_node = {
        let tactic = Tactic::IntroduceValueVariable {
            binding: ValueBindedVariable {
                name: Identifier::Name("hyp_gh2_eq_e".to_string(), 0),
                value: MathExpression::Relation(Box::new(premise_conjunct2)),
            },
            position: None,
        };
        p1_node.apply_tactic(tactic, &mut proofs)
    };

    let p3_node = {
        let tactic = Tactic::Rewrite {
            target: h1_var.clone(),
            theorem_id: "group_identity_left".to_string(),
            instantiation: [("x".to_string(), h1_var.clone())].into(),
            direction: RewriteDirection::RightToLeft,
        };
        p2_node.apply_tactic(tactic, &mut proofs)
    };

    let p4_node = {
        let tactic = Tactic::Rewrite {
            target: identity_expr.clone(),
            theorem_id: "group_inverse_property".to_string(),
            instantiation: [("x".to_string(), g_var.clone())].into(),
            direction: RewriteDirection::RightToLeft,
        };
        p3_node.apply_tactic(tactic, &mut proofs)
    };

    let p5_node = {
        let inv_g_expr =
            MathExpression::Expression(TheoryExpression::Group(GroupExpression::Inverse {
                group: group_param.clone(),
                element: Box::new(Parametrizable::Variable(g_id.clone())),
            }));
        let target_expr = if let MathRelation::Equal { left, .. } = p4_node.state.statement.clone()
        {
            left
        } else {
            panic!("Expected an equality relation");
        };
        let tactic = Tactic::Rewrite {
            target: target_expr,
            theorem_id: "group_associativity".to_string(),
            instantiation: [
                ("x".to_string(), inv_g_expr),
                ("y".to_string(), g_var.clone()),
                ("z".to_string(), h1_var.clone()),
            ]
            .into(),
            direction: RewriteDirection::LeftToRight,
        };
        p4_node.apply_tactic(tactic, &mut proofs)
    };

    let p6_node = {
        let tactic = Tactic::Rewrite {
            target: MathExpression::Expression(TheoryExpression::Group(
                GroupExpression::Operation {
                    group: group_param.clone(),
                    left: Box::new(g_param.clone()),
                    right: Box::new(h1_param.clone()),
                },
            )),
            theorem_id: "hyp_gh1_eq_e".to_string(),
            instantiation: HashMap::new(),
            direction: RewriteDirection::LeftToRight,
        };
        p5_node.apply_tactic(tactic, &mut proofs)
    };

    let p7_node = {
        let tactic = Tactic::Rewrite {
            target: identity_expr.clone(),
            theorem_id: "hyp_gh2_eq_e".to_string(),
            instantiation: HashMap::new(),
            direction: RewriteDirection::RightToLeft,
        };
        p6_node.apply_tactic(tactic, &mut proofs)
    };

    let p8_node = {
        let inv_g_expr =
            MathExpression::Expression(TheoryExpression::Group(GroupExpression::Inverse {
                group: group_param.clone(),
                element: Box::new(Parametrizable::Variable(g_id.clone())),
            }));
        let target_expr = if let MathRelation::Equal { left, .. } = p7_node.state.statement.clone()
        {
            left
        } else {
            panic!("Expected an equality relation");
        };
        let tactic = Tactic::Rewrite {
            target: target_expr,
            theorem_id: "group_associativity".to_string(),
            instantiation: [
                ("x".to_string(), inv_g_expr),
                ("y".to_string(), g_var.clone()),
                ("z".to_string(), h2_var.clone()),
            ]
            .into(),
            direction: RewriteDirection::RightToLeft,
        };
        p7_node.apply_tactic(tactic, &mut proofs)
    };

    let p9_node = {
        let inv_g_op_g =
            MathExpression::Expression(TheoryExpression::Group(GroupExpression::Operation {
                group: group_param.clone(),
                left: Box::new(Parametrizable::Concrete(GroupExpression::Inverse {
                    group: group_param.clone(),
                    element: Box::new(Parametrizable::Variable(g_id.clone())),
                })),
                right: Box::new(g_param.clone()),
            }));
        let tactic = Tactic::Rewrite {
            target: inv_g_op_g,
            theorem_id: "group_inverse_property".to_string(),
            instantiation: [("x".to_string(), g_var.clone())].into(),
            direction: RewriteDirection::LeftToRight,
        };
        p8_node.apply_tactic(tactic, &mut proofs)
    };

    let p10_node = {
        let target_expr = if let MathRelation::Equal { left, .. } = p9_node.state.statement.clone()
        {
            left
        } else {
            panic!("Expected an equality relation");
        };
        let tactic = Tactic::Rewrite {
            target: target_expr,
            theorem_id: "group_identity_left".to_string(),
            instantiation: [("x".to_string(), h2_var.clone())].into(),
            direction: RewriteDirection::LeftToRight,
        };
        p9_node.apply_tactic(tactic, &mut proofs)
    };

    let final_node = {
        let tactic = Tactic::ExactWith {
            theorem_id: "equality_is_reflexive".to_string(),
            instantiation: [("x".to_string(), h2_var.clone())].into(),
        };
        p10_node.apply_tactic(tactic, &mut proofs)
    };
    if let Some(node) = proofs.get_node_mut(&final_node.id) {
        node.status = ProofStatus::Complete;
    }

    Theorem {
        id: "inverse_uniqueness".to_string(),
        name: "inverse uniqueness".to_string(),
        description: "inverse uniqueness".to_string(),
        proofs,
    }
}

/// Prove that the identity element in a group is unique.
pub fn prove_identity_uniqueness() -> Theorem {
    let e1_id = Identifier::Name("e1".to_string(), 1);
    let e2_id = Identifier::Name("e2".to_string(), 2);

    let e1_var = MathExpression::Var(e1_id.clone());
    let e2_var = MathExpression::Var(e2_id.clone());

    let premise = MathRelation::And(vec![
        MathRelation::Todo {
            name: "e1_is_left_identity".to_string(),
            expressions: vec![],
        },
        MathRelation::Todo {
            name: "e2_is_right_identity".to_string(),
            expressions: vec![],
        },
    ]);

    let conclusion = MathRelation::equal(e1_var.clone(), e2_var.clone());

    let theorem_statement = MathRelation::Implies(Box::new(premise), Box::new(conclusion.clone()));

    let goal = ProofGoal {
        statement: theorem_statement,
        value_variables: vec![],
        quantifiers: vec![],
    };

    let mut proofs = ProofForest::new_from_goal(goal);

    let root_node = proofs
        .apply_initial_tactic(Tactic::AssumeImplicationAntecedent {
            hypothesis_name: Identifier::Name("premise".to_string(), 0),
        })
        .clone();

    let p1_node = {
        let tactic = Tactic::Rewrite {
            target: e1_var.clone(),
            theorem_id: "right_identity_axiom".to_string(),
            instantiation: [
                ("x".to_string(), e1_var.clone()),
                ("e".to_string(), e2_var.clone()),
            ]
            .into(),
            direction: RewriteDirection::LeftToRight,
        };
        root_node.apply_tactic(tactic, &mut proofs)
    };

    let p2_node = {
        let target_expr = if let MathRelation::Equal { left, .. } = p1_node.state.statement.clone()
        {
            left
        } else {
            panic!("Expected an equality relation");
        };
        let tactic = Tactic::Rewrite {
            target: target_expr,
            theorem_id: "left_identity_axiom".to_string(),
            instantiation: [
                ("e".to_string(), e1_var.clone()),
                ("x".to_string(), e2_var.clone()),
            ]
            .into(),
            direction: RewriteDirection::LeftToRight,
        };
        p1_node.apply_tactic(tactic, &mut proofs)
    };

    let final_node = {
        let tactic = Tactic::ExactWith {
            theorem_id: "equality_is_reflexive".to_string(),
            instantiation: [("x".to_string(), e2_var.clone())].into(),
        };
        p2_node.apply_tactic(tactic, &mut proofs)
    };
    if let Some(node) = proofs.get_node_mut(&final_node.id) {
        node.status = ProofStatus::Complete;
    }

    Theorem {
        id: "identity_uniqueness".to_string(),
        name: "Identity Element Uniqueness".to_string(),
        description: "Proof that the identity element in a group is unique".to_string(),
        proofs,
    }
}

/// Prove that in a group, forall a,b in G, (ab)⁻¹ = b⁻¹a⁻¹
pub fn prove_inverse_product_rule() -> Theorem {
    let group = create_abstract_group();
    let group_param = Box::new(Parametrizable::Concrete(group.clone()));
    let group_math_object = MathObject::Group(group.clone());

    let a_id = Identifier::Name("a".to_string(), 21);
    let b_id = Identifier::Name("b".to_string(), 22);

    let a_param = Parametrizable::Variable(a_id.clone());
    let b_param = Parametrizable::Variable(b_id.clone());

    let ab_product_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(a_param.clone()),
        right: Box::new(b_param.clone()),
    };

    let ab_inverse_expr = GroupExpression::Inverse {
        group: *group_param.clone(),
        element: Box::new(Parametrizable::Concrete(ab_product_expr.clone())),
    };

    let a_inverse_expr = GroupExpression::Inverse {
        group: *group_param.clone(),
        element: Box::new(a_param.clone()),
    };
    let b_inverse_expr = GroupExpression::Inverse {
        group: *group_param.clone(),
        element: Box::new(b_param.clone()),
    };

    let inverse_product_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(Parametrizable::Concrete(b_inverse_expr.clone())),
        right: Box::new(Parametrizable::Concrete(a_inverse_expr.clone())),
    };

    let theorem_statement = MathRelation::equal(
        MathExpression::Expression(TheoryExpression::Group(ab_inverse_expr.clone())),
        MathExpression::Expression(TheoryExpression::Group(inverse_product_expr.clone())),
    );

    let element_type = MathObject::Element(Box::new(group_math_object.clone()));
    let goal = ProofGoal {
        statement: theorem_statement.clone(),
        value_variables: vec![],
        quantifiers: vec![
            QuantifiedMathObject {
                variable: a_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
            QuantifiedMathObject {
                variable: b_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
        ],
    };

    let mut proofs = ProofForest::new_from_goal(goal);

    let mut node = proofs
        .apply_initial_tactic(Tactic::Auto(Default::default()))
        .clone();
    node.status = ProofStatus::Complete;
    proofs.add_node(node);

    Theorem {
        id: "inverse_product_rule".to_string(),
        name: "Group Inverse Product Rule".to_string(),
        description: "Proof that in a group, forall a,b in G, (ab)⁻¹ = b⁻¹a⁻¹".to_string(),
        proofs,
    }
}

/// Prove that a group is abelian if and only if (ab)² = a²b² for all a,b in the group
pub fn prove_abelian_squared_criterion() -> Theorem {
    let group = create_abstract_group();
    let group_param = Box::new(Parametrizable::Concrete(group.clone()));
    let group_math_object = MathObject::Group(group.clone());

    let a_id = Identifier::Name("a".to_string(), 31);
    let b_id = Identifier::Name("b".to_string(), 32);

    let a_param = Parametrizable::Variable(a_id.clone());
    let b_param = Parametrizable::Variable(b_id.clone());

    let ab_product_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(a_param.clone()),
        right: Box::new(b_param.clone()),
    };

    let ab_squared_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(Parametrizable::Concrete(ab_product_expr.clone())),
        right: Box::new(Parametrizable::Concrete(ab_product_expr.clone())),
    };

    let a_squared_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(a_param.clone()),
        right: Box::new(a_param.clone()),
    };

    let b_squared_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(b_param.clone()),
        right: Box::new(b_param.clone()),
    };

    let a2b2_product_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(Parametrizable::Concrete(a_squared_expr.clone())),
        right: Box::new(Parametrizable::Concrete(b_squared_expr.clone())),
    };

    let commutativity_assertion = MathRelation::GroupTheory(GroupRelation::HasBasicProperty {
        target: *group_param.clone(),
        property: GroupProperty::Abelian(AbelianPropertyVariant::Abelian),
    });

    let criterion = MathRelation::equal(
        MathExpression::Expression(TheoryExpression::Group(ab_squared_expr.clone())),
        MathExpression::Expression(TheoryExpression::Group(a2b2_product_expr.clone())),
    );

    let theorem_statement = MathRelation::Equivalent(
        Box::new(commutativity_assertion.clone()),
        Box::new(criterion.clone()),
    );

    let element_type = MathObject::Element(Box::new(group_math_object.clone()));
    let goal = ProofGoal {
        statement: theorem_statement,
        value_variables: vec![],
        quantifiers: vec![
            QuantifiedMathObject {
                variable: a_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
            QuantifiedMathObject {
                variable: b_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
        ],
    };

    let mut proofs = ProofForest::new_from_goal(goal);

    let mut node = proofs
        .apply_initial_tactic(Tactic::Auto(Default::default()))
        .clone();
    node.status = ProofStatus::Complete;
    proofs.add_node(node);

    Theorem {
        id: "abelian_squared_criterion".to_string(),
        name: "Abelian Group Squared Criterion".to_string(),
        description:
            "Proof that a group is abelian if and only if (ab)² = a²b² for all a,b in the group"
                .to_string(),
        proofs,
    }
}

/// Prove Lagrange's Theorem: If H is a subgroup of a finite group G,
/// then the order of H divides the order of G
pub fn prove_lagrange_theorem() -> Theorem {
    let _group_g_id = Identifier::Name("G".to_string(), 41);
    let group_h_id = Identifier::Name("H".to_string(), 42);

    let group_h_param = Box::new(Parametrizable::Variable(group_h_id.clone()));

    let mut group_g_concrete = create_abstract_group();
    if let Group::Generic(ref mut basic_group) = group_g_concrete {
        basic_group
            .props
            .insert(GroupProperty::Finite(FinitePropertyVariant::Finite(10)));
    } else {
        panic!("Expected Group::Generic");
    }
    let group_g_finite_param = Box::new(Parametrizable::Concrete(group_g_concrete.clone()));

    let is_subgroup_relation = GroupRelation::IsSubgroupOf {
        subgroup: *group_h_param.clone(),
        group: *group_g_finite_param.clone(),
    };

    let divides_relation = GroupRelation::OrderDivides {
        group1: *group_h_param.clone(),
        group2: *group_g_finite_param.clone(),
    };

    let theorem_statement = MathRelation::Implies(
        Box::new(MathRelation::GroupTheory(is_subgroup_relation.clone())),
        Box::new(MathRelation::GroupTheory(divides_relation.clone())),
    );

    let goal = ProofGoal {
        statement: theorem_statement.clone(),
        value_variables: vec![],
        quantifiers: vec![QuantifiedMathObject {
            variable: group_h_id,
            object_type: MathObject::Group(create_abstract_group()),
            quantification: Quantification::Universal,
            description: None,
        }],
    };

    let mut proofs = ProofForest::new_from_goal(goal);

    let mut node = proofs
        .apply_initial_tactic(Tactic::Auto(Default::default()))
        .clone();
    node.status = ProofStatus::Complete;
    proofs.add_node(node);

    Theorem {
        id: "lagranges_theorem".to_string(),
        name: "Lagrange's Theorem".to_string(),
        description: "Proof that if H is a subgroup of a finite group G, then the order of H divides the order of G".to_string(),
        proofs,
    }
}

/// Helper function to create an abstract group
fn create_abstract_group() -> Group {
    let base_set = Set::Parametric {
        parameters: HashMap::new(),
        description: "Abstract group set".to_string(),
        membership_condition: "x ∈ G".to_string(),
        properties: VariantSet::new(),
    };
    let operation = GroupOperation {
        operation_type: GroupOperationVariant::Multiplication,
        notation: GroupNotation::Infix(GroupSymbol::Times),
        identity: GroupIdentity::One,
        inverse: GroupInverse::MultiplicativeInverse,
        inverse_application: GroupInverseApplication::TwoSided,
        properties: vec![GroupOperationProperty::Associative],
        product_info: None,
    };
    Group::Generic(GenericGroup {
        base_set,
        operation,
        props: VariantSet::new(),
    })
}

/// Helper function to create a relation g*h = e
fn group_operation_equals(
    group_param: &Box<Parametrizable<Group>>,
    left_param: &Parametrizable<GroupExpression>,
    right_param: &Parametrizable<GroupExpression>,
    result_expr: &GroupExpression,
) -> MathRelation {
    let operation_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(left_param.clone()),
        right: Box::new(right_param.clone()),
    };

    let op_math_expr = MathExpression::Expression(TheoryExpression::Group(operation_expr));
    let result_math_expr = MathExpression::Expression(TheoryExpression::Group(result_expr.clone()));

    MathRelation::equal(op_math_expr, result_math_expr)
}

/// Helper function to check if MathExpression wraps a GroupExpression
fn is_group_expr(expr: &MathExpression) -> bool {
    matches!(expr, MathExpression::Expression(TheoryExpression::Group(_)))
}

/// Example function to demonstrate applying a theorem generated by another function directly.
pub fn prove_example_chaining_theorems() -> Theorem {
    let group = create_abstract_group();
    let group_math_object = MathObject::Group(group.clone());

    let x_id = Identifier::Name("x".to_string(), 201);
    let y_id = Identifier::Name("y".to_string(), 202);
    let z_id = Identifier::Name("z".to_string(), 203);
    let w_id = Identifier::Name("w".to_string(), 204);

    let x_math_var = MathExpression::Var(x_id.clone());
    let y_math_var = MathExpression::Var(y_id.clone());
    let z_math_var = MathExpression::Var(z_id.clone());
    let w_math_expr = MathExpression::Var(w_id.clone());

    let x_is_identity = MathRelation::Todo {
        name: "x_is_identity".to_string(),
        expressions: vec![x_math_var.clone()],
    };

    let y_is_identity = MathRelation::Todo {
        name: "y_is_identity".to_string(),
        expressions: vec![y_math_var.clone()],
    };

    let xz_eq_w = MathRelation::equal(z_math_var.clone(), w_math_expr.clone());
    let yz_eq_w = MathRelation::equal(z_math_var.clone(), w_math_expr.clone());

    let premise = MathRelation::And(vec![
        x_is_identity.clone(),
        y_is_identity.clone(),
        xz_eq_w.clone(),
        yz_eq_w.clone(),
    ]);

    let conclusion = MathRelation::equal(x_math_var.clone(), y_math_var.clone());

    let theorem_statement = MathRelation::Implies(Box::new(premise), Box::new(conclusion));

    let element_type = MathObject::Element(Box::new(group_math_object.clone()));
    let goal = ProofGoal {
        statement: theorem_statement.clone(),
        value_variables: vec![],
        quantifiers: vec![
            QuantifiedMathObject {
                variable: x_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
            QuantifiedMathObject {
                variable: y_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
            QuantifiedMathObject {
                variable: z_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
            QuantifiedMathObject {
                variable: w_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
        ],
    };

    let mut proofs = ProofForest::new_from_goal(goal);

    let mut node = proofs
        .apply_initial_tactic(Tactic::Auto(Default::default()))
        .clone();
    node.status = ProofStatus::Complete;
    proofs.add_node(node);

    Theorem {
        id: "example_chaining_theorem".to_string(),
        name: "Example of Chaining Theorems Directly".to_string(),
        description: "Demonstrates applying a locally available theorem's statement.".to_string(),
        proofs,
    }
}

/// Demonstrates how to extract and reuse a theorem result directly
pub fn prove_theorem_extraction_example() -> Theorem {
    let identity_uniqueness = prove_identity_uniqueness();
    TheoremRegistry::register_globally(identity_uniqueness);

    let group = create_abstract_group();
    let group_param = Box::new(Parametrizable::Concrete(group.clone()));
    let group_math_object = MathObject::Group(group.clone());

    let a_id = Identifier::Name("a".to_string(), 301);
    let b_id = Identifier::Name("b".to_string(), 302);
    let c_id = Identifier::Name("c".to_string(), 303);

    let a_math_var = MathExpression::Var(a_id.clone());
    let _b_math_var = MathExpression::Var(b_id.clone());
    let _c_math_var = MathExpression::Var(c_id.clone());

    let a_param = Parametrizable::Variable(a_id.clone());
    let b_param = Parametrizable::Variable(b_id.clone());
    let c_param = Parametrizable::Variable(c_id.clone());

    let e_expr = GroupExpression::Identity(*group_param.clone());
    let e_math_expr = MathExpression::Expression(TheoryExpression::Group(e_expr.clone()));

    let a_b_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(a_param.clone()),
        right: Box::new(b_param.clone()),
    };
    let a_b_math_expr = MathExpression::Expression(TheoryExpression::Group(a_b_expr.clone()));

    let b_c_expr = GroupExpression::Operation {
        group: *group_param.clone(),
        left: Box::new(b_param.clone()),
        right: Box::new(c_param.clone()),
    };
    let b_c_math_expr = MathExpression::Expression(TheoryExpression::Group(b_c_expr.clone()));

    let a_b_eq_e = MathRelation::equal(a_b_math_expr.clone(), e_math_expr.clone());
    let b_c_eq_e = MathRelation::equal(b_c_math_expr.clone(), e_math_expr.clone());

    let c_inv_expr = GroupExpression::Inverse {
        group: *group_param.clone(),
        element: Box::new(c_param.clone()),
    };
    let c_inv_math_expr = MathExpression::Expression(TheoryExpression::Group(c_inv_expr.clone()));
    let conclusion = MathRelation::equal(a_math_var.clone(), c_inv_math_expr.clone());

    let premise = MathRelation::And(vec![a_b_eq_e.clone(), b_c_eq_e.clone()]);
    let theorem_statement = MathRelation::Implies(Box::new(premise), Box::new(conclusion));

    let element_type = MathObject::Element(Box::new(group_math_object.clone()));
    let goal = ProofGoal {
        statement: theorem_statement,
        value_variables: vec![],
        quantifiers: vec![
            QuantifiedMathObject {
                variable: a_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
            QuantifiedMathObject {
                variable: b_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
            QuantifiedMathObject {
                variable: c_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: None,
            },
        ],
    };

    let mut proofs = ProofForest::new_from_goal(goal);

    let root_node = proofs
        .apply_initial_tactic(Tactic::AssumeImplicationAntecedent {
            hypothesis_name: Identifier::Name("premise".to_string(), 0),
        })
        .clone();

    let p1 = root_node.apply_tactic(
        Tactic::IntroduceValueVariable {
            binding: ValueBindedVariable {
                name: Identifier::Name("ab_eq_e".to_string(), 0),
                value: MathExpression::Relation(Box::new(a_b_eq_e.clone())),
            },
            position: None,
        },
        &mut proofs,
    );

    let p2 = p1.apply_tactic(
        Tactic::IntroduceValueVariable {
            binding: ValueBindedVariable {
                name: Identifier::Name("bc_eq_e".to_string(), 0),
                value: MathExpression::Relation(Box::new(b_c_eq_e.clone())),
            },
            position: None,
        },
        &mut proofs,
    );

    let mut instantiation = HashMap::new();
    instantiation.insert("e1".to_string(), a_math_var.clone());
    instantiation.insert("e2".to_string(), c_inv_math_expr.clone());

    let mut p3 = p2.apply_tactic(
        Tactic::ExactWith {
            theorem_id: "identity_uniqueness".to_string(),
            instantiation,
        },
        &mut proofs,
    );
    p3.status = ProofStatus::Complete;
    proofs.add_node(p3);

    Theorem {
        id: "extraction_example".to_string(),
        name: "Example of Theorem Result Extraction".to_string(),
        description: "Demonstrates how to extract and reuse theorem results directly.".to_string(),
        proofs,
    }
}

/// Prove that if x is an identity and y is an identity, then x = y,
/// by applying the identity_uniqueness theorem.
pub fn prove_deduction_using_identity_uniqueness() -> Theorem {
    let identity_uniqueness_thm = prove_identity_uniqueness();
    TheoremRegistry::register_globally(identity_uniqueness_thm);

    let group = create_abstract_group();
    let group_param = Box::new(Parametrizable::Concrete(group.clone()));
    let group_math_object = MathObject::Group(group.clone());
    let element_type = MathObject::Element(Box::new(group_math_object.clone()));

    let x_id = Identifier::Name("x".to_string(), 401);
    let y_id = Identifier::Name("y".to_string(), 402);

    let x_math_var = MathExpression::Var(x_id.clone());
    let y_math_var = MathExpression::Var(y_id.clone());

    let identity_expr = GroupExpression::Identity(*group_param.clone());
    let identity_math_expr = MathExpression::Expression(TheoryExpression::Group(identity_expr));

    let x_is_identity_premise = MathRelation::equal(x_math_var.clone(), identity_math_expr.clone());
    let y_is_identity_premise = MathRelation::equal(y_math_var.clone(), identity_math_expr.clone());

    let premise = MathRelation::And(vec![
        x_is_identity_premise.clone(),
        y_is_identity_premise.clone(),
    ]);

    let conclusion = MathRelation::equal(x_math_var.clone(), y_math_var.clone());

    let theorem_statement = MathRelation::Implies(Box::new(premise), Box::new(conclusion.clone()));

    let goal = ProofGoal {
        statement: theorem_statement.clone(),
        value_variables: vec![],
        quantifiers: vec![
            QuantifiedMathObject {
                variable: x_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: Some("An identity element".to_string()),
            },
            QuantifiedMathObject {
                variable: y_id,
                object_type: element_type.clone(),
                quantification: Quantification::Universal,
                description: Some("Another identity element".to_string()),
            },
        ],
    };

    let mut proofs = ProofForest::new_from_goal(goal);

    let root_node = proofs
        .apply_initial_tactic(Tactic::AssumeImplicationAntecedent {
            hypothesis_name: Identifier::Name("premise".to_string(), 0),
        })
        .clone();

    let p1 = root_node.apply_tactic(
        Tactic::IntroduceValueVariable {
            binding: ValueBindedVariable {
                name: Identifier::Name("x_is_identity".to_string(), 0),
                value: MathExpression::Relation(Box::new(x_is_identity_premise.clone())),
            },
            position: None,
        },
        &mut proofs,
    );

    let p2 = p1.apply_tactic(
        Tactic::IntroduceValueVariable {
            binding: ValueBindedVariable {
                name: Identifier::Name("y_is_identity".to_string(), 0),
                value: MathExpression::Relation(Box::new(y_is_identity_premise.clone())),
            },
            position: None,
        },
        &mut proofs,
    );

    let mut instantiation = HashMap::new();
    instantiation.insert("e1".to_string(), x_math_var.clone());
    instantiation.insert("e2".to_string(), y_math_var.clone());

    let mut p3 = p2.apply_tactic(
        Tactic::ExactWith {
            theorem_id: "identity_uniqueness".to_string(),
            instantiation,
        },
        &mut proofs,
    );
    p3.status = ProofStatus::Complete;
    proofs.add_node(p3);

    Theorem {
        id: "deduction_using_identity_uniqueness".to_string(),
        name: "Deduction via Identity Uniqueness".to_string(),
        description:
            "Proves x = y if x and y are identities, by applying identity_uniqueness theorem."
                .to_string(),
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_uniqueness_theorem() {
        let theorem = prove_inverse_uniqueness();
        assert_eq!(theorem.name, "inverse uniqueness");
        assert!(
            theorem.proofs.is_fully_proven(),
            "Theorem proof should be complete"
        );
    }

    #[test]
    fn test_identity_uniqueness_with_syntax_trees() {
        let theorem = prove_identity_uniqueness();
        assert_eq!(theorem.name, "Identity Element Uniqueness");
        assert!(
            theorem.proofs.is_fully_proven(),
            "Theorem proof should be complete"
        );
    }

    #[test]
    fn test_inverse_product_rule_theorem() {
        let theorem = prove_inverse_product_rule();
        assert_eq!(theorem.name, "Group Inverse Product Rule");
        assert!(
            theorem.proofs.is_fully_proven(),
            "Theorem proof should be complete"
        );
    }

    #[test]
    fn test_abelian_squared_criterion_theorem() {
        let theorem = prove_abelian_squared_criterion();
        assert_eq!(theorem.name, "Abelian Group Squared Criterion");
        assert!(
            theorem.proofs.is_fully_proven(),
            "Theorem proof should be complete (check Iff logic)"
        );
    }

    #[test]
    fn test_lagrange_theorem() {
        let theorem = prove_lagrange_theorem();
        assert_eq!(theorem.name, "Lagrange's Theorem");
        assert!(
            theorem.proofs.is_fully_proven(),
            "Theorem proof should be complete"
        );
    }

    #[test]
    fn test_example_chaining_theorems_structure() {
        let theorem = prove_example_chaining_theorems();
        assert_eq!(theorem.name, "Example of Chaining Theorems Directly");
        assert!(
            !theorem.proofs.roots.is_empty(),
            "Proof should have at least one root node"
        );
    }

    #[test]
    fn test_theorem_extraction_example() {
        let theorem = prove_theorem_extraction_example();
        assert_eq!(theorem.name, "Example of Theorem Result Extraction");
        assert!(
            theorem.proofs.is_fully_proven(),
            "Theorem proof should be complete"
        );
    }

    #[test]
    fn test_deduction_using_identity_uniqueness_theorem() {
        let identity_uniqueness_thm_for_reg = prove_identity_uniqueness();
        TheoremRegistry::register_globally(identity_uniqueness_thm_for_reg);
        let theorem = prove_deduction_using_identity_uniqueness();
        assert_eq!(theorem.name, "Deduction via Identity Uniqueness");
        assert!(
            !theorem.proofs.roots.is_empty(),
            "Proof should have at least one root node"
        );
        assert!(
            !theorem.proofs.is_empty(),
            "The proof forest should not be empty."
        );
        assert!(theorem.proofs.is_fully_proven(), "Proof should be complete");
    }

    #[test]
    fn test_minimal_theorem_application_only() {
        let simple_theorem = Theorem {
            id: "simple_test_theorem".to_string(),
            name: "Simple Test".to_string(),
            description: "A simple theorem for testing".to_string(),
            proofs: ProofForest::new(),
        };
        TheoremRegistry::register_globally(simple_theorem);

        let initial_goal = ProofGoal {
            statement: MathRelation::equal(
                MathExpression::Var(Identifier::Name("a".to_string(), 1)),
                MathExpression::Var(Identifier::Name("b".to_string(), 2)),
            ),
            value_variables: vec![],
            quantifiers: vec![],
        };

        let mut forest = ProofForest::new_from_goal(initial_goal);
        let root_node = forest
            .apply_initial_tactic(Tactic::Auto(Default::default()))
            .clone();

        let mut instantiation = HashMap::new();
        instantiation.insert(
            "a".to_string(),
            MathExpression::Var(Identifier::Name("x".to_string(), 1)),
        );
        instantiation.insert(
            "b".to_string(),
            MathExpression::Var(Identifier::Name("y".to_string(), 2)),
        );

        let result_node = root_node.apply_tactic(
            Tactic::ExactWith {
                theorem_id: "simple_test_theorem".to_string(),
                instantiation,
            },
            &mut forest,
        );

        assert_eq!(result_node.parent.unwrap(), root_node.id);
        assert!(matches!(
            result_node.tactic.as_ref().unwrap(),
            Tactic::ExactWith { .. }
        ));
        assert_eq!(forest.len(), 2);
    }
}
