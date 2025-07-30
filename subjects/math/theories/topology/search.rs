use crate::subjects::math::formalism::proof::ContextEntry;
use crate::subjects::math::formalism::traits::IsCompatible;

use super::definitions::{TopologicalSpace, TopologicalSpaceProperty, Topology};
use std::collections::HashSet;

impl IsCompatible<Topology> for Topology {
    fn is_compatible(
        &self,
        _target_context: &Vec<ContextEntry>,
        pattern: &Topology,
        _pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // The pattern's properties must be a subset of the target's.
        pattern.properties.is_subset(&self.properties)
    }
}

impl IsCompatible<TopologicalSpace> for TopologicalSpace {
    fn is_compatible(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &TopologicalSpace,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        // 1. Check base sets.
        if !self.base_set.is_subset_of(&pattern.base_set) {
            return false;
        }

        // 2. Check the topologies themselves.
        if !self
            .topology
            .is_compatible(target_context, &pattern.topology, pattern_context)
        {
            return false;
        }

        // 3. Check the topological space properties.
        let self_props: HashSet<_> = self.properties.iter().collect();
        let pattern_props: HashSet<_> = pattern.properties.iter().collect();
        if !pattern_props.is_subset(&self_props) {
            return false;
        }

        true
    }
}
