use std::any::Any;
use std::collections::HashSet;

use crate::subjects::math::formalism::debug::ShortDebug;
use crate::subjects::math::formalism::detag::TryDetag;
use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::objects::MathObject;
use crate::subjects::math::formalism::proof::ContextEntry;
use crate::subjects::math::formalism::proof::tactics::{ContextOrStatement, Target};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::theories::groups::definitions::{Group, GroupExpression, GroupRelation};
use std::fmt::Debug;

impl Search for i32 {
    fn get_located<T: 'static + Clone>(&self, target: String) -> Option<Located<T>> {
        // i32 is a primitive type and doesn't contain Located<T> elements
        None
    }

    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> HashSet<String> {
        // i32 is a primitive type - it doesn't recursively contain expressions to search
        HashSet::new()
    }
}

impl IsCompatible<i32> for i32 {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &i32,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // For i32, compatibility is simple value equality
        self == pattern
    }
}

pub trait Search {
    // ✅ REPLACED: Located-aware version that preserves variable information
    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> HashSet<String>;

    fn get_located<T: 'static + Clone>(&self, id: String) -> Option<Located<T>>;
}

pub trait IsCompatible<P> {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &P,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool;
}

impl Search for MathExpression {
    fn get_located<T: 'static + Clone>(&self, target: String) -> Option<Located<T>> {
        match self {
            MathExpression::Object(obj) => obj.get_located(target),
            MathExpression::Relation(rel) => rel.get_located(target),
            MathExpression::Expression(expr) => expr.get_located(target),
            MathExpression::Number(number) => todo!(),
            MathExpression::ViewAs { expression, view } => todo!(),
        }
    }

    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> HashSet<String> {
        let mut matches = HashSet::new();
        let is_in_scope_now = in_target_scope || current_id == target.id;

        if is_in_scope_now {
            if self.is_compatible(
                target.clone(),
                target_context,
                &pattern.data.unwrap(&pattern_context),
                pattern_context,
            ) {
                matches.insert(current_id.clone());
            }
        }

        let sub_matches = match self {
            MathExpression::Object(obj) => obj.find_matches(
                target.clone(),
                current_id.clone(),
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            MathExpression::Relation(rel) => rel.find_matches(
                target.clone(),
                current_id.clone(),
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            MathExpression::Expression(expr) => expr.find_matches(
                target.clone(),
                current_id.clone(),
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            MathExpression::Number(number) => todo!(),
            MathExpression::ViewAs { expression, view } => todo!(),
        };
        matches.extend(sub_matches);

        matches
    }
}

impl IsCompatible<MathExpression> for MathExpression {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (self, pattern) {
            (MathExpression::Object(self_obj), MathExpression::Object(pattern_obj)) => {
                self_obj.is_compatible(target, target_context, &pattern_obj, pattern_context)
            }
            (MathExpression::Relation(self_rel), MathExpression::Relation(pattern_rel)) => {
                self_rel.is_compatible(target, target_context, &pattern_rel, pattern_context)
            }
            (MathExpression::Expression(self_expr), MathExpression::Expression(pattern_expr)) => {
                self_expr.is_compatible(target, target_context, &pattern_expr, pattern_context)
            }
            _ => false,
        }
    }
}

// Located<T> delegates to the inner unwrapped type
impl<T: 'static + Clone + Search + std::fmt::Debug + IsCompatible<T> + ShortDebug> Search
    for Located<T>
{
    fn get_located<U: 'static + Clone>(&self, target: String) -> Option<Located<U>> {
        if self.id == target {
            // Try direct downcast
            let any_self: &dyn Any = self;
            if let Some(located_u) = any_self.downcast_ref::<Located<U>>() {
                return Some(located_u.clone());
            }
        }
        None
    }

    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> HashSet<String> {
        let mut matches = HashSet::new();
        let is_in_scope_now = in_target_scope || current_id == target.id;

        // println!(
        //     "DEBUG: finding matches in self: {:#?}, of pattern: {:#?}",
        //     self, pattern
        // );

        // First, check if we're in scope and can match the pattern directly
        if is_in_scope_now {
            match (&self.data, &pattern.data) {
                (Parametrizable::Variable(self_var), Parametrizable::Variable(pattern_var)) => {
                    // Both variables - check compatibility using try_detag to convert types
                    // Pattern variable can unify with target variable if types are compatible
                    if let Ok(pattern_detagged) = pattern.data.unwrap(&pattern_context).try_detag()
                    {
                        if self.data.unwrap(&target_context).is_compatible(
                            target.clone(),
                            target_context,
                            &pattern_detagged, // ✅ FIXED: Now &T instead of &MathExpression
                            pattern_context,
                        ) {
                            matches.insert(current_id.clone());
                        }
                    }
                }
                (Parametrizable::Variable(_), Parametrizable::Concrete(_)) => {
                    // Our data is a variable, pattern is concrete - potential match but complex
                    // Skip for now to avoid unwrap issues
                    // println!(
                    //     "DEBUG: unable to match because self is variable: {}, pattern is concrete: {}",
                    //     self.short_debug(),
                    //     pattern.short_debug()
                    // );
                }
                (
                    Parametrizable::Concrete(arc_inner),
                    Parametrizable::Concrete(pattern_concrete),
                ) => {
                    matches.extend(arc_inner.find_matches(
                        target.clone(),
                        current_id.clone(),
                        target_context,
                        pattern,
                        pattern_context,
                        is_in_scope_now,
                    ));
                }
                (Parametrizable::Concrete(arc_inner), Parametrizable::Variable(pattern_var)) => {
                    // CRITICAL FIX: Concrete expression containing a variable that matches the pattern
                    // This handles cases like GroupExpression::Element { element: Variable("e") }
                    // matching against pattern Variable("e")

                    // remove the Located<Parametrizable<>> to continue
                    let inner_matches = arc_inner.find_matches(
                        target.clone(),
                        current_id.clone(),
                        target_context,
                        pattern,
                        pattern_context,
                        is_in_scope_now,
                    );
                    matches.extend(inner_matches);
                }
            }
        }

        matches
    }
}

