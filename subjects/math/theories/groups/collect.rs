use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::expressions::TheoryExpression;
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::proof::path_index::{PathError, ReplaceableAtPath};
use crate::subjects::math::formalism::proof::collect::CollectSubExpressions;
use crate::subjects::math::theories::groups::definitions::{
    Group, GroupElement, GroupExpression, GroupRelation,
};

impl CollectSubExpressions for GroupExpression {
    fn collect_sub_expressions_with_paths(
        &self,
        current_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        if depth > 100 {
            println!(
                "Warning: Max traversal depth {} reached at path {:?}",
                depth, current_path
            );
            return;
        }
        // Add GroupExpression itself, wrapped as MathExpression
        collected_targets.push((
            current_path.clone(),
            MathExpression::Expression(TheoryExpression::Group(self.clone())),
        ));

        match self {
            GroupExpression::Operation { left, right, .. } => {
                // We need to handle Box<Parametrizable<GroupExpression>> specially
                // Since the Box<T> implementation in the main traversal.rs will delegate to
                // the Parametrizable<GroupExpression> implementation we just added
                let mut path_l = current_path.clone();
                path_l.push(1);
                let mut path_r = current_path.clone();
                path_r.push(2);

                // The Box delegate will call Parametrizable's implementation
                if let Some(expr) = get_concrete_expression(left) {
                    expr.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                }

                if let Some(expr) = get_concrete_expression(right) {
                    expr.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
                }
            }
            GroupExpression::Inverse { element, .. } => {
                let mut path_e = current_path.clone();
                path_e.push(1);
                if let Some(expr) = get_concrete_expression(element) {
                    expr.collect_sub_expressions_with_paths(path_e, collected_targets, depth + 1);
                }
            }
            GroupExpression::Commutator { a, b, .. } => {
                let mut path_a = current_path.clone();
                path_a.push(1);
                a.collect_sub_expressions_with_paths(path_a, collected_targets, depth + 1);
                let mut path_b = current_path.clone();
                path_b.push(2);
                b.collect_sub_expressions_with_paths(path_b, collected_targets, depth + 1);
            }
            GroupExpression::Coset { element, .. } => {
                let mut path_e = current_path.clone();
                path_e.push(1);
                element.collect_sub_expressions_with_paths(path_e, collected_targets, depth + 1);
            }
            GroupExpression::Power { base, .. } => {
                let mut path_b = current_path.clone();
                path_b.push(1);
                base.collect_sub_expressions_with_paths(path_b, collected_targets, depth + 1);
            }
            GroupExpression::ElementOrder { element, .. } => {
                let mut path_e = current_path.clone();
                path_e.push(1);
                element.collect_sub_expressions_with_paths(path_e, collected_targets, depth + 1);
            }
            // GroupExpression::Identity, GroupExpression::Generator etc. are leafs in terms of MathExpressions
            _ => {}
        }
    }
}

// Helper function to extract a concrete GroupExpression from Box<Parametrizable<GroupExpression>>
fn get_concrete_expression(
    boxed_param: &Box<Parametrizable<GroupExpression>>,
) -> Option<&GroupExpression> {
    match &**boxed_param {
        Parametrizable::Concrete(expr) => Some(expr),
        Parametrizable::Variable(_) => None,
    }
}

