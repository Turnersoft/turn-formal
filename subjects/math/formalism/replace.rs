use crate::subjects::math::theories::groups::definitions::{
    GenericGroup, Group, GroupElement, GroupExpression, GroupHomomorphism, GroupOperation,
};
use crate::subjects::math::theories::rings::definitions::Ring;
use crate::subjects::math::theories::zfc::definitions::Set;
use crate::{
    subjects::math::formalism::{
        collect_identifier::CollectIdentifier,
        expressions::{MathExpression, TheoryExpression},
        extract::Parametrizable,
        interpretation::TypeViewOperator,
        location::Located,
        objects::MathObject,
        proof::{ContextEntry, tactics::Target},
        relations::MathRelation,
    },
    turn_render::Identifier,
};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

use super::debug::ShortDebug;
use super::detag::TryDetag;
use super::search::Search;

/// A trait for instantiating meta-variables by comparing a concrete expression (`self`) to a pattern.
/// we only map meta-variables in pattern to expression/variables in target, we will never allow pattern to have more
pub trait Instantiable: Sized {
    fn instantiate(
        &self, // The concrete target expression
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String>;
}

/// A trait for substituting meta-variables in a template (`self`) using a pre-computed map.
pub trait Substitutable: Sized {
    fn substitute(
        &self, // The replacement template
        instantiations: &HashMap<Identifier, String>,
        target: &Located<MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self;
}

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
impl<T: Replace + Clone + 'static> Replace for Located<T> {
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
            println!(
                "DEBUG: REPLACE - Replacement: {}",
                replacement.short_debug()
            );
            println!(
                "DEBUG: REPLACE - Manual instantiations: {:?}",
                manual_instantiations
            );
        }
        // ✅ FIXED: Match on (self.data, pattern.data) pairs like Search does
        if current_id == target_id {
            println!(
                "DEBUG: REPLACE - self.data: {:?}",
                match &self.data {
                    Parametrizable::Concrete(_) => "Concrete",
                    Parametrizable::Variable(_) => "Variable",
                }
            );
            println!(
                "DEBUG: REPLACE - pattern.data: {:?}",
                match &pattern.data {
                    Parametrizable::Concrete(_) => "Concrete",
                    Parametrizable::Variable(_) => "Variable",
                }
            );
        }
        match (&self.data, &pattern.data) {
            // Case 1: Self is concrete, pattern is variable - potential replacement target
            (Parametrizable::Concrete(_), Parametrizable::Variable(pattern_var)) => {
                if current_id == target_id {
                    // ✅ FOUND TARGET: Replace with instantiated replacement
                    let mut instantiations = HashMap::new();

                    // Create instantiation mapping
                    if let Some(target_var_id) = self.variable_id() {
                        instantiations.insert(pattern_var.clone(), target_var_id.to_string());
                    } else {
                        instantiations.insert(pattern_var.clone(), current_id.to_string());
                    }

                    // Add manual instantiations
                    for (theorem_var, goal_var) in manual_instantiations {
                        instantiations.insert(theorem_var.clone(), goal_var.to_string());
                    }

                    // Create a MathExpression target for substitution
                    let math_expr_target = if std::any::type_name::<T>()
                        == std::any::type_name::<MathExpression>()
                    {
                        let any_self: &dyn std::any::Any = self;
                        if let Some(math_expr_self) =
                            any_self.downcast_ref::<Located<MathExpression>>()
                        {
                            math_expr_self.clone()
                        } else {
                            // Fallback: create a Located<MathExpression> containing self
                            Located::new_concrete(MathExpression::Object(Arc::new(MathObject::Group(
                                crate::subjects::math::theories::groups::definitions::Group::Generic(
                                    crate::subjects::math::theories::groups::definitions::GenericGroup {
                                        base_set: crate::subjects::math::theories::groups::definitions::GenericGroup::default().base_set,
                                        operation: crate::subjects::math::theories::groups::definitions::GroupOperation::default(),
                                        props: crate::subjects::math::theories::VariantSet::new(),
                                    }
                                )
                            ))))
                        }
                    } else {
                        // For non-MathExpression types, convert self to a MathExpression wrapper
                        // Use self's ID to maintain the reference structure
                        Located {
                            id: self.id.clone(),
                            data: Parametrizable::Concrete(Arc::new(MathExpression::Object(Arc::new(MathObject::Group(
                                crate::subjects::math::theories::groups::definitions::Group::Generic(
                                    crate::subjects::math::theories::groups::definitions::GenericGroup {
                                        base_set: crate::subjects::math::theories::groups::definitions::GenericGroup::default().base_set,
                                        operation: crate::subjects::math::theories::groups::definitions::GroupOperation::default(),
                                        props: crate::subjects::math::theories::VariantSet::new(),
                                    }
                                )
                            ))))),
                        }
                    };

                    // Apply substitution to replacement
                    println!(
                        "DEBUG: SUBSTITUTE - About to substitute replacement: {}",
                        replacement.short_debug()
                    );
                    println!(
                        "DEBUG: SUBSTITUTE - With instantiations: {:?}",
                        instantiations
                    );
                    let substituted_replacement =
                        replacement.substitute(&instantiations, &math_expr_target, target_context);
                    println!(
                        "DEBUG: SUBSTITUTE - Result: {}",
                        substituted_replacement.short_debug()
                    );

                    // ✅ FIXED: Return the substituted replacement instead of self.clone()
                    println!(
                        "DEBUG: SUBSTITUTE - Type name T: {}",
                        std::any::type_name::<T>()
                    );
                    if std::any::type_name::<T>() == std::any::type_name::<MathExpression>() {
                        if let Some(math_expr) = substituted_replacement.concrete_value() {
                            let new_located = Located::new_concrete(math_expr.as_ref().clone());
                            if let Some(result) =
                                (&new_located as &dyn std::any::Any).downcast_ref::<Located<T>>()
                            {
                                println!(
                                    "DEBUG: SUBSTITUTE - Returning converted result: {}",
                                    new_located.short_debug()
                                );
                                return result.clone();
                            }
                        }
                        // If conversion failed, return the substituted replacement directly
                        if let Some(result) = (&substituted_replacement as &dyn std::any::Any)
                            .downcast_ref::<Located<T>>()
                        {
                            println!(
                                "DEBUG: SUBSTITUTE - Returning direct result: {}",
                                substituted_replacement.short_debug()
                            );
                            return result.clone();
                        }
                    }

                    // If all conversions fail, return self but this indicates a type issue

                    return self.clone();
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

            // Case 2: Self is variable, pattern is variable - match any target variable
            (Parametrizable::Variable(self_var), Parametrizable::Variable(pattern_var)) => {
                if current_id == target_id {
                    // ✅ FIXED: Apply manual instantiations to replacement expression
                    let mut instantiations = HashMap::new();
                    println!(
                        "DEBUG: CASE2_AUTO_INST - Creating {} -> {}",
                        pattern_var.body, self_var
                    );
                    instantiations.insert(pattern_var.clone(), self_var.to_string());

                    // Add manual instantiations (these are the key ones like x -> g)
                    for (theorem_var, goal_var) in manual_instantiations {
                        println!(
                            "DEBUG: MANUAL_INST - Adding {} -> {}",
                            theorem_var.body, goal_var.body
                        );
                        instantiations.insert(theorem_var.clone(), goal_var.to_string());
                    }

                    // Create simple target for substitution (avoid unimplemented Set)
                    let math_expr_target = Located::new_concrete(MathExpression::Object(Arc::new(MathObject::Group(
                        crate::subjects::math::theories::groups::definitions::Group::Generic(
                            crate::subjects::math::theories::groups::definitions::GenericGroup {
                                base_set: crate::subjects::math::theories::groups::definitions::GenericGroup::default().base_set,
                                operation: crate::subjects::math::theories::groups::definitions::GroupOperation::default(),
                                props: crate::subjects::math::theories::VariantSet::new(),
                            }
                        )
                    ))));

                    let substituted_replacement =
                        replacement.substitute(&instantiations, &math_expr_target, target_context);

                    println!(
                        "DEBUG: SUBSTITUTE - Result: {}",
                        substituted_replacement.short_debug()
                    );

                    // ✅ FIXED: Return the substituted replacement

                    // Handle MathExpression -> GroupExpression conversion
                    if std::any::type_name::<T>()
                        == std::any::type_name::<
                            crate::subjects::math::theories::groups::definitions::GroupExpression,
                        >()
                    {
                        if let Some(math_expr) = substituted_replacement.concrete_value() {
                            if let crate::subjects::math::formalism::expressions::MathExpression::Expression(
                                crate::subjects::math::formalism::expressions::TheoryExpression::Group(group_expr)
                            ) = math_expr.as_ref() {
                                let new_located = Located::new_concrete(group_expr.clone());
                                if let Some(result) = (&new_located as &dyn std::any::Any).downcast_ref::<Located<T>>() {
                                    return result.clone();
                                }
                            }
                        }
                    }

                    if std::any::type_name::<T>() == std::any::type_name::<MathExpression>() {
                        if let Some(math_expr) = substituted_replacement.concrete_value() {
                            let new_located = Located::new_concrete(math_expr.as_ref().clone());
                            if let Some(result) =
                                (&new_located as &dyn std::any::Any).downcast_ref::<Located<T>>()
                            {
                                return result.clone();
                            }
                        }
                        if let Some(result) = (&substituted_replacement as &dyn std::any::Any)
                            .downcast_ref::<Located<T>>()
                        {
                            return result.clone();
                        }
                    }
                }
                self.clone()
            }

            // Case 3: Both concrete - recurse into inner content
            (Parametrizable::Concrete(self_concrete), Parametrizable::Concrete(_)) => {
                if current_id == target_id {
                    // ✅ Create automatic instantiations by comparing concrete content
                    let mut instantiations = HashMap::new();

                    // Try to get automatic instantiations for MathExpression
                    if std::any::type_name::<T>() == std::any::type_name::<MathExpression>() {
                        if let (Some(self_concrete), Some(pattern_concrete)) =
                            (self.concrete_value(), pattern.concrete_value())
                        {
                            if let (Some(self_math), Some(pattern_math)) = (
                                (self_concrete.as_ref() as &dyn std::any::Any)
                                    .downcast_ref::<MathExpression>(),
                                (pattern_concrete.as_ref() as &dyn std::any::Any)
                                    .downcast_ref::<MathExpression>(),
                            ) {
                                instantiations.extend(self_math.instantiate(
                                    target_context,
                                    pattern_math,
                                    pattern_and_replacement_context,
                                ));
                            }
                        }
                    }

                    // Add manual instantiations (they override automatic ones if needed)
                    for (theorem_var, goal_var) in manual_instantiations {
                        instantiations.insert(theorem_var.clone(), goal_var.to_string());
                    }

                    // Create target for substitution using self as the source of Located<> objects
                    let self_as_target = if std::any::type_name::<T>()
                        == std::any::type_name::<MathExpression>()
                    {
                        if let Some(self_concrete) = self.concrete_value() {
                            if let Some(self_math) = (self_concrete.as_ref() as &dyn std::any::Any)
                                .downcast_ref::<MathExpression>()
                            {
                                Located::new_concrete(self_math.clone())
                            } else {
                                // Fallback to generic target
                                Located::new_concrete(MathExpression::Object(Arc::new(MathObject::Group(
                                    crate::subjects::math::theories::groups::definitions::Group::Generic(
                                        crate::subjects::math::theories::groups::definitions::GenericGroup::default()
                                    )
                                ))))
                            }
                        } else {
                            // Fallback for non-concrete self
                            Located::new_concrete(MathExpression::Object(Arc::new(MathObject::Group(
                                crate::subjects::math::theories::groups::definitions::Group::Generic(
                                    crate::subjects::math::theories::groups::definitions::GenericGroup::default()
                                )
                            ))))
                        }
                    } else {
                        // Fallback for non-MathExpression types
                        Located::new_concrete(MathExpression::Object(Arc::new(MathObject::Group(
                            crate::subjects::math::theories::groups::definitions::Group::Generic(
                                crate::subjects::math::theories::groups::definitions::GenericGroup::default()
                            )
                        ))))
                    };

                    // Apply substitution to replacement using self as target
                    let substituted_replacement =
                        replacement.substitute(&instantiations, &self_as_target, target_context);

                    // Return the substituted replacement (try type conversion)
                    if std::any::type_name::<T>() == std::any::type_name::<MathExpression>() {
                        if let Some(math_expr) = substituted_replacement.concrete_value() {
                            let new_located = Located::new_concrete(math_expr.as_ref().clone());
                            if let Some(result) =
                                (&new_located as &dyn std::any::Any).downcast_ref::<Located<T>>()
                            {
                                return result.clone();
                            }
                        }
                        if let Some(result) = (&substituted_replacement as &dyn std::any::Any)
                            .downcast_ref::<Located<T>>()
                        {
                            return result.clone();
                        }
                    }

                    // Handle MathExpression -> GroupExpression conversion for Case 3
                    if std::any::type_name::<T>()
                        == std::any::type_name::<
                            crate::subjects::math::theories::groups::definitions::GroupExpression,
                        >()
                    {
                        if let Some(math_expr) = substituted_replacement.concrete_value() {
                            if let crate::subjects::math::formalism::expressions::MathExpression::Expression(
                                crate::subjects::math::formalism::expressions::TheoryExpression::Group(group_expr)
                            ) = math_expr.as_ref() {
                                let new_located = Located::new_concrete(group_expr.clone());
                                if let Some(result) = (&new_located as &dyn std::any::Any).downcast_ref::<Located<T>>() {
                                    return result.clone();
                                }
                            }
                        }
                    }

                    // Fallback: apply the recursive replacement
                }

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

            // Case 4: Self is variable, pattern is concrete - no structural match
            (Parametrizable::Variable(_), Parametrizable::Concrete(_)) => self.clone(),
        }
    }
}

// ✅ REMOVED: All helper functions integrated into trait implementations

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

impl Instantiable for MathExpression {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        match (self, pattern) {
            // If pattern is a variable reference and self is concrete, create instantiation
            // Note: MathExpression doesn't have direct variables anymore, they're in Located<T>

            // Recursively handle each variant
            (MathExpression::Object(target_obj), MathExpression::Object(pattern_obj)) => {
                instantiations.extend(target_obj.instantiate(
                    target_context,
                    pattern_obj,
                    pattern_context,
                ));
            }
            (MathExpression::Expression(target_expr), MathExpression::Expression(pattern_expr)) => {
                instantiations.extend(target_expr.instantiate(
                    target_context,
                    pattern_expr,
                    pattern_context,
                ));
            }
            (MathExpression::Relation(target_rel), MathExpression::Relation(pattern_rel)) => {
                instantiations.extend(target_rel.instantiate(
                    target_context,
                    pattern_rel,
                    pattern_context,
                ));
            }
            (MathExpression::Number(target_num), MathExpression::Number(pattern_num)) => {
                // Numbers are concrete, no variables to instantiate
            }
            (
                MathExpression::ViewAs {
                    expression: target_expr,
                    view: target_view,
                },
                MathExpression::ViewAs {
                    expression: pattern_expr,
                    view: pattern_view,
                },
            ) => {
                instantiations.extend(target_expr.instantiate(
                    target_context,
                    pattern_expr,
                    pattern_context,
                ));
                instantiations.extend(target_view.instantiate(
                    target_context,
                    pattern_view,
                    pattern_context,
                ));
            }
            // Different variants don't match
            _ => {}
        }

        instantiations
    }
}

// Simplified implementation for Located<T> without complex trait bounds
impl<T> Instantiable for Located<T>
where
    T: Clone + Instantiable + ShortDebug + 'static,
{
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        match (&self.data, &pattern.data) {
            // Pattern is a variable, target is concrete - create instantiation
            (Parametrizable::Concrete(_), Parametrizable::Variable(pattern_var)) => {
                // ✅ FIXED: Check if self (the target) is actually a variable first
                if let Some(target_var_id) = self.variable_id() {
                    // Target Located<T> itself contains a variable - use its name
                    instantiations.insert(pattern_var.clone(), target_var_id.to_string());
                } else {
                    // Target is concrete - extract meaningful representation using existing methods
                    let meaningful_id = if let Some(concrete_val) = self.concrete_value() {
                        // Try short_debug for simple clean representations
                        let debug_str = concrete_val.short_debug();
                        if debug_str.len() < 20
                            && !debug_str.contains("Located")
                            && !debug_str.contains('-')
                            && !debug_str.contains("elem")
                        {
                            debug_str
                        } else {
                            self.id.clone()
                        }
                    } else {
                        self.id.clone()
                    };
                    instantiations.insert(pattern_var.clone(), meaningful_id);
                }
            }
            // ✅ ADD MISSING CASE: Target is variable, pattern is concrete - create instantiation
            (Parametrizable::Variable(target_var), Parametrizable::Concrete(_)) => {
                // This case happens when target is a variable and pattern is concrete
                // We can't create an instantiation in this direction (concrete → variable)
                // This is the correct behavior - no instantiation should be created
            }
            // Both are concrete - we can't recursively instantiate without knowing T's structure
            (Parametrizable::Concrete(target_val), Parametrizable::Concrete(pattern_val)) => {
                // For specific types, we need specific implementations
                // The generic implementation just returns empty for now
                instantiations.extend(target_val.instantiate(
                    target_context,
                    pattern_val,
                    pattern_context,
                ));
            }
            // Pattern is concrete, target is variable - no instantiation possible
            (Parametrizable::Variable(_), Parametrizable::Concrete(_)) => {
                // No instantiation possible when target is variable and pattern is concrete
            }
            // Both are variables - only match if they're the same variable
            (Parametrizable::Variable(target_var), Parametrizable::Variable(pattern_var)) => {
                if target_var == pattern_var {
                    // Same variable, no new instantiation needed
                } else {
                    // Different variables - create instantiation: pattern_var → target_var.body
                    instantiations.insert(pattern_var.clone(), target_var.body.clone());
                }
            }
        }

        instantiations
    }
}

