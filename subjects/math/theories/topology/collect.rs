use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::proof::path_index::{PathError, ReplaceableAtPath};
use crate::subjects::math::formalism::proof::collect::CollectSubExpressions;
use crate::subjects::math::theories::topology::relations::TopologyRelation;

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
            TopologyRelation::IsOpen { entity, set, space } => match current_idx {
                1 => Ok(TopologyRelation::IsOpen {
                    entity,
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsOpen {
                    entity,
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsClosed { entity, set, space } => match current_idx {
                1 => Ok(TopologyRelation::IsClosed {
                    entity,
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsClosed {
                    entity,
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsNeighborhood {
                entity,
                set,
                point,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsNeighborhood {
                    entity,
                    set: set.replace_at_path(remaining_path, replacement)?,
                    point,
                    space,
                }),
                2 => Ok(TopologyRelation::IsNeighborhood {
                    entity,
                    set,
                    point: point.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::IsNeighborhood {
                    entity,
                    set,
                    point,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsBasis {
                entity,
                collection,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsBasis {
                    entity,
                    collection: collection.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsBasis {
                    entity,
                    collection,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsClosure {
                entity,
                closure,
                set,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsClosure {
                    entity,
                    closure: closure.replace_at_path(remaining_path, replacement)?,
                    set,
                    space,
                }),
                2 => Ok(TopologyRelation::IsClosure {
                    entity,
                    closure,
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::IsClosure {
                    entity,
                    closure,
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsInterior {
                entity,
                interior,
                set,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsInterior {
                    entity,
                    interior: interior.replace_at_path(remaining_path, replacement)?,
                    set,
                    space,
                }),
                2 => Ok(TopologyRelation::IsInterior {
                    entity,
                    interior,
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::IsInterior {
                    entity,
                    interior,
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsBoundary {
                entity,
                boundary,
                set,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsBoundary {
                    entity,
                    boundary: boundary.replace_at_path(remaining_path, replacement)?,
                    set,
                    space,
                }),
                2 => Ok(TopologyRelation::IsBoundary {
                    entity,
                    boundary,
                    set: set.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::IsBoundary {
                    entity,
                    boundary,
                    set,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsConnected { entity, space } => match current_idx {
                1 => Ok(TopologyRelation::IsConnected {
                    entity,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsPathConnected { entity, space } => match current_idx {
                1 => Ok(TopologyRelation::IsPathConnected {
                    entity,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsCompact { entity, space } => match current_idx {
                1 => Ok(TopologyRelation::IsCompact {
                    entity,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsHausdorff { entity, space } => match current_idx {
                1 => Ok(TopologyRelation::IsHausdorff {
                    entity,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsLocallyCompact { entity, space } => match current_idx {
                1 => Ok(TopologyRelation::IsLocallyCompact {
                    entity,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsParacompact { entity, space } => match current_idx {
                1 => Ok(TopologyRelation::IsParacompact {
                    entity,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsMetrizable { entity, space } => match current_idx {
                1 => Ok(TopologyRelation::IsMetrizable {
                    entity,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsContinuous {
                entity,
                function,
                domain,
                codomain,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsContinuous {
                    entity,
                    function: function.replace_at_path(remaining_path, replacement)?,
                    domain,
                    codomain,
                }),
                2 => Ok(TopologyRelation::IsContinuous {
                    entity,
                    function,
                    domain: domain.replace_at_path(remaining_path, replacement)?,
                    codomain,
                }),
                3 => Ok(TopologyRelation::IsContinuous {
                    entity,
                    function,
                    domain,
                    codomain: codomain.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsHomeomorphism {
                entity,
                function,
                domain,
                codomain,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsHomeomorphism {
                    entity,
                    function: function.replace_at_path(remaining_path, replacement)?,
                    domain,
                    codomain,
                }),
                2 => Ok(TopologyRelation::IsHomeomorphism {
                    entity,
                    function,
                    domain: domain.replace_at_path(remaining_path, replacement)?,
                    codomain,
                }),
                3 => Ok(TopologyRelation::IsHomeomorphism {
                    entity,
                    function,
                    domain,
                    codomain: codomain.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::AreHomeomorphic {
                entity,
                first,
                second,
            } => match current_idx {
                1 => Ok(TopologyRelation::AreHomeomorphic {
                    entity,
                    first: first.replace_at_path(remaining_path, replacement)?,
                    second,
                }),
                2 => Ok(TopologyRelation::AreHomeomorphic {
                    entity,
                    first,
                    second: second.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::Converges {
                entity,
                sequence,
                limit,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::Converges {
                    entity,
                    sequence: sequence.replace_at_path(remaining_path, replacement)?,
                    limit,
                    space,
                }),
                2 => Ok(TopologyRelation::Converges {
                    entity,
                    sequence,
                    limit: limit.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                3 => Ok(TopologyRelation::Converges {
                    entity,
                    sequence,
                    limit,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsSubspace {
                entity,
                subspace,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsSubspace {
                    entity,
                    subspace: subspace.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsSubspace {
                    entity,
                    subspace,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::IsOpenCover {
                entity,
                cover,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::IsOpenCover {
                    entity,
                    cover: cover.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::IsOpenCover {
                    entity,
                    cover,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::HasFiniteSubcover {
                entity,
                cover,
                space,
            } => match current_idx {
                1 => Ok(TopologyRelation::HasFiniteSubcover {
                    entity,
                    cover: cover.replace_at_path(remaining_path, replacement)?,
                    space,
                }),
                2 => Ok(TopologyRelation::HasFiniteSubcover {
                    entity,
                    cover,
                    space: space.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            TopologyRelation::Custom {
                entity,
                name,
                mut parameters,
            } => {
                if current_idx >= 100 {
                    let vec_idx = current_idx - 100;
                    if vec_idx < parameters.len() {
                        parameters[vec_idx] = parameters[vec_idx]
                            .clone()
                            .replace_at_path(remaining_path, replacement)?;
                        Ok(TopologyRelation::Custom {
                            entity,
                            name,
                            parameters,
                        })
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
