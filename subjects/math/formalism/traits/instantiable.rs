use crate::subjects::math::theories::groups::definitions::{
    GenericGroup, Group, GroupElement, GroupExpression, GroupHomomorphism, GroupOperation,
    GroupRelation,
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
        traits::{collect_identifier::CollectIdentifier, debug::ShortDebug, detag::TryDetag},
    },
    turn_render::Identifier,
};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

pub enum InstantiationType {
    Identifier(Identifier),
    LocatedId(String),
}

/// A trait for instantiating meta-variables by comparing a concrete expression (`self`) to a pattern.
/// we only map meta-variables in pattern to expression/variables in target, we will never allow pattern to have more
pub trait Instantiable: Sized {
    fn instantiate(
        &self, // The concrete target expression
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, InstantiationType>;
}

impl Instantiable for MathExpression {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, InstantiationType> {
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
    ) -> HashMap<Identifier, InstantiationType> {
        let mut instantiations = HashMap::new();

        match (&self.data, &pattern.data) {
            // Pattern is a variable, target is concrete - create instantiation
            (Parametrizable::Concrete(_), Parametrizable::Variable(pattern_var)) => {
                // ✅ FIXED: Always use the ID of the matched Located<> wrapper
                // This ensures proper recursive type matching during substitution
                instantiations.insert(
                    pattern_var.clone(),
                    InstantiationType::LocatedId(self.id.clone()),
                );
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
                // Different variables - create instantiation: pattern_var → target_var.body
                instantiations.insert(
                    pattern_var.clone(),
                    InstantiationType::Identifier(target_var.clone()),
                );
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
    ) -> HashMap<Identifier, InstantiationType> {
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
    ) -> HashMap<Identifier, InstantiationType> {
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
    ) -> HashMap<Identifier, InstantiationType> {
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
                instantiations.extend(target_rel.instantiate(
                    target_context,
                    pattern_rel,
                    pattern_context,
                ));
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
    ) -> HashMap<Identifier, InstantiationType> {
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
    ) -> HashMap<Identifier, InstantiationType> {
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

// Implementation for Option<T>
impl<T: Instantiable> Instantiable for Option<T> {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, InstantiationType> {
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
    ) -> HashMap<Identifier, InstantiationType> {
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

// Implementation for GroupElement
impl Instantiable for crate::subjects::math::theories::groups::definitions::GroupElement {
    fn instantiate(
        &self,
        _target_context: &Vec<ContextEntry>,
        _pattern: &Self,
        _pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, InstantiationType> {
        // GroupElement is a concrete type with no variables
        HashMap::new()
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
    ) -> HashMap<Identifier, InstantiationType> {
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
    ) -> HashMap<Identifier, InstantiationType> {
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
    ) -> HashMap<Identifier, InstantiationType> {
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