// Implementation for Arc<T>
impl<T: Instantiable> Instantiable for Arc<T> {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        (**self).instantiate(target_context, &**pattern, pattern_context)
    }
}

// Implementation for TheoryExpression
impl Instantiable for TheoryExpression {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        match (self, pattern) {
            (TheoryExpression::Group(target_group), TheoryExpression::Group(pattern_group)) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
            }
            (TheoryExpression::Ring(target_ring), TheoryExpression::Ring(pattern_ring)) => {
                // TODO: Implement when RingExpression has Instantiable
            }
            (TheoryExpression::Field(target_field), TheoryExpression::Field(pattern_field)) => {
                // TODO: Implement when FieldExpression has Instantiable
            }
            // Different variants don't match
            _ => {}
        }

        instantiations
    }
}

// Implementation for MathRelation
impl Instantiable for MathRelation {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        match (self, pattern) {
            (
                MathRelation::Equal {
                    left: target_left,
                    right: target_right,
                },
                MathRelation::Equal {
                    left: pattern_left,
                    right: pattern_right,
                },
            ) => {
                instantiations.extend(target_left.instantiate(
                    target_context,
                    pattern_left,
                    pattern_context,
                ));
                instantiations.extend(target_right.instantiate(
                    target_context,
                    pattern_right,
                    pattern_context,
                ));
            }
            (MathRelation::And(target_rels), MathRelation::And(pattern_rels)) => {
                for (target_rel, pattern_rel) in target_rels.iter().zip(pattern_rels.iter()) {
                    instantiations.extend(target_rel.instantiate(
                        target_context,
                        pattern_rel,
                        pattern_context,
                    ));
                }
            }
            (MathRelation::Or(target_rels), MathRelation::Or(pattern_rels)) => {
                for (target_rel, pattern_rel) in target_rels.iter().zip(pattern_rels.iter()) {
                    instantiations.extend(target_rel.instantiate(
                        target_context,
                        pattern_rel,
                        pattern_context,
                    ));
                }
            }
            (MathRelation::Not(target_rel), MathRelation::Not(pattern_rel)) => {
                instantiations.extend(target_rel.instantiate(
                    target_context,
                    pattern_rel,
                    pattern_context,
                ));
            }
            (
                MathRelation::Implies(target_left, target_right),
                MathRelation::Implies(pattern_left, pattern_right),
            ) => {
                instantiations.extend(target_left.instantiate(
                    target_context,
                    pattern_left,
                    pattern_context,
                ));
                instantiations.extend(target_right.instantiate(
                    target_context,
                    pattern_right,
                    pattern_context,
                ));
            }
            (
                MathRelation::Equivalent(target_left, target_right),
                MathRelation::Equivalent(pattern_left, pattern_right),
            ) => {
                instantiations.extend(target_left.instantiate(
                    target_context,
                    pattern_left,
                    pattern_context,
                ));
                instantiations.extend(target_right.instantiate(
                    target_context,
                    pattern_right,
                    pattern_context,
                ));
            }
            // Base cases and different variants
            (MathRelation::True, MathRelation::True)
            | (MathRelation::False, MathRelation::False) => {
                // No variables to instantiate
            }
            // Theory-specific relations
            (MathRelation::GroupTheory(target_rel), MathRelation::GroupTheory(pattern_rel)) => {
                // TODO: Implement when GroupRelation has Instantiable
            }
            // Other theory relations...
            _ => {
                // Different variants or unimplemented theory relations
            }
        }

