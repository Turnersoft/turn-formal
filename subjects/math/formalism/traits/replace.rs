use crate::subjects::math::formalism::proof::ContextEntryVecExt;
use crate::subjects::math::theories::groups::definitions::{
    GenericGroup, Group, GroupExpression, GroupHomomorphism, GroupOperation,
};
use crate::subjects::math::theories::rings::definitions::Ring;
use crate::subjects::math::theories::zfc::definitions::Set;
use crate::{
    subjects::math::formalism::{
        expressions::{MathExpression, TheoryExpression},
        extract::Parametrizable,
        interpretation::TypeViewOperator,
        location::Located,
        objects::MathObject,
        proof::{ContextEntry, tactics::Target},
        relations::MathRelation,
        traits::{
            collect_identifier::CollectIdentifier, debug::ShortDebug, detag::TryDetag,
            instantiable::Instantiable, search::Search, substitutable::Substitutable,
        },
    },
    turn_render::Identifier,
};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

use super::IsCompatible;
use super::instantiable::InstantiationType;

pub trait Replace: Sized {
    // ✅ SIMPLIFIED: Remove target parameter since it's redundant
    fn replace(
        &self,
        current_id: &str,
        target_id: &str,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        replacement: &Located<MathExpression>,
        pattern_and_replacement_context: &Vec<ContextEntry>,
        manual_instantiations: &HashMap<Identifier, Identifier>,
    ) -> Self;
}

