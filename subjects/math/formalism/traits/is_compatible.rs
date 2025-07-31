use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::objects::MathObject;
use crate::subjects::math::formalism::proof::ContextEntry;
use crate::subjects::math::formalism::proof::tactics::Target;
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::traits::debug::ShortDebug;
use std::fmt::Debug;

pub trait IsCompatible<P> {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &P,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool;
}

impl IsCompatible<i32> for i32 {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &i32,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // For i32, compatibility is simple value equality
        self == pattern
    }
}

impl IsCompatible<MathExpression> for MathExpression {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (self, pattern) {
            (MathExpression::Object(self_obj), MathExpression::Object(pattern_obj)) => {
                self_obj.is_compatible(target_context, &pattern_obj, pattern_context)
            }
            (MathExpression::Relation(self_rel), MathExpression::Relation(pattern_rel)) => {
                self_rel.is_compatible(target_context, &pattern_rel, pattern_context)
            }
            (MathExpression::Expression(self_expr), MathExpression::Expression(pattern_expr)) => {
                self_expr.is_compatible(target_context, &pattern_expr, pattern_context)
            }
            _ => false,
        }
    }
}

impl<T: 'static + Clone + IsCompatible<T> + Debug> IsCompatible<Located<T>> for Located<T> {
    fn is_compatible(
        &self,
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
                self_concrete.is_compatible(target_context, pattern_concrete, pattern_context)
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

impl IsCompatible<MathRelation> for MathRelation {
    fn is_compatible(
        &self,
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
                left.is_compatible(target_context, &pattern_left, pattern_context)
                    && right.is_compatible(target_context, &pattern_right, pattern_context)
            }
            (MathRelation::And(locateds), MathRelation::And(pattern_locateds)) => {
                // todo: we should allow order difference, the list is cummutative and associative
                locateds.iter().all(|located| {
                    located.data.unwrap(&target_context).is_compatible(
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
                        target_context,
                        &located.data.unwrap(&pattern_context),
                        pattern_context,
                    )
                })
            }
            (MathRelation::Not(located), MathRelation::Not(pattern_located)) => {
                located.data.unwrap(&target_context).is_compatible(
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
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                ) && located1.data.unwrap(&target_context).is_compatible(
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
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                ) && located1.data.unwrap(&target_context).is_compatible(
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                ) && located1.data.unwrap(&target_context).is_compatible(
                    target_context,
                    &located1.data.unwrap(&pattern_context),
                    pattern_context,
                )
            }
            _ => false,
        }
    }
}

impl IsCompatible<MathObject> for MathObject {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &MathObject,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (self, pattern) {
            (MathObject::Group(self_group), MathObject::Group(pattern_group)) => {
                self_group.is_compatible(target_context, &pattern_group, pattern_context)
            }
            _ => false,
        }
    }
}

impl IsCompatible<TheoryExpression> for TheoryExpression {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &TheoryExpression,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // Deep structural compatibility check using StructurallyEquivalent trait

        match (self, pattern) {
            (TheoryExpression::Group(self_group), TheoryExpression::Group(pattern_group)) => {
                self_group.is_compatible(target_context, pattern_group, pattern_context)
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