        instantiations
    }
}

// Implementation for MathObject
impl Instantiable for MathObject {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        match (self, pattern) {
            (MathObject::Group(target_group), MathObject::Group(pattern_group)) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
            }
            // Other MathObject variants
            _ => {
                // TODO: Implement for other object types when needed
            }
        }

        instantiations
    }
}

// Implementation for TypeViewOperator
impl Instantiable for TypeViewOperator {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        match (self, pattern) {
            (
                TypeViewOperator::AsGroupElement {
                    group: target_group,
                },
                TypeViewOperator::AsGroupElement {
                    group: pattern_group,
                },
            ) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
            }
            (
                TypeViewOperator::AsGroup {
                    operation: target_op,
                },
                TypeViewOperator::AsGroup {
                    operation: pattern_op,
                },
            ) => {
                if let (Some(target_expr), Some(pattern_expr)) = (target_op, pattern_op) {
                    instantiations.extend(target_expr.instantiate(
                        target_context,
                        pattern_expr,
                        pattern_context,
                    ));
                }
            }
            (
                TypeViewOperator::AsHomomorphism {
                    source: target_src,
                    target: target_tgt,
                },
                TypeViewOperator::AsHomomorphism {
                    source: pattern_src,
                    target: pattern_tgt,
                },
            ) => {
                instantiations.extend(target_src.instantiate(
                    target_context,
                    pattern_src,
                    pattern_context,
                ));
                instantiations.extend(target_tgt.instantiate(
                    target_context,
                    pattern_tgt,
                    pattern_context,
                ));
            }
            (
                TypeViewOperator::AsFunction { domain: target_dom },
                TypeViewOperator::AsFunction {
                    domain: pattern_dom,
                },
            ) => {
                if let (Some(target_expr), Some(pattern_expr)) = (target_dom, pattern_dom) {
                    instantiations.extend(target_expr.instantiate(
                        target_context,
                        pattern_expr,
                        pattern_context,
                    ));
                }
            }
            (
                TypeViewOperator::Custom {
                    parameters: target_params,
                    ..
                },
                TypeViewOperator::Custom {
                    parameters: pattern_params,
                    ..
                },
            ) => {
                for (target_param, pattern_param) in target_params.iter().zip(pattern_params.iter())
                {
                    instantiations.extend(target_param.instantiate(
                        target_context,
                        pattern_param,
                        pattern_context,
                    ));
                }
            }
            // Simple cases with no parameters
            (TypeViewOperator::AsCyclicGroup, TypeViewOperator::AsCyclicGroup)
            | (TypeViewOperator::AsPoint, TypeViewOperator::AsPoint)
            | (
                TypeViewOperator::AsLinearTransformation,
                TypeViewOperator::AsLinearTransformation,
            ) => {
                // No variables to instantiate
            }
            // Different variants
            _ => {}
        }

        instantiations
    }
}

