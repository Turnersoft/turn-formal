use crate::subjects::math::formalism::automation::registry::get_theorem_registry;
use crate::subjects::math::formalism::debug::ShortDebug;
use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::objects::MathObject;
use crate::subjects::math::formalism::proof::{
    ContextEntry, DefinitionState, ProofGoal, Tactic,
};
use crate::subjects::math::formalism::relations::{MathRelation, Quantification};
use crate::subjects::math::formalism::replace::{Instantiable, Replace, Substitutable};
use crate::subjects::math::formalism::search::Search;
use crate::subjects::math::formalism::theorem::Theorem;
use crate::subjects::math::theories::groups::definitions::{Group, GroupElement, GroupExpression};
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::number_theory::definitions::Number as TTNumber;
use crate::subjects::math::theories::zfc::definitions::{Set, SetProperty};
use crate::turn_render::{Identifier, MathNode, RichText, RichTextSegment};
use std::collections::HashMap;
use std::sync::Arc;

use super::{RelationSource, RewriteDirection, Target};

use std::thread;

#[derive(Clone)]
pub enum TacticApplicationResult {
    SingleGoal(ProofGoal),
    MultiGoal(Vec<ProofGoal>),
    ProofComplete,
    NoChange,
    Error(String),
}

impl std::fmt::Debug for TacticApplicationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TacticApplicationResult::SingleGoal(_) => {
                write!(f, "TacticApplicationResult::SingleGoal(...)")
            }
            TacticApplicationResult::MultiGoal(goals) => write!(
                f,
                "TacticApplicationResult::MultiGoal({} goals)",
                goals.len()
            ),
            TacticApplicationResult::ProofComplete => {
                write!(f, "TacticApplicationResult::ProofComplete")
            }
            TacticApplicationResult::NoChange => write!(f, "TacticApplicationResult::NoChange"),
            TacticApplicationResult::Error(msg) => {
                write!(f, "TacticApplicationResult::Error({})", msg)
            }
        }
    }
}

