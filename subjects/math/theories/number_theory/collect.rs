use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::proof::collect::CollectSubExpressions;
use crate::subjects::math::formalism::proof::path_index::{PathError, ReplaceableAtPath};
use crate::subjects::math::theories::number_theory::definitions::NumberTheoryRelation;

impl NumberTheoryRelation {
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
            NumberTheoryRelation::LessThan {
                entity: _,
                left,
                right,
            }
            | NumberTheoryRelation::GreaterThan {
                entity: _,
                left,
                right,
            }
            | NumberTheoryRelation::LessThanOrEqual {
                entity: _,
                left,
                right,
            }
            | NumberTheoryRelation::GreaterThanOrEqual {
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
            NumberTheoryRelation::Divides {
                entity: _,
                divisor,
                dividend,
            } => {
                let mut path_divisor = base_path.clone();
                path_divisor.push(1); // divisor is field 1
                divisor.collect_sub_expressions_with_paths(
                    path_divisor,
                    collected_targets,
                    depth + 1,
                );
                let mut path_dividend = base_path.clone();
                path_dividend.push(2); // dividend is field 2
                dividend.collect_sub_expressions_with_paths(
                    path_dividend,
                    collected_targets,
                    depth + 1,
                );
            }
            NumberTheoryRelation::Congruent {
                entity: _,
                left,
                right,
                modulus,
            } => {
                let mut path_l = base_path.clone();
                path_l.push(1);
                left.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                let mut path_r = base_path.clone();
                path_r.push(2);
                right.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
                let mut path_m = base_path.clone();
                path_m.push(3);
                modulus.collect_sub_expressions_with_paths(path_m, collected_targets, depth + 1);
            }
            NumberTheoryRelation::IsPrime { entity: _, number }
            | NumberTheoryRelation::IsComposite { entity: _, number } => {
                let mut path_n = base_path.clone();
                path_n.push(1);
                number.collect_sub_expressions_with_paths(path_n, collected_targets, depth + 1);
            }
            // Handle other NumberTheoryRelation variants as needed
            _ => {}
        }
    }
}

impl ReplaceableAtPath for NumberTheoryRelation {
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
            NumberTheoryRelation::LessThan {
                entity,
                left,
                right,
            } => match current_idx {
                1 => Ok(NumberTheoryRelation::LessThan {
                    entity,
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(NumberTheoryRelation::LessThan {
                    entity,
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::LessThanOrEqual {
                entity,
                left,
                right,
            } => match current_idx {
                1 => Ok(NumberTheoryRelation::LessThanOrEqual {
                    entity,
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(NumberTheoryRelation::LessThanOrEqual {
                    entity,
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::GreaterThan {
                entity,
                left,
                right,
            } => match current_idx {
                1 => Ok(NumberTheoryRelation::GreaterThan {
                    entity,
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(NumberTheoryRelation::GreaterThan {
                    entity,
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::GreaterThanOrEqual {
                entity,
                left,
                right,
            } => match current_idx {
                1 => Ok(NumberTheoryRelation::GreaterThanOrEqual {
                    entity,
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(NumberTheoryRelation::GreaterThanOrEqual {
                    entity,
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::Divides {
                entity,
                divisor,
                dividend,
            } => match current_idx {
                1 => Ok(NumberTheoryRelation::Divides {
                    entity,
                    divisor: divisor.replace_at_path(remaining_path, replacement)?,
                    dividend,
                }),
                2 => Ok(NumberTheoryRelation::Divides {
                    entity,
                    divisor,
                    dividend: dividend.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::Congruent {
                entity,
                left,
                right,
                modulus,
            } => match current_idx {
                1 => Ok(NumberTheoryRelation::Congruent {
                    entity,
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                    modulus,
                }),
                2 => Ok(NumberTheoryRelation::Congruent {
                    entity,
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                    modulus,
                }),
                3 => Ok(NumberTheoryRelation::Congruent {
                    entity,
                    left,
                    right,
                    modulus: modulus.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::IsPrime { entity, number } => match current_idx {
                1 => Ok(NumberTheoryRelation::IsPrime {
                    entity,
                    number: number.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::IsComposite { entity, number } => match current_idx {
                1 => Ok(NumberTheoryRelation::IsComposite {
                    entity,
                    number: number.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            // Handle other NumberTheoryRelation variants as needed
            _ => Err(PathError::NotImplemented),
        }
    }
}
