use crate::subjects::math::formalism::automation::registry::get_theorem_registry;
use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::proof::tactics::Tactic;
use crate::subjects::math::formalism::proof::tactics::implement::TacticApplicationResult;
use crate::subjects::math::formalism::proof::tactics::{RelationSource, RewriteDirection, Target};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::traits::search::Search;
use crate::turn_render::Identifier;
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::section_node::{
    AutomatedTacticDisplay, CaseDisplayNode, InstantiationPair, LinkTarget, MatchVerification,
    ProofDisplayNode, RewriteDirectionDisplay, RewriteStep, RichText, RichTextSegment,
    SectionContentNode, SimplificationStep, SubstitutionPreview, TacticDisplayNode,
    VariableBindingType,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Result structure for rewrite transformations with actual before/after states
struct RewriteTransformationResult {
    before_expression: MathNode,
    after_expression: MathNode,
    theorem_lhs: MathNode,
    theorem_rhs: MathNode,
    explanation: crate::turn_render::section_node::RichText,
    transformation_meaningful: bool,
    proof_completed: bool,
}

/// Proof state tracking for rendering
#[derive(Debug, Clone)]
pub struct ProofStateTracker {
    pub current_goal: Option<crate::subjects::math::formalism::proof::ProofGoal>,
    pub initial_goal: Option<crate::subjects::math::formalism::proof::ProofGoal>,
    pub completed: bool,
}

impl ProofStateTracker {
    pub fn new(initial_goal: crate::subjects::math::formalism::proof::ProofGoal) -> Self {
        Self {
            current_goal: Some(initial_goal.clone()),
            initial_goal: Some(initial_goal),
            completed: false,
        }
    }

    pub fn apply_tactic(&mut self, tactic: &Tactic) -> TacticApplicationResult {
        if let Some(current_goal) = &self.current_goal {
            let result = tactic.apply_to_goal(current_goal);

            match &result {
                TacticApplicationResult::SingleGoal(new_goal) => {
                    self.current_goal = Some(new_goal.clone());
                }
                TacticApplicationResult::ProofComplete => {
                    self.completed = true;
                    self.current_goal = None;
                }
                TacticApplicationResult::MultiGoal(goals) => {
                    // For multi-goal tactics, take the first goal as current state
                    // In a full system, we'd need to track multiple branches
                    if let Some(first_goal) = goals.first() {
                        self.current_goal = Some(first_goal.clone());
                    }
                }
                TacticApplicationResult::NoChange | TacticApplicationResult::Error(_) => {
                    // Keep current goal unchanged
                }
            }

            result
        } else {
            TacticApplicationResult::Error("No current goal to apply tactic to".to_string())
        }
    }
}

impl Tactic {
    /// Convert this tactic to a rich display node for frontend rendering
    pub fn to_display_node(&self) -> crate::turn_render::section_node::TacticDisplayNode {
        match self {
            Tactic::Rewrite {
                using_rule,
                target: at_target,
                direction,
                instantiations,
            } => {
                let (theorem_id, instantiation) = match using_rule {
                    RelationSource::LocalAssumption(id) => (id.to_string(), HashMap::new()),
                    RelationSource::Theorem(id, index) => (id.to_string(), HashMap::new()),
                };

                // Create a placeholder expression from the target string
                let target_expr = MathExpression::Relation(Arc::new(MathRelation::True));

                // Compute meaningful transformation
                let transformation = self.compute_rewrite_transformation(
                    &target_expr,
                    &theorem_id,
                    direction,
                    &instantiation,
                );

                let direction_display = match direction {
                    RewriteDirection::Forward => RewriteDirectionDisplay::LeftToRight {
                        left_side: transformation.theorem_lhs.clone(),
                        right_side: transformation.theorem_rhs.clone(),
                        explanation: RichText {
                            segments: vec![
                                RichTextSegment::Text("Result: ".to_string()),
                                RichTextSegment::Math(transformation.after_expression.clone()),
                            ],
                            alignment: None,
                        },
                    },
                    RewriteDirection::Backward => RewriteDirectionDisplay::RightToLeft {
                        left_side: transformation.theorem_rhs.clone(),
                        right_side: transformation.theorem_lhs.clone(),
                        explanation: RichText {
                            segments: vec![
                                RichTextSegment::Text("Result: ".to_string()),
                                RichTextSegment::Math(transformation.after_expression.clone()),
                            ],
                            alignment: None,
                        },
                    },
                };

                let instantiation_pairs = instantiation
                    .iter()
                    .map(|(var, expr)| InstantiationPair {
                        variable_name: RichText {
                            segments: vec![RichTextSegment::Text(var.to_string())],
                            alignment: None,
                        },
                        variable_value: expr.to_turn_math(format!("inst-{}", var.to_string())),
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
                    step_by_step_transformation: vec![],
                    theorem_link: Some(LinkTarget::TheoremId(theorem_id.clone())),
                }
            }

            Tactic::AssumeImplicationAntecedent { with_name } => {
                TacticDisplayNode::AssumeImplicationAntecedent {
                    implication_statement: MathNode {
                        id: "implication".to_string(),
                        content: Arc::new(MathNodeContent::Text("implication".to_string())),
                    },
                    hypothesis_name: RichText {
                        segments: vec![RichTextSegment::Math(MathNode {
                            id: "hyp-name".to_string(),
                            content: Arc::new(MathNodeContent::Identifier(with_name.clone())),
                        })],
                        alignment: None,
                    },
                    antecedent: MathNode {
                        id: "antecedent".to_string(),
                        content: Arc::new(MathNodeContent::Text("antecedent".to_string())),
                    },
                    consequent: MathNode {
                        id: "consequent".to_string(),
                        content: Arc::new(MathNodeContent::Text("consequent".to_string())),
                    },
                    context_explanation: RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "Assume antecedent as hypothesis '{}'",
                            with_name.to_string()
                        ))],
                        alignment: None,
                    },
                }
            }

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

            Tactic::ByReflexivity => TacticDisplayNode::Auto {
                automated_tactic_type: AutomatedTacticDisplay::Auto {
                    search_tree: None,
                    successful_tactics: vec![],
                    failed_attempts: vec![],
                },
                search_depth: Some(0),
                tactics_attempted: vec![],
                successful_path: None,
                execution_summary: RichText {
                    segments: vec![RichTextSegment::Text(
                        "Applied reflexivity (x = x)".to_string(),
                    )],
                    alignment: None,
                },
            },

            Tactic::IntroduceLetBinding {
                target_expression,
                with_name,
            } => TacticDisplayNode::IntroduceValueVariable {
                variable_name: RichText {
                    segments: vec![RichTextSegment::Text(with_name.to_string())],
                    alignment: None,
                },
                variable_value: MathNode {
                    id: "let-binding".to_string(),
                    content: Arc::new(MathNodeContent::Text(target_expression.id.clone())),
                },
                binding_type: VariableBindingType::Let,
                context_explanation: RichText {
                    segments: vec![RichTextSegment::Text(
                        "Let binding introduction".to_string(),
                    )],
                    alignment: None,
                },
                position: Some(0),
            },

            // Other tactics with placeholder display nodes
            _ => TacticDisplayNode::Auto {
                automated_tactic_type: AutomatedTacticDisplay::Auto {
                    search_tree: None,
                    successful_tactics: vec![],
                    failed_attempts: vec![],
                },
                search_depth: Some(0),
                tactics_attempted: vec![],
                successful_path: None,
                execution_summary: RichText {
                    segments: vec![RichTextSegment::Text(format!(
                        "Tactic not implemented for display: {:?}",
                        self
                    ))],
                    alignment: None,
                },
            },
        }
    }

    /// Compute meaningful rewrite transformation showing actual mathematical changes
    fn compute_rewrite_transformation(
        &self,
        target: &MathExpression,
        theorem_id: &str,
        direction: &RewriteDirection,
        instantiation: &HashMap<Identifier, MathExpression>,
    ) -> RewriteTransformationResult {
        // Get theorem information for meaningful display
        let (theorem_lhs, theorem_rhs) = self.get_theorem_sides(theorem_id, instantiation);

        // For the "after" expression, compute the actual mathematical result
        let after_expr = if theorem_id.starts_with("hyp_") {
            // For hypothesis, just show that it's applied
            MathNode {
                id: "after-expr".to_string(),
                content: Arc::new(MathNodeContent::Text(format!(
                    "Applied hypothesis: {}",
                    theorem_id
                ))),
            }
        } else {
            // For theorems, compute the actual result of applying the rewrite
            self.apply_rewrite_transformation(theorem_id, direction, target, instantiation)
        };

        RewriteTransformationResult {
            before_expression: target.to_turn_math("before-expr".to_string()),
            after_expression: after_expr.clone(),
            theorem_lhs,
            theorem_rhs,
            explanation: RichText {
                segments: vec![
                    RichTextSegment::Math(target.to_turn_math("before-expr".to_string())),
                    RichTextSegment::Text(" = ".to_string()),
                    RichTextSegment::Math(after_expr),
                    RichTextSegment::Text(format!(" (by {})", theorem_id)),
                ],
                alignment: None,
            },
            transformation_meaningful: true,
            proof_completed: false, // We don't know completion status at render time
        }
    }

    /// Apply the actual rewrite transformation and return the mathematical result
    fn apply_rewrite_transformation(
        &self,
        theorem_id: &str,
        direction: &RewriteDirection,
        target: &MathExpression,
        instantiation: &HashMap<Identifier, MathExpression>,
    ) -> MathNode {
        // Get the actual theorem from the registry
        let registry = get_theorem_registry();
        if let Some(theorem) = registry.get(theorem_id) {
            if let Some(concrete_rel) = theorem.proofs.initial_goal.statement.concrete_value() {
                if let MathRelation::Equal {
                    left: theorem_lhs,
                    right: theorem_rhs,
                    ..
                } = concrete_rel.as_ref()
                {
                    // Apply instantiation to theorem sides
                    let mut instantiated_lhs = theorem_lhs.clone();
                    let mut instantiated_rhs = theorem_rhs.clone();

                    // Apply variable substitutions from instantiation
                    for (var, value) in instantiation {
                        instantiated_lhs =
                            self.substitute_variable(&instantiated_lhs, &var.to_string(), value);
                        instantiated_rhs =
                            self.substitute_variable(&instantiated_rhs, &var.to_string(), value);
                    }

                    // Apply the rewrite based on direction
                    let result_expr = match direction {
                        RewriteDirection::Forward => {
                            // If target matches LHS, replace with RHS
                            if self.expressions_match(target, &instantiated_lhs.data) {
                                instantiated_rhs.data.clone()
                            } else {
                                // If target doesn't match exactly, show that we applied the theorem
                                return self.create_transformation_description(
                                    theorem_id, direction, target,
                                );
                            }
                        }
                        RewriteDirection::Backward => {
                            // If target matches RHS, replace with LHS
                            if self.expressions_match(target, &instantiated_rhs.data) {
                                instantiated_lhs.data.clone()
                            } else {
                                // If target doesn't match exactly, show that we applied the theorem
                                return self.create_transformation_description(
                                    theorem_id, direction, target,
                                );
                            }
                        }
                    };

                    return result_expr
                        .unwrap(&vec![])
                        .to_turn_math("after-expr".to_string());
                }
            }
        }

        // Fallback if theorem not found or not an equality
        self.create_transformation_description(theorem_id, direction, target)
    }

    /// Create a description of the transformation when exact matching fails
    fn create_transformation_description(
        &self,
        theorem_id: &str,
        direction: &RewriteDirection,
        _target: &MathExpression,
    ) -> MathNode {
        if theorem_id.starts_with("hyp_") {
            MathNode {
                id: "after-expr".to_string(),
                content: Arc::new(MathNodeContent::Text(format!(
                    "Applied hypothesis: {}",
                    theorem_id.replace("hyp_", "").replace("_", " ")
                ))),
            }
        } else {
            MathNode {
                id: "after-expr".to_string(),
                content: Arc::new(MathNodeContent::Text(format!(
                    "Applied {} ({})",
                    theorem_id,
                    match direction {
                        RewriteDirection::Forward => "left-to-right",
                        RewriteDirection::Backward => "right-to-left",
                    }
                ))),
            }
        }
    }

    /// Check if two expressions match (simple structural comparison)
    fn expressions_match(
        &self,
        expr1: &MathExpression,
        expr2: &Parametrizable<Arc<MathExpression>>,
    ) -> bool {
        // Simple structural comparison - could be enhanced with unification
        if let Parametrizable::Concrete(concrete_expr) = expr2 {
            expr1 == &**concrete_expr
        } else {
            false
        }
    }

    /// Substitute a variable in an expression with a value
    fn substitute_variable(
        &self,
        expr: &Located<MathExpression>,
        var: &str,
        value: &MathExpression,
    ) -> Located<MathExpression> {
        let mut new_expr = expr.clone();
        if let Some(var_id) = new_expr.variable_id() {
            if var_id.to_string() == var {
                new_expr = Located::new_concrete(value.clone());
            }
        }
        // This is a simplified substitution. A full implementation would recurse.
        new_expr
    }

    /// Substitute a variable in an Arc-wrapped expression with a value
    fn substitute_variable_arc(
        &self,
        expr: &Located<MathExpression>,
        var: &str,
        value: &MathExpression,
    ) -> Located<MathExpression> {
        let mut new_expr = expr.clone();
        if let Some(var_id) = new_expr.variable_id() {
            if var_id.to_string() == var {
                new_expr = Located::new_concrete(value.clone());
            }
        }
        // This is a simplified substitution. A full implementation would recurse.
        new_expr
    }

    fn get_theorem_sides(
        &self,
        theorem_id: &str,
        _instantiation: &HashMap<Identifier, MathExpression>,
    ) -> (MathNode, MathNode) {
        let registry = get_theorem_registry();
        if let Some(theorem) = registry.get(theorem_id) {
            if let Some(concrete_rel) = theorem.proofs.initial_goal.statement.concrete_value() {
                if let crate::subjects::math::formalism::relations::MathRelation::Equal {
                    left,
                    right,
                    ..
                } = concrete_rel.as_ref()
                {
                    // Apply instantiation
                    let lhs = left.clone();
                    let rhs = right.clone();

                    // TODO: Apply instantiation substitution
                    // For now, we'll show the theorem as-is for simplicity

                    return (
                        lhs.data
                            .unwrap(&vec![])
                            .to_turn_math("theorem-lhs".to_string()),
                        rhs.data
                            .unwrap(&vec![])
                            .to_turn_math("theorem-rhs".to_string()),
                    );
                }
            }
        }

        // Fallback - show more detail about why we failed to get theorem
        (
            MathNode {
                id: "theorem-lhs".to_string(),
                content: Arc::new(MathNodeContent::Text(format!(
                    "Failed to retrieve LHS of theorem '{}'",
                    theorem_id
                ))),
            },
            MathNode {
                id: "theorem-rhs".to_string(),
                content: Arc::new(MathNodeContent::Text(format!(
                    "Failed to retrieve RHS of theorem '{}'",
                    theorem_id
                ))),
            },
        )
    }
}