// ✅ FIXED: Implement Replace for Located<T> using pattern matching like Search
impl<T: Replace + Clone + 'static + Debug + Search + Instantiable> Replace for Located<T> {
    fn replace(
        &self,
        current_id: &str,
        target_id: &str,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        replacement: &Located<MathExpression>,
        pattern_and_replacement_context: &Vec<ContextEntry>,
        manual_instantiations: &HashMap<Identifier, Identifier>,
    ) -> Self {
        match (&self.data, &pattern.data) {
            // Case 1: Self is concrete, pattern is variable - potential replacement target
            (_, Parametrizable::Variable(pattern_var)) => {
                if current_id == target_id {
                    // ✅ FOUND TARGET: Replace with instantiated replacement
                    let mut instantiations: HashMap<Identifier, InstantiationType> = HashMap::new();

                    match &self.data {
                        Parametrizable::Variable(id) => {
                            instantiations.insert(
                                pattern_var.clone(),
                                InstantiationType::Identifier(id.clone()),
                            );
                        }
                        Parametrizable::Concrete(_) => {
                            instantiations.insert(
                                pattern_var.clone(),
                                InstantiationType::LocatedId(current_id.to_string()),
                            );
                        }
                    };

                    // Add manual instantiations, it overwrite the instantiation found in action.
                    for (theorem_var, goal_var) in manual_instantiations {
                        instantiations.insert(
                            theorem_var.clone(),
                            InstantiationType::Identifier(goal_var.clone()),
                        );
                    }

                    find_missing_var_in_replacement_and_extend_instantiations(
                        target_context,
                        replacement,
                        pattern_and_replacement_context,
                        &mut instantiations,
                    );
                    // todo: perform substitution on replacement in one .substitute() call.
                    let substituted_replacement =
                        replacement.substitute(&instantiations, self, target_context);
                    // Convert Located<MathExpression> to Located<T> using the existing try_detag
                    match TryDetag::<T>::try_detag(
                        &substituted_replacement.data.unwrap(target_context),
                    ) {
                        Ok(inner_t) => {
                            // We got a T directly, wrap it in a new Located<T>
                            Located {
                                id: substituted_replacement.id.clone(),
                                data: Parametrizable::Concrete(Arc::new(inner_t.clone())),
                            }
                        }
                        Err(e) => {
                            println!("DEBUG: Failed to convert substituted replacement: {}", e);
                            self.clone()
                        }
                    }

                    // return self.clone();
                } else {
                    // Not target - recurse into concrete content
                    match &self.data {
                        Parametrizable::Concrete(concrete) => {
                            let new_concrete = concrete.replace(
                                current_id,
                                target_id,
                                target_context,
                                pattern,
                                replacement,
                                pattern_and_replacement_context,
                                manual_instantiations,
                            );
                            Located {
                                id: self.id.clone(),
                                data: Parametrizable::Concrete(Arc::new(new_concrete)),
                            }
                        }
                        Parametrizable::Variable(_) => self.clone(),
                    }
                }
            }

            // Case 2: Both concrete - recurse into inner content
            (
                Parametrizable::Concrete(self_concrete),
                Parametrizable::Concrete(pattern_concrete),
            ) => {
                if current_id == target_id {
                    // Try to get automatic instantiations for MathExpression
                    // instantiations.extend(self_concrete.instantiate(
                    //     target_context,
                    //     pattern_concrete,
                    //     pattern_and_replacement_context,
                    // ));
                    // todo: perform instantiation on pattern vs target
                    match TryDetag::<T>::try_detag(pattern_concrete.as_ref()) {
                        Ok(pattern_concrete) => {
                            let mut instantiations = self_concrete.as_ref().instantiate(
                                target_context,
                                pattern_concrete,
                                pattern_and_replacement_context,
                            );
                            // todo: check if the instantiations has all meta-variables in replacement, if not panic
                            find_missing_var_in_replacement_and_extend_instantiations(
                                target_context,
                                replacement,
                                pattern_and_replacement_context,
                                &mut instantiations,
                            );
                            // todo: perform substitution on replacement in one .substitute() call.
                            let substituted_replacement =
                                replacement.substitute(&instantiations, self, target_context);
                            // todo: Convert Located<MathExpression> to Located<T> using the existing try_detag
                            match TryDetag::<T>::try_detag(
                                &substituted_replacement.data.unwrap(target_context),
                            ) {
                                Ok(inner_t) => {
                                    // We got a T directly, wrap it in a new Located<T>
                                    match substituted_replacement.data {
                                        Parametrizable::Concrete(concrete) => Located {
                                            id: substituted_replacement.id.clone(),
                                            data: Parametrizable::Concrete(Arc::new(
                                                inner_t.clone(),
                                            )),
                                        },
                                        Parametrizable::Variable(variable) => Located {
                                            id: substituted_replacement.id.clone(),
                                            data: Parametrizable::Variable(variable.clone()),
                                        },
                                    }
                                    // Located {
                                    //     id: substituted_replacement.id.clone(),
                                    //     data: Parametrizable::Concrete(Arc::new(inner_t.clone())),
                                    // }
                                }
                                Err(e) => {
                                    println!(
                                        "DEBUG: Failed to convert substituted replacement: {}",
                                        e
                                    );
                                    self.clone()
                                }
                            }
                        }
                        Err(e) => {
                            panic!(
                                "Failed to convert pattern concrete: {:#?} to {}",
                                pattern_concrete.as_ref(),
                                std::any::type_name::<T>()
                            );
                            self.clone()
                        }
                    }

                    // return self.clone();
                } else {
                    let new_concrete = self_concrete.replace(
                        current_id,
                        target_id,
                        target_context,
                        pattern,
                        replacement,
                        pattern_and_replacement_context,
                        manual_instantiations,
                    );
                    Located {
                        id: self.id.clone(),
                        data: Parametrizable::Concrete(Arc::new(new_concrete)),
                    }
                }
            }

            // Case 3: Self is variable, pattern is concrete - no structural match possible
            (Parametrizable::Variable(_), Parametrizable::Concrete(_)) => self.clone(),
        }
    }
}

