use std::collections::{HashMap, HashSet};

use crate::subjects::math::formalism::expressions::TheoryExpression;
use crate::subjects::math::formalism::expressions::{Identifier, MathExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::theories::groups::definitions::GroupExpression;

/// Represents the state of unification, primarily the substitution map.
#[derive(Debug, Clone, Default)]
pub struct UnificationContext {
    substitutions: HashMap<Identifier, MathExpression>,
}

impl UnificationContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_substitution(
        &mut self,
        var: Identifier,
        expr: MathExpression,
    ) -> Result<(), UnificationError> {
        if occurs_check(&var, &expr, &self.substitutions) {
            return Err(UnificationError::OccursCheckFailed { var, in_expr: expr });
        }
        self.substitutions.insert(var, expr);
        Ok(())
    }

    pub fn get_substitution(&self, var: &Identifier) -> Option<&MathExpression> {
        self.substitutions.get(var)
    }

    pub fn dereference<'a>(&'a self, expr: &'a MathExpression) -> &'a MathExpression {
        match expr {
            MathExpression::Var(id) => match self.substitutions.get(id) {
                Some(subst_expr) => self.dereference(subst_expr),
                None => expr,
            },
            _ => expr,
        }
    }

    pub fn into_map(self) -> HashMap<Identifier, MathExpression> {
        self.substitutions
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnificationError {
    Mismatch {
        expr1: Box<MathExpression>,
        expr2: Box<MathExpression>,
        reason: String,
    },
    RelationMismatch {
        rel1: Box<MathRelation>,
        rel2: Box<MathRelation>,
        reason: String,
    },
    OccursCheckFailed {
        var: Identifier,
        in_expr: MathExpression,
    },
}

impl std::fmt::Display for UnificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnificationError::Mismatch {
                expr1,
                expr2,
                reason,
            } => write!(
                f,
                "Unification Mismatch: Cannot unify {:?} and {:?}. Reason: {}",
                expr1, expr2, reason
            ),
            UnificationError::RelationMismatch { rel1, rel2, reason } => write!(
                f,
                "Unification Relation Mismatch: Cannot unify {:?} and {:?}. Reason: {}",
                rel1, rel2, reason
            ),
            UnificationError::OccursCheckFailed { var, in_expr } => write!(
                f,
                "Occurs Check Failed: Variable {:?} occurs within expression {:?}",
                var, in_expr
            ),
        }
    }
}

impl std::error::Error for UnificationError {}

// --- Core Unification Functions ---

/// Unify two mathematical expressions in a given context
pub fn unify(
    expr1: &MathExpression,
    expr2: &MathExpression,
    ctx: &mut UnificationContext,
) -> Result<(), UnificationError> {
    match (expr1, expr2) {
        // Handle Variable Cases
        (MathExpression::Var(id1), other) => unify_var(id1, other, ctx),
        (other, MathExpression::Var(id2)) => unify_var(id2, other, ctx), // Symmetric

        // Handle Structural Cases
        (MathExpression::Number(n1), MathExpression::Number(n2)) => {
            if n1 == n2 {
                Ok(())
            } else {
                Err(UnificationError::Mismatch {
                    expr1: Box::new(expr1.clone()),
                    expr2: Box::new(expr2.clone()),
                    reason: "Numbers differ".into(),
                })
            }
        }
        (MathExpression::Object(o1), MathExpression::Object(o2)) => {
            if o1 == o2 {
                Ok(())
            } else {
                Err(UnificationError::Mismatch {
                    expr1: Box::new(expr1.clone()),
                    expr2: Box::new(expr2.clone()),
                    reason: "Objects differ".into(),
                })
            }
        }
        (MathExpression::Relation(r1), MathExpression::Relation(r2)) => {
            unify_relations(r1, r2, ctx)
        }
        (MathExpression::Expression(te1), MathExpression::Expression(te2)) => {
            unify_theory_expressions(te1, te2, ctx)
        }
        (
            MathExpression::ViewAs {
                expression: e1_inner,
                view: v1,
            },
            MathExpression::ViewAs {
                expression: e2_inner,
                view: v2,
            },
        ) => {
            if v1 != v2 {
                Err(UnificationError::Mismatch {
                    expr1: Box::new(expr1.clone()),
                    expr2: Box::new(expr2.clone()),
                    reason: "Views differ".into(),
                })
            } else {
                unify(e1_inner, e2_inner, ctx)
            }
        }
        // Mismatch Case
        (e1, e2) => Err(UnificationError::Mismatch {
            expr1: Box::new(e1.clone()),
            expr2: Box::new(e2.clone()),
            reason: "Different expression kinds".into(),
        }),
    }
}

