use std::any::Any;
use std::collections::HashSet;

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::objects::MathObject;
use crate::subjects::math::formalism::proof::ContextEntry;
use crate::subjects::math::formalism::proof::tactics::{ContextOrStatement, Target};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::traits::debug::ShortDebug;
use crate::subjects::math::formalism::traits::detag::TryDetag;
use crate::subjects::math::theories::groups::definitions::{Group, GroupExpression, GroupRelation};
use std::fmt::Debug;

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

    fn get_located<T: 'static + Clone + std::fmt::Debug>(&self, id: String) -> Option<Located<T>>;
}

impl Search for i32 {
    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
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

impl Search for MathExpression {
    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
        // First, try to find the expression with the given ID in sub-expressions
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

// Located<T> delegates to the inner unwrapped type
impl<T: 'static + Clone + Search + std::fmt::Debug + IsCompatible<T> + ShortDebug> Search
    for Located<T>
{
    fn get_located<U: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<U>> {
        // First check if this Located<T> itself has the target ID
        if self.id == target {
            // Compare only the type name, not the exact type
            // This allows finding Located<U> when we have Located<T> where T and U have the same type name
            // For example: GroupExpression::Inverse can be found when GroupExpression::Element is expected
            let t_type_name = std::any::type_name::<T>();
            let u_type_name = std::any::type_name::<U>();

            if t_type_name == u_type_name {
                // Types match, return self directly
                // This is safe because we've verified the types are the same
                let any_self: &dyn Any = self;
                if let Some(located_u) = any_self.downcast_ref::<Located<U>>() {
                    return Some(located_u.clone());
                }
            }
        }

        // If not found at this level, recursively search through nested Located<T> objects
        match &self.data {
            Parametrizable::Concrete(arc_data) => {
                // Recursively search through the inner data
                arc_data.as_ref().get_located(target)
            }
            Parametrizable::Variable(_) => {
                // Variables don't contain nested Located<T> objects
                None
            }
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

        // println!(
        //     "DEBUG: finding matches in self: {:#?}, of pattern: {:#?}",
        //     self, pattern
        // );

        // First, check if we're in scope and can match the pattern directly

        match (&self.data, &pattern.data) {
            (Parametrizable::Variable(self_var), Parametrizable::Variable(pattern_var)) => {
                // Both variables - check compatibility using try_detag to convert types
                // Pattern variable can unify with target variable if types are compatible
                if is_in_scope_now {
                    if let Ok(pattern_detagged) = pattern.data.unwrap(&pattern_context).try_detag()
                    {
                        if self.data.unwrap(&target_context).is_compatible(
                            target_context,
                            &pattern_detagged,
                            pattern_context,
                        ) {
                            matches.insert(current_id.clone());
                        }
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
            (Parametrizable::Concrete(arc_inner), _) => {
                matches.extend(arc_inner.find_matches(
                    target.clone(),
                    current_id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                ));
            }
        }

        matches
    }
}

impl Search for MathRelation {
    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
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
            if let Ok(pattern_rel) = pattern.data.unwrap(&pattern_context).try_detag() {
                if self.is_compatible(target_context, &pattern_rel, pattern_context) {
                    matches.insert(current_id.clone());
                    // println!("DEBUG: found match in current scope: {:#?}", current_id);
                } else {
                    // println!("DEBUG: no match in current scope: {:#?}", current_id);
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
                // println!("DEBUG: left_matches: {:#?}", left_matches);
                let right_matches = right.find_matches(
                    target.clone(),
                    right.id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                );
                // println!(
                //     "DEBUG: finding right_matches with target: {:#?} vs pattern: {:#?}",
                //     right, pattern
                // );
                // println!("DEBUG: right_matches: {:#?}", right_matches);
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
            if let Ok(pattern_obj) = pattern.data.unwrap(&pattern_context).try_detag() {
                if self.is_compatible(target_context, &pattern_obj, pattern_context) {
                    matches.insert(current_id.clone());
                }
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

    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
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

        // Only check compatibility if the pattern is also an expression, todo: this check is redundant to the parent check
        if is_in_scope_now {
            if let Ok(pattern_expr) = pattern.data.unwrap(&pattern_context).try_detag() {
                if self.is_compatible(target_context, &pattern_expr, pattern_context) {
                    matches.insert(current_id.clone());
                }
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

    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
        match self {
            TheoryExpression::Group(group) => group.get_located(target),
            _ => todo!(),
        }
    }
}

// Import the IsCompatible trait for use in Search implementations
use crate::subjects::math::formalism::traits::is_compatible::IsCompatible;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::theories::groups::definitions::{
        GenericGroup, Group, GroupElement, GroupExpression,
    };
    use crate::turn_render::Identifier;
    use std::sync::Arc;

    #[test]
    fn test_get_located_type_matching() {
        // Test 1: Create a Located<GroupExpression> with GroupExpression::Inverse
        let inverse_expr = GroupExpression::Inverse {
            group: Located::new_variable(Identifier::new_simple("G".to_string())),
            element: Located::new_variable(Identifier::new_simple("g".to_string())),
        };
        let mut located_inverse = Located::new_concrete(inverse_expr);
        located_inverse.id = "test-inverse-id".to_string();

        println!("=== Test 1: Located<GroupExpression> with Inverse ===");
        println!(
            "Type name of Located<GroupExpression>: {}",
            std::any::type_name::<Located<GroupExpression>>()
        );
        println!(
            "Type name of Located<GroupExpression::Inverse>: {}",
            std::any::type_name::<Located<GroupExpression>>()
        );
        println!("Located inverse: {:#?}", located_inverse);

        // Test 2: Try to get_located with GroupExpression type
        let result = located_inverse.get_located::<GroupExpression>("test-inverse-id".to_string());
        println!("get_located::<GroupExpression> result: {:#?}", result);

        // Test 3: Try to get_located with a different GroupExpression variant type
        // This should work because both are GroupExpression
        let result2 = located_inverse.get_located::<GroupExpression>("test-inverse-id".to_string());
        println!("get_located::<GroupExpression> result2: {:#?}", result2);

        // Test 4: Create a Located<GroupExpression> with GroupExpression::Element
        let element_expr = GroupExpression::Element {
            group: Located::new_variable(Identifier::new_simple("G".to_string())),
            element: None,
        };
        let mut located_element = Located::new_concrete(element_expr);
        located_element.id = "test-element-id".to_string();

        println!("\n=== Test 4: Located<GroupExpression> with Element ===");
        println!("Located element: {:#?}", located_element);

        // Test 5: Try to get_located with GroupExpression type
        let result3 = located_element.get_located::<GroupExpression>("test-element-id".to_string());
        println!("get_located::<GroupExpression> result3: {:#?}", result3);

        // Test 6: Test type name comparison
        let t_type_name = std::any::type_name::<GroupExpression>();
        let u_type_name = std::any::type_name::<GroupExpression>();
        println!("\n=== Test 6: Type name comparison ===");
        println!("GroupExpression type name: {}", t_type_name);
        println!("GroupExpression type name: {}", u_type_name);
        println!("Type names match: {}", t_type_name == u_type_name);

        // Test 7: Test with different types
        let t_type_name = std::any::type_name::<GroupExpression>();
        let u_type_name = std::any::type_name::<Group>();
        println!("\n=== Test 7: Different type comparison ===");
        println!("GroupExpression type name: {}", t_type_name);
        println!("Group type name: {}", u_type_name);
        println!("Type names match: {}", t_type_name == u_type_name);

        // Test 8: Test the actual scenario from the failing test
        println!("\n=== Test 8: Actual scenario simulation ===");
        let target_id = "a269c683-014e-4246-8967-707f5d931e3e".to_string();
        let result4 = located_inverse.get_located::<GroupExpression>(target_id.clone());
        println!("Looking for ID: {}", target_id);
        println!("Result: {:#?}", result4);

        // Test 9: Test with wrong ID
        let result5 = located_inverse.get_located::<GroupExpression>("wrong-id".to_string());
        println!("Looking for wrong ID, result: {:#?}", result5);
    }

    #[test]
    fn test_get_located_with_variables() {
        // Test with variable data
        let mut variable_located: Located<GroupExpression> =
            Located::new_variable(Identifier::new_simple("test_var".to_string()));
        variable_located.id = "variable-test-id".to_string();

        println!("\n=== Test with Variable Data ===");
        println!("Variable located: {:#?}", variable_located);

        let result =
            variable_located.get_located::<GroupExpression>("variable-test-id".to_string());
        println!("get_located result: {:#?}", result);
    }

    #[test]
    fn test_get_located_nested_search() {
        // Test that get_located searches recursively through nested structures
        let nested_inverse = GroupExpression::Inverse {
            group: Located::new_variable(Identifier::new_simple("G".to_string())),
            element: Located::new_variable(Identifier::new_simple("g".to_string())),
        };

        let operation = GroupExpression::Operation {
            group: Located::new_variable(Identifier::new_simple("G".to_string())),
            left: Located::new_concrete(nested_inverse),
            right: Located::new_variable(Identifier::new_simple("h1".to_string())),
        };

        let mut located_operation = Located::new_concrete(operation);
        located_operation.id = "top-level-id".to_string();

        println!("\n=== Test Nested Search ===");
        println!("Located operation: {:#?}", located_operation);

        // The nested_inverse should have been assigned a random ID
        // Let's try to find it by searching recursively
        let result = located_operation.get_located::<GroupExpression>("top-level-id".to_string());
        println!("get_located for top-level-id: {:#?}", result);

        // This should demonstrate that we need recursive search
        println!(
            "Note: The nested Inverse has its own ID that we can't find with current implementation"
        );

        // Test finding the nested ID
        let nested_id = "4ff3b55c-f418-4cdb-9ac8-4b492db7bdc6"; // This is the ID of the nested Inverse from the current output
        let result2 = located_operation.get_located::<GroupExpression>(nested_id.to_string());
        println!("get_located for nested ID {}: {:#?}", nested_id, result2);
    }
}