fn find_missing_var_in_replacement_and_extend_instantiations(
    target_context: &Vec<ContextEntry>,
    replacement: &Located<MathExpression>,
    pattern_and_replacement_context: &Vec<ContextEntry>,
    instantiations: &mut HashMap<Identifier, InstantiationType>,
) {
    // todo: check if the instantiations has all meta-variables in replacement,
    let replacement_meta_vars = replacement.collect_identifier();
    let missing_vars: Vec<Identifier> = replacement_meta_vars
        .iter()
        .filter_map(|var| {
            if !instantiations.contains_key(var) {
                // todo: try to find replacement in the context.
                // but be strict the name must be identical and type compatible.
                // of it is the only variable that is compatible with this identifier
                if let Some(entry) = target_context.find_variable(var) {
                    if let Some(pattern_entry) = pattern_and_replacement_context.find_variable(var)
                    {
                        if entry.ty.is_compatible(
                            target_context,
                            &pattern_entry.ty,
                            pattern_and_replacement_context,
                        ) {
                            // todo: Types are compatible, we can use this variable in the instantiation
                            instantiations.insert(
                                var.clone(),
                                InstantiationType::Identifier(entry.name.clone()),
                            );
                            None // Not missing anymore
                        } else {
                            // Name matches but type is incompatible
                            Some(var.clone())
                        }
                    } else {
                        // Variable not found in pattern context, but found in target context
                        // This might be a local variable that can be used
                        None // Assume it's available
                    }
                } else {
                    // Variable not found in target context at all
                    Some(var.clone())
                }
            } else {
                None
            }
        })
        .collect();
    // todo: if not: try to find replacement in the context. but be strict the name must be identical and type compatible.

    if !missing_vars.is_empty() {
        panic!(
            "Missing instantiations for meta-variables in replacement: {:?}",
            missing_vars
        );
    }
}

impl Replace for MathExpression {
    fn replace(
        &self,
        current_id: &str,
        target_id: &str,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        replacement: &Located<MathExpression>,
        pattern_and_replacement_context: &Vec<ContextEntry>,
        manual_instantiations: &HashMap<Identifier, Identifier>,
    ) -> Self {
        if current_id == target_id {
            // Extract concrete pattern for instantiation
            let concrete_pattern = match pattern.concrete_value() {
                Some(concrete) => concrete.as_ref(),
                None => return self.clone(), // Can't instantiate with variable pattern yet
            };
            let instantiations = self.instantiate(
                target_context,
                concrete_pattern,
                pattern_and_replacement_context,
            );
            let target_located = Located::new_concrete(self.clone());
            let concrete_replacement = match replacement.concrete_value() {
                Some(concrete) => concrete.as_ref(),
                None => return self.clone(), // Can't substitute with variable replacement yet
            };
            return concrete_replacement.substitute(
                &instantiations,
                &target_located,
                pattern_and_replacement_context,
            );
        }

        match self {
            MathExpression::Relation(rel) => MathExpression::Relation(Arc::new(rel.replace(
                current_id,
                target_id,
                target_context,
                pattern,
                replacement,
                pattern_and_replacement_context,
                manual_instantiations,
            ))),
            MathExpression::Object(obj) => MathExpression::Object(Arc::new(obj.replace(
                current_id,
                target_id,
                target_context,
                pattern,
                replacement,
                pattern_and_replacement_context,
                manual_instantiations,
            ))),
            MathExpression::Expression(expr) => MathExpression::Expression(expr.replace(
                current_id,
                target_id,
                target_context,
                pattern,
                replacement,
                pattern_and_replacement_context,
                manual_instantiations,
            )),
            _ => self.clone(),
        }
    }
}

// We implement the container traits by delegating to the main `MathExpression` implementation.
// This avoids code duplication.

impl Replace for MathObject {
    fn replace(
        &self,
        current_id: &str,
        target_id: &str,

        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        replacement: &Located<MathExpression>,
        pattern_and_replacement_context: &Vec<ContextEntry>,
        manual_instantiations: &HashMap<Identifier, Identifier>,
    ) -> Self {
        let expr_wrapper = MathExpression::Object(Arc::new(self.clone()));
        let result_expr = expr_wrapper.replace(
            current_id,
            target_id,
            target_context,
            pattern,
            replacement,
            pattern_and_replacement_context,
            manual_instantiations,
        );
        if let MathExpression::Object(new_obj) = result_expr {
            (*new_obj).clone()
        } else {
            self.clone() // Fallback if the replacement is not an object, which indicates a rule logic error.
        }
    }
}

