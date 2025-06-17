use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::proof::tactics::search_replace::SearchReplace;
use crate::subjects::math::formalism::proof::{
    RewriteDirection, Tactic, get_theorem_registry, tactics::AutomatedTactic,
};
use crate::turn_render::Identifier;
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::section_node::{
    AutomatedTacticDisplay, CaseDisplayNode, InstantiationPair, LinkTarget, MatchVerification,
    ProofDisplayNode, RewriteDirectionDisplay, RewriteStep, RichText, RichTextSegment,
    SectionContentNode, SimplificationStep, SubstitutionPreview, TacticDisplayNode,
    VariableBindingType,
};
use std::collections::HashMap;

/// Simple result structure for rewrite transformations
struct RewriteTransformationResult {
    before_expression: MathNode,
    after_expression: MathNode,
    theorem_lhs: MathNode,
    theorem_rhs: MathNode,
    explanation: crate::turn_render::section_node::RichText,
    transformation_meaningful: bool,
}

impl Tactic {
    /// Convert this tactic to a rich display node for frontend rendering
    pub fn to_display_node(&self) -> crate::turn_render::section_node::TacticDisplayNode {
        match self {
            Tactic::ExactWith {
                theorem_id,
                instantiation,
            } => {
                let instantiation_pairs = instantiation
                    .iter()
                    .map(|(var, expr)| InstantiationPair {
                        variable_name: RichText {
                            segments: vec![RichTextSegment::Text(var.clone())],
                            alignment: None,
                        },
                        variable_value: expr.to_turn_math(format!("inst-{}", var)),
                        type_information: None,
                    })
                    .collect();

                TacticDisplayNode::ExactWith {
                    theorem_name: RichText {
                        segments: vec![RichTextSegment::Text(theorem_id.clone())],
                        alignment: None,
                    },
                    theorem_statement: RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "Theorem statement for {}",
                            theorem_id
                        ))],
                        alignment: None,
                    },
                    instantiation_mapping: instantiation_pairs,
                    match_verification: MatchVerification {
                        pattern: MathNode {
                            id: "theorem-pattern".to_string(),
                            content: Box::new(MathNodeContent::Text("theorem pattern".to_string())),
                        },
                        target: MathNode {
                            id: "current-goal".to_string(),
                            content: Box::new(MathNodeContent::Text("current goal".to_string())),
                        },
                        match_success: true,
                        match_explanation: RichText {
                            segments: vec![RichTextSegment::Text(
                                "Goal matches theorem exactly".to_string(),
                            )],
                            alignment: None,
                        },
                        unification_details: vec![],
                    },
                    theorem_link: Some(LinkTarget::TheoremId(theorem_id.clone())),
                }
            }

            Tactic::Rewrite {
                theorem_id,
                direction,
                target,
                instantiation,
            } => {
                // Compute meaningful transformation
                let transformation = self.compute_rewrite_transformation(
                    theorem_id,
                    direction,
                    target,
                    instantiation,
                );

                let direction_display = match direction {
                    RewriteDirection::LeftToRight => RewriteDirectionDisplay::LeftToRight {
                        left_side: transformation.theorem_lhs.clone(),
                        right_side: transformation.theorem_rhs.clone(),
                        explanation: RichText {
                            segments: vec![RichTextSegment::Text(
                                "Applying theorem left-to-right".to_string(),
                            )],
                            alignment: None,
                        },
                    },
                    RewriteDirection::RightToLeft => RewriteDirectionDisplay::RightToLeft {
                        left_side: transformation.theorem_lhs.clone(),
                        right_side: transformation.theorem_rhs.clone(),
                        explanation: RichText {
                            segments: vec![RichTextSegment::Text(
                                "Applying theorem right-to-left".to_string(),
                            )],
                            alignment: None,
                        },
                    },
                };

                let instantiation_pairs = instantiation
                    .iter()
                    .map(|(var, expr)| InstantiationPair {
                        variable_name: RichText {
                            segments: vec![RichTextSegment::Text(var.clone())],
                            alignment: None,
                        },
                        variable_value: expr.to_turn_math(format!("inst-{}", var)),
                        type_information: None,
                    })
                    .collect();

                TacticDisplayNode::Rewrite {
                    target_expression: transformation.before_expression.clone(),
                    theorem_name: RichText {
                        segments: vec![RichTextSegment::Text(theorem_id.clone())],
                        alignment: None,
                    },
                    theorem_rule: RichText {
                        segments: vec![RichTextSegment::Text(format!("Theorem: {}", theorem_id))],
                        alignment: None,
                    },
                    instantiation_mapping: instantiation_pairs,
                    direction: direction_display,
                    step_by_step_transformation: vec![RewriteStep {
                        step_number: 1,
                        before: transformation.before_expression,
                        after: transformation.after_expression,
                        rule_applied: RichText {
                            segments: vec![RichTextSegment::Text(theorem_id.clone())],
                            alignment: None,
                        },
                        explanation: transformation.explanation,
                        highlighted_region: if transformation.transformation_meaningful {
                            Some(vec![0])
                        } else {
                            None
                        },
                    }],
                    theorem_link: Some(LinkTarget::TheoremId(theorem_id.clone())),
                }
            }

            Tactic::AssumeImplicationAntecedent { hypothesis_name } => {
                TacticDisplayNode::AssumeImplicationAntecedent {
                    implication_statement: MathNode {
                        id: "implication".to_string(),
                        content: Box::new(MathNodeContent::String("P → Q".to_string())),
                    },
                    hypothesis_name: RichText {
                        segments: vec![RichTextSegment::Math(
                            hypothesis_name.to_turn_math("hypothesis-name".to_string()),
                        )],
                        alignment: None,
                    },
                    antecedent: MathNode {
                        id: "antecedent".to_string(),
                        content: Box::new(MathNodeContent::String("P".to_string())),
                    },
                    consequent: MathNode {
                        id: "consequent".to_string(),
                        content: Box::new(MathNodeContent::String("Q".to_string())),
                    },
                    context_explanation: RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "To prove P → Q, assume P as hypothesis and prove Q"
                        ))],
                        alignment: None,
                    },
                }
            }

            Tactic::IntroduceFreshVariable {
                target_quantifier,
                fresh_variable_name,
            } => TacticDisplayNode::IntroduceFreshVariable {
                target_quantifier: RichText {
                    segments: vec![
                        RichTextSegment::Text("∀".to_string()),
                        RichTextSegment::Math(
                            target_quantifier.to_turn_math("target-quantifier".to_string()),
                        ),
                    ],
                    alignment: None,
                },
                fresh_variable_name: RichText {
                    segments: vec![RichTextSegment::Math(
                        fresh_variable_name.to_turn_math("fresh-variable".to_string()),
                    )],
                    alignment: None,
                },
                explanation: RichText {
                    segments: vec![RichTextSegment::Text(
                        "Introduce arbitrary element for universal quantifier".to_string(),
                    )],
                    alignment: None,
                },
                mathematical_context: Some(RichText {
                    segments: vec![RichTextSegment::Text(
                        "Universal quantifier elimination".to_string(),
                    )],
                    alignment: None,
                }),
            },

            Tactic::ProvideWitness {
                target_quantifier,
                witness,
            } => TacticDisplayNode::ProvideWitness {
                target_quantifier: RichText {
                    segments: vec![
                        RichTextSegment::Text("∃".to_string()),
                        RichTextSegment::Math(
                            target_quantifier.to_turn_math("target-quantifier".to_string()),
                        ),
                    ],
                    alignment: None,
                },
                witness_expression: witness.to_turn_math("witness".to_string()),
                witness_explanation: RichText {
                    segments: vec![RichTextSegment::Text(
                        "Provide concrete witness for existential quantifier".to_string(),
                    )],
                    alignment: None,
                },
                verification_steps: vec![],
            },

            Tactic::IntroduceQuantifier { object, position } => {
                TacticDisplayNode::IntroduceQuantifier {
                    object_description: RichText {
                        segments: vec![RichTextSegment::Text(format!("{:?}", object))],
                        alignment: None,
                    },
                    position: *position,
                    before_state: None,
                    after_state: None,
                }
            }

            Tactic::IntroduceValueVariable { binding, position } => {
                TacticDisplayNode::IntroduceValueVariable {
                    variable_name: RichText {
                        segments: vec![RichTextSegment::Math(
                            binding.name.to_turn_math("variable-name".to_string()),
                        )],
                        alignment: None,
                    },
                    variable_value: binding.value.to_turn_math("variable-value".to_string()),
                    binding_type: VariableBindingType::Let, // Default binding type
                    context_explanation: RichText {
                        segments: vec![RichTextSegment::Text(
                            "Introduce value variable".to_string(),
                        )],
                        alignment: None,
                    },
                    position: *position,
                }
            }

            Tactic::Auto(auto_tactic) => TacticDisplayNode::Auto {
                automated_tactic_type: AutomatedTacticDisplay::Auto {
                    search_tree: None,
                    successful_tactics: vec![],
                    failed_attempts: vec![],
                },
                search_depth: Some(3),
                tactics_attempted: vec![],
                successful_path: None,
                execution_summary: RichText {
                    segments: vec![RichTextSegment::Text("Automated proof search".to_string())],
                    alignment: None,
                },
            },

            // Add all missing tactic variants with placeholder implementations
            Tactic::ReorderQuantifiers { new_order } => TacticDisplayNode::ReorderQuantifiers {
                original_order: vec![],
                new_order: new_order
                    .iter()
                    .map(|id| RichText {
                        segments: vec![RichTextSegment::Text(format!("{:?}", id))],
                        alignment: None,
                    })
                    .collect(),
                justification: RichText {
                    segments: vec![RichTextSegment::Text("Reorder quantifiers".to_string())],
                    alignment: None,
                },
            },

            Tactic::UniversalCaseAnalysis {
                target_quantifier,
                cases,
            } => TacticDisplayNode::UniversalCaseAnalysis {
                target_quantifier: RichText {
                    segments: vec![RichTextSegment::Text(format!("{:?}", target_quantifier))],
                    alignment: None,
                },
                cases: vec![], // TODO: Convert CaseCondition to CaseDisplayNode
                exhaustiveness_proof: Some(RichText {
                    segments: vec![RichTextSegment::Text("Cases are exhaustive".to_string())],
                    alignment: None,
                }),
            },

            Tactic::SubstituteValueVariable { target_variable } => {
                TacticDisplayNode::SubstituteValueVariable {
                    target_variable: RichText {
                        segments: vec![RichTextSegment::Text(format!("{:?}", target_variable))],
                        alignment: None,
                    },
                    substitution_preview: SubstitutionPreview {
                        before: MathNode {
                            id: "before".to_string(),
                            content: Box::new(MathNodeContent::Text("Before".to_string())),
                        },
                        after: MathNode {
                            id: "after".to_string(),
                            content: Box::new(MathNodeContent::Text(
                                "After substitution".to_string(),
                            )),
                        },
                        highlighted_changes: vec![],
                    },
                    justification: RichText {
                        segments: vec![RichTextSegment::Text(
                            "Substitute value variable".to_string(),
                        )],
                        alignment: None,
                    },
                }
            }

            Tactic::RewriteInValueBinding {
                target_variable,
                target_sub_expression,
                replacement,
                justification,
            } => TacticDisplayNode::RewriteInValueBinding {
                target_variable: RichText {
                    segments: vec![RichTextSegment::Text(format!("{:?}", target_variable))],
                    alignment: None,
                },
                target_subexpression: target_sub_expression.to_turn_math("target".to_string()),
                replacement: replacement.to_turn_math("replacement".to_string()),
                justification: vec![],
                step_by_step: vec![],
            },

            Tactic::RemoveValueVariable { target_variable } => {
                TacticDisplayNode::RemoveValueVariable {
                    target_variable: RichText {
                        segments: vec![RichTextSegment::Text(format!("{:?}", target_variable))],
                        alignment: None,
                    },
                    reason: RichText {
                        segments: vec![RichTextSegment::Text(
                            "Variable no longer needed".to_string(),
                        )],
                        alignment: None,
                    },
                    cleanup_explanation: None,
                }
            }

            Tactic::SplitConjunction { target, index } => TacticDisplayNode::SplitConjunction {
                target_conjunction: MathNode {
                    id: "conjunction".to_string(),
                    content: Box::new(MathNodeContent::Text("A ∧ B".to_string())),
                },
                conjuncts: vec![],
                selected_index: *index,
                remaining_goals: vec![],
            },

            Tactic::SplitDisjunction { target, index } => TacticDisplayNode::SplitDisjunction {
                target_disjunction: MathNode {
                    id: "disjunction".to_string(),
                    content: Box::new(MathNodeContent::Text("A ∨ B".to_string())),
                },
                disjuncts: vec![],
                chosen_index: *index,
                chosen_disjunct: MathNode {
                    id: "chosen".to_string(),
                    content: Box::new(MathNodeContent::Text("Chosen disjunct".to_string())),
                },
                strategy_explanation: RichText {
                    segments: vec![RichTextSegment::Text("Split disjunction".to_string())],
                    alignment: None,
                },
            },

            Tactic::StatementCaseAnalysis { target, cases } => {
                TacticDisplayNode::StatementCaseAnalysis {
                    target_expression: target.to_turn_math("target".to_string()),
                    cases: vec![], // TODO: Convert cases
                    exhaustiveness_proof: None,
                }
            }

            Tactic::Simplify {
                target_path,
                original_expr,
                simplified_expr,
            } => TacticDisplayNode::Simplify {
                target_path: target_path.clone(),
                original_expression: original_expr.to_turn_math("original".to_string()),
                simplified_expression: simplified_expr.to_turn_math("simplified".to_string()),
                simplification_steps: vec![],
                rules_used: vec![],
            },

            Tactic::Induction {
                target_relation_path,
                base_case_value,
                induction_variable_name,
                induction_hypothesis_name,
            } => TacticDisplayNode::Induction {
                target_relation: MathNode {
                    id: "target-relation".to_string(),
                    content: Box::new(MathNodeContent::Text("Target relation".to_string())),
                },
                induction_variable: RichText {
                    segments: vec![RichTextSegment::Text(format!(
                        "{:?}",
                        induction_variable_name
                    ))],
                    alignment: None,
                },
                base_case_value: base_case_value.to_turn_math("base".to_string()),
                base_case_proof: ProofDisplayNode {
                    title: Some(RichText {
                        segments: vec![RichTextSegment::Text("Base case".to_string())],
                        alignment: None,
                    }),
                    strategy: vec![],
                    steps: vec![],
                    qed_symbol: Some("□".to_string()),
                },
                inductive_hypothesis: RichText {
                    segments: vec![RichTextSegment::Text(format!(
                        "Assume {:?}",
                        induction_hypothesis_name
                    ))],
                    alignment: None,
                },
                inductive_step_proof: ProofDisplayNode {
                    title: Some(RichText {
                        segments: vec![RichTextSegment::Text("Inductive step".to_string())],
                        alignment: None,
                    }),
                    strategy: vec![],
                    steps: vec![],
                    qed_symbol: Some("□".to_string()),
                },
                induction_principle: RichText {
                    segments: vec![RichTextSegment::Text("Mathematical induction".to_string())],
                    alignment: None,
                },
            },
        }
    }

    /// Compute meaningful rewrite transformation showing actual mathematical changes
    fn compute_rewrite_transformation(
        &self,
        theorem_id: &str,
        direction: &RewriteDirection,
        target: &MathExpression,
        instantiation: &HashMap<String, MathExpression>,
    ) -> RewriteTransformationResult {
        let registry = get_theorem_registry().lock().unwrap();

        if theorem_id.starts_with("hyp_") {
            // For hypothesis equations, show the hypothesis being applied
            return RewriteTransformationResult {
                before_expression: target.to_turn_math("before-expr".to_string()),
                after_expression: MathNode {
                    id: "hyp-result".to_string(),
                    content: Box::new(MathNodeContent::Text(format!(
                        "Applied hypothesis: {}",
                        theorem_id
                    ))),
                },
                theorem_lhs: MathNode {
                    id: "hyp-lhs".to_string(),
                    content: Box::new(MathNodeContent::Text("Hypothesis LHS".to_string())),
                },
                theorem_rhs: MathNode {
                    id: "hyp-rhs".to_string(),
                    content: Box::new(MathNodeContent::Text("Hypothesis RHS".to_string())),
                },
                explanation: crate::turn_render::section_node::RichText {
                    segments: vec![crate::turn_render::section_node::RichTextSegment::Text(
                        format!(
                            "Applied hypothesis {} to rewrite the expression",
                            theorem_id
                        ),
                    )],
                    alignment: None,
                },
                transformation_meaningful: true,
            };
        }

        if let Some(theorem) = registry.get_theorem(theorem_id) {
            if let crate::subjects::math::formalism::relations::MathRelation::Equal {
                left,
                right,
                ..
            } = &theorem.proofs.initial_goal.statement
            {
                // Convert instantiation to the format needed by SearchReplace
                let mut ident_instantiation = HashMap::new();
                for (k, v) in instantiation.iter() {
                    ident_instantiation.insert(Identifier::new_simple(k.clone()), v.clone());
                }

                // Determine the to_replace and replacement expressions based on direction
                let (to_replace, replacement) = match direction {
                    RewriteDirection::LeftToRight => (left.clone(), right.clone()),
                    RewriteDirection::RightToLeft => (right.clone(), left.clone()),
                };

                // Instantiate the theorem expressions with the provided substitutions
                let instantiated_to_replace =
                    SearchReplace::substitute_variables(&to_replace, &ident_instantiation);
                let instantiated_replacement =
                    SearchReplace::substitute_variables(&replacement, &ident_instantiation);

                // For meaningful transformations, we need to show what the target actually represents
                // If target is just a variable, show the theorem transformation directly
                let (meaningful_before, meaningful_after) = if let MathExpression::Var(_) = target {
                    // For variables like h1, show the actual theorem transformation
                    // E.g., for group_identity_left: x -> e * x (right-to-left)
                    match direction {
                        RewriteDirection::LeftToRight => (
                            instantiated_to_replace.clone(),
                            instantiated_replacement.clone(),
                        ),
                        RewriteDirection::RightToLeft => (
                            instantiated_replacement.clone(),
                            instantiated_to_replace.clone(),
                        ),
                    }
                } else {
                    // For complex expressions, perform the actual replacement
                    let result_expr = SearchReplace::replace_all_in_expression(
                        target,
                        &instantiated_to_replace,
                        &instantiated_replacement,
                    );
                    (target.clone(), result_expr)
                };

                // Check if the transformation is meaningful (not just x -> x)
                let transformation_meaningful = meaningful_before != meaningful_after;

                return RewriteTransformationResult {
                    before_expression: meaningful_before.to_turn_math("before-expr".to_string()),
                    after_expression: meaningful_after.to_turn_math("after-expr".to_string()),
                    theorem_lhs: instantiated_to_replace.to_turn_math("theorem-lhs".to_string()),
                    theorem_rhs: instantiated_replacement.to_turn_math("theorem-rhs".to_string()),
                    explanation: crate::turn_render::section_node::RichText {
                        segments: vec![crate::turn_render::section_node::RichTextSegment::Text(
                            format!(
                                "Applied theorem '{}' {} to transform expression",
                                theorem_id,
                                match direction {
                                    RewriteDirection::LeftToRight => "left-to-right",
                                    RewriteDirection::RightToLeft => "right-to-left",
                                }
                            ),
                        )],
                        alignment: None,
                    },
                    transformation_meaningful,
                };
            }
        }

        // Fallback for theorem not found or not an equality
        RewriteTransformationResult {
            before_expression: target.to_turn_math("before-expr".to_string()),
            after_expression: MathNode {
                id: "theorem-not-found".to_string(),
                content: Box::new(MathNodeContent::Text(format!(
                    "THEOREM_NOT_FOUND: {}",
                    theorem_id
                ))),
            },
            theorem_lhs: MathNode {
                id: "lhs".to_string(),
                content: Box::new(MathNodeContent::Text("LHS".to_string())),
            },
            theorem_rhs: MathNode {
                id: "rhs".to_string(),
                content: Box::new(MathNodeContent::Text("RHS".to_string())),
            },
            explanation: crate::turn_render::section_node::RichText {
                segments: vec![crate::turn_render::section_node::RichTextSegment::Text(
                    format!("Could not apply theorem '{}'", theorem_id),
                )],
                alignment: None,
            },
            transformation_meaningful: false,
        }
    }
}
