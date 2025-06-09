use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::proof::collect::CollectSubExpressions;
use crate::subjects::math::formalism::proof::path_index::{PathError, ReplaceableAtPath};
use crate::subjects::math::theories::topology::definitions::TopologyRelation;

impl TopologyRelation {
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
            TopologyRelation::IsOpen { set, space, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                set.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                space.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            TopologyRelation::IsClosed { set, space, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                set.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                space.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            TopologyRelation::IsNeighborhood {
                set, point, space, ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                set.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                point.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
                let mut path3 = base_path.clone();
                path3.push(3);
                space.collect_sub_expressions_with_paths(path3, collected_targets, depth + 1);
            }
            TopologyRelation::IsBasis {
                collection, space, ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                collection.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                space.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            TopologyRelation::IsClosure {
                closure,
                set,
                space,
                ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                closure.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                set.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
                let mut path3 = base_path.clone();
                path3.push(3);
                space.collect_sub_expressions_with_paths(path3, collected_targets, depth + 1);
            }
            TopologyRelation::IsInterior {
                interior,
                set,
                space,
                ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                interior.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                set.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
                let mut path3 = base_path.clone();
                path3.push(3);
                space.collect_sub_expressions_with_paths(path3, collected_targets, depth + 1);
            }
            TopologyRelation::IsBoundary {
                boundary,
                set,
                space,
                ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                boundary.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                set.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
                let mut path3 = base_path.clone();
                path3.push(3);
                space.collect_sub_expressions_with_paths(path3, collected_targets, depth + 1);
            }
            TopologyRelation::IsConnected { space, .. }
            | TopologyRelation::IsPathConnected { space, .. }
            | TopologyRelation::IsCompact { space, .. }
            | TopologyRelation::IsHausdorff { space, .. }
            | TopologyRelation::IsLocallyCompact { space, .. }
            | TopologyRelation::IsParacompact { space, .. }
            | TopologyRelation::IsMetrizable { space, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                space.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
            }
            TopologyRelation::IsContinuous {
                function,
                domain,
                codomain,
                ..
            }
            | TopologyRelation::IsHomeomorphism {
                function,
                domain,
                codomain,
                ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                function.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                domain.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
                let mut path3 = base_path.clone();
                path3.push(3);
                codomain.collect_sub_expressions_with_paths(path3, collected_targets, depth + 1);
            }
            TopologyRelation::AreHomeomorphic { first, second, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                first.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                second.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            TopologyRelation::Converges {
                sequence,
                limit,
                space,
                ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                sequence.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                limit.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
                let mut path3 = base_path.clone();
                path3.push(3);
                space.collect_sub_expressions_with_paths(path3, collected_targets, depth + 1);
            }
            TopologyRelation::IsSubspace {
                subspace, space, ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                subspace.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                space.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            TopologyRelation::IsOpenCover { cover, space, .. }
            | TopologyRelation::HasFiniteSubcover { cover, space, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                cover.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                space.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            TopologyRelation::Custom { parameters, .. } => {
                for (i, expr) in parameters.iter().enumerate() {
                    let mut path = base_path.clone();
                    path.push(100 + i);
                    expr.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
                }
            }
        }
    }
}

impl ReplaceableAtPath for TopologyRelation {
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
            TopologyRelation::IsOpen { set, space } => match current_idx {
                1 => Ok(TopologyRelation::IsOpen {
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsOpen {
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsClosed { set, space } => match current_idx {
                1 => Ok(TopologyRelation::IsClosed {
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsClosed {
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsNeighborhood { set, point, space } => match current_idx {
                1 => Ok(TopologyRelation::IsNeighborhood {
                    set: set.replace_at_path(remaining_path, replacement)?,
                    point,
                    space,
                }),
                2 => Ok(TopologyRelation::IsNeighborhood {
                    set,
                    point: point.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::IsNeighborhood {
                    set,
                    point,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsBasis { collection, space } => match current_idx {
                1 => Ok(TopologyRelation::IsBasis {
                    collection: collection.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsBasis {
                    collection,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsClosure {
                closure,
                set,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsClosure {
                    closure: closure.replace_at_path(remaining_path, replacement)?,
                    set,
                    space,
                }),
                2 => Ok(TopologyRelation::IsClosure {
                    closure,
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::IsClosure {
                    closure,
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsInterior {
                interior,
                set,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsInterior {
                    interior: interior.replace_at_path(remaining_path, replacement)?,
                    set,
                    space,
                }),
                2 => Ok(TopologyRelation::IsInterior {
                    interior,
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::IsInterior {
                    interior,
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsBoundary {
                boundary,
                set,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsBoundary {
                    boundary: boundary.replace_at_path(remaining_path, replacement)?,
                    set,
                    space,
                }),
                2 => Ok(TopologyRelation::IsBoundary {
                    boundary,
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::IsBoundary {
                    boundary,
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsConnected { space } => match current_idx {
                1 => Ok(TopologyRelation::IsConnected {
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsPathConnected { space } => match current_idx {
                1 => Ok(TopologyRelation::IsPathConnected {
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsCompact { space } => match current_idx {
                1 => Ok(TopologyRelation::IsCompact {
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsHausdorff { space } => match current_idx {
                1 => Ok(TopologyRelation::IsHausdorff {
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsLocallyCompact { space } => match current_idx {
                1 => Ok(TopologyRelation::IsLocallyCompact {
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsParacompact { space } => match current_idx {
                1 => Ok(TopologyRelation::IsParacompact {
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsMetrizable { space } => match current_idx {
                1 => Ok(TopologyRelation::IsMetrizable {
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsContinuous {
                function,
                domain,
                codomain,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsContinuous {
                    function: function.replace_at_path(remaining_path, replacement)?,
                    domain,
                    codomain,
                }),
                2 => Ok(TopologyRelation::IsContinuous {
                    function,
                    domain: domain.replace_at_path(remaining_path, replacement)?,
                    codomain,
                }),
                3 => Ok(TopologyRelation::IsContinuous {
                    function,
                    domain,
                    codomain: codomain.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsHomeomorphism {
                function,
                domain,
                codomain,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsHomeomorphism {
                    function: function.replace_at_path(remaining_path, replacement)?,
                    domain,
                    codomain,
                }),
                2 => Ok(TopologyRelation::IsHomeomorphism {
                    function,
                    domain: domain.replace_at_path(remaining_path, replacement)?,
                    codomain,
                }),
                3 => Ok(TopologyRelation::IsHomeomorphism {
                    function,
                    domain,
                    codomain: codomain.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::AreHomeomorphic { first, second } => match current_idx {
                1 => Ok(TopologyRelation::AreHomeomorphic {
                    first: first.replace_at_path(remaining_path, replacement)?,
                    second,
                }),
                2 => Ok(TopologyRelation::AreHomeomorphic {
                    first,
                    second: second.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::Converges {
                sequence,
                limit,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::Converges {
                    sequence: sequence.replace_at_path(remaining_path, replacement)?,
                    limit,
                    space,
                }),
                2 => Ok(TopologyRelation::Converges {
                    sequence,
                    limit: limit.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::Converges {
                    sequence,
                    limit,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsSubspace { subspace, space } => match current_idx {
                1 => Ok(TopologyRelation::IsSubspace {
                    subspace: subspace.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsSubspace {
                    subspace,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsOpenCover { cover, space } => match current_idx {
                1 => Ok(TopologyRelation::IsOpenCover {
                    cover: cover.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsOpenCover {
                    cover,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::HasFiniteSubcover { cover, space } => match current_idx {
                1 => Ok(TopologyRelation::HasFiniteSubcover {
                    cover: cover.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::HasFiniteSubcover {
                    cover,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::Custom {
                name,
                mut parameters,
            } => {
                if current_idx >= 100 {
                    let vec_idx = current_idx - 100;
                    if vec_idx < parameters.len() {
                        parameters[vec_idx] = parameters[vec_idx]
                            .clone()
                            .replace_at_path(remaining_path, replacement)?;
                        Ok(TopologyRelation::Custom { name, parameters })
                    } else {
                        Err(PathError::InvalidPath)
                    }
                } else {
                    Err(PathError::InvalidPath)
                }
            }
        }
    }
}
