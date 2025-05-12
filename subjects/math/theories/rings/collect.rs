use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::proof::path_index::{PathError, ReplaceableAtPath};
use crate::subjects::math::formalism::proof::collect::CollectSubExpressions;
use crate::subjects::math::theories::rings::definitions::{
    FieldExpression, RingExpression, RingRelation,
};

impl CollectSubExpressions for RingExpression {
    fn collect_sub_expressions_with_paths(
        &self,
        current_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        if depth > 100 {
            return;
        }
        // Add self first
        collected_targets.push((
            current_path.clone(),
            MathExpression::Expression(TheoryExpression::Ring(self.clone())),
        ));

        match self {
            RingExpression::Addition { left, right, .. } => {
                let mut path_l = current_path.clone();
                path_l.push(1);
                left.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                let mut path_r = current_path.clone();
                path_r.push(2);
                right.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
            }
            RingExpression::Multiplication { left, right, .. } => {
                let mut path_l = current_path.clone();
                path_l.push(1);
                left.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                let mut path_r = current_path.clone();
                path_r.push(2);
                right.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
            }
            RingExpression::AdditiveInverse { element, .. } => {
                let mut path_e = current_path.clone();
                path_e.push(1);
                element.collect_sub_expressions_with_paths(path_e, collected_targets, depth + 1);
            }
            RingExpression::Power { base, .. } => {
                let mut path_b = current_path.clone();
                path_b.push(1);
                base.collect_sub_expressions_with_paths(path_b, collected_targets, depth + 1);
            }
            // Element, Variable, Zero, One are leaf nodes in terms of containing *other* expressions
            _ => {}
        }
    }
}

impl CollectSubExpressions for FieldExpression {
    fn collect_sub_expressions_with_paths(
        &self,
        current_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        if depth > 100 {
            return;
        }
        // Add self first
        collected_targets.push((
            current_path.clone(),
            MathExpression::Expression(TheoryExpression::Field(self.clone())),
        ));

        match self {
            FieldExpression::Addition { left, right, .. } => {
                let mut path_l = current_path.clone();
                path_l.push(1);
                left.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                let mut path_r = current_path.clone();
                path_r.push(2);
                right.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
            }
            FieldExpression::Multiplication { left, right, .. } => {
                let mut path_l = current_path.clone();
                path_l.push(1);
                left.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                let mut path_r = current_path.clone();
                path_r.push(2);
                right.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
            }
            FieldExpression::Division {
                numerator,
                denominator,
                ..
            } => {
                let mut path_n = current_path.clone();
                path_n.push(1);
                numerator.collect_sub_expressions_with_paths(path_n, collected_targets, depth + 1);
                let mut path_d = current_path.clone();
                path_d.push(2);
                denominator.collect_sub_expressions_with_paths(
                    path_d,
                    collected_targets,
                    depth + 1,
                );
            }
            FieldExpression::AdditiveInverse { element, .. } => {
                let mut path_e = current_path.clone();
                path_e.push(1);
                element.collect_sub_expressions_with_paths(path_e, collected_targets, depth + 1);
            }
            FieldExpression::MultiplicativeInverse { element, .. } => {
                let mut path_e = current_path.clone();
                path_e.push(1);
                element.collect_sub_expressions_with_paths(path_e, collected_targets, depth + 1);
            }
            FieldExpression::Power { base, .. } => {
                let mut path_b = current_path.clone();
                path_b.push(1);
                base.collect_sub_expressions_with_paths(path_b, collected_targets, depth + 1);
            }
            // Element, Variable, Zero, One are leaf nodes
            _ => {}
        }
    }
}