impl<T: 'static + Clone + IsCompatible<T> + Debug> IsCompatible<Located<T>> for Located<T> {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<T>,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (&self.data, &pattern.data) {
            (Parametrizable::Variable(self_var), Parametrizable::Variable(pattern_var)) => {
                // Both are variables - check if they have compatible names or types
                // For now, consider them compatible if they have the same name
                // More sophisticated type checking could be added here
                self.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &pattern.data.unwrap(&pattern_context),
                    pattern_context,
                )
            }
            (
                Parametrizable::Concrete(self_concrete),
                Parametrizable::Concrete(pattern_concrete),
            ) => {
                // Both concrete - delegate to inner type compatibility
                self_concrete.is_compatible(
                    target,
                    target_context,
                    pattern_concrete,
                    pattern_context,
                )
            }
            (Parametrizable::Variable(_), Parametrizable::Concrete(_)) => {
                // Variable can potentially match concrete - this is complex type matching
                // For now, return false but this could be enhanced
                false
            }
            (Parametrizable::Concrete(_), Parametrizable::Variable(_)) => {
                // Concrete can potentially unify with variable - this is unification
                // For now, return false but this could be enhanced
                false
            }
        }
    }
}

impl Search for MathRelation {
    fn get_located<T: 'static + Clone>(&self, target: String) -> Option<Located<T>> {
        match self {
            MathRelation::And(locateds) => todo!(),
            MathRelation::Or(locateds) => todo!(),
            MathRelation::Not(located) => todo!(),
            MathRelation::Implies(located, located1) => todo!(),
            MathRelation::Equivalent(located, located1) => todo!(),
            MathRelation::True => todo!(),
            MathRelation::False => todo!(),
            MathRelation::NumberTheory(number_theory_relation) => todo!(),
            MathRelation::SetTheory(set_relation) => todo!(),
            MathRelation::GroupTheory(group_relation) => todo!(),
            MathRelation::RingTheory(ring_relation) => todo!(),
            MathRelation::TopologyTheory(topology_relation) => todo!(),
            MathRelation::CategoryTheory(category_relation) => todo!(),
            MathRelation::ProbabilityTheory(probability_relation) => todo!(),
            MathRelation::Equal { left, right } => left
                .get_located::<T>(target.clone())
                .or_else(|| right.get_located::<T>(target.clone())),
        }
    }

    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> HashSet<String> {
        let mut matches = HashSet::new();
        let is_in_scope_now = in_target_scope || current_id == target.id;

        // Only check compatibility if the pattern is also a relation
        if is_in_scope_now {
            match pattern.concrete_value() {
                Some(concrete_pattern) => {
                    if let Ok(pattern_rel) = concrete_pattern.try_detag() {
                        if self.is_compatible(
                            target.clone(),
                            target_context,
                            &pattern_rel,
                            pattern_context,
                        ) {
                            matches.insert(current_id.clone());
                            println!("DEBUG: found match in current scope: {:#?}", current_id);
                        } else {
                            println!("DEBUG: no match in current scope: {:#?}", current_id);
                        }
                    } else {
                        println!("DEBUG: pattern is not a relation: {:#?}", concrete_pattern);
                    }
                }
                None => {
                    // Pattern is a variable - could match this relation
                    // matches.insert(current_id.clone());
                }
            }
        }

        let sub_matches = match self {
            MathRelation::Equal { left, right } => {
                let mut left_matches = left.find_matches(
                    target.clone(),
                    left.id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                );
                println!("DEBUG: left_matches: {:#?}", left_matches);
                let right_matches = right.find_matches(
                    target.clone(),
                    right.id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                );
                println!("DEBUG: right_matches: {:#?}", right_matches);
                left_matches.extend(right_matches);
                left_matches
            }
            MathRelation::And(locateds) => todo!(),
            MathRelation::Or(locateds) => todo!(),
            MathRelation::Not(located) => todo!(),
            MathRelation::Implies(located, located1) => todo!(),
            MathRelation::Equivalent(located, located1) => todo!(),
            MathRelation::True => todo!(),
            MathRelation::False => todo!(),
            MathRelation::NumberTheory(number_theory_relation) => todo!(),
            MathRelation::SetTheory(set_relation) => todo!(),
            MathRelation::GroupTheory(group_relation) => todo!(),
            MathRelation::RingTheory(ring_relation) => todo!(),
            MathRelation::TopologyTheory(topology_relation) => todo!(),
            MathRelation::CategoryTheory(category_relation) => todo!(),
            MathRelation::ProbabilityTheory(probability_relation) => todo!(),
        };

        matches.extend(sub_matches);
        matches
    }
}