impl GroupRelation {
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
            GroupRelation::IsSubgroupOf {
                subgroup, group, ..
            } => {
                let mut path_sub = base_path.clone();
                path_sub.push(1);
                match subgroup {
                    Parametrizable::Concrete(g) => {
                        g.collect_sub_expressions_with_paths(path_sub, collected_targets, depth + 1)
                    }
                    Parametrizable::Variable(_) => {} // Variables are leaf nodes
                }
                let mut path_g = base_path.clone();
                path_g.push(2);
                match group {
                    Parametrizable::Concrete(g) => {
                        g.collect_sub_expressions_with_paths(path_g, collected_targets, depth + 1)
                    }
                    Parametrizable::Variable(_) => {} // Variables are leaf nodes
                }
            }
            GroupRelation::IsNormalSubgroupOf {
                subgroup, group, ..
            } => {
                let mut path_sub = base_path.clone();
                path_sub.push(1);
                match subgroup {
                    Parametrizable::Concrete(g) => {
                        g.collect_sub_expressions_with_paths(path_sub, collected_targets, depth + 1)
                    }
                    Parametrizable::Variable(_) => {} // Variables are leaf nodes
                }
                let mut path_g = base_path.clone();
                path_g.push(2);
                match group {
                    Parametrizable::Concrete(g) => {
                        g.collect_sub_expressions_with_paths(path_g, collected_targets, depth + 1)
                    }
                    Parametrizable::Variable(_) => {} // Variables are leaf nodes
                }
            }
            GroupRelation::IsInCenterOf { element, group, .. } => {
                let mut path_e = base_path.clone();
                path_e.push(1);
                match element {
                    Parametrizable::Concrete(e) => {
                        // Do nothing for now - GroupElement doesn't implement CollectSubExpressions
                        // We would need a proper implementation for GroupElement if it contains MathExpression
                    }
                    Parametrizable::Variable(_) => {} // Variables are leaf nodes
                }
                let mut path_g = base_path.clone();
                path_g.push(2);
                match group {
                    Parametrizable::Concrete(g) => {
                        g.collect_sub_expressions_with_paths(path_g, collected_targets, depth + 1)
                    }
                    Parametrizable::Variable(_) => {} // Variables are leaf nodes
                }
            }
            GroupRelation::HasOrderInGroup {
                element,
                group,
                order,
                ..
            } => {
                let mut path_e = base_path.clone();
                path_e.push(1);
                match element {
                    Parametrizable::Concrete(e) => {
                        // Do nothing for now - GroupElement doesn't implement CollectSubExpressions
                    }
                    Parametrizable::Variable(_) => {} // Variables are leaf nodes
                }

                let mut path_g = base_path.clone();
                path_g.push(2);
                match group {
                    Parametrizable::Concrete(g) => {
                        g.collect_sub_expressions_with_paths(path_g, collected_targets, depth + 1)
                    }
                    Parametrizable::Variable(_) => {} // Variables are leaf nodes
                }

                // For order (which is a usize), no need to traverse - it's a leaf node
            }
            GroupRelation::AreConjugateIn {
                element1,
                element2,
                group,
                ..
            } => {
                let mut path_e1 = base_path.clone();
                path_e1.push(1);
                element1.collect_sub_expressions_with_paths(path_e1, collected_targets, depth + 1);
                let mut path_e2 = base_path.clone();
                path_e2.push(2);
                element2.collect_sub_expressions_with_paths(path_e2, collected_targets, depth + 1);
                let mut path_g = base_path.clone();
                path_g.push(3);
                group.collect_sub_expressions_with_paths(path_g, collected_targets, depth + 1);
            }
            GroupRelation::IsQuotientOf {
                quotient,
                group,
                normal_subgroup,
                ..
            } => {
                let mut path_q = base_path.clone();
                path_q.push(1);
                quotient.collect_sub_expressions_with_paths(path_q, collected_targets, depth + 1);
                let mut path_g = base_path.clone();
                path_g.push(2);
                group.collect_sub_expressions_with_paths(path_g, collected_targets, depth + 1);
                let mut path_n = base_path.clone();
                path_n.push(3);
                normal_subgroup.collect_sub_expressions_with_paths(
                    path_n,
                    collected_targets,
                    depth + 1,
                );
            }
            // Handle other cases as needed
            _ => {}
        }
    }
}

impl CollectSubExpressions for Group {
    fn collect_sub_expressions_with_paths(
        &self,
        _current_path: Vec<usize>,
        _collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        _depth: usize,
    ) {
        // Groups themselves don't contain nested MathExpressions in their direct definition,
        // unless fields like `base_set` are MathExpressions, which seems unlikely.
        // For now, assume Group is a leaf in terms of MathExpression traversal.
    }
}

impl CollectSubExpressions for GroupElement {
    fn collect_sub_expressions_with_paths(
        &self,
        _current_path: Vec<usize>,
        _collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        _depth: usize,
    ) {
        // GroupElement is a leaf node in terms of MathExpression traversal.
    }
}