impl Substitutable for MathExpression {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, String>,
        target: &Located<MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        // This is the base case for the recursion: if the expression is a meta-variable, we replace it.
        // if let MathExpression::Object(obj) = self {
        //     if let MathObject::Group(Group::Generic(generic_group)) = &**obj {
        //         if let Set::Parametric { description, .. } = &generic_group.base_set {
        //             if is_meta_variable(description, context) {
        //                 let id_key = Identifier::new_simple(description.clone());
        //                 if let Some(substituted_expr) = instantiations.get(&id_key) {
        //                     return substituted_expr.clone();
        //                 }
        //             }
        //         }
        //     }
        // }
        // todo: the above is completely wrong.

        // Otherwise, we recurse.
        match self {
            MathExpression::Relation(rel) => {
                MathExpression::Relation(rel.substitute(instantiations, target, context))
            }
            MathExpression::Object(obj) => {
                MathExpression::Object(obj.substitute(instantiations, target, context))
            }
            MathExpression::Expression(expr) => {
                MathExpression::Expression(expr.substitute(instantiations, target, context))
            }
            MathExpression::ViewAs { expression, view } => MathExpression::ViewAs {
                expression: expression.substitute(instantiations, target, context),
                view: view.clone(),
            },
            _ => self.clone(),
        }
    }
}