impl IsCompatible<MathRelation> for MathRelation {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathRelation,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (self, pattern) {
            (
                MathRelation::Equal { left, right },
                MathRelation::Equal {
                    left: pattern_left,
                    right: pattern_right,
                },
            ) => {
                // todo: we should check both equality from both hand side.
                left.is_compatible(
                    target.clone(),
                    target_context,
                    &pattern_left,
                    pattern_context,
                ) && right.is_compatible(target, target_context, &pattern_right, pattern_context)
            }
            (MathRelation::And(locateds), MathRelation::And(pattern_locateds)) => {
                // todo: we should allow order difference, the list is cummutative and associative
                locateds.iter().all(|located| {
                    located.data.unwrap(&target_context).is_compatible(
                        target.clone(),
                        target_context,
                        &located.data.unwrap(&pattern_context),
                        pattern_context,
                    )
                })
            }
            (MathRelation::Or(locateds), MathRelation::Or(pattern_locateds)) => {
                // todo: we should allow order random pair we need only one node from target and pattern list to make a valid pair
                locateds.iter().any(|located| {
                    located.data.unwrap(&target_context).is_compatible(
                        target.clone(),
                        target_context,
                        &located.data.unwrap(&pattern_context),
                        pattern_context,
                    )
                })
            }
            (MathRelation::Not(located), MathRelation::Not(pattern_located)) => {
                located.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                )
            }
            (
                MathRelation::Implies(located, located1),
                MathRelation::Implies(pattern_located, pattern_located1),
            ) => {
                located.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                ) && located1.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located1.data.unwrap(&pattern_context),
                    pattern_context,
                )
            }
            (
                MathRelation::Equivalent(located, located1),
                MathRelation::Equivalent(pattern_located, pattern_located1),
            ) => {
                // todo: the antecedent and precedent can we reversed, we need to try in both directions
                located.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                ) && located1.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                ) && located1.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located1.data.unwrap(&pattern_context),
                    pattern_context,
                )
            }
            _ => false,
        }
    }
}