impl ReplaceableAtPath for GroupExpression {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            if let MathExpression::Expression(TheoryExpression::Group(new_gr_exp)) = replacement {
                return Ok(new_gr_exp);
            } else {
                return Err(PathError::TypeMismatch);
            }
        }

        let current_idx = path[0];
        let remaining_path = &path[1..];

        match self {
            GroupExpression::Operation {
                group,
                mut left,
                mut right,
            } => match current_idx {
                1 => {
                    left = left.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(GroupExpression::Operation { group, left, right })
                }
                2 => {
                    right = right.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(GroupExpression::Operation { group, left, right })
                }
                _ => Err(PathError::InvalidPath),
            },
            GroupExpression::Inverse { group, mut element } => {
                if current_idx == 1 {
                    element = element.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(GroupExpression::Inverse { group, element })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            GroupExpression::Commutator {
                group,
                mut a,
                mut b,
            } => match current_idx {
                1 => {
                    a = a.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(GroupExpression::Commutator { group, a, b })
                }
                2 => {
                    b = b.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(GroupExpression::Commutator { group, a, b })
                }
                _ => Err(PathError::InvalidPath),
            },
            GroupExpression::Power {
                group,
                mut base,
                exponent,
            } => {
                if current_idx == 1 {
                    base = base.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(GroupExpression::Power {
                        group,
                        base,
                        exponent,
                    })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            GroupExpression::ElementOrder { group, mut element } => {
                if current_idx == 1 {
                    element = element.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(GroupExpression::ElementOrder { group, element })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            GroupExpression::Coset {
                group,
                subgroup,
                mut element,
                is_left,
            } => {
                if current_idx == 1 {
                    element = element.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(GroupExpression::Coset {
                        group,
                        subgroup,
                        element,
                        is_left,
                    })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            // GroupExpression::Identity, GroupExpression::Generator are leaf nodes for replacement
            _ => Err(PathError::NotImplemented),
        }
    }
}

impl ReplaceableAtPath for GroupRelation {
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
            GroupRelation::IsSubgroupOf {
                mut subgroup,
                mut group,
            } => match current_idx {
                1 => {
                    // Special handling for Parametrizable<Group>
                    match subgroup {
                        Parametrizable::Concrete(g) => {
                            let _: Group = g; // Just to confirm type
                            // We can't easily replace a concrete Group with a MathExpression
                            // For now, return an error
                            return Err(PathError::NotImplemented);
                        }
                        Parametrizable::Variable(_) => {
                            // Can't replace a variable directly
                            return Err(PathError::NotImplemented);
                        }
                    }
                }
                2 => {
                    // Similar handling for group
                    match group {
                        Parametrizable::Concrete(g) => {
                            let _: Group = g; // Just to confirm type
                            return Err(PathError::NotImplemented);
                        }
                        Parametrizable::Variable(_) => {
                            return Err(PathError::NotImplemented);
                        }
                    }
                }
                _ => Err(PathError::InvalidPath),
            },
            // Similar handling for other GroupRelation variants
            _ => Err(PathError::NotImplemented),
        }
    }
}

impl ReplaceableAtPath for Group {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        _replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            Err(PathError::TypeMismatch) // Cannot replace a Group with a MathExpr
        } else {
            Err(PathError::InvalidPath)
        }
    }
}

impl ReplaceableAtPath for GroupElement {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        _replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            Err(PathError::TypeMismatch) // Cannot replace a GroupElement with a MathExpr directly
        } else {
            Err(PathError::InvalidPath) // Cannot go deeper than GroupElement
        }
    }
}

// Implement CollectSubExpressions for Parametrizable<GroupExpression>
impl CollectSubExpressions for Parametrizable<GroupExpression> {
    fn collect_sub_expressions_with_paths(
        &self,
        current_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        match self {
            Parametrizable::Concrete(gr_exp) => {
                gr_exp.collect_sub_expressions_with_paths(current_path, collected_targets, depth)
            }
            Parametrizable::Variable(_) => {
                // Variables are leaf nodes
            }
        }
    }
}

// Implement for GroupElement, Group, and other relevant types
impl CollectSubExpressions for Parametrizable<GroupElement> {
    fn collect_sub_expressions_with_paths(
        &self,
        _current_path: Vec<usize>,
        _collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        _depth: usize,
    ) {
        // GroupElement doesn't contain MathExpressions, so treat as leaf
    }
}

impl CollectSubExpressions for Parametrizable<Group> {
    fn collect_sub_expressions_with_paths(
        &self,
        _current_path: Vec<usize>,
        _collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        _depth: usize,
    ) {
        // Group doesn't contain MathExpressions, so treat as leaf
    }
}
