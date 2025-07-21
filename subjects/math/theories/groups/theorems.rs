// Module: src/formalize_v2/subjects/math/theories/groups/theorems.rs
// Defines theorems specific to group theory directly using the unified theorem system

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::objects::MathObject;

use crate::subjects::math::formalism::proof::tactics::{
    ContextOrStatement, RelationSource, RewriteDirection, Tactic, Target,
};
use crate::subjects::math::formalism::proof::{
    ContextEntry, DefinitionState, NodeRole, ProofForest, ProofGoal, ProofNode, Quantifier,
};
use crate::subjects::math::formalism::relations::{MathRelation, Quantification};
use crate::subjects::math::formalism::theorem::Theorem;
use crate::turn_render::{Identifier, RichText, RichTextSegment};

use super::super::VariantSet;
use super::definitions::{
    AbelianPropertyVariant, CanonicityVariant, CenterGroup, CommutatorSubgroup, ComplexityVariant,
    FinitePropertyVariant, FreeGroup, GenericGroup, Group, GroupAction, GroupActionProperty,
    GroupElement, GroupExpression, GroupHomomorphism, GroupIdentity, GroupInverse,
    GroupInverseApplication, GroupNotation, GroupOperation, GroupOperationVariant, GroupProperty,
    GroupRelation, GroupSymbol, ImageGroup, KernelGroup, LieGroup, LieGroupProperty,
    MatrixProperty, ModularAdditiveGroup, ModularMultiplicativeGroup, ModularProperty,
};