/// Helper: Handles Var vs Anything (including Var vs Var)
fn unify_var(
    var_id: &Identifier,
    other_expr: &MathExpression,
    ctx: &mut UnificationContext,
) -> Result<(), UnificationError> {
    // 1. Check if var_id is already substituted
    if let Some(subst1) = ctx.get_substitution(var_id) {
        let subst1_clone = subst1.clone();
        return unify(&subst1_clone, other_expr, ctx);
    }

    // 2. Check if other_expr is the *same variable* as var_id
    if let MathExpression::Var(other_id) = other_expr {
        if var_id == other_id {
            return Ok(()); // Same variable, already unified
        }
        // 3. Check if other_id is substituted
        if let Some(subst2) = ctx.get_substitution(other_id) {
            let subst2_clone = subst2.clone();
            return unify_var(var_id, &subst2_clone, ctx);
        }
    }

    // 4. Neither var_id nor other_expr (if it was a var) is substituted.
    let deref_other = ctx.dereference(other_expr);

    // 5. Occurs Check: var_id in deref_other
    if occurs_check(var_id, deref_other, &ctx.substitutions) {
        return Err(UnificationError::OccursCheckFailed {
            var: var_id.clone(),
            in_expr: deref_other.clone(),
        });
    }

    // 6. Add substitution: var_id -> deref_other
    ctx.substitutions
        .insert(var_id.clone(), deref_other.clone());
    Ok(())
}

/// Unify two math relations
fn unify_relations(
    rel1: &MathRelation,
    rel2: &MathRelation,
    ctx: &mut UnificationContext,
) -> Result<(), UnificationError> {
    match (rel1, rel2) {
        (
            MathRelation::Equal {
                left: l1,
                right: r1,
                ..
            },
            MathRelation::Equal {
                left: l2,
                right: r2,
                ..
            },
        ) => {
            unify(l1, l2, ctx)?;
            unify(r1, r2, ctx)
        }
        (MathRelation::Implies(a1, c1), MathRelation::Implies(a2, c2)) => {
            unify_relations(a1, a2, ctx)?;
            unify_relations(c1, c2, ctx)
        }
        (MathRelation::Equivalent(l1, r1), MathRelation::Equivalent(l2, r2)) => {
            unify_relations(l1, l2, ctx)?;
            unify_relations(r1, r2, ctx)
        }
        (MathRelation::And(v1), MathRelation::And(v2))
        | (MathRelation::Or(v1), MathRelation::Or(v2)) => {
            if v1.len() != v2.len() {
                return Err(UnificationError::RelationMismatch {
                    rel1: Box::new(rel1.clone()),
                    rel2: Box::new(rel2.clone()),
                    reason: "Different number of conjuncts/disjuncts".into(),
                });
            }
            for (r1, r2) in v1.iter().zip(v2.iter()) {
                unify_relations(r1, r2, ctx)?;
            }
            Ok(())
        }
        (MathRelation::Not(r1), MathRelation::Not(r2)) => unify_relations(r1, r2, ctx),
        (
            MathRelation::Todo {
                name: n1,
                expressions: e1,
            },
            MathRelation::Todo {
                name: n2,
                expressions: e2,
            },
        ) => {
            if n1 != n2 || e1.len() != e2.len() {
                return Err(UnificationError::RelationMismatch {
                    rel1: Box::new(rel1.clone()),
                    rel2: Box::new(rel2.clone()),
                    reason: "Todo name or arity mismatch".into(),
                });
            }
            for (expr1, expr2) in e1.iter().zip(e2.iter()) {
                unify(expr1, expr2, ctx)?;
            }
            Ok(())
        }
        // Add other theory-specific relation cases as needed
        (r1, r2) => Err(UnificationError::RelationMismatch {
            rel1: Box::new(r1.clone()),
            rel2: Box::new(r2.clone()),
            reason: "Different relation kinds".into(),
        }),
    }
}

