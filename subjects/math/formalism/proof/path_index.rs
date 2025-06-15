// Path indexing for traversing expressions by numeric path
// This allows us to uniquely identify subexpressions
// without having to use string-based paths

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::relations::MathRelation;

// Error type for path-based operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathError {
    NotFound,        // The specified path does not lead to a sub-expression.
    InvalidPath, // The path is structurally invalid for the given expression (e.g., wrong index).
    TypeMismatch, // The replacement expression is not of a type that can replace the target.
    ImmutableTarget, // The target found at the path cannot be replaced.
    NotImplemented, // Functionality for this specific AST node type or path scenario is not yet implemented.
}

// Conceptual trait for types that can be replaced at a path.
// This allows Parametrizable<T> to delegate to T's specific replacement logic.
pub trait ReplaceableAtPath: Sized {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError>;
}

// Inherent methods for MathExpression
impl MathExpression {
    pub fn replace_at_path(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<MathExpression, PathError> {
        if path.is_empty() {
            return Ok(replacement);
        }

        let current_idx = path[0];
        let remaining_path = &path[1..];

        match self {
            MathExpression::Relation(rel_box) => {
                if current_idx == 1 {
                    // Path convention from traversal.rs
                    let new_rel = rel_box.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(MathExpression::Relation(new_rel))
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathExpression::ViewAs {
                expression: expr_box,
                view,
            } => {
                if current_idx == 1 {
                    // Path convention
                    let new_expr = expr_box.replace_at_path(remaining_path, replacement)?;
                    Ok(MathExpression::ViewAs {
                        expression: Box::new(new_expr),
                        view,
                    })
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathExpression::Expression(theory_expr) => {
                if current_idx == 1 {
                    // Path convention
                    let new_theory_expr =
                        theory_expr.replace_at_path_recursive(remaining_path, replacement)?;
                    Ok(MathExpression::Expression(new_theory_expr))
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathExpression::Var(_) | MathExpression::Number(_) | MathExpression::Object(_) => {
                Err(PathError::InvalidPath) // Cannot go deeper into leaf nodes
            }
        }
    }
}

// MathRelation inherent method will become an impl of ReplaceableAtPath
impl ReplaceableAtPath for MathRelation {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement_expr: MathExpression,
    ) -> Result<Self, PathError> {
        // Output type is Self (MathRelation), not Box<MathRelation>
        if path.is_empty() {
            if let MathExpression::Relation(new_rel_box) = replacement_expr {
                return Ok(*new_rel_box); // Dereference the box
            } else {
                return Err(PathError::TypeMismatch);
            }
        }

        let current_idx = path[0];
        let remaining_path = &path[1..];

        match self {
            MathRelation::Equal { meta, left, right } => match current_idx {
                1 => {
                    let new_left = left.replace_at_path(remaining_path, replacement_expr)?;
                    Ok(MathRelation::Equal {
                        meta,
                        left: new_left,
                        right,
                    })
                }
                2 => {
                    let new_right = right.replace_at_path(remaining_path, replacement_expr)?;
                    Ok(MathRelation::Equal {
                        meta,
                        left,
                        right: new_right,
                    })
                }
                _ => Err(PathError::InvalidPath),
            },
            MathRelation::Todo {
                name,
                mut expressions,
            } => {
                if current_idx >= 100 {
                    let vec_idx = current_idx - 100;
                    if vec_idx < expressions.len() {
                        expressions[vec_idx] = expressions[vec_idx]
                            .clone()
                            .replace_at_path(remaining_path, replacement_expr)?;
                        Ok(MathRelation::Todo { name, expressions })
                    } else {
                        Err(PathError::InvalidPath)
                    }
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::Implies(premise, conclusion) => match current_idx {
                1 => {
                    let new_premise =
                        premise.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::Implies(new_premise, conclusion))
                }
                2 => {
                    let new_conclusion =
                        conclusion.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::Implies(premise, new_conclusion))
                }
                _ => Err(PathError::InvalidPath),
            },
            MathRelation::Equivalent(left, right) => match current_idx {
                1 => {
                    let new_left =
                        left.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::Equivalent(new_left, right))
                }
                2 => {
                    let new_right =
                        right.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::Equivalent(left, new_right))
                }
                _ => Err(PathError::InvalidPath),
            },
            MathRelation::And(mut relations) => {
                if current_idx >= 100 {
                    let vec_idx = current_idx - 100;
                    if vec_idx < relations.len() {
                        relations[vec_idx] = relations[vec_idx]
                            .clone()
                            .replace_at_path_recursive(remaining_path, replacement_expr)?;
                        Ok(MathRelation::And(relations))
                    } else {
                        Err(PathError::InvalidPath)
                    }
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::Or(mut relations) => {
                if current_idx >= 100 {
                    let vec_idx = current_idx - 100;
                    if vec_idx < relations.len() {
                        relations[vec_idx] = relations[vec_idx]
                            .clone()
                            .replace_at_path_recursive(remaining_path, replacement_expr)?;
                        Ok(MathRelation::Or(relations))
                    } else {
                        Err(PathError::InvalidPath)
                    }
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::Not(rel) => {
                if current_idx == 1 {
                    let new_rel =
                        rel.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::Not(new_rel))
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::SetTheory(st_rel) => {
                if current_idx == 201 {
                    // Path convention for SetTheory variant
                    let new_st_rel =
                        st_rel.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::SetTheory(new_st_rel))
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::NumberTheory(nt_rel) => {
                if current_idx == 202 {
                    // Path convention for NumberTheory variant
                    let new_nt_rel =
                        nt_rel.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::NumberTheory(new_nt_rel))
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::GroupTheory(gt_rel) => {
                if current_idx == 203 {
                    // Path convention for GroupTheory variant
                    let new_gt_rel =
                        gt_rel.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::GroupTheory(new_gt_rel))
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::RingTheory(rt_rel) => {
                if current_idx == 204 {
                    // Path convention for RingTheory variant
                    let new_rt_rel =
                        rt_rel.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::RingTheory(new_rt_rel))
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::TopologyTheory(tt_rel) => {
                if current_idx == 205 {
                    // Path convention for TopologyTheory variant
                    let new_tt_rel =
                        tt_rel.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::TopologyTheory(new_tt_rel))
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::CategoryTheory(ct_rel) => {
                if current_idx == 206 {
                    // Path convention for CategoryTheory variant
                    let new_ct_rel =
                        ct_rel.replace_at_path_recursive(remaining_path, replacement_expr)?;
                    Ok(MathRelation::CategoryTheory(new_ct_rel))
                } else {
                    Err(PathError::InvalidPath)
                }
            }
            MathRelation::True | MathRelation::False => {
                // Cannot replace sub-expressions in a leaf node.
                Err(PathError::InvalidPath)
            }
        }
    }
}

// TheoryExpression inherent method becomes an impl of ReplaceableAtPath
impl ReplaceableAtPath for TheoryExpression {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            if let MathExpression::Expression(new_theory_expr) = replacement {
                return Ok(new_theory_expr);
            } else {
                return Err(PathError::TypeMismatch);
            }
        }

        let current_idx = path[0];
        let remaining_path = &path[1..];

        match self {
            TheoryExpression::Group(gr_exp) => {
                // Path from traversal for Group variant is not used here for dispatch,
                // current_idx refers to fields *within* GroupExpression if path goes deeper.
                // If path targets GroupExpression itself (e.g. path=[1] from MathExpr -> TheoryExpr(Group)),
                // that's handled by MathExpression::replace_at_path.
                // This impl is for path *into* gr_exp.
                // The current_idx here should match fields of GroupExpression (1 for left, 2 for right in Operation etc.)
                let new_gr_exp = gr_exp.replace_at_path_recursive(path, replacement)?; // Pass the full path for GroupExpression to handle
                Ok(TheoryExpression::Group(new_gr_exp))
            }
            TheoryExpression::Ring(r_exp) => {
                // Similar logic as Group: RingExpression needs to handle its internal path.
                let new_r_exp = r_exp.replace_at_path_recursive(path, replacement)?;
                Ok(TheoryExpression::Ring(new_r_exp))
            }
            TheoryExpression::Field(f_exp) => {
                // Similar logic as Group: FieldExpression needs to handle its internal path.
                let new_f_exp = f_exp.replace_at_path_recursive(path, replacement)?;
                Ok(TheoryExpression::Field(new_f_exp))
            }
        }
    }
}

// Box delegating impl
impl<T: ReplaceableAtPath + Sized> ReplaceableAtPath for Box<T> {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError> {
        (*self)
            .replace_at_path_recursive(path, replacement)
            .map(Box::new)
    }
}

// Generic parameterizable impl
impl<T> ReplaceableAtPath for Parametrizable<T>
where
    T: Clone + ReplaceableAtPath,
{
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError> {
        match self {
            Parametrizable::Concrete(value) => {
                let new_value = value.replace_at_path_recursive(path, replacement)?;
                Ok(Parametrizable::Concrete(new_value))
            }
            Parametrizable::Variable(_var_name) => {
                if path.is_empty() {
                    // This implies replacing the variable itself with a concrete expression.
                    // The replacement MathExpression must be interpretable as a T or a new Parametrizable<T>.
                    // This logic is complex and depends on how Parametrizable::Variable is used.
                    // For now, let's assume it becomes a Concrete value if replacement is valid for T.
                    // This might require T::try_from(replacement) or similar.
                    Err(PathError::NotImplemented) // Placeholder for Var -> Concrete replacement
                } else {
                    Err(PathError::InvalidPath)
                }
            }
        }
    }
}

// Primitive types

impl ReplaceableAtPath for i32 {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        _replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            Err(PathError::TypeMismatch) // Cannot replace an i32 with a MathExpr directly
        } else {
            Err(PathError::InvalidPath) // Cannot go deeper than i32
        }
    }
}

impl ReplaceableAtPath for String {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        _replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            Err(PathError::TypeMismatch) // Cannot replace a String with a MathExpr directly
        } else {
            Err(PathError::InvalidPath) // Cannot go deeper than String
        }
    }
}

impl ReplaceableAtPath for bool {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        _replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            Err(PathError::TypeMismatch) // Cannot replace a bool with a MathExpr directly
        } else {
            Err(PathError::InvalidPath) // Cannot go deeper than bool
        }
    }
}

impl ReplaceableAtPath for usize {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        _replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            Err(PathError::TypeMismatch) // Cannot replace a usize with a MathExpr
        } else {
            Err(PathError::InvalidPath) // Cannot descend into usize
        }
    }
}
