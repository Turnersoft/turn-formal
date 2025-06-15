// subjects/math/formalism/proof/traversal.rs
// Defines inherent methods for collecting potential theorem targets with their paths.

use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::relations::MathRelation;

// Theory-specific relations and expressions that might contain MathExpressions
use crate::foundational_theories::category_theory::definitions::CategoryRelation;
use crate::subjects::math::theories::groups::definitions::GroupRelation;
use crate::subjects::math::theories::number_theory::definitions::NumberTheoryRelation;
use crate::subjects::math::theories::rings::definitions::RingRelation;
use crate::subjects::math::theories::topology::definitions::TopologyRelation;
use crate::subjects::math::theories::zfc::definitions::SetRelation;

const MAX_DEPTH: usize = 100;

// Helper trait to generalize collection over Box<T> and other containers if needed.
pub trait CollectSubExpressions {
    fn collect_sub_expressions_with_paths(
        &self,
        current_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    );
}

impl CollectSubExpressions for MathExpression {
    fn collect_sub_expressions_with_paths(
        &self,
        current_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        if depth > MAX_DEPTH {
            println!(
                "Warning: Max traversal depth {} reached at path {:?}",
                depth, current_path
            );
            return;
        }

        collected_targets.push((current_path.clone(), self.clone()));

        match self {
            MathExpression::Relation(rel_box) => {
                let mut path = current_path.clone();
                path.push(1); // Convention: Boxed MathRelation is at index 1
                rel_box.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
            }
            MathExpression::ViewAs {
                expression,
                view: _,
            } => {
                let mut path = current_path.clone();
                path.push(1); // Convention: 'expression' field is at index 1
                expression.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
            }
            MathExpression::Expression(theory_expr) => {
                let mut path = current_path.clone();
                path.push(1); // Convention: TheoryExpression is at index 1
                theory_expr.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
            }
            MathExpression::Var(_) | MathExpression::Number(_) | MathExpression::Object(_) => {}
        }
    }
}

impl CollectSubExpressions for MathRelation {
    fn collect_sub_expressions_with_paths(
        &self,
        current_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        if depth > MAX_DEPTH {
            println!(
                "Warning: Max traversal depth {} reached at path {:?}",
                depth, current_path
            );
            return;
        }

        collected_targets.push((
            current_path.clone(),
            MathExpression::Relation(Box::new(self.clone())),
        ));

        match self {
            MathRelation::Equal { left, right, .. } => {
                let mut path_l = current_path.clone();
                path_l.push(1);
                left.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                let mut path_r = current_path.clone();
                path_r.push(2);
                right.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
            }
            MathRelation::Todo { expressions, .. } => {
                for (i, expr) in expressions.iter().enumerate() {
                    let mut path = current_path.clone();
                    path.push(100 + i);
                    expr.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
                }
            }
            MathRelation::Implies(premise, conclusion) => {
                let mut path_p = current_path.clone();
                path_p.push(1);
                premise.collect_sub_expressions_with_paths(path_p, collected_targets, depth + 1);
                let mut path_c = current_path.clone();
                path_c.push(2);
                conclusion.collect_sub_expressions_with_paths(path_c, collected_targets, depth + 1);
            }
            MathRelation::Equivalent(left, right) => {
                let mut path_l = current_path.clone();
                path_l.push(1);
                left.collect_sub_expressions_with_paths(path_l, collected_targets, depth + 1);
                let mut path_r = current_path.clone();
                path_r.push(2);
                right.collect_sub_expressions_with_paths(path_r, collected_targets, depth + 1);
            }
            MathRelation::And(relations) | MathRelation::Or(relations) => {
                for (i, rel) in relations.iter().enumerate() {
                    let mut path = current_path.clone();
                    path.push(100 + i);
                    rel.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
                }
            }
            MathRelation::Not(rel) => {
                let mut path = current_path.clone();
                path.push(1);
                rel.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
            }
            MathRelation::SetTheory(st_rel) => {
                let mut path = current_path.clone();
                path.push(201); // Discriminant for SetTheory
                st_rel.collect_contained_expressions(path, collected_targets, depth + 1);
            }
            MathRelation::NumberTheory(nt_rel) => {
                let mut path = current_path.clone();
                path.push(202);
                nt_rel.collect_contained_expressions(path, collected_targets, depth + 1);
            }
            MathRelation::GroupTheory(gt_rel) => {
                let mut path = current_path.clone();
                path.push(203);
                gt_rel.collect_contained_expressions(path, collected_targets, depth + 1);
            }
            MathRelation::RingTheory(rt_rel) => {
                let mut path = current_path.clone();
                path.push(204);
                rt_rel.collect_contained_expressions(path, collected_targets, depth + 1);
            }
            MathRelation::TopologyTheory(tt_rel) => {
                let mut path = current_path.clone();
                path.push(205);
                tt_rel.collect_contained_expressions(path, collected_targets, depth + 1);
            }
            MathRelation::CategoryTheory(ct_rel) => {
                let mut path = current_path.clone();
                path.push(206);
                ct_rel.collect_contained_expressions(path, collected_targets, depth + 1);
            }
            MathRelation::True | MathRelation::False => {
                // Leaf nodes, no sub-expressions to collect.
            }
        }
    }
}

impl<T> CollectSubExpressions for Box<T>
where
    T: CollectSubExpressions + ?Sized,
{
    fn collect_sub_expressions_with_paths(
        &self,
        current_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        (**self).collect_sub_expressions_with_paths(current_path, collected_targets, depth);
    }
}

impl CollectSubExpressions for TheoryExpression {
    fn collect_sub_expressions_with_paths(
        &self,
        current_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        if depth > MAX_DEPTH {
            println!(
                "Warning: Max traversal depth {} reached at path {:?}",
                depth, current_path
            );
            return;
        }
        collected_targets.push((
            current_path.clone(),
            MathExpression::Expression(self.clone()),
        ));

        match self {
            TheoryExpression::Group(gr_exp) => {
                let mut path = current_path.clone();
                path.push(1); // Group expr is field 1
                gr_exp.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
            }
            TheoryExpression::Ring(r_exp) => {
                let mut path = current_path.clone();
                path.push(2); // Ring expr is field 2
                r_exp.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
            }
            TheoryExpression::Field(f_exp) => {
                let mut path = current_path.clone();
                path.push(3); // Field expr is field 3
                f_exp.collect_sub_expressions_with_paths(path, collected_targets, depth + 1);
            }
        }
    }
}

// Implementation for primitive types
impl CollectSubExpressions for i32 {
    fn collect_sub_expressions_with_paths(
        &self,
        _current_path: Vec<usize>,
        _collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        _depth: usize,
    ) {
        // i32 is a leaf node
    }
}

impl CollectSubExpressions for String {
    fn collect_sub_expressions_with_paths(
        &self,
        _current_path: Vec<usize>,
        _collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        _depth: usize,
    ) {
        // String is a leaf node
    }
}

impl CollectSubExpressions for bool {
    fn collect_sub_expressions_with_paths(
        &self,
        _current_path: Vec<usize>,
        _collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        _depth: usize,
    ) {
        // bool is a leaf node
    }
}

impl CollectSubExpressions for usize {
    fn collect_sub_expressions_with_paths(
        &self,
        _current_path: Vec<usize>,
        _collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        _depth: usize,
    ) {
        // usize is a leaf node
    }
}