impl RingRelation {
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
            RingRelation::IsSubringOf { subring, ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                subring.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                ring.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            RingRelation::IsIdealOf { ideal, ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                ideal.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                ring.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            RingRelation::IsPrimeIdeal { ideal, ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                ideal.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                ring.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            RingRelation::IsMaximalIdeal { ideal, ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                ideal.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                ring.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            RingRelation::IsPrincipalIdeal {
                ideal,
                ring,
                generator,
                ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                ideal.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                ring.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
                let mut path3 = base_path.clone();
                path3.push(3);
                generator.collect_sub_expressions_with_paths(path3, collected_targets, depth + 1);
            }
            RingRelation::IsUnit { element, ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                element.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                ring.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            RingRelation::IsIrreducible { element, ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                element.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                ring.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            RingRelation::IsPrime { element, ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                element.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                ring.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
            }
            RingRelation::IsField { ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                ring.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
            }
            RingRelation::IsIntegralDomain { ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                ring.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
            }
            RingRelation::IsUFD { ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                ring.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
            }
            RingRelation::IsPID { ring, .. } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                ring.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
            }
            RingRelation::AreAssociates {
                first,
                second,
                ring,
                ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                first.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                second.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
                let mut path3 = base_path.clone();
                path3.push(3);
                ring.collect_sub_expressions_with_paths(path3, collected_targets, depth + 1);
            }
            RingRelation::Divides {
                divisor,
                dividend,
                ring,
                ..
            } => {
                let mut path1 = base_path.clone();
                path1.push(1);
                divisor.collect_sub_expressions_with_paths(path1, collected_targets, depth + 1);
                let mut path2 = base_path.clone();
                path2.push(2);
                dividend.collect_sub_expressions_with_paths(path2, collected_targets, depth + 1);
                let mut path3 = base_path.clone();
                path3.push(3);
                ring.collect_sub_expressions_with_paths(path3, collected_targets, depth + 1);
            }
            RingRelation::Custom { parameters, .. } => {
                for (i, expr) in parameters.iter().enumerate() {
                    let mut path = base_path.clone();
                    path.push(100 + i);
                    expr.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
                }
            }
        }
    }
}

impl ReplaceableAtPath for RingExpression {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            // Attempt to replace self with a MathExpression::Expression(TheoryExpression::Ring(...))
            if let MathExpression::Expression(TheoryExpression::Ring(new_ring_exp)) = replacement {
                return Ok(new_ring_exp);
            } else {
                return Err(PathError::TypeMismatch);
            }
        }
        let current_idx = path[0];
        let remaining_path = &path[1..];

        match self {
            RingExpression::Addition { ring, left, right } => match current_idx {
                1 => Ok(RingExpression::Addition {
                    ring,
                    left: left.replace_at_path_recursive(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(RingExpression::Addition {
                    ring,
                    left,
                    right: right.replace_at_path_recursive(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingExpression::Multiplication { ring, left, right } => match current_idx {
                1 => Ok(RingExpression::Multiplication {
                    ring,
                    left: left.replace_at_path_recursive(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(RingExpression::Multiplication {
                    ring,
                    left,
                    right: right.replace_at_path_recursive(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingExpression::AdditiveInverse { ring, element } => {
                if current_idx == 1 {
                    Ok(RingExpression::AdditiveInverse {
                        ring,
                        element: element.replace_at_path_recursive(remaining_path, replacement)?,
                    })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            RingExpression::Power {
                ring,
                base,
                exponent,
            } => {
                if current_idx == 1 {
                    Ok(RingExpression::Power {
                        ring,
                        base: base.replace_at_path_recursive(remaining_path, replacement)?,
                        exponent,
                    })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            // Element, Variable, Zero, One cannot be descended into further via path
            RingExpression::Element(_)
            | RingExpression::Variable { .. }
            | RingExpression::Zero(_)
            | RingExpression::One(_) => Err(PathError::InvalidPath),
        }
    }
}

impl ReplaceableAtPath for FieldExpression {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            if let MathExpression::Expression(TheoryExpression::Field(new_field_exp)) = replacement
            {
                return Ok(new_field_exp);
            } else {
                return Err(PathError::TypeMismatch);
            }
        }
        let current_idx = path[0];
        let remaining_path = &path[1..];

        match self {
            FieldExpression::Addition { field, left, right } => match current_idx {
                1 => Ok(FieldExpression::Addition {
                    field,
                    left: left.replace_at_path_recursive(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(FieldExpression::Addition {
                    field,
                    left,
                    right: right.replace_at_path_recursive(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            FieldExpression::Multiplication { field, left, right } => match current_idx {
                1 => Ok(FieldExpression::Multiplication {
                    field,
                    left: left.replace_at_path_recursive(remaining_path, replacement)?,
                    right,
                }),
                2 => Ok(FieldExpression::Multiplication {
                    field,
                    left,
                    right: right.replace_at_path_recursive(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            FieldExpression::Division {
                field,
                numerator,
                denominator,
            } => match current_idx {
                1 => Ok(FieldExpression::Division {
                    field,
                    numerator: numerator.replace_at_path_recursive(remaining_path, replacement)?,
                    denominator,
                }),
                2 => Ok(FieldExpression::Division {
                    field,
                    numerator,
                    denominator: denominator
                        .replace_at_path_recursive(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            FieldExpression::AdditiveInverse { field, element } => {
                if current_idx == 1 {
                    Ok(FieldExpression::AdditiveInverse {
                        field,
                        element: element.replace_at_path_recursive(remaining_path, replacement)?,
                    })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            FieldExpression::MultiplicativeInverse { field, element } => {
                if current_idx == 1 {
                    Ok(FieldExpression::MultiplicativeInverse {
                        field,
                        element: element.replace_at_path_recursive(remaining_path, replacement)?,
                    })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            FieldExpression::Power {
                field,
                base,
                exponent,
            } => {
                if current_idx == 1 {
                    Ok(FieldExpression::Power {
                        field,
                        base: base.replace_at_path_recursive(remaining_path, replacement)?,
                        exponent,
                    })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            // Element, Variable, Zero, One cannot be descended into further via path
            FieldExpression::Element(_)
            | FieldExpression::Variable { .. }
            | FieldExpression::Zero(_)
            | FieldExpression::One(_) => Err(PathError::InvalidPath),
        }
    }
}

impl ReplaceableAtPath for RingRelation {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            // Cannot replace a Relation with a generic MathExpression unless it's MathExpression::Relation
            // The caller (MathRelation::replace_at_path_recursive) handles the MathExpression::Relation check.
            return Err(PathError::TypeMismatch);
        }

        let current_idx = path[0];
        let remaining_path = &path[1..];

        match self {
            RingRelation::IsSubringOf {
                entity,
                subring,
                ring,
            } => match current_idx {
                1 => Ok(RingRelation::IsSubringOf {
                    entity,
                    subring: subring.replace_at_path(remaining_path, replacement)?,
                    ring,
                }),
                2 => Ok(RingRelation::IsSubringOf {
                    entity,
                    subring,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsIdealOf {
                entity,
                ideal,
                ring,
            } => match current_idx {
                1 => Ok(RingRelation::IsIdealOf {
                    entity,
                    ideal: ideal.replace_at_path(remaining_path, replacement)?,
                    ring,
                }),
                2 => Ok(RingRelation::IsIdealOf {
                    entity,
                    ideal,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsPrimeIdeal {
                entity,
                ideal,
                ring,
            } => match current_idx {
                1 => Ok(RingRelation::IsPrimeIdeal {
                    entity,
                    ideal: ideal.replace_at_path(remaining_path, replacement)?,
                    ring,
                }),
                2 => Ok(RingRelation::IsPrimeIdeal {
                    entity,
                    ideal,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsMaximalIdeal {
                entity,
                ideal,
                ring,
            } => match current_idx {
                1 => Ok(RingRelation::IsMaximalIdeal {
                    entity,
                    ideal: ideal.replace_at_path(remaining_path, replacement)?,
                    ring,
                }),
                2 => Ok(RingRelation::IsMaximalIdeal {
                    entity,
                    ideal,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsPrincipalIdeal {
                entity,
                ideal,
                ring,
                generator,
            } => match current_idx {
                1 => Ok(RingRelation::IsPrincipalIdeal {
                    entity,
                    ideal: ideal.replace_at_path(remaining_path, replacement)?,
                    ring,
                    generator,
                }),
                2 => Ok(RingRelation::IsPrincipalIdeal {
                    entity,
                    ideal,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                    generator,
                }),
                3 => Ok(RingRelation::IsPrincipalIdeal {
                    entity,
                    ideal,
                    ring,
                    generator: generator.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsUnit {
                entity,
                element,
                ring,
            } => match current_idx {
                1 => Ok(RingRelation::IsUnit {
                    entity,
                    element: element.replace_at_path(remaining_path, replacement)?,
                    ring,
                }),
                2 => Ok(RingRelation::IsUnit {
                    entity,
                    element,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsIrreducible {
                entity,
                element,
                ring,
            } => match current_idx {
                1 => Ok(RingRelation::IsIrreducible {
                    entity,
                    element: element.replace_at_path(remaining_path, replacement)?,
                    ring,
                }),
                2 => Ok(RingRelation::IsIrreducible {
                    entity,
                    element,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsPrime {
                entity,
                element,
                ring,
            } => match current_idx {
                1 => Ok(RingRelation::IsPrime {
                    entity,
                    element: element.replace_at_path(remaining_path, replacement)?,
                    ring,
                }),
                2 => Ok(RingRelation::IsPrime {
                    entity,
                    element,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsField { entity, ring } => match current_idx {
                1 => Ok(RingRelation::IsField {
                    entity,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsIntegralDomain { entity, ring } => match current_idx {
                1 => Ok(RingRelation::IsIntegralDomain {
                    entity,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsUFD { entity, ring } => match current_idx {
                1 => Ok(RingRelation::IsUFD {
                    entity,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::IsPID { entity, ring } => match current_idx {
                1 => Ok(RingRelation::IsPID {
                    entity,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::AreAssociates {
                entity,
                first,
                second,
                ring,
            } => match current_idx {
                1 => Ok(RingRelation::AreAssociates {
                    entity,
                    first: first.replace_at_path(remaining_path, replacement)?,
                    second,
                    ring,
                }),
                2 => Ok(RingRelation::AreAssociates {
                    entity,
                    first,
                    second: second.replace_at_path(remaining_path, replacement)?,
                    ring,
                }),
                3 => Ok(RingRelation::AreAssociates {
                    entity,
                    first,
                    second,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::Divides {
                entity,
                divisor,
                dividend,
                ring,
            } => match current_idx {
                1 => Ok(RingRelation::Divides {
                    entity,
                    divisor: divisor.replace_at_path(remaining_path, replacement)?,
                    dividend,
                    ring,
                }),
                2 => Ok(RingRelation::Divides {
                    entity,
                    divisor,
                    dividend: dividend.replace_at_path(remaining_path, replacement)?,
                    ring,
                }),
                3 => Ok(RingRelation::Divides {
                    entity,
                    divisor,
                    dividend,
                    ring: ring.replace_at_path(remaining_path, replacement)?,
                }),
                _ => Err(PathError::InvalidPath),
            },
            RingRelation::Custom {
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
                        Ok(RingRelation::Custom {
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
