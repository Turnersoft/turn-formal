use crate::subjects::math::formalism::{
    expressions::MathExpression,
    proof::{ContextEntry, tactics::Target},
    search::{IsCompatible, Search},
};

use super::definitions::{GenericGroup, Group, GroupExpression, TopologicalGroup, TrivialGroup};

impl Search for Group {
    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> Vec<String> {
        let group_pattern = pattern.get_object().unwrap().get_group().unwrap();
        let mut matches = Vec::new();
        let is_in_scope_now = in_target_scope || current_id == target.id;

        if is_in_scope_now {
            if (*self).is_compatible(
                target.clone(),
                target_context,
                &group_pattern,
                pattern_context,
            ) {
                matches.push(current_id.clone());
            }
        }

        matches
    }
}

// this impl of the enum decides we don't need to impl Search for inner types anymore.
impl IsCompatible<Group> for Group {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &Group,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (self, pattern) {
            (Group::Generic(l), Group::Generic(r)) => {
                l.is_compatible(target.clone(), target_context, &r, pattern_context)
            }
            (Group::Trivial(l), Group::Trivial(r)) => {
                l.is_compatible(target.clone(), target_context, &r, pattern_context)
            }
            _ => false,
        }
    }
}

impl IsCompatible<GenericGroup> for GenericGroup {
    fn is_compatible(
        &self,
        target: Target,
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
        target: Target,
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
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &TopologicalGroup,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // 1. Check the core group structures.
        if !self.core.is_compatible(
            target.clone(),
            target_context,
            &pattern.core,
            pattern_context,
        ) {
            return false;
        }

        // 2. Check the topological space structures.
        if !self.topology.is_compatible(
            target.clone(),
            target_context,
            &pattern.topology,
            pattern_context,
        ) {
            return false;
        }

        // 3. Check the topological group properties.
        if !pattern.props.is_subset(&self.props) {
            return false;
        }

        true
    }
}

// impl Search for GroupExpression {
//     fn find_matches(
//         &self,
//         target_id: String,
//         current_id: String,
//         target_context: &Vec<ContextEntry>,
//         pattern: &MathExpression,
//         pattern_context: &Vec<ContextEntry>,
//         in_target_scope: bool,
//     ) -> Vec<String> {
//         match (self, with) {
//             (GroupExpression::Identity(l), GroupExpression::Identity(r)) => {
//                 vec![]
//             }
//             _ => vec![],
//         }
//     }
// }