impl<T: Substitutable + Clone + Debug + 'static> Substitutable for Located<T>
where
    T: TryDetag<T>,
{
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, String>,
        target: &Located<MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match &self.data {
            Parametrizable::Concrete(arc_value) => {
                let substituted_value = arc_value.substitute(instantiations, target, context);

                Located {
                    id: self.id.clone(),
                    data: Parametrizable::Concrete(substituted_value),
                }
            }
            Parametrizable::Variable(id) => {
                if let Some(substituted_name) = instantiations.get(id) {
                    // ✅ FIXED: Create a new variable with the substituted name
                    // instead of trying to look it up by ID
                    if !substituted_name.contains('-') && substituted_name.len() < 20 {
                        // Looks like a variable name - create a new variable
                        Located::new_variable(crate::turn_render::Identifier::new_simple(
                            substituted_name.clone(),
                        ))
                    } else {
                        // Looks like a UUID - try to look it up in target
                        if let Some(located_expr) = target
                            .data
                            .unwrap_arc(context)
                            .get_located::<T>(substituted_name.clone())
                        {
                            located_expr
                        } else {
                            // ✅ FIXED: Never allow UUIDs to become identifier.body!
                            // Instead, keep the original variable when lookup fails
                            self.clone()
                        }
                    }
                } else {
                    self.clone()
                }
            }
        }
    }
}