/// Unify theory expressions (stub implementation)
fn unify_theory_expressions(
    te1: &TheoryExpression,
    te2: &TheoryExpression,
    _ctx: &mut UnificationContext,
) -> Result<(), UnificationError> {
    // For now, just return a mismatch - proper implementation needed
    Err(UnificationError::Mismatch {
        expr1: Box::new(MathExpression::Expression(te1.clone())),
        expr2: Box::new(MathExpression::Expression(te2.clone())),
        reason: "Theory expression unification not implemented yet".into(),
    })
}

// --- Occurs Check Functions ---

/// Root occurs check function
fn occurs_check(
    var_id: &Identifier,
    expr: &MathExpression,
    substitutions: &HashMap<Identifier, MathExpression>,
) -> bool {
    let mut visited = HashSet::new();
    occurs_check_recursive(var_id, expr, substitutions, &mut visited)
}

/// Recursive occurs check implementation
fn occurs_check_recursive(
    var_id: &Identifier,
    expr: &MathExpression,
    substitutions: &HashMap<Identifier, MathExpression>,
    visited: &mut HashSet<Identifier>,
) -> bool {
    match expr {
        MathExpression::Var(current_id) => {
            if current_id == var_id {
                return true;
            }
            if let Some(subst_expr) = substitutions.get(current_id) {
                if visited.insert(current_id.clone()) {
                    let result = occurs_check_recursive(var_id, subst_expr, substitutions, visited);
                    visited.remove(current_id);
                    return result;
                }
            }
            false
        }
        MathExpression::Relation(rel_box) => {
            occurs_check_in_relation(var_id, rel_box, substitutions, visited)
        }
        MathExpression::Expression(theory_expr) => {
            occurs_check_in_theory_expr(var_id, theory_expr, substitutions, visited)
        }
        MathExpression::ViewAs { expression, .. } => {
            occurs_check_recursive(var_id, expression, substitutions, visited)
        }
        MathExpression::Number(_) | MathExpression::Object(_) => false,
    }
}

/// Check for variable occurrences in relations
fn occurs_check_in_relation(
    var_id: &Identifier,
    rel: &MathRelation,
    substitutions: &HashMap<Identifier, MathExpression>,
    visited: &mut HashSet<Identifier>,
) -> bool {
    match rel {
        MathRelation::Equal { left, right, .. } => {
            occurs_check_recursive(var_id, left, substitutions, visited)
                || occurs_check_recursive(var_id, right, substitutions, visited)
        }
        MathRelation::Implies(ant, cons) => {
            occurs_check_in_relation(var_id, ant, substitutions, visited)
                || occurs_check_in_relation(var_id, cons, substitutions, visited)
        }
        MathRelation::Equivalent(left, right) => {
            occurs_check_in_relation(var_id, left, substitutions, visited)
                || occurs_check_in_relation(var_id, right, substitutions, visited)
        }
        MathRelation::And(rels) | MathRelation::Or(rels) => rels
            .iter()
            .any(|r| occurs_check_in_relation(var_id, r, substitutions, visited)),
        MathRelation::Not(rel) => occurs_check_in_relation(var_id, rel, substitutions, visited),
        MathRelation::Todo { expressions, .. } => expressions
            .iter()
            .any(|e| occurs_check_recursive(var_id, e, substitutions, visited)),
        _ => false, // For other relation types (currently handled as false)
    }
}

/// Check for variable occurrences in theory expressions
fn occurs_check_in_theory_expr(
    var_id: &Identifier,
    theory_expr: &TheoryExpression,
    substitutions: &HashMap<Identifier, MathExpression>,
    visited: &mut HashSet<Identifier>,
) -> bool {
    match theory_expr {
        TheoryExpression::Group(gr_exp) => {
            // Group expressions have their own occurs check
            occurs_check_in_group_expr(var_id, gr_exp, substitutions, visited)
        }
        TheoryExpression::Ring(_) | TheoryExpression::Field(_) => {
            // Not implemented - treat as false
            false
        }
    }
}