impl Search for MathObject {
    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> HashSet<String> {
        let mut matches = HashSet::new();
        let is_in_scope_now = in_target_scope || current_id == target.id;

        // Only check compatibility if the pattern is also an object
        if is_in_scope_now {
            match pattern.concrete_value() {
                Some(concrete_pattern) => {
                    if let Ok(pattern_obj) = concrete_pattern.try_detag() {
                        if self.is_compatible(
                            target.clone(),
                            target_context,
                            &pattern_obj,
                            pattern_context,
                        ) {
                            matches.insert(current_id.clone());
                        }
                    }
                }
                None => (),
            }
        }

        let sub_matches = match self {
            MathObject::Group(group) => group.find_matches(
                target.clone(),
                current_id.clone(),
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            MathObject::Ring(ring) => todo!(),
            MathObject::Field(field) => todo!(),
            MathObject::Module(module) => todo!(),
            MathObject::Algebra(algebra) => todo!(),
            MathObject::TopologicalSpace(topological_space) => todo!(),
            MathObject::VectorSpace(vector_space) => todo!(),
            MathObject::Set(set) => todo!(),
            MathObject::Function(function) => todo!(),
        };

        matches.extend(sub_matches);
        matches
    }

    fn get_located<T: 'static + Clone>(&self, target: String) -> Option<Located<T>> {
        match self {
            MathObject::Group(group) => group.get_located(target),
            MathObject::Ring(ring) => todo!(),
            MathObject::Field(field) => todo!(),
            MathObject::Module(module) => todo!(),
            MathObject::Algebra(algebra) => todo!(),
            MathObject::TopologicalSpace(topological_space) => todo!(),
            MathObject::VectorSpace(vector_space) => todo!(),
            MathObject::Set(set) => todo!(),
            MathObject::Function(function) => todo!(),
        }
    }
}

impl IsCompatible<MathObject> for MathObject {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathObject,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        todo!()
    }
}

impl Search for TheoryExpression {
    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &Located<MathExpression>,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> HashSet<String> {
        let mut matches = HashSet::new();
        let is_in_scope_now = in_target_scope || current_id == target.id;

        // Only check compatibility if the pattern is also an expression
        if is_in_scope_now {
            match pattern.concrete_value() {
                Some(concrete_pattern) => {
                    if let Ok(pattern_expr) = concrete_pattern.try_detag() {
                        if self.is_compatible(
                            target.clone(),
                            target_context,
                            &pattern_expr,
                            pattern_context,
                        ) {
                            matches.insert(current_id.clone());
                        }
                    }
                }
                None => (),
            }
        }

        let sub_matches = match self {
            TheoryExpression::Group(group) => group.find_matches(
                target.clone(),
                current_id.clone(),
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            _ => todo!(),
        };

        // For now, we don't recursively search within theory expressions
        // This can be enhanced later to search within group/ring/field expressions
        matches.extend(sub_matches);
        matches
    }

    fn get_located<T: 'static + Clone>(&self, target: String) -> Option<Located<T>> {
        match self {
            TheoryExpression::Group(group) => group.get_located(target),
            _ => todo!(),
        }
    }
}

impl IsCompatible<TheoryExpression> for TheoryExpression {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &TheoryExpression,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // Deep structural compatibility check using StructurallyEquivalent trait

        match (self, pattern) {
            (TheoryExpression::Group(self_group), TheoryExpression::Group(pattern_group)) => {
                self_group.is_compatible(target, target_context, pattern_group, pattern_context)
            }
            // (TheoryExpression::Ring(self_ring), TheoryExpression::Ring(pattern_ring)) => {
            //     self_ring.structurally_equivalent(pattern_ring, target_context)
            // }
            // (TheoryExpression::Field(self_field), TheoryExpression::Field(pattern_field)) => {
            //     self_field.structurally_equivalent(pattern_field, target_context)
            // }
            _ => false,
        }
    }
}