// Generic implementations for Arc-wrapped types
impl<T: Substitutable> Substitutable for Arc<T> {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, String>,
        target: &Located<MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        Arc::new((**self).substitute(instantiations, target, context))
    }
}

impl Substitutable for MathObject {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, String>,
        target: &Located<MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match &self {
            MathObject::Group(group) => {
                MathObject::Group(group.substitute(instantiations, target, context))
            }
            MathObject::Set(set) => {
                MathObject::Set(set.substitute(instantiations, target, context))
            }
            // Add other MathObject variants here
            _ => self.clone(),
        }
    }
}

impl Substitutable for MathRelation {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, String>,
        target: &Located<MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match &self {
            MathRelation::Equal { left, right } => {
                // For Arc-wrapped expressions, we need to unwrap, substitute, and re-wrap
                let new_left_expr =
                    left.data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                let new_right_expr =
                    right
                        .data
                        .unwrap(context)
                        .substitute(instantiations, target, context);

                MathRelation::Equal {
                    left: Located::new_concrete(new_left_expr),
                    right: Located::new_concrete(new_right_expr),
                }
            }
            MathRelation::And(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        let new_rel =
                            r.data
                                .unwrap(context)
                                .substitute(instantiations, target, context);
                        Located::new_concrete(new_rel)
                    })
                    .collect();
                MathRelation::And(new_relations)
            }
            MathRelation::Or(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        let new_rel =
                            r.data
                                .unwrap(context)
                                .substitute(instantiations, target, context);
                        Located::new_concrete(new_rel)
                    })
                    .collect();
                MathRelation::Or(new_relations)
            }
            MathRelation::Not(relation) => {
                let new_rel =
                    relation
                        .data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                MathRelation::Not(Located::new_concrete(new_rel))
            }
            MathRelation::Implies(left, right) => {
                let new_left =
                    left.data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                let new_right =
                    right
                        .data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                MathRelation::Implies(
                    Located::new_concrete(new_left),
                    Located::new_concrete(new_right),
                )
            }
            MathRelation::Equivalent(left, right) => {
                let new_left =
                    left.data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                let new_right =
                    right
                        .data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                MathRelation::Equivalent(
                    Located::new_concrete(new_left),
                    Located::new_concrete(new_right),
                )
            }
            _ => self.clone(),
        }
    }
}

fn is_meta_variable(name: &str, _context: &Vec<ContextEntry>) -> bool {
    // A simple heuristic: if the name is all uppercase, it's a meta-variable.
    !name.is_empty() && name.chars().all(|c| c.is_ascii_uppercase())
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

// Implementation for TheoryExpression Substitutable
impl Substitutable for TheoryExpression {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, String>,
        target: &Located<MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match self {
            TheoryExpression::Group(group_expr) => {
                TheoryExpression::Group(group_expr.substitute(instantiations, target, context))
            }
            _ => self.clone(), // TODO: Implement for other theory expressions
        }
    }
}

// ===== GROUP THEORY IMPLEMENTATIONS =====

