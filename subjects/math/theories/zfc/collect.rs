use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::proof::collect::CollectSubExpressions;
use crate::subjects::math::formalism::proof::path_index::{PathError, ReplaceableAtPath};
use crate::subjects::math::theories::zfc::relations::SetTheoryRelation;

impl SetTheoryRelation {
    pub fn collect_contained_expressions(
        &self,
        base_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        if depth > 100 {
            return;
        }
        match self {
            SetTheoryRelation::ElementOf {
                entity: _,
                element,
                set,
            } => {
                let mut path_e = base_path.clone();
                path_e.push(1);
                element.collect_sub_expressions_with_paths(path_e, collected_targets, depth + 1);
                let mut path_s = base_path.clone();
                path_s.push(2);
                set.collect_sub_expressions_with_paths(path_s, collected_targets, depth + 1);
            }
            SetTheoryRelation::SubsetOf {
                entity: _,
                subset,
                superset,
            } => {
                let mut path_sub = base_path.clone();
                path_sub.push(1);
                subset.collect_sub_expressions_with_paths(path_sub, collected_targets, depth + 1);
                let mut path_super = base_path.clone();
                path_super.push(2);
                superset.collect_sub_expressions_with_paths(
                    path_super,
                    collected_targets,
                    depth + 1,
                );
            }
            SetTheoryRelation::Equal {
                entity: _,
                left,
                right,
            } => {
                let mut path_l = base_path.clone();
                path_l.push(1);
                left.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                let mut path_r = base_path.clone();
                path_r.push(2);
                right.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
            }
            SetTheoryRelation::Disjoint {
                entity: _,
                first,
                second,
            } => {
                let mut path_f = base_path.clone();
                path_f.push(1);
                first.collect_sub_expressions_with_paths(path_f, collected_targets, depth + 1);
                let mut path_s = base_path.clone();
                path_s.push(2);
                second.collect_sub_expressions_with_paths(path_s, collected_targets, depth + 1);
            }
            SetTheoryRelation::Union {
                entity: _,
                result,
                first,
                second,
            } => {
                let mut path_res = base_path.clone();
                path_res.push(1);
                result.collect_sub_expressions_with_paths(path_res, collected_targets, depth + 1);
                let mut path_f = base_path.clone();
                path_f.push(2);
                first.collect_sub_expressions_with_paths(path_f, collected_targets, depth + 1);
                let mut path_s = base_path.clone();
                path_s.push(3);
                second.collect_sub_expressions_with_paths(path_s, collected_targets, depth + 1);
            }
            SetTheoryRelation::Intersection {
                entity: _,
                result,
                first,
                second,
            } => {
                let mut path_res = base_path.clone();
                path_res.push(1);
                result.collect_sub_expressions_with_paths(path_res, collected_targets, depth + 1);
                let mut path_f = base_path.clone();
                path_f.push(2);
                first.collect_sub_expressions_with_paths(path_f, collected_targets, depth + 1);
                let mut path_s = base_path.clone();
                path_s.push(3);
                second.collect_sub_expressions_with_paths(path_s, collected_targets, depth + 1);
            }
            // Handle other SetTheoryRelation variants as needed
            _ => {}
        }
    }
}

impl ReplaceableAtPath for SetTheoryRelation {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            return Err(PathError::TypeMismatch);
        }

        let current_idx = path[0];
        let remaining_path = &path[1..];

        match self {
            SetTheoryRelation::ElementOf {
                entity,
                element,
                set,
            } => match current_idx {
                1 => Ok(SetTheoryRelation::ElementOf {
                    entity,
                    element: element.replace_at_path(remaining_path, replacement)?,
                    set,
                }),
                2 => Ok(SetTheoryRelation::ElementOf {
                    entity,
                    element,
                    set: set.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            SetTheoryRelation::SubsetOf {
                entity,
                subset,
                superset,
            } => match current_idx {
                1 => Ok(SetTheoryRelation::SubsetOf {
                    entity,
                    subset: subset.replace_at_path(remaining_path, replacement)?,
                    superset,
                }),
                2 => Ok(SetTheoryRelation::SubsetOf {
                    entity,
                    subset,
                    superset: superset.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            SetTheoryRelation::Equal {
                entity,
                left,
                right,
            } => match current_idx {
                1 => Ok(SetTheoryRelation::Equal {
                    entity,
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(SetTheoryRelation::Equal {
                    entity,
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            SetTheoryRelation::Disjoint {
                entity,
                first,
                second,
            } => match current_idx {
                1 => Ok(SetTheoryRelation::Disjoint {
                    entity,
                    first: first.replace_at_path(remaining_path, replacement)?,
                    second,
                }),
                2 => Ok(SetTheoryRelation::Disjoint {
                    entity,
                    first,
                    second: second.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            // Handle other SetTheoryRelation variants as needed
            _ => Err(PathError::NotImplemented),
        }
    }
}