/// Prove the theorem that in a group, inverses are unique
pub fn prove_inverse_uniqueness() -> Theorem {
    // Create a simple, direct proof using left cancellation
    let group = Group::new_generic();
    let group_id = Identifier::new_simple("G".to_string());

    let g_id = Identifier::new_simple("g".to_string());
    let h1_id = Identifier::new_simple("h1".to_string());
    let h2_id = Identifier::new_simple("h2".to_string());
    let e_id = Identifier::new_simple("e".to_string());

    let identity_expr = MathExpression::Expression(TheoryExpression::Group(
        GroupExpression::Identity(Located::new_variable(group_id.clone())),
    ));

    // Premise: g*h1 = e ∧ g*h2 = e
    let premise_conjunct1 = MathRelation::Equal {
        left: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
            GroupExpression::Operation {
                group: Located::new_variable(group_id.clone()),
                left: Located::new_variable(g_id.clone()),
                right: Located::new_variable(h1_id.clone()),
            },
        ))),
        right: Located::new_variable(e_id.clone()),
    };
    let premise_conjunct2 = MathRelation::Equal {
        left: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
            GroupExpression::Operation {
                group: Located::new_variable(group_id.clone()),
                left: Located::new_variable(g_id.clone()),
                right: Located::new_variable(h2_id.clone()),
            },
        ))),
        right: Located::new_variable(e_id.clone()),
    };

    let premise = MathRelation::And(vec![
        Located::new_concrete(premise_conjunct1.clone()),
        Located::new_concrete(premise_conjunct2.clone()),
    ]);

    // Conclusion: h1 = h2
    let conclusion = MathRelation::Equal {
        left: Located::new_variable(h1_id.clone()),
        right: Located::new_variable(h2_id.clone()),
    };

    let goal_statement = MathRelation::Implies(
        Located::new_concrete(premise),
        Located::new_concrete(conclusion.clone()),
    );

    let goal = ProofGoal {
        context: vec![
            ContextEntry {
                name: group_id.clone(),
                ty: Located::new_concrete(MathExpression::Object(Arc::new(MathObject::Group(
                    group.clone(),
                )))),
                definition: DefinitionState::Abstract,
                description: None,
            },
            ContextEntry {
                name: g_id.clone(),
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
                name: h1_id.clone(),
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
                name: h2_id.clone(),
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
                name: e_id.clone(),
                ty: Located::new_concrete(MathExpression::Expression(TheoryExpression::Group(
                    GroupExpression::Identity(Located::new_variable(group_id.clone())),
                ))),
                definition: DefinitionState::Abstract,
                description: None,
            },
        ],
        quantifiers: vec![],
        statement: Located::new_concrete(goal_statement.clone()),
    };

    let mut proofs = ProofForest::new_from_goal(goal);

    // Step 1: Assume the antecedent of the implication.
    // Goal: (g*h1 = e ∧ g*h2 = e) → (h1 = h2)
    // New Goal: h1 = h2
    let p1_node = {
        let tactic = Tactic::AssumeImplicationAntecedent {
            with_name: Identifier::new_simple("premise".to_string()),
        };
        proofs.apply_initial_tactic(tactic).clone()
    };

    // Step 2: Split the conjunctive premise into two separate hypotheses.
    // Premise: g*h1 = e ∧ g*h2 = e
    // New Hypotheses: hyp_gh1_eq_e: g*h1 = e, hyp_gh2_eq_e: g*h2 = e
    let hyp1 = Identifier::new_simple("hyp_gh1_eq_e".to_string());
    let hyp2 = Identifier::new_simple("hyp_gh2_eq_e".to_string());
    let p2_node = {
        let tactic = Tactic::SplitAssumptionConjunction {
            target_hypothesis: Identifier::new_simple("premise".to_string()),
            with_names: vec![hyp1.clone(), hyp2.clone()],
        };
        p1_node.apply_tactic(tactic, &mut proofs).primary_node()
    };

    // Step 3: Rewrite h1 on the LHS of the goal using the left identity axiom (h1 -> e * h1).
    // Goal: h1 = h2
    // New Goal: e * h1 = h2
    let p3_node = {
        let tactic = {
            if let Some(statement_arc) = p2_node.get_goal().statement.concrete_value() {
                if let MathRelation::Equal { left, .. } = statement_arc.as_ref() {
                    Tactic::Rewrite {
                        using_rule: RelationSource::Theorem(
                            "group_inverse_axiom".to_string(),
                            Some(1),
                        ),
                        target: Target::new(ContextOrStatement::Statement, left.id.clone()),
                        direction: RewriteDirection::Backward,
                    }
                } else {
                    panic!("p2 goal not an equality")
                }
            } else {
                panic!("p2 goal is not a statement")
            }
        };
        p2_node.apply_tactic(tactic, &mut proofs).primary_node()
    };

    // Step 4: Rewrite e on the LHS using the inverse property (e -> g⁻¹ * g).
    // Goal: e * h1 = h2
    // New Goal: (g⁻¹ * g) * h1 = h2
    let p4_node = {
        let tactic = {
            if let Some(statement_arc) = p3_node.get_goal().statement.concrete_value() {
                if let MathRelation::Equal { left, .. } = statement_arc.as_ref() {
                    Tactic::Rewrite {
                        using_rule: RelationSource::Theorem(
                            "group_inverse_axiom".to_string(),
                            Some(1),
                        ),
                        target: Target::new(ContextOrStatement::Statement, left.id.clone()),
                        direction: RewriteDirection::Backward,
                    }
                } else {
                    panic!("p3 goal not an equality")
                }
            } else {
                panic!("p3 goal is not a statement")
            }
        };
        p3_node.apply_tactic(tactic, &mut proofs).primary_node()
    };

    // Step 5: Apply associativity to regroup the expression on the LHS.
    // Goal: (g⁻¹ * g) * h1 = h2
    // New Goal: g⁻¹ * (g * h1) = h2
    let p5_node = {
        let tactic = {
            if let Some(statement_arc) = p4_node.get_goal().statement.concrete_value() {
                if let MathRelation::Equal { left, .. } = statement_arc.as_ref() {
                    Tactic::Rewrite {
                        using_rule: RelationSource::Theorem(
                            "group_associativity".to_string(),
                            None,
                        ),
                        target: Target::new(ContextOrStatement::Statement, left.id.clone()),
                        direction: RewriteDirection::Forward,
                    }
                } else {
                    panic!("p4 goal not an equality")
                }
            } else {
                panic!("p4 goal is not a statement")
            }
        };
        p4_node.apply_tactic(tactic, &mut proofs).primary_node()
    };

    // Step 6: Rewrite (g * h1) using the hypothesis `hyp_gh1_eq_e`.
    // Goal: g⁻¹ * (g * h1) = h2
    // New Goal: g⁻¹ * e = h2
    let p6_node = {
        let tactic = {
            if let Some(statement_arc) = p5_node.get_goal().statement.concrete_value() {
                if let MathRelation::Equal { left, .. } = statement_arc.as_ref() {
                    Tactic::Rewrite {
                        using_rule: RelationSource::LocalAssumption(hyp1.clone()),
                        target: Target::new(ContextOrStatement::Statement, left.id.clone()),
                        direction: RewriteDirection::Forward,
                    }
                } else {
                    panic!("p5 goal not an equality")
                }
            } else {
                panic!("p5 goal is not a statement")
            }
        };
        p5_node.apply_tactic(tactic, &mut proofs).primary_node()
    };

    // Step 7: Rewrite `e` to `g*h2` using hypothesis `hyp_gh2_eq_e`
    // Goal: g⁻¹ * e = h2
    // New Goal: g⁻¹ * (g * h2) = h2
    let p7_node = {
        let tactic = {
            if let Some(statement_arc) = p6_node.get_goal().statement.concrete_value() {
                if let MathRelation::Equal { left, .. } = statement_arc.as_ref() {
                    Tactic::Rewrite {
                        using_rule: RelationSource::LocalAssumption(hyp2.clone()),
                        target: Target::new(ContextOrStatement::Statement, left.id.clone()),
                        direction: RewriteDirection::Backward, // e -> g*h2
                    }
                } else {
                    panic!("p6 goal not an equality")
                }
            } else {
                panic!("p6 goal is not a statement")
            }
        };
        p6_node.apply_tactic(tactic, &mut proofs).primary_node()
    };

    // Step 8: Apply associativity to regroup the expression on the LHS.
    // Goal: g⁻¹ * (g * h2) = h2
    // New Goal: (g⁻¹ * g) * h2 = h2
    let p8_node = {
        let tactic = {
            if let Some(statement_arc) = p7_node.get_goal().statement.concrete_value() {
                if let MathRelation::Equal { left, .. } = statement_arc.as_ref() {
                    Tactic::Rewrite {
                        using_rule: RelationSource::Theorem(
                            "group_associativity".to_string(),
                            None,
                        ),
                        target: Target::new(ContextOrStatement::Statement, left.id.clone()),
                        direction: RewriteDirection::Forward,
                    }
                } else {
                    panic!("p7 goal not an equality")
                }
            } else {
                panic!("p7 goal is not a statement")
            }
        };
        p7_node.apply_tactic(tactic, &mut proofs).primary_node()
    };

    // Step 9: Rewrite (g⁻¹ * g) to e using the inverse property.
    // Goal: (g⁻¹ * g) * h2 = h2
    // New Goal: e * h2 = h2
    let p9_node = {
        let tactic = {
            if let Some(statement_arc) = p8_node.get_goal().statement.concrete_value() {
                if let MathRelation::Equal { left, .. } = statement_arc.as_ref() {
                    Tactic::Rewrite {
                        using_rule: RelationSource::Theorem(
                            "group_inverse_axiom".to_string(),
                            Some(0),
                        ),
                        target: Target::new(ContextOrStatement::Statement, left.id.clone()),
                        direction: RewriteDirection::Forward,
                    }
                } else {
                    panic!("p8 goal not an equality")
                }
            } else {
                panic!("p8 goal is not a statement")
            }
        };
        p8_node.apply_tactic(tactic, &mut proofs).primary_node()
    };

    // Step 10: Rewrite e * h2 to h2 using the identity property.
    // Goal: e * h2 = h2
    // New Goal: h2 = h2
    let p10_node = {
        let tactic = {
            if let Some(statement_arc) = p9_node.get_goal().statement.concrete_value() {
                if let MathRelation::Equal { left, .. } = statement_arc.as_ref() {
                    Tactic::Rewrite {
                        using_rule: RelationSource::Theorem(
                            "group_identity_axiom".to_string(),
                            Some(0),
                        ),
                        target: Target::new(ContextOrStatement::Statement, left.id.clone()),
                        direction: RewriteDirection::Forward,
                    }
                } else {
                    panic!("p9 goal not an equality")
                }
            } else {
                panic!("p9 goal is not a statement")
            }
        };
        p9_node.apply_tactic(tactic, &mut proofs).primary_node()
    };

    // Step 11: The goal is now h2 = h2, which is true by reflexivity.
    // Goal: h2 = h2
    // Proof Complete.
    let final_outcome = p10_node
        .apply_tactic(Tactic::ByReflexivity, &mut proofs)
        .primary_node()
        .should_complete();

    Theorem {
        id: "inverse_uniqueness".to_string(),
        name: "Inverse Uniqueness in a Group".to_string(),
        description: "Uniqueness of inverse: if g*h1=e and g*h2=e, then h1=h2".to_string(),
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use crate::subjects::math::formalism::automation::registry::get_theorem_registry;

    use super::*;

    #[test]
    fn test_prove_inverse_uniqueness() {
        std::thread::Builder::new()
            .stack_size(5000 * 1024)
            .spawn(|| {
                let theorem = prove_inverse_uniqueness();
                // Optional: Add assertions or prints for verification
            })
            .unwrap()
            .join()
            .unwrap();

        // let theorem = prove_inverse_uniqueness();
    }
}