// Implementation for Group
impl Instantiable for Group {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        // For now, treat groups as atomic - we don't traverse into their internal structure
        // since groups in expressions are typically used as identifiers
        // Only match if they're the same variant
        match (self, pattern) {
            (Group::Generic(_), Group::Generic(_))
            | (Group::Trivial(_), Group::Trivial(_))
            | (Group::Cyclic(_), Group::Cyclic(_)) => {
                // Same group types, but we don't traverse internal structure
            }
            _ => {
                // Different group types, no match
            }
        }

        instantiations
    }
}

// Implementation for GroupExpression
impl Instantiable for GroupExpression {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        match (self, pattern) {
            (
                GroupExpression::Element {
                    group: target_group,
                    element: target_element,
                },
                GroupExpression::Element {
                    group: pattern_group,
                    element: pattern_element,
                },
            ) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
                // todo: this is quite redendunt.
                // instantiations.extend(target_element.instantiate(
                //     target_context,
                //     pattern_element,
                //     pattern_context,
                // ));
            }
            (GroupExpression::Identity(target_group), GroupExpression::Identity(pattern_group)) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
            }
            (
                GroupExpression::Operation {
                    group: target_group,
                    left: target_left,
                    right: target_right,
                },
                GroupExpression::Operation {
                    group: pattern_group,
                    left: pattern_left,
                    right: pattern_right,
                },
            ) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
                let left_inst =
                    target_left.instantiate(target_context, pattern_left, pattern_context);
                instantiations.extend(left_inst);
                let right_inst =
                    target_right.instantiate(target_context, pattern_right, pattern_context);
                instantiations.extend(right_inst);
            }
            (
                GroupExpression::Inverse {
                    group: target_group,
                    element: target_element,
                },
                GroupExpression::Inverse {
                    group: pattern_group,
                    element: pattern_element,
                },
            ) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
                instantiations.extend(target_element.instantiate(
                    target_context,
                    pattern_element,
                    pattern_context,
                ));
            }
            (
                GroupExpression::Commutator {
                    group: target_group,
                    a: target_a,
                    b: target_b,
                },
                GroupExpression::Commutator {
                    group: pattern_group,
                    a: pattern_a,
                    b: pattern_b,
                },
            ) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
                instantiations.extend(target_a.instantiate(
                    target_context,
                    pattern_a,
                    pattern_context,
                ));
                instantiations.extend(target_b.instantiate(
                    target_context,
                    pattern_b,
                    pattern_context,
                ));
            }
            (
                GroupExpression::Coset {
                    group: target_group,
                    element: target_element,
                    subgroup: target_subgroup,
                    is_left: target_is_left,
                },
                GroupExpression::Coset {
                    group: pattern_group,
                    element: pattern_element,
                    subgroup: pattern_subgroup,
                    is_left: pattern_is_left,
                },
            ) => {
                if target_is_left == pattern_is_left {
                    instantiations.extend(target_group.instantiate(
                        target_context,
                        pattern_group,
                        pattern_context,
                    ));
                    instantiations.extend(target_element.instantiate(
                        target_context,
                        pattern_element,
                        pattern_context,
                    ));
                    instantiations.extend(target_subgroup.instantiate(
                        target_context,
                        pattern_subgroup,
                        pattern_context,
                    ));
                }
            }
            (
                GroupExpression::Power {
                    group: target_group,
                    base: target_base,
                    exponent: target_exponent,
                },
                GroupExpression::Power {
                    group: pattern_group,
                    base: pattern_base,
                    exponent: pattern_exponent,
                },
            ) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
                instantiations.extend(target_base.instantiate(
                    target_context,
                    pattern_base,
                    pattern_context,
                ));
                // instantiations.extend(target_exponent.instantiate(
                //     target_context,
                //     pattern_exponent,
                //     pattern_context,
                // ));
            }
            (
                GroupExpression::GroupOrder {
                    group: target_group,
                },
                GroupExpression::GroupOrder {
                    group: pattern_group,
                },
            ) => {
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
            }
            (
                GroupExpression::ElementOrder {
                    element: target_element,
                    group: target_group,
                },
                GroupExpression::ElementOrder {
                    element: pattern_element,
                    group: pattern_group,
                },
            ) => {
                instantiations.extend(target_element.instantiate(
                    target_context,
                    pattern_element,
                    pattern_context,
                ));
                instantiations.extend(target_group.instantiate(
                    target_context,
                    pattern_group,
                    pattern_context,
                ));
            }
            (
                GroupExpression::Homomorphism(target_hom),
                GroupExpression::Homomorphism(pattern_hom),
            ) => {
                instantiations.extend(target_hom.instantiate(
                    target_context,
                    pattern_hom,
                    pattern_context,
                ));
            }
            // Skip ActionOnElement for now since GroupAction is complex
            _ => {
                // Different variants or unsupported cases
            }
        }

        instantiations
    }
}

// Implementation for GroupHomomorphism
impl Instantiable for GroupHomomorphism {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        instantiations.extend(self.domain.instantiate(
            target_context,
            &pattern.domain,
            pattern_context,
        ));
        instantiations.extend(self.codomain.instantiate(
            target_context,
            &pattern.codomain,
            pattern_context,
        ));

