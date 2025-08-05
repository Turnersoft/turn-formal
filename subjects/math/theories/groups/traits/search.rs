use std::collections::HashSet;
use std::sync::Arc;

use crate::subjects::math::formalism::{
    expressions::{MathExpression, TheoryExpression},
    extract::Parametrizable,
    location::Located,
    proof::{ContextEntry, tactics::Target},
    traits::{IsCompatible, Search, detag::TryDetag},
};

use super::super::definitions::{
    GenericGroup, Group, GroupAction, GroupElement, GroupExpression, GroupHomomorphism,
    TopologicalGroup, TrivialGroup,
};

impl Search for GroupHomomorphism {
    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
        // GroupHomomorphism is a terminal type and doesn't contain Located<T> elements
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
        // GroupHomomorphism is a terminal type - it doesn't recursively contain expressions to search
        HashSet::new()
    }
}

impl IsCompatible<GroupHomomorphism> for GroupHomomorphism {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &GroupHomomorphism,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // For now, consider GroupHomomorphisms compatible if they have the same structure
        // This could be enhanced with more sophisticated checking
        self.domain.data.unwrap(target_context).is_compatible(
            target_context,
            &pattern.domain.data.unwrap(pattern_context),
            pattern_context,
        ) && self.codomain.data.unwrap(target_context).is_compatible(
            target_context,
            &pattern.codomain.data.unwrap(pattern_context),
            pattern_context,
        )
    }
}

impl Search for GroupAction {
    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
        // GroupAction is a terminal type and doesn't contain Located<T> elements
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
        // GroupAction is a terminal type - it doesn't recursively contain expressions to search
        HashSet::new()
    }
}

impl IsCompatible<GroupAction> for GroupAction {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &GroupAction,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // GroupAction compatibility - for now, basic structural comparison
        // This could be enhanced with more sophisticated checking
        todo!()
    }
}

impl Search for GroupElement {
    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
        // GroupElement is a terminal type and doesn't contain Located<T> elements
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

        match pattern.concrete_value() {
            Some(concrete_pattern) => {
                if let Ok(group_elem_pattern) = concrete_pattern.try_detag() {
                    let is_in_scope_now = in_target_scope || current_id == target.id;

                    if is_in_scope_now {
                        if (*self).is_compatible(
                            target_context,
                            group_elem_pattern,
                            pattern_context,
                        ) {
                            matches.insert(current_id.clone());
                        }
                    }
                }
            }
            None => {
                // Pattern is a variable - terminal type doesn't recursively match
            }
        }

        matches
    }
}

impl IsCompatible<GroupElement> for GroupElement {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &GroupElement,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // For GroupElement, we do direct comparison since they're concrete values
        match (self, pattern) {
            (GroupElement::Integer(a), GroupElement::Integer(b)) => a == b,
            (GroupElement::Permutation(a), GroupElement::Permutation(b)) => a == b,
            (GroupElement::Matrix(a), GroupElement::Matrix(b)) => a == b,
            (GroupElement::Symbol(a), GroupElement::Symbol(b)) => a == b,
            _ => false, // Different variants are not compatible
        }
    }
}

impl Search for Group {
    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
        // First, try to detag self to see if it matches the requested type T
        if let Ok(detagged_self) = TryDetag::<T>::try_detag(self) {
            return Some(Located {
                id: target,
                data: Parametrizable::Concrete(Arc::new((*detagged_self).clone())),
            });
        }

        // If direct detagging doesn't work, return None
        // The parent wrapper (like MathObject) should handle further type matching
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

        match pattern.concrete_value() {
            Some(concrete_pattern) => {
                if let Ok(group_pattern) = concrete_pattern.try_detag() {
                    let is_in_scope_now = in_target_scope || current_id == target.id;

                    if is_in_scope_now {
                        if (*self).is_compatible(target_context, group_pattern, pattern_context) {
                            matches.insert(current_id.clone());
                        }
                    }
                }
            }
            None => {
                // Pattern is a variable - could match this group
                // let is_in_scope_now = in_target_scope || current_id == target.id;
                // if is_in_scope_now {
                //     matches.insert(current_id.clone());
                // }
            }
        }