/// Check for variable occurrences in group expressions
/// This function was duplicated in the original code
fn occurs_check_in_group_expr(
    var_id: &Identifier,
    gr_exp: &GroupExpression,
    substitutions: &HashMap<Identifier, MathExpression>,
    visited: &mut HashSet<Identifier>,
) -> bool {
    match gr_exp {
        GroupExpression::Operation {
            left, right, group, ..
        } => {
            occurs_check_in_parametrizable_gr_expr(var_id, left, substitutions, visited)
                || occurs_check_in_parametrizable_gr_expr(var_id, right, substitutions, visited)
                || occurs_check_in_parametrizable_group(var_id, group, substitutions, visited)
        }
        GroupExpression::Inverse { element, group, .. } => {
            occurs_check_in_parametrizable_gr_expr(var_id, element, substitutions, visited)
                || occurs_check_in_parametrizable_group(var_id, group, substitutions, visited)
        }
        GroupExpression::Commutator { a, b, group, .. } => {
            occurs_check_in_parametrizable_gr_expr(var_id, a, substitutions, visited)
                || occurs_check_in_parametrizable_gr_expr(var_id, b, substitutions, visited)
                || occurs_check_in_parametrizable_group(var_id, group, substitutions, visited)
        }
        GroupExpression::Power {
            base,
            exponent,
            group,
            ..
        } => {
            occurs_check_in_parametrizable_gr_expr(var_id, base, substitutions, visited)
                || occurs_check_in_parametrizable_group(var_id, group, substitutions, visited)
                || occurs_check_in_parametrizable_i32(var_id, exponent, substitutions, visited)
        }
        GroupExpression::ElementOrder { element, group, .. } => {
            occurs_check_in_parametrizable_gr_expr(var_id, element, substitutions, visited)
                || occurs_check_in_parametrizable_group(var_id, group, substitutions, visited)
        }
        GroupExpression::Coset {
            element,
            group,
            subgroup,
            ..
        } => {
            occurs_check_in_parametrizable_gr_expr(var_id, element, substitutions, visited)
                || occurs_check_in_parametrizable_group(var_id, group, substitutions, visited)
                || occurs_check_in_parametrizable_group(var_id, subgroup, substitutions, visited)
        }
        GroupExpression::Identity(group_param)
        | GroupExpression::GroupOrder { group: group_param } => {
            occurs_check_in_parametrizable_group(var_id, group_param, substitutions, visited)
        }
        GroupExpression::Element {
            element: _element_param,
            group: group_param,
        } => occurs_check_in_parametrizable_group(var_id, group_param, substitutions, visited),
        _ => false,
    }
}

/// Check for variable occurrences in parametrizable group expressions
/// This function was duplicated in the original code
fn occurs_check_in_parametrizable_gr_expr(
    var_id: &Identifier,
    param: &Parametrizable<GroupExpression>,
    substitutions: &HashMap<Identifier, MathExpression>,
    visited: &mut HashSet<Identifier>,
) -> bool {
    match param {
        Parametrizable::Concrete(gr_expr) => {
            occurs_check_in_group_expr(var_id, gr_expr, substitutions, visited)
        }
        Parametrizable::Variable(id) => occurs_check_recursive(
            var_id,
            &MathExpression::Var(id.clone()),
            substitutions,
            visited,
        ),
    }
}

/// Check for variable occurrences in parametrizable groups
/// This function was duplicated in the original code
fn occurs_check_in_parametrizable_group(
    var_id: &Identifier,
    param: &Parametrizable<crate::subjects::math::theories::groups::definitions::Group>,
    substitutions: &HashMap<Identifier, MathExpression>,
    visited: &mut HashSet<Identifier>,
) -> bool {
    match param {
        Parametrizable::Concrete(_group) => false,
        Parametrizable::Variable(id) => occurs_check_recursive(
            var_id,
            &MathExpression::Var(id.clone()),
            substitutions,
            visited,
        ),
    }
}

