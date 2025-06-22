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
            NumberTheoryRelation::LessThan { left, right }
            | NumberTheoryRelation::GreaterThan { left, right }
            | NumberTheoryRelation::LessThanOrEqual { left, right }
            | NumberTheoryRelation::GreaterThanOrEqual { left, right } => {
                let mut path_l = base_path.clone();
                path_l.push(1);
                left.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                let mut path_r = base_path.clone();
                path_r.push(2);
                right.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
            }
            NumberTheoryRelation::Divides { divisor, dividend } => {
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
            NumberTheoryRelation::IsPrime { number }
            | NumberTheoryRelation::IsComposite { number } => {
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
            NumberTheoryRelation::LessThan { left, right } => match current_idx {
                1 => Ok(NumberTheoryRelation::LessThan {
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(NumberTheoryRelation::LessThan {
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::LessThanOrEqual { left, right } => match current_idx {
                1 => Ok(NumberTheoryRelation::LessThanOrEqual {
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(NumberTheoryRelation::LessThanOrEqual {
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::GreaterThan { left, right } => match current_idx {
                1 => Ok(NumberTheoryRelation::GreaterThan {
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(NumberTheoryRelation::GreaterThan {
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::GreaterThanOrEqual { left, right } => match current_idx {
                1 => Ok(NumberTheoryRelation::GreaterThanOrEqual {
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(NumberTheoryRelation::GreaterThanOrEqual {
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::Divides { divisor, dividend } => match current_idx {
                1 => Ok(NumberTheoryRelation::Divides {
                    divisor: divisor.replace_at_path(remaining_path, replacement)?,
                    dividend,
                }),
                2 => Ok(NumberTheoryRelation::Divides {
                    divisor,
                    dividend: dividend.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::Congruent {
                left,
                right,
                modulus,
            } => match current_idx {
                1 => Ok(NumberTheoryRelation::Congruent {
                    left: left.replace_at_path(remaining_path, replacement)?,
                    right,
                    modulus,
                }),
                2 => Ok(NumberTheoryRelation::Congruent {
                    left,
                    right: right.replace_at_path(remaining_path, replacement)?,
                    modulus,
                }),
                3 => Ok(NumberTheoryRelation::Congruent {
                    left,
                    right,
                    modulus: modulus.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::IsPrime { number } => match current_idx {
                1 => Ok(NumberTheoryRelation::IsPrime {
                    number: number.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            NumberTheoryRelation::IsComposite { number } => match current_idx {
                1 => Ok(NumberTheoryRelation::IsComposite {
                    number: number.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            // Handle other NumberTheoryRelation variants as needed
            _ => Err(PathError::NotImplemented),
        }
    }
}