        matches
    }
}

// this impl of the enum decides we don't need to impl Search for inner types anymore.
impl IsCompatible<Group> for Group {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Group,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (self, pattern) {
            (Group::Generic(l), Group::Generic(r)) => {
                l.is_compatible(target_context, &r, pattern_context)
            }
            (Group::Trivial(l), Group::Trivial(r)) => {
                l.is_compatible(target_context, &r, pattern_context)
            }
            _ => false,
        }
    }
}

impl IsCompatible<GenericGroup> for GenericGroup {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &GenericGroup,
        _pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // For one group to be compatible with a more general pattern,
        // three conditions must be met.

        // 1. The core operations must be identical.
        if self.operation != pattern.operation {
            return false;
        }

        // 2. The pattern's properties must be a subset of the target's properties.
        // This allows a general `Group` pattern to match a specific `AbelianGroup` target.
        if !pattern.props.is_subset(&self.props) {
            return false;
        }

        // 3. The target's base set must be a subset of the pattern's base set.
        // This allows a pattern over `Integers` to match a target over `Even Integers`.
        if !self.base_set.is_subset_of(&pattern.base_set) {
            return false;
        }

        true
    }
}

impl IsCompatible<TrivialGroup> for TrivialGroup {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &TrivialGroup,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        todo!()
    }
}

impl IsCompatible<TopologicalGroup> for TopologicalGroup {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &TopologicalGroup,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // 1. Check the core group structures.
        if !self
            .core
            .is_compatible(target_context, &pattern.core, pattern_context)
        {
            return false;
        }

        // 2. Check the topological space structures.
        if !self
            .topology
            .is_compatible(target_context, &pattern.topology, pattern_context)
        {
            return false;
        }

        // 3. Check the topological group properties.
        if !pattern.props.is_subset(&self.props) {
            return false;
        }

        true
    }
}