/// Check for variable occurrences in parametrizable i32 values
/// This function was duplicated in the original code
fn occurs_check_in_parametrizable_i32(
    var_id: &Identifier,
    param: &Parametrizable<i32>,
    substitutions: &HashMap<Identifier, MathExpression>,
    visited: &mut HashSet<Identifier>,
) -> bool {
    match param {
        Parametrizable::Concrete(_) => false,
        Parametrizable::Variable(id) => occurs_check_recursive(
            var_id,
            &MathExpression::Var(id.clone()),
            substitutions,
            visited,
        ),
    }
}

/// Apply instantiations to a math expression
pub fn apply_instantiations(
    expr: &MathExpression,
    substitutions: &HashMap<Identifier, MathExpression>,
) -> MathExpression {
    match expr {
        MathExpression::Var(id) => {
            if let Some(subst) = substitutions.get(id) {
                // Apply recursively to handle chains of substitutions
                apply_instantiations(subst, substitutions)
            } else {
                expr.clone()
            }
        }
        MathExpression::Relation(rel) => {
            MathExpression::Relation(apply_instantiations_to_relation(rel, substitutions))
        }
        MathExpression::Expression(theory_expr) => MathExpression::Expression(
            apply_instantiations_to_theory_expr(theory_expr, substitutions),
        ),
        MathExpression::ViewAs { expression, view } => MathExpression::ViewAs {
            expression: Box::new(apply_instantiations(expression, substitutions)),
            view: view.clone(),
        },
        // These types don't contain variables
        MathExpression::Number(_) | MathExpression::Object(_) => expr.clone(),
    }
}

/// Apply instantiations to a relation
pub fn apply_instantiations_to_relation(
    rel: &MathRelation,
    substitutions: &HashMap<Identifier, MathExpression>,
) -> Box<MathRelation> {
    match rel {
        MathRelation::Equal { meta, left, right } => Box::new(MathRelation::Equal {
            meta: meta.clone(),
            left: apply_instantiations(left, substitutions),
            right: apply_instantiations(right, substitutions),
        }),
        MathRelation::Implies(ant, cons) => Box::new(MathRelation::Implies(
            apply_instantiations_to_relation(ant, substitutions),
            apply_instantiations_to_relation(cons, substitutions),
        )),
        MathRelation::Equivalent(left, right) => Box::new(MathRelation::Equivalent(
            apply_instantiations_to_relation(left, substitutions),
            apply_instantiations_to_relation(right, substitutions),
        )),
        MathRelation::And(rels) => {
            let new_rels = rels
                .iter()
                .map(|r| *apply_instantiations_to_relation(r, substitutions))
                .collect();
            Box::new(MathRelation::And(new_rels))
        }
        MathRelation::Or(rels) => {
            let new_rels = rels
                .iter()
                .map(|r| *apply_instantiations_to_relation(r, substitutions))
                .collect();
            Box::new(MathRelation::Or(new_rels))
        }
        MathRelation::Not(rel) => Box::new(MathRelation::Not(apply_instantiations_to_relation(
            rel,
            substitutions,
        ))),
        MathRelation::Todo { name, expressions } => {
            let new_exprs = expressions
                .iter()
                .map(|e| apply_instantiations(e, substitutions))
                .collect();
            Box::new(MathRelation::Todo {
                name: name.clone(),
                expressions: new_exprs,
            })
        }
        // Other relations would just be cloned for now
        _ => Box::new(rel.clone()),
    }
}

/// Apply instantiations to a theory expression
fn apply_instantiations_to_theory_expr(
    theory_expr: &TheoryExpression,
    substitutions: &HashMap<Identifier, MathExpression>,
) -> TheoryExpression {
    match theory_expr {
        TheoryExpression::Group(gr_exp) => {
            // Delegate to a helper specific to GroupExpression (not implemented yet)
            TheoryExpression::Group(gr_exp.clone())
        }
        // Other theory expressions just cloned for now
        _ => theory_expr.clone(),
    }
}
