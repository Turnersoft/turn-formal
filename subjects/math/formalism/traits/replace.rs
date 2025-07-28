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
impl<T: Replace + Clone + 'static + std::fmt::Debug> Replace for Located<T> {
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

                    // Instead of mapping to string, map to the actual structured expression
                    // Find the actual Located<T> for self_var in the target
                    // Create a MathExpression target for lookup
                    let math_expr_target = Located::new_concrete(MathExpression::Object(Arc::new(MathObject::Group(
                        crate::subjects::math::theories::groups::definitions::Group::Generic(
                            crate::subjects::math::theories::groups::definitions::GenericGroup {
                                base_set: crate::subjects::math::theories::groups::definitions::GenericGroup::default().base_set,
                                operation: crate::subjects::math::theories::groups::definitions::GroupOperation::default(),
                                props: crate::subjects::math::theories::VariantSet::new(),
                            }
                        )
                    ))));

                    if let Some(located_expr) = math_expr_target
                        .data
                        .unwrap_arc(target_context)
                        .get_located::<T>(self_var.to_string())
                    {
                        // Store the actual Located<T> instead of just the string
                        instantiations.insert(pattern_var.clone(), located_expr.id.clone());
                    } else {
                        // Fallback to string mapping if lookup fails
                        instantiations.insert(pattern_var.clone(), self_var.to_string());
                    }

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