impl Replace for MathRelation {
    fn replace(
        &self,
        current_id: &str,
        target_id: &str,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        replacement: &Located<MathExpression>,
        pattern_and_replacement_context: &Vec<ContextEntry>,
        manual_instantiations: &HashMap<Identifier, Identifier>,
    ) -> Self {
        // If this node is the target, we perform the replacement at the MathExpression level.
        if current_id == target_id {
            // Extract concrete pattern for instantiation
            let concrete_pattern = match pattern.concrete_value() {
                Some(concrete) => concrete.as_ref(),
                None => return self.clone(), // Can't instantiate with variable pattern yet
            };
            let target_expr = MathExpression::Relation(Arc::new(self.clone()));
            let instantiations = target_expr.instantiate(
                target_context,
                concrete_pattern,
                pattern_and_replacement_context,
            );
            let target_located = Located::new_concrete(target_expr);
            let concrete_replacement = match replacement.concrete_value() {
                Some(concrete) => concrete.as_ref(),
                None => return self.clone(), // Can't substitute with variable replacement yet
            };
            let result_expr = concrete_replacement.substitute(
                &instantiations,
                &target_located,
                pattern_and_replacement_context,
            );
            if let MathExpression::Relation(new_rel) = result_expr {
                return (*new_rel).clone();
            }
            return self.clone(); // Fallback on type mismatch
        }

        // Otherwise, we recurse into the children.
        match self {
            MathRelation::Equal { left, right } => {
                // ✅ SIMPLIFIED: Use Located<T> Replace implementation
                let new_left = left.replace(
                    &left.id,
                    target_id,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                    manual_instantiations,
                );
                let new_right = right.replace(
                    &right.id,
                    target_id,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                    manual_instantiations,
                );
                MathRelation::Equal {
                    left: new_left,
                    right: new_right,
                }
            }
            MathRelation::Implies(left, right) => {
                let new_left = left.replace(
                    &left.id,
                    target_id,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                    manual_instantiations,
                );
                let new_right = right.replace(
                    &right.id,
                    target_id,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                    manual_instantiations,
                );
                MathRelation::Implies(new_left, new_right)
            }
            MathRelation::Equivalent(left, right) => {
                let new_left = left.replace(
                    &left.id,
                    target_id,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                    manual_instantiations,
                );
                let new_right = right.replace(
                    &right.id,
                    target_id,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                    manual_instantiations,
                );
                MathRelation::Equivalent(new_left, new_right)
            }
            MathRelation::And(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        r.replace(
                            current_id,
                            target_id,
                            target_context,
                            pattern,
                            replacement,
                            pattern_and_replacement_context,
                            manual_instantiations,
                        )
                    })
                    .collect();
                MathRelation::And(new_relations)
            }
            MathRelation::Or(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        r.replace(
                            current_id,
                            target_id,
                            target_context,
                            pattern,
                            replacement,
                            pattern_and_replacement_context,
                            manual_instantiations,
                        )
                    })
                    .collect();
                MathRelation::Or(new_relations)
            }
            MathRelation::Not(relation) => {
                let new_relation = relation.replace(
                    current_id,
                    target_id,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                    manual_instantiations,
                );
                MathRelation::Not(new_relation)
            }
            MathRelation::True => MathRelation::True,
            MathRelation::False => MathRelation::False,
            MathRelation::NumberTheory(number_theory_relation) => todo!(),
            MathRelation::SetTheory(set_relation) => todo!(),
            MathRelation::GroupTheory(group_relation) => {
                MathRelation::GroupTheory(group_relation.replace(
                    current_id,
                    target_id,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                    manual_instantiations,
                ))
            }
            MathRelation::RingTheory(ring_relation) => todo!(),
            MathRelation::TopologyTheory(topology_relation) => todo!(),
            MathRelation::CategoryTheory(category_relation) => todo!(),
            MathRelation::ProbabilityTheory(probability_relation) => todo!(),
        }
    }
}

// Implementation for TheoryExpression Replace
impl Replace for TheoryExpression {
    fn replace(
        &self,
        current_id: &str,
        target_id: &str,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        replacement: &Located<MathExpression>,
        pattern_and_replacement_context: &Vec<ContextEntry>,
        manual_instantiations: &HashMap<Identifier, Identifier>,
    ) -> Self {
        match self {
            TheoryExpression::Group(group_expr) => TheoryExpression::Group(group_expr.replace(
                current_id,
                target_id,
                target_context,
                pattern,
                replacement,
                pattern_and_replacement_context,
                manual_instantiations,
            )),
            _ => self.clone(), // TODO: Implement for other theory expressions
        }
    }
}