impl Search for GroupExpression {
    fn get_located<T: 'static + Clone + std::fmt::Debug>(
        &self,
        target: String,
    ) -> Option<Located<T>> {
        match self {
            GroupExpression::Operation { group, left, right } => group
                .get_located(target.clone())
                .or_else(|| left.get_located(target.clone()))
                .or_else(|| right.get_located(target)),
            GroupExpression::Element { group, element } => {
                group.get_located(target.clone()).or_else(|| {
                    if let Some(elem) = element {
                        elem.get_located(target)
                    } else {
                        None
                    }
                })
            }
            GroupExpression::Identity(group) => group.get_located(target),
            GroupExpression::Inverse { group, element } => group
                .get_located(target.clone())
                .or_else(|| element.get_located(target)),
            GroupExpression::Power {
                group,
                base,
                exponent,
            } => group
                .get_located(target.clone())
                .or_else(|| base.get_located(target.clone()))
                .or_else(|| exponent.get_located(target)),
            GroupExpression::Commutator { group, a, b } => group
                .get_located(target.clone())
                .or_else(|| a.get_located(target.clone()))
                .or_else(|| b.get_located(target)),
            GroupExpression::Coset {
                group,
                element,
                subgroup,
                ..
            } => group
                .get_located(target.clone())
                .or_else(|| element.get_located(target.clone()))
                .or_else(|| subgroup.get_located(target)),
            GroupExpression::GroupOrder { group } => group.get_located(target),
            GroupExpression::ElementOrder { element, group } => element
                .get_located(target.clone())
                .or_else(|| group.get_located(target)),
            GroupExpression::Homomorphism(hom) => hom.get_located(target),
            GroupExpression::ActionOnElement { action, element } => action
                .get_located(target.clone())
                .or_else(|| element.get_located(target)),
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

        // Only check compatibility if the pattern is also an expression, todo: this check is redundant to the parent check
        // if is_in_scope_now {
        //     match pattern.concrete_value() {
        //         Some(concrete_pattern) => {
        //             if let Ok(pattern_expr) = concrete_pattern.try_detag() {
        //                 if self.is_compatible(target_context, &pattern_expr, pattern_context) {
        //                     matches.insert(current_id.clone());
        //                 }
        //             }
        //         }
        //         None => {
        //             // Pattern is a variable - could match this expression
        //             // matches.insert(current_id.clone());
        //         }
        //     }
        // }
        if is_in_scope_now {
            if let Ok(pattern_expr) = pattern.data.unwrap(&pattern_context).try_detag() {
                if self.is_compatible(target_context, &pattern_expr, pattern_context) {
                    matches.insert(current_id.clone());
                }
            }
        }

        // ✅ CRITICAL FIX: Add recursive search within GroupExpression structure
        let sub_matches = match self {
            GroupExpression::Operation { group, left, right } => {
                let mut operation_matches = HashSet::new();

                // Search within the group
                operation_matches.extend(group.find_matches(
                    target.clone(),
                    group.id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                ));

                // Search within the left operand
                operation_matches.extend(left.find_matches(
                    target.clone(),
                    left.id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                ));

                // Search within the right operand
                operation_matches.extend(right.find_matches(
                    target.clone(),
                    right.id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                ));

                operation_matches
            }
            GroupExpression::Element { group, element } => {
                let mut element_matches = HashSet::new();
                element_matches.extend(group.find_matches(
                    target.clone(),
                    group.id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                ));
                if let Some(elem) = element {
                    element_matches.extend(elem.find_matches(
                        target.clone(),
                        elem.id.clone(),
                        target_context,
                        pattern,
                        pattern_context,
                        is_in_scope_now,
                    ));
                }
                element_matches
            }
            GroupExpression::Identity(group) => group.find_matches(
                target.clone(),
                group.id.clone(),
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            GroupExpression::Inverse { group, element } => {
                let mut inverse_matches = HashSet::new();
                inverse_matches.extend(group.find_matches(
                    target.clone(),
                    group.id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                ));
                inverse_matches.extend(element.find_matches(
                    target.clone(),
                    element.id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                ));
                inverse_matches
            }
            // Add recursive search for other variants as needed
            _ => HashSet::new(),
        };

        matches.extend(sub_matches);
        matches
    }
}

impl IsCompatible<GroupExpression> for GroupExpression {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &GroupExpression,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (self, pattern) {
            (GroupExpression::Identity(l), GroupExpression::Identity(r)) => {
                l.data.unwrap(target_context).is_compatible(
                    target_context,
                    &r.data.unwrap(target_context),
                    pattern_context,
                )
            }
            (
                GroupExpression::Inverse { group, element },
                GroupExpression::Inverse {
                    group: r_group,
                    element: r_element,
                },
            ) => group.data.unwrap(target_context).is_compatible(
                target_context,
                &r_group.data.unwrap(target_context),
                pattern_context,
            ),
            (
                GroupExpression::Operation { group, left, right },
                GroupExpression::Operation {
                    group: r_group,
                    left: r_left,
                    right: r_right,
                },
            ) => {
                let group_compatible = group.data.unwrap(target_context).is_compatible(
                    target_context,
                    &r_group.data.unwrap(pattern_context),
                    pattern_context,
                );
                let left_compatible = left.data.unwrap(target_context).is_compatible(
                    target_context,
                    &r_left.data.unwrap(pattern_context),
                    pattern_context,
                );
                let right_compatible = right.data.unwrap(target_context).is_compatible(
                    target_context,
                    &r_right.data.unwrap(pattern_context),
                    pattern_context,
                );
                group_compatible && left_compatible && right_compatible
                // group_compatible
            }
            (
                GroupExpression::Element {
                    group: l_group,
                    element: l_element,
                },
                GroupExpression::Element {
                    group: r_group,
                    element: r_element,
                },
            ) => l_group.data.unwrap(target_context).is_compatible(
                target_context,
                &r_group.data.unwrap(target_context),
                pattern_context,
            ),
            (
                GroupExpression::Inverse { group, element },
                GroupExpression::Element {
                    group: r_group,
                    element: r_element,
                },
            ) => {
                let group_compatible = group.data.unwrap(target_context).is_compatible(
                    target_context,
                    &r_group.data.unwrap(pattern_context),
                    pattern_context,
                );
                group_compatible
            }
            _ => {
                // println!(
                //     "unimplemented, trying to compare {:#?} with {:#?}",
                //     self, pattern
                // );
                false
            }
        }
    }
}