        instantiations
    }
}

// Implementation for Option<T>
impl<T: Instantiable> Instantiable for Option<T> {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        match (self, pattern) {
            (Some(target_val), Some(pattern_val)) => {
                instantiations.extend(target_val.instantiate(
                    target_context,
                    pattern_val,
                    pattern_context,
                ));
            }
            (None, None) => {
                // Both are None, no instantiation needed
            }
            _ => {
                // One is Some, other is None - no match
            }
        }

        instantiations
    }
}

// Implementation for Vec<T>
impl<T: Instantiable> Instantiable for Vec<T> {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        let mut instantiations = HashMap::new();

        // Only instantiate if both vectors have the same length
        if self.len() == pattern.len() {
            for (target_item, pattern_item) in self.iter().zip(pattern.iter()) {
                instantiations.extend(target_item.instantiate(
                    target_context,
                    pattern_item,
                    pattern_context,
                ));
            }
        }

        instantiations
    }
}

// // Implementation for primitive types (they don't contain variables)
// impl Instantiable for i32 {
//     fn instantiate(
//         &self,
//         _target_context: &Vec<ContextEntry>,
//         _pattern: &Self,
//         _pattern_context: &Vec<ContextEntry>,
//     ) -> HashMap<Identifier, MathExpression> {
//         HashMap::new()
//     }
// }

// impl Instantiable for u32 {
//     fn instantiate(
//         &self,
//         _target_context: &Vec<ContextEntry>,
//         _pattern: &Self,
//         _pattern_context: &Vec<ContextEntry>,
//     ) -> HashMap<Identifier, MathExpression> {
//         HashMap::new()
//     }
// }

// impl Instantiable for usize {
//     fn instantiate(
//         &self,
//         _target_context: &Vec<ContextEntry>,
//         _pattern: &Self,
//         _pattern_context: &Vec<ContextEntry>,
//     ) -> HashMap<Identifier, MathExpression> {
//         HashMap::new()
//     }
// }

// impl Instantiable for String {
//     fn instantiate(
//         &self,
//         _target_context: &Vec<ContextEntry>,
//         _pattern: &Self,
//         _pattern_context: &Vec<ContextEntry>,
//     ) -> HashMap<Identifier, MathExpression> {
//         HashMap::new()
//     }
// }

// Implementation for GroupElement
impl Instantiable for crate::subjects::math::theories::groups::definitions::GroupElement {
    fn instantiate(
        &self,
        _target_context: &Vec<ContextEntry>,
        _pattern: &Self,
        _pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, String> {
        // GroupElement is a concrete type with no variables
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::theories::groups::definitions::{
        GenericGroup, Group, GroupExpression,
    };
    use crate::turn_render::Identifier;

    #[test]
    fn test_instantiate_group_expression() {
        // Create a concrete target: G * x where G is concrete, x and y are concrete
        let target_expr = GroupExpression::Operation {
            group: Located::new_concrete(Group::Generic(GenericGroup::default())),
            left: Located::new_concrete(GroupExpression::Identity(Located::new_concrete(
                Group::Generic(GenericGroup::default()),
            ))),
            right: Located::new_concrete(GroupExpression::Identity(Located::new_concrete(
                Group::Generic(GenericGroup::default()),
            ))),
        };

        // Create a pattern: ?G * ?x * ?y where ?G, ?x, and ?y are variables
        let pattern_expr = GroupExpression::Operation {
            group: Located::new_variable(Identifier::new_simple("G_var".to_string())),
            left: Located::new_variable(Identifier::new_simple("x_var".to_string())),
            right: Located::new_variable(Identifier::new_simple("y_var".to_string())),
        };

        // Instantiate
        let context = vec![];
        let instantiations = target_expr.instantiate(&context, &pattern_expr, &context);

        // Check that we found instantiations for the pattern variables
        // Only G_var should be instantiated because left and right are concrete expressions in target
        assert!(instantiations.contains_key(&Identifier::new_simple("G_var".to_string())));
        assert!(instantiations.contains_key(&Identifier::new_simple("x_var".to_string())));
        assert!(instantiations.contains_key(&Identifier::new_simple("y_var".to_string())));
        assert_eq!(instantiations.len(), 3);
    }

    #[test]
    fn test_instantiate_math_expression() {
        // Create a concrete target expression
        let target_expr = MathExpression::ViewAs {
            expression: Located::new_variable(Identifier::new_simple("x".to_string())),
            view: Located::new_concrete(TypeViewOperator::AsCyclicGroup),
        };

        // Create a pattern with a variable view
        let pattern_expr = MathExpression::ViewAs {
            expression: Located::new_variable(Identifier::new_simple("x_var".to_string())),
            view: Located::new_variable(Identifier::new_simple("view_var".to_string())),
        };

        // Instantiate
        let context = vec![];
        let instantiations = target_expr.instantiate(&context, &pattern_expr, &context);

        // Check that we found instantiations
        assert!(instantiations.contains_key(&Identifier::new_simple("view_var".to_string())));
        // Note: x_var won't be instantiated because both target and pattern have variables in that position
    }
}