impl ProofGoal {
    pub fn find_relation_by_name(&self, name: &Identifier) -> Option<Located<MathRelation>> {
        self.context
            .iter()
            .find(|entry| &entry.name == name)
            .and_then(|entry| {
                if let Some(concrete_value) = entry.ty.concrete_value() {
                    if let MathExpression::Relation(rel) = concrete_value.as_ref() {
                        Some(Located::from_arc(rel.clone()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }

    pub fn is_name_used(&self, name: &Identifier) -> bool {
        self.context.iter().any(|entry| &entry.name == name)
    }
}

/// Core tactic application system - implements all tactic rules
impl Tactic {
    /// Apply a tactic to a proof goal and return the result
    pub fn apply_to_goal(&self, goal: &ProofGoal) -> TacticApplicationResult {
        match self {
            Tactic::AssumeImplicationAntecedent { with_name } => {
                Self::apply_assume_implication_antecedent(goal, with_name)
            }
            Tactic::SplitGoalConjunction => {
                if let Some(statement_arc) = goal.statement.concrete_value() {
                    if let MathRelation::And(conjuncts) = statement_arc.as_ref() {
                    let goals = conjuncts
                        .iter()
                        .map(|conjunct| {
                            let mut sub_goal = goal.clone();
                                if let Some(conjunct_arc) = conjunct.concrete_value() {
                                    sub_goal.statement = Located::from_arc(conjunct_arc.clone());
                                }
                            sub_goal
                        })
                            .collect::<Vec<_>>();
                    TacticApplicationResult::MultiGoal(goals)
                } else {
                    TacticApplicationResult::Error("Goal is not a conjunction.".to_string())
                    }
                } else {
                    TacticApplicationResult::Error("Goal statement is not a relation.".to_string())
                }
            }
            Tactic::SplitGoalDisjunction { disjunct_index } => {
                if let Some(statement_arc) = goal.statement.concrete_value() {
                    if let MathRelation::Or(disjuncts) = statement_arc.as_ref() {
                    if *disjunct_index < disjuncts.len() {
                        let mut new_goal = goal.clone();
                            if let Some(disjunct_arc) = disjuncts[*disjunct_index].concrete_value() {
                                new_goal.statement = Located::from_arc(disjunct_arc.clone());
                            }
                        TacticApplicationResult::SingleGoal(new_goal)
                    } else {
                        TacticApplicationResult::Error("Disjunct index out of bounds.".to_string())
                    }
                } else {
                    TacticApplicationResult::Error("Goal is not a disjunction.".to_string())
                    }
                } else {
                    TacticApplicationResult::Error("Goal statement is not a relation.".to_string())
                }
            }
            Tactic::CaseAnalysis { .. } => {
                TacticApplicationResult::Error("CaseAnalysis not implemented".to_string())
            }
            Tactic::Induction {
                variable_name,
                hypothesis_name,
            } => {
                let base_case_value = MathExpression::Number(TTNumber {});
                Self::apply_induction(
                    goal,
                    // Since we target the whole statement for induction, we pass the statement's ID
                    &goal.statement.id,
                    &base_case_value,
                    variable_name,
                    hypothesis_name,
                )
            }
            Tactic::ProvideWitness {
                target_quantifier,
                witness,
            } => Self::apply_provide_witness(goal, target_quantifier, witness),
            Tactic::SplitAssumptionConjunction {
                target_hypothesis,
                with_names,
            } => {
                if let Some(relation) = goal.find_relation_by_name(target_hypothesis) {
                    if let Some(relation_arc) = relation.concrete_value() {
                        if let MathRelation::And(conjuncts) = relation_arc.as_ref() {
                        if conjuncts.len() != with_names.len() {
                            return TacticApplicationResult::Error(
                                "Number of names for conjuncts does not match".to_string(),
                            );
                        }
                            let mut new_goal = goal.clone();
                        for (conjunct, name) in conjuncts.iter().zip(with_names.iter()) {
                                if let Some(conjunct_arc) = conjunct.concrete_value() {
                            let new_entry = ContextEntry {
                                name: name.clone(),
                                        ty: Located::new_concrete(MathExpression::Relation(
                                            conjunct_arc.clone(),
                                )),
                                definition: DefinitionState::Abstract,
                                description: Some(RichText::text(format!(
                                    "From splitting {}",
                                    target_hypothesis.to_string()
                                ))),
                            };
                            new_goal.context.push(new_entry);
                                }
                        }
                        TacticApplicationResult::SingleGoal(new_goal)
                    } else {
                        TacticApplicationResult::Error(
                            "Target hypothesis is not a conjunction.".to_string(),
                        )
                        }
                    } else {
                        TacticApplicationResult::Error("Target hypothesis is not a concrete relation.".to_string())
                    }
                } else {
                    TacticApplicationResult::Error("Target hypothesis not found.".to_string())
                }
            }
            Tactic::SplitAssumptionDisjunction { .. } => TacticApplicationResult::Error(
                "SplitAssumptionDisjunction not implemented".to_string(),
            ),

            Tactic::ByRelation(source) => Self::apply_exact_with(goal, source),

            Tactic::ByReflexivity => Self::apply_reflexivity(goal),
            Tactic::ByContradiction {
                hypothesis1,
                hypothesis2,
            } => {
                if let (Some(entry1), Some(entry2)) = (
                    goal.context.iter().find(|e| &e.name == hypothesis1),
                    goal.context.iter().find(|e| &e.name == hypothesis2),
                ) {
                    if Self::entries_contradict(entry1, entry2) {
                        TacticApplicationResult::ProofComplete
                    } else {
                        TacticApplicationResult::Error("Hypotheses do not contradict.".to_string())
                    }
                } else {
                    TacticApplicationResult::Error("One or both hypotheses not found.".to_string())
                }
            }
            Tactic::ByGoalContradiction {
                conflicting_hypothesis,
            } => {
                if let Some(entry) = goal
                    .context
                    .iter()
                    .find(|e| &e.name == conflicting_hypothesis)
                {
                    if let Some(goal_statement_arc) = goal.statement.concrete_value() {
                        let goal_relation_as_expr = Located::new_concrete(MathExpression::Relation(
                            goal_statement_arc.clone(),
                    ));
                    let contradictory_entry = ContextEntry {
                        name: Identifier::new_simple("goal".to_string()),
                        ty: goal_relation_as_expr,
                        definition: DefinitionState::Abstract,
                        description: None,
                    };
                    if Self::entries_contradict(entry, &contradictory_entry) {
                        TacticApplicationResult::ProofComplete
                    } else {
                        TacticApplicationResult::Error(
                            "Hypothesis does not contradict goal.".to_string(),
                        )
                        }
                    } else {
                        TacticApplicationResult::Error("Goal statement is not concrete.".to_string())
                    }
                } else {
                    TacticApplicationResult::Error("Hypothesis not found.".to_string())
                }
            }
            Tactic::Rewrite {
                using_rule,
                target,
                direction,
                instantiations,
            } => match using_rule {
                RelationSource::LocalAssumption(id) => {
                    Self::apply_rewrite_with_local_assumption(goal, target, id, direction, &instantiations)
                }
                RelationSource::Theorem(id, node_index) => {
                    Self::apply_rewrite_with_theorem(goal, target, id, *node_index, direction, &instantiations)
                }
            },
            Tactic::UnfoldDefinition {
                definition_to_unfold,
                target,
            } => Self::apply_unfold_definition(goal, target, definition_to_unfold),
            Tactic::IntroduceLetBinding {
                target_expression,
                with_name,
            } => Self::apply_let_binding(goal, target_expression, with_name),
            Tactic::RenameBoundVariable {
                target,
                from_name,
                to_name,
            } => todo!(),
            Tactic::Revert {
                hypothesis_to_revert,
            } => todo!(),
            Tactic::SearchAssumptions => todo!(),
            Tactic::SearchTheoremLibrary => todo!(),
            Tactic::Search => todo!(),
            Tactic::Simplify { target } => todo!(),
            Tactic::Auto {
                depth,
                with_tactics,
            } => todo!(),
            Tactic::DisproveByTheorem { theorem_id } => todo!(),
        }
    }

    fn apply_unfold_definition(
        goal: &ProofGoal,
        target: &Target,
        definition_to_unfold: &Identifier,
    ) -> TacticApplicationResult {
        if let Some(entry) = goal
            .context
            .iter()
            .find(|e| &e.name == definition_to_unfold)
        {
            if let DefinitionState::Inlined = &entry.definition {
                // The pattern is the variable itself.
                let pattern_expr = MathExpression::Object(Arc::new(MathObject::Set(Set::Parametric {
                    parameters: HashMap::new(),
                    description: entry.name.to_string(),
                    membership_condition: "".to_string(),
                    properties: VariantSet::new(),
                })));

                // The replacement is the body of the definition
                if let Some(replacement_arc) = entry.ty.concrete_value() {
                    let replacement = replacement_arc.as_ref();

                let mut new_goal = goal.clone();
                let target_clone = target.clone();

                // We need a dummy expression for the variable to create a valid pattern
                let dummy_var_expr =
                    MathExpression::Object(Arc::new(MathObject::Set(Set::Parametric {
                        parameters: HashMap::new(),
                        description: entry.name.to_string(),
                        membership_condition: "".to_string(),
                        properties: VariantSet::new(),
                    })));

                    if let Some(current_statement_arc) = new_goal.statement.concrete_value() {
                        let pattern_located = Located::new_concrete(dummy_var_expr.clone());
                        let replacement_located = Located::new_concrete(replacement.clone());
                        let new_statement = current_statement_arc.replace(
                    &new_goal.statement.id,
                    &target_clone.id,
                    &new_goal.context,
                    &pattern_located,
                    &replacement_located,
                    &goal.context,
                    &HashMap::new(), // No manual instantiations for unfold definition
                );
                        new_goal.statement = Located::new_concrete(new_statement);
                    }

                return TacticApplicationResult::SingleGoal(new_goal);
                } else {
                    return TacticApplicationResult::Error(
                        "Definition body is not concrete.".to_string(),
                    );
                }
            } else {
                return TacticApplicationResult::Error(  
                    "Definition is not inlined, cannot unfold.".to_string(),
                );
            }
        }
        todo!()
    }

    fn apply_expand_definition(
        goal: &ProofGoal,
        target: &Target,
        name: &Identifier,
    ) -> TacticApplicationResult {
        todo!()
    }

    fn apply_let_binding(
        goal: &ProofGoal,
        target_expression: &Target,
        with_name: &Identifier,
    ) -> TacticApplicationResult {
        let mut new_goal = goal.clone();
        if new_goal.is_name_used(with_name) {
            return TacticApplicationResult::Error(format!(
                "Name {} is already in use.",
                with_name
            ));
        }

        // The logic to extract the expression to bind needs to be more robust.
        // For now, we assume the `replace` function can find and substitute the target.
        // We need to know what the expression at the target is to create the binding.
        // This functionality seems to be missing from `Target`.
        // Let's assume for now we can't get the expression and we create a placeholder.
        // A proper implementation would require enhancing `Target` or the search mechanism.

        // This is a placeholder for the real expression that would be found at the target
        let expr_to_bind = MathExpression::Relation(Arc::new(MathRelation::True));

        let variable_expr = MathExpression::Object(Arc::new(MathObject::Set(Set::Parametric {
                    parameters: HashMap::new(),
            description: with_name.to_string(),
                    membership_condition: String::new(),
                    properties: VariantSet::new(),
        })));

        // This is not quite right. `replace` needs a pattern. In `let`, the pattern is the `target_expression` itself.
        // But the `replace` function takes a pattern and a replacement.
        // The current `replace` function is designed for `pattern = replacement` style rules.
        // A "let" is more like a local definition.

        // We will need to adjust the `replace` logic or use a different approach.
        // For now, I will leave this as an error, as the current tools are insufficient.

        todo!()
    }

    fn apply_provide_witness(
        goal: &ProofGoal,
        target_quantifier: &Identifier,
        witness: &MathExpression,
    ) -> TacticApplicationResult {
        // Find the target quantifier in the ordered list
        let quantifier_index = goal.quantifiers.iter().position(|q| &q.variable_name == target_quantifier);
        
        if let Some(index) = quantifier_index {
            let quantifier = &goal.quantifiers[index];
            
            // Ensure it's an existential quantifier
            if !matches!(quantifier.quantification, Quantification::Existential | Quantification::UniqueExistential) {
                return TacticApplicationResult::Error(format!(
                    "Cannot provide witness for non-existential quantifier: {:?}",
                    quantifier.quantification
                ));
            }
            
            let mut new_goal = goal.clone();
            
            // Remove the existential quantifier from the list
            new_goal.quantifiers.remove(index);
            
            
            // Substitute the witness in the statement
            if let Some(goal_statement_arc) = goal.statement.concrete_value() {
                // Create substitution map for the witness
                let located_witness = Located::new_concrete(witness.clone());

                let substitution_map = HashMap::from([(target_quantifier.clone(), located_witness.id.clone())]);

                let substituted_expr = goal_statement_arc.substitute(&substitution_map, &located_witness, &goal.context);
                
                new_goal.statement = Located::from_arc(substituted_expr);
            }
            
            // Also substitute in remaining quantifiers that might depend on this variable
            // This handles the dependency chain correctly
            for remaining_quantifier in &mut new_goal.quantifiers {
                // If remaining quantifiers reference the substituted variable in their constraints,
                // we would need to substitute there too. This would require extending the
                // Quantifier struct to include domain constraints.
            }
            
            // Add the witness as a concrete binding in the context
            let witness_entry = ContextEntry {
                name: target_quantifier.clone(),
                ty: Located::new_concrete(witness.clone()),
                definition: DefinitionState::Separate(Located::new_concrete(witness.clone())),
                description: Some(RichText::text(format!(
                    "Witness provided for existential quantifier"
                ))),
            };
            new_goal.context.push(witness_entry);
            
            TacticApplicationResult::SingleGoal(new_goal)
        } else {
            TacticApplicationResult::Error(format!(
                "Quantifier {} not found in goal", 
                target_quantifier
            ))
        }
    }

    fn apply_exact_with(
        goal: &ProofGoal,
        theorem_or_local_assumption: &RelationSource,
    ) -> TacticApplicationResult {
        let (theorem_statement, theorem_context) = match theorem_or_local_assumption {
            RelationSource::LocalAssumption(id) => {
                if let Some(relation) = goal.find_relation_by_name(id) {
                    (relation, goal.context.clone())
                } else {
                    return TacticApplicationResult::Error("Assumption not found".to_string());
                }
            }
            RelationSource::Theorem(id, node_index) => {
                let registry = get_theorem_registry();
                if let Some(theorem) = registry.get(id) {
                    (
                        theorem.proofs.initial_goal.statement.clone(),
                        theorem.proofs.initial_goal.context.clone(),
                    )
                } else {
                    return TacticApplicationResult::Error("Theorem not found".to_string());
                }
            }
        };

        let goal_expr = if let Some(goal_arc) = goal.statement.concrete_value() {
            MathExpression::Relation(goal_arc.clone())
        } else {
            return TacticApplicationResult::Error("Goal statement is not concrete".to_string());
        };
        
        let theorem_expr = if let Some(theorem_arc) = theorem_statement.concrete_value() {
            MathExpression::Relation(theorem_arc.clone())
        } else {
            return TacticApplicationResult::Error("Theorem statement is not concrete".to_string());
        };

        let instantiations =
            goal_expr.instantiate(&goal.context, &theorem_expr, &theorem_context);

        // Create a MathExpression wrapper for substitution
        let theorem_expr_wrapper = if let Some(theorem_arc) = theorem_statement.concrete_value() {
            MathExpression::Relation(theorem_arc.clone())
        } else {
            return TacticApplicationResult::Error("Theorem statement is not concrete".to_string());
        };
        
        // Create Located wrapper for the goal expression to use as target
        let goal_located = Located::new_concrete(goal_expr.clone());
        let instantiated_expr = theorem_expr_wrapper.substitute(&instantiations, &goal_located, &theorem_context);
        let instantiated_statement = if let MathExpression::Relation(rel) = instantiated_expr {
            Located::from_arc(rel)
        } else {
            return TacticApplicationResult::Error("Substitution returned non-relation".to_string());
        };
        
        if let (Some(instantiated_arc), Some(goal_arc)) = (
            instantiated_statement.concrete_value(),
            goal.statement.concrete_value()
        ) {
            if instantiated_arc == goal_arc {
            TacticApplicationResult::ProofComplete
        } else {
            TacticApplicationResult::Error(
                "Provided statement does not exactly match goal.".to_string(),
            )
            }
        } else {
            TacticApplicationResult::Error("Cannot compare non-concrete statements".to_string())
        }
    }

    fn apply_rewrite_with_local_assumption(
        goal: &ProofGoal,
        target: &Target,
        assumption_id: &Identifier,
        direction: &RewriteDirection,
        instantiations: &HashMap<Identifier, Identifier>,
    ) -> TacticApplicationResult {
        println!("DEBUG: LOCAL_ASSUMPTION - Looking for assumption: {}", assumption_id.body);
        println!("DEBUG: LOCAL_ASSUMPTION - Available assumptions in context:");
        for (i, entry) in goal.context.iter().enumerate() {
            println!("  [{}]: {}: {}", i, entry.name.body, entry.ty.short_debug());
        }
        
        if let Some(assumption) = goal.find_relation_by_name(assumption_id) {
            println!("DEBUG: LOCAL_ASSUMPTION - Found assumption: {}", assumption.short_debug());
            if let Some(goal_concrete) = goal.statement.concrete_value() {
                println!("DEBUG: LOCAL_ASSUMPTION - Current goal is: {:?}", goal_concrete);
            } else {
                println!("DEBUG: LOCAL_ASSUMPTION - Current goal: {}", goal.statement.short_debug());
            }
            // For a local assumption, there's no sub-indexing.
            if let Some(assumption_arc) = assumption.concrete_value() {
                Self::find_and_apply_rewrite(goal, target, direction, assumption_arc.as_ref(), &goal.context, None, instantiations)
            } else {
                TacticApplicationResult::Error("Assumption is not concrete".to_string())
            }
        } else {
            TacticApplicationResult::Error("Assumption not found".to_string())
        }
    }

    fn apply_rewrite_with_theorem(
        goal: &ProofGoal,
        target: &Target,
        theorem_id: &str,
        node_index: Option<usize>,
        direction: &RewriteDirection,
        instantiations: &HashMap<Identifier, Identifier>,
    ) -> TacticApplicationResult {
        let registry = get_theorem_registry();
        if let Some(theorem) = registry.get(theorem_id) {
            let theorem_statement = &theorem.proofs.initial_goal.statement;
            let theorem_context = &theorem.proofs.initial_goal.context;
            if let Some(theorem_arc) = theorem_statement.concrete_value() {
            Self::find_and_apply_rewrite(
                goal,
                target,
                direction,
                    theorem_arc.as_ref(),
                theorem_context,
                node_index,
                instantiations,
            )
            } else {
                TacticApplicationResult::Error("Theorem statement is not concrete".to_string())
            }
        } else {
            TacticApplicationResult::Error(format!(
                "Theorem {} not found or not an equality.",
                theorem_id
            ))
        }
    }

    /// Helper method to find and apply a rewrite using a specific rule statement.
    /// It is called by both `apply_rewrite_with_theorem` and `apply_rewrite_with_local_assumption`.
    fn find_and_apply_rewrite(
        goal: &ProofGoal,
        target: &Target,
        direction: &RewriteDirection,
        rule_statement: &MathRelation,
        rule_context: &Vec<ContextEntry>,
        node_index: Option<usize>,
        instantiations: &HashMap<Identifier, Identifier>,
    ) -> TacticApplicationResult {
        match &rule_statement {
            MathRelation::Equal { left, right } => {

                
                let (pattern_loc, replacement_loc) = if *direction == RewriteDirection::Forward {
                    (left, right)
                } else {
                    (right, left)
                };

                // ✅ FIXED: Keep Located wrapper instead of unwrapping
                // let pattern = pattern_loc.data.unwrap(rule_context);
                // let replacement = replacement_loc.data.unwrap(rule_context);

                // search if the pattern match the target expression
                
                let matches = goal.statement.data.unwrap(&goal.context).find_matches(
                    target.clone(),
                    goal.statement.id.clone(),
                    &goal.context,
                        pattern_loc,  // ✅ Pass Located<MathExpression>
                    rule_context,
                    false,
                );
                
                // ✅ FIXED: Allow multiple matches, only reject when no matches found
                if matches.len() == 0 {
                    return TacticApplicationResult::Error(format!(
                        "Pattern does not match target expression in goal. Found {} matches.",
                        matches.len()
                    ));
                }
                // Select the best match: prefer target.id if available, otherwise first match
                let selected_match = if matches.contains(&target.id) {
                    &target.id
                } else {
                    matches.iter().next().unwrap()
                };

                // Manual instantiations only (automatic instantiation happens in replace)
                let mut combined_instantiations = HashMap::new();
                
                // Add manual instantiations (they override automatic ones)
                for (theorem_var, goal_var) in instantiations {
                    combined_instantiations.insert(theorem_var.clone(), goal_var.to_string());
                }

                // replace the target expression with the replacement expression
                let mut new_goal = goal.clone();
                let new_statement = goal.statement.replace(
                    &new_goal.statement.id,
                        selected_match,
                    &new_goal.context,
                        pattern_loc,      // ✅ Pass Located<MathExpression>
                        replacement_loc,  // ✅ Pass Located<MathExpression>
                    rule_context,
                    &combined_instantiations.iter().map(|(k, v)| (k.clone(), Identifier::new_simple(v.clone()))).collect(),  // ✅ Pass combined instantiations
                );
                new_goal.statement = new_statement;

                TacticApplicationResult::SingleGoal(new_goal)
                // } else {
                //     TacticApplicationResult::Error("Goal statement is not concrete".to_string())
                // }
            }
            MathRelation::Implies(antecedent, consequent) => {
                if *direction == RewriteDirection::Backward {
                                return TacticApplicationResult::Error(
                        "Cannot rewrite backward with an implication.".to_string(),
                    );
                }

                // ✅ FIXED: Use Located wrappers for implications too
                // let pattern =
                //     MathExpression::Relation(Arc::new(antecedent.data.unwrap(rule_context).clone()));
                // let replacement =
                //     MathExpression::Relation(Arc::new(consequent.data.unwrap(rule_context).clone()));

                // Create Located wrappers for the relation expressions
                let pattern_loc = Located::new_concrete(MathExpression::Relation(
                    antecedent.concrete_value().unwrap().clone()
                ));
                let replacement_loc = Located::new_concrete(MathExpression::Relation(
                    consequent.concrete_value().unwrap().clone()
                ));

                // search if the pattern match the target expression
                if let Some(goal_statement_arc) = goal.statement.concrete_value() {
                    let matches = goal_statement_arc.find_matches(
                    target.clone(),
                    goal.statement.id.clone(),
                    &goal.context,
                        &pattern_loc,  // ✅ Pass Located<MathExpression>
                    rule_context,
                    false,
                );

                // ✅ FIXED: Allow multiple matches, only reject when no matches found
                if matches.len() == 0 {
                    return TacticApplicationResult::Error(format!(
                        "Pattern does not match target expression in goal. Found {} matches.",
                        matches.len()
                    ));
                }

                // replace the target expression with the replacement expression
                let mut new_goal = goal.clone();
                let new_statement = goal_statement_arc.replace(
                    &new_goal.statement.id,
                    &target.id,
                    &new_goal.context,
                    &pattern_loc,      // ✅ Pass Located<MathExpression>
                    &replacement_loc,  // ✅ Pass Located<MathExpression>
                    rule_context,
                    &HashMap::new(), // No manual instantiations for implications
                );
                    new_goal.statement = Located::new_concrete(new_statement);

                TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error("Goal statement is not concrete".to_string())
                }
            }
            MathRelation::And(conjuncts) => {
                if let Some(idx) = node_index {
                    if let Some(conjunct) = conjuncts.get(idx) {
                        // Clone is necessary because unwrap consumes the object.
                        let concrete_relation = conjunct.data.clone().unwrap(rule_context);
                        // Recursive call to handle the selected conjunct.
                        Self::find_and_apply_rewrite(
                            goal,
                            target,
                            direction,
                            &concrete_relation,
                            rule_context,
                            None,
                            instantiations,
                        )
                    } else {
                        TacticApplicationResult::Error(format!(
                            "Index {} is out of bounds for the conjunction.",
                            idx
                        ))
                            }
                        } else {
                    // No index provided, so try each conjunct in order.
                    for conjunct in conjuncts {
                        let concrete_relation = conjunct.data.clone().unwrap(rule_context);
                        let result = Self::find_and_apply_rewrite(
                            goal,
                            target,
                            direction,
                            &concrete_relation,
                            rule_context,
                            None, // Pass None since we are iterating.
                            instantiations,
                        );

                        // If the rewrite was successful (i.e., not an error), return it.
                        if !matches!(&result, TacticApplicationResult::Error(_)) {
                            return result;
                        }
                    }

                    // If the loop completes, no conjunct worked.
                    TacticApplicationResult::Error(
                        "A conjunction was used for rewriting, but no clause could be applied successfully to the target."
                            .to_string(),
                    )
                }
            }
            MathRelation::Equivalent(left, right) => {
                let (pattern_loc, replacement_loc) = if *direction == RewriteDirection::Forward {
                    (left, right)
                } else {
                    (right, left)
                };

                // ✅ FIXED: Use Located wrappers for equivalence too
                // let pattern =
                //     MathExpression::Relation(Arc::new(pattern_loc.data.unwrap(rule_context).clone()));
                // let replacement =
                //     MathExpression::Relation(Arc::new(replacement_loc.data.unwrap(rule_context).clone()));

                // Create Located wrappers for the relation expressions
                let pattern_located = Located::new_concrete(MathExpression::Relation(
                    pattern_loc.concrete_value().unwrap().clone()
                ));
                let replacement_located = Located::new_concrete(MathExpression::Relation(
                    replacement_loc.concrete_value().unwrap().clone()
                ));
 
                // search if the pattern match the target expression
                if let Some(goal_statement_arc) = goal.statement.concrete_value() {
                    let matches = goal_statement_arc.find_matches(
                    target.clone(),
                    goal.statement.id.clone(),
                    &goal.context,
                        &pattern_located,  // ✅ Pass Located<MathExpression>
                    rule_context,
                    false,
                );

                if matches.len() == 0 {
                    return TacticApplicationResult::Error(format!(
                        "Pattern does not match target expression in goal. Found {} matches.",
                        matches.len()
                    ));
                }

                // replace the target expression with the replacement expression
                    let mut new_goal = goal.clone();
                    let new_statement = goal_statement_arc.replace(
                    &new_goal.statement.id,
                        &target.id,
                    &new_goal.context,
                        &pattern_located,      // ✅ Pass Located<MathExpression>
                        &replacement_located,  // ✅ Pass Located<MathExpression>
                    rule_context,
                    &HashMap::new(), // No manual instantiations for equivalence
                );
                    new_goal.statement = Located::new_concrete(new_statement);

                TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error("Goal statement is not concrete".to_string())
                }
            }
            _ => TacticApplicationResult::Error(
                "The provided rule is not an equality, implication, equivalent, or a conjunction containing one."
                    .to_string(),
            ),
        }
    }

    fn apply_assume_implication_antecedent(
        goal: &ProofGoal,
        hypothesis_name: &Identifier,
    ) -> TacticApplicationResult {
        if let Some(statement_arc) = goal.statement.concrete_value() {
            if let MathRelation::Implies(antecedent, consequent) = statement_arc.as_ref() {
            let antecedent_ty = antecedent.data.clone().unwrap(&goal.context);
            let new_entry = ContextEntry {
                name: hypothesis_name.clone(),
                    ty: Located::new_concrete(MathExpression::Relation(Arc::new(antecedent_ty))),
                definition: DefinitionState::Abstract,
                description: Some(RichText::text("Result of assuming antecedent".to_string())),
            };
            let mut new_context = goal.context.clone();
            new_context.push(new_entry);
            let new_statement = consequent.data.unwrap(&new_context);
            let new_goal = ProofGoal {
                context: new_context,
                    quantifiers: goal.quantifiers.clone(),
                    statement: Located::new_concrete(new_statement),
            };
            TacticApplicationResult::SingleGoal(new_goal)
        } else {
            TacticApplicationResult::Error("Goal is not an implication.".to_string())
            }
        } else {
            TacticApplicationResult::Error("Goal statement is not concrete.".to_string())
        }
    }

    fn apply_simplify(goal: &ProofGoal, target: &Target) -> TacticApplicationResult {
        // This is a placeholder for a much more complex simplification engine.
        // let mut new_goal = goal.clone();
        // if let Some(located_relation) = target.locate_in_goal_mut(&mut new_goal) {
        //     located_relation.data = Self::apply_basic_simplifications(&located_relation.data);
        //     TacticApplicationResult::SingleGoal(new_goal)
        // } else {
        //     TacticApplicationResult::Error("Target for simplification not found.".to_string())
        // }
        todo!()
    }

    fn apply_reflexivity(goal: &ProofGoal) -> TacticApplicationResult {
        if let Some(statement_arc) = goal.statement.concrete_value() {
            if let MathRelation::Equal { left, right } = statement_arc.as_ref() {
            if left.data.unwrap(&goal.context) == right.data.unwrap(&goal.context) {
                TacticApplicationResult::ProofComplete
            } else {
                TacticApplicationResult::Error("Sides of equality are not identical.".to_string())
            }
        } else {
            TacticApplicationResult::Error("Goal is not an equality.".to_string())
            }
        } else {
            TacticApplicationResult::Error("Goal statement is not concrete.".to_string())
        }
    }

    fn apply_induction(
        goal: &ProofGoal,
        _target_relation_id: &String,
        base_case_value: &MathExpression,
        induction_variable_name: &Identifier,
        induction_hypothesis_name: &Identifier,
    ) -> TacticApplicationResult {
        // Base Case Goal
        let located_base_case = Located::new_concrete(base_case_value.clone());
        let substitution_map_base =
            HashMap::from([(induction_variable_name.clone(), located_base_case.id.clone())]);
        let mut base_case_goal = goal.clone();
        
        // Use MathExpression wrapper for substitution
        if let Some(goal_statement_arc) = goal.statement.concrete_value() {
            let goal_expr_wrapper = MathExpression::Relation(goal_statement_arc.clone());
            let base_case_expr = goal_expr_wrapper.substitute(&substitution_map_base, &located_base_case, &goal.context);
            if let MathExpression::Relation(rel) = base_case_expr {
                base_case_goal.statement = Located::from_arc(rel);
            }
        }

        // Inductive Step Goal
        let k_var = MathExpression::Object(Arc::new(MathObject::Set(Set::Parametric {
            parameters: HashMap::new(),
            description: "k".to_string(),
            membership_condition: String::new(),
            properties: VariantSet::new(),
        })));
        let k_plus_one_expr = MathExpression::Expression(TheoryExpression::Group(
            GroupExpression::Operation {
                group: Located::new_concrete(Group::new_generic()),
                left: Located::new_concrete(GroupExpression::Element {
                    group: Located::new_variable(induction_variable_name.clone()),
                    element: None,
                }),
                right: Located::new_concrete(GroupExpression::Element {
                    group: Located::new_concrete(Group::new_generic()),
                    element: Some(Located::new_concrete(GroupElement::Integer(1)))
                }),
            },
        ));
        let located_k_var = Located::new_concrete(k_var.clone());
        let substitution_map_k = HashMap::from([(induction_variable_name.clone(), located_k_var.id.clone())]);
        if let Some(goal_statement_arc) = goal.statement.concrete_value() {
            let induction_hypothesis_expr = MathExpression::Relation(goal_statement_arc.clone());
            let induction_hypothesis_result = induction_hypothesis_expr.substitute(&substitution_map_k, &located_k_var, &goal.context);

        let located_k_plus_one = Located::new_concrete(k_plus_one_expr.clone());
        let substitution_map_k_plus_1 =
            HashMap::from([(induction_variable_name.clone(), located_k_plus_one.id.clone())]);
        let mut inductive_step_goal = goal.clone();
            let inductive_step_expr = MathExpression::Relation(goal_statement_arc.clone());
            let inductive_step_result = inductive_step_expr.substitute(&substitution_map_k_plus_1, &located_k_plus_one, &goal.context);
            if let MathExpression::Relation(rel) = inductive_step_result {
                inductive_step_goal.statement = Located::from_arc(rel);
        }

        // Add the induction hypothesis to the context of the inductive step goal
            if let MathExpression::Relation(hyp_rel) = induction_hypothesis_result {
            inductive_step_goal.context.push(ContextEntry {
                name: induction_hypothesis_name.clone(),
                    ty: Located::new_concrete(MathExpression::Relation(hyp_rel)),
                definition: DefinitionState::Abstract,
                description: Some(RichText::text("Induction Hypothesis".to_string())),
            });
        }

        TacticApplicationResult::MultiGoal(vec![base_case_goal, inductive_step_goal])
        } else {
            TacticApplicationResult::Error("Goal statement is not concrete".to_string())
        }
    }

    fn apply_revert(goal: &ProofGoal, hypothesis_name: &Identifier) -> TacticApplicationResult {
        let mut new_goal = goal.clone();
        if let Some(index) = new_goal
            .context
            .iter()
            .position(|entry| &entry.name == hypothesis_name)
        {
            let entry = new_goal.context.remove(index);
            if let Some(ty_arc) = entry.ty.concrete_value() {
                if let MathExpression::Relation(relation) = ty_arc.as_ref() {
                    if let Some(current_statement_arc) = new_goal.statement.concrete_value() {
                let new_statement = MathRelation::Implies(
                            Located::from_arc(relation.clone()),
                            Located::from_arc(current_statement_arc.clone()),
                );
                        new_goal.statement = Located::new_concrete(new_statement);
                TacticApplicationResult::SingleGoal(new_goal)
                    } else {
                        TacticApplicationResult::Error("Goal statement is not concrete".to_string())
                    }
            } else {
                TacticApplicationResult::Error(
                    "Cannot revert a non-relation hypothesis".to_string(),
                )
                }
            } else {
                TacticApplicationResult::Error("Hypothesis type is not concrete".to_string())
            }
        } else {
            TacticApplicationResult::Error("Hypothesis not found".to_string())
    }
    }

    fn entries_contradict(entry1: &ContextEntry, entry2: &ContextEntry) -> bool {
        if let Some(ty1_arc) = entry1.ty.concrete_value() {
            if let MathExpression::Relation(rel1) = ty1_arc.as_ref() {
            if let MathRelation::Not(negated_rel1) = rel1.as_ref() {
                    if let Some(ty2_arc) = entry2.ty.concrete_value() {
                        if let MathExpression::Relation(rel2) = ty2_arc.as_ref() {
                    return negated_rel1.data.unwrap(&vec![]) == **rel2;
                }
            }
        }
            }
        }
        if let Some(ty2_arc) = entry2.ty.concrete_value() {
            if let MathExpression::Relation(rel2) = ty2_arc.as_ref() {
            if let MathRelation::Not(negated_rel2) = rel2.as_ref() {
                    if let Some(ty1_arc) = entry1.ty.concrete_value() {
                        if let MathExpression::Relation(rel1) = ty1_arc.as_ref() {
                    return negated_rel2.data.unwrap(&vec![]) == **rel1;
                        }
                    }
                }
            }
        }
        false
    }

    fn apply_basic_simplifications(relation: &MathRelation) -> MathRelation {
        // Placeholder for a real simplification engine
        relation.clone()
    }
}

// Helper trait to locate a sub-expression within a larger expression
trait LocateAndReplace {
    fn locate_and_replace<F>(&mut self, target_id: &str, replacer: F) -> bool
    where
        F: FnOnce(&mut Self);
}

impl LocateAndReplace for MathExpression {
    fn locate_and_replace<F>(&mut self, target_id: &str, replacer: F) -> bool
    where
        F: FnOnce(&mut Self),
    {
        // A full implementation requires traversing the expression tree
        false
    }
}

impl MathExpression {
    fn as_relation(&self) -> Option<MathRelation> {
        if let MathExpression::Relation(r) = self {
            Some((**r).clone())
        } else {
            None
        }
    }
}
