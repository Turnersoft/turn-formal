use crate::{
    foundational_theories::type_theory::calculi::cic::reduction,
    subjects::math::formalism::{
        expressions::{MathExpression, TheoryExpression},
        location::Located,
        objects::MathObject,
        relations::MathRelation,
    },
};

use super::{proof::ContextEntry, proof::tactics::Target};

pub trait Search {
    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> Vec<String>;
    // why we don't return a target? becuase we only need to
    // know how many matches are there. if we only found one,
    // we can use the target directly to do replace
}

pub trait IsCompatible<P> {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &P,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool;
}

impl Search for MathExpression {
    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> Vec<String> {
        let mut matches = Vec::new();
        let is_in_scope_now = in_target_scope || current_id == target.id;

        if is_in_scope_now {
            // if let MathExpression::Object(current_obj) = &self {
            //     if current_obj.is_compatible(target_context, pattern, pattern_context) {
            //         matches.push(self.id.clone());
            //     }
            // }
            if self.is_compatible(target.clone(), target_context, pattern, pattern_context) {
                matches.push(current_id.clone());
            }
        }

        let sub_matches = match &self {
            MathExpression::Relation(rel) => rel.find_matches(
                target,
                current_id,
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            MathExpression::Object(obj) => obj.find_matches(
                target,
                current_id,
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            MathExpression::Expression(expr) => expr.find_matches(
                target,
                current_id,
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            _ => Vec::new(),
        };

        matches.extend(sub_matches);
        matches
    }
}

impl IsCompatible<MathExpression> for MathExpression {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (self, pattern) {
            (MathExpression::Object(self_obj), MathExpression::Object(pattern_obj)) => {
                self_obj.is_compatible(target, target_context, &pattern_obj, pattern_context)
            }
            (MathExpression::Relation(self_rel), MathExpression::Relation(pattern_rel)) => {
                self_rel.is_compatible(target, target_context, &pattern_rel, pattern_context)
            }
            (MathExpression::Expression(self_expr), MathExpression::Expression(pattern_expr)) => {
                self_expr.is_compatible(target, target_context, &pattern_expr, pattern_context)
            }
            _ => false,
        }
    }
}

impl Search for MathRelation {
    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> Vec<String> {
        let pattern_rel = pattern.get_relation().unwrap();
        let mut matches = Vec::new();
        let is_in_scope_now = in_target_scope || current_id == target.id;

        if is_in_scope_now {
            if self.is_compatible(
                target.clone(),
                target_context,
                &pattern_rel,
                pattern_context,
            ) {
                matches.push(current_id.clone());
            }
        }

        let sub_matches = match self {
            MathRelation::Equal { left, right } => {
                let mut left_matches = left.data.unwrap(&target_context).find_matches(
                    target.clone(),
                    current_id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                );
                let right_matches = right.data.unwrap(&target_context).find_matches(
                    target.clone(),
                    current_id.clone(),
                    target_context,
                    pattern,
                    pattern_context,
                    is_in_scope_now,
                );
                left_matches.extend(right_matches);
                left_matches
            }
            MathRelation::And(locateds) => todo!(),
            MathRelation::Or(locateds) => todo!(),
            MathRelation::Not(located) => todo!(),
            MathRelation::Implies(located, located1) => todo!(),
            MathRelation::Equivalent(located, located1) => todo!(),
            MathRelation::True => todo!(),
            MathRelation::False => todo!(),
            MathRelation::NumberTheory(number_theory_relation) => todo!(),
            MathRelation::SetTheory(set_relation) => todo!(),
            MathRelation::GroupTheory(group_relation) => todo!(),
            MathRelation::RingTheory(ring_relation) => todo!(),
            MathRelation::TopologyTheory(topology_relation) => todo!(),
            MathRelation::CategoryTheory(category_relation) => todo!(),
            MathRelation::ProbabilityTheory(probability_relation) => todo!(),
        };

        matches.extend(sub_matches);
        matches
    }
}

impl IsCompatible<MathRelation> for MathRelation {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathRelation,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        match (self, pattern) {
            (
                MathRelation::Equal { left, right },
                MathRelation::Equal {
                    left: pattern_left,
                    right: pattern_right,
                },
            ) => {
                // todo: we should check both equality from both hand side.
                left.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &pattern_left.data.unwrap(&pattern_context),
                    pattern_context,
                ) && right.data.unwrap(&target_context).is_compatible(
                    target,
                    target_context,
                    &pattern_right.data.unwrap(&pattern_context),
                    pattern_context,
                )
            }
            (MathRelation::And(locateds), MathRelation::And(pattern_locateds)) => {
                // todo: we should allow order difference, the list is cummutative and associative
                locateds.iter().all(|located| {
                    located.data.unwrap(&target_context).is_compatible(
                        target.clone(),
                        target_context,
                        &located.data.unwrap(&pattern_context),
                        pattern_context,
                    )
                })
            }
            (MathRelation::Or(locateds), MathRelation::Or(pattern_locateds)) => {
                // todo: we should allow order random pair we need only one node from target and pattern list to make a valid pair
                locateds.iter().any(|located| {
                    located.data.unwrap(&target_context).is_compatible(
                        target.clone(),
                        target_context,
                        &located.data.unwrap(&pattern_context),
                        pattern_context,
                    )
                })
            }
            (MathRelation::Not(located), MathRelation::Not(pattern_located)) => {
                located.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                )
            }
            (
                MathRelation::Implies(located, located1),
                MathRelation::Implies(pattern_located, pattern_located1),
            ) => {
                located.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                ) && located1.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located1.data.unwrap(&pattern_context),
                    pattern_context,
                )
            }
            (
                MathRelation::Equivalent(located, located1),
                MathRelation::Equivalent(pattern_located, pattern_located1),
            ) => {
                // todo: the antecedent and precedent can we reversed, we need to try in both directions
                located.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                ) && located1.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located.data.unwrap(&pattern_context),
                    pattern_context,
                ) && located1.data.unwrap(&target_context).is_compatible(
                    target.clone(),
                    target_context,
                    &located1.data.unwrap(&pattern_context),
                    pattern_context,
                )
            }
            _ => false,
        }
    }
}

impl Search for MathObject {
    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> Vec<String> {
        let pattern_obj = pattern.get_object().unwrap();
        let mut matches = Vec::new();
        let is_in_scope_now = in_target_scope || current_id == target.id;

        if is_in_scope_now {
            if self.is_compatible(
                target.clone(),
                target_context,
                &pattern_obj,
                pattern_context,
            ) {
                matches.push(current_id.clone());
            }
        }

        let sub_matches = match self {
            MathObject::Group(group) => group.find_matches(
                target.clone(),
                current_id.clone(),
                target_context,
                pattern,
                pattern_context,
                is_in_scope_now,
            ),
            MathObject::Ring(ring) => todo!(),
            MathObject::Field(field) => todo!(),
            MathObject::Module(module) => todo!(),
            MathObject::Algebra(algebra) => todo!(),
            MathObject::TopologicalSpace(topological_space) => todo!(),
            MathObject::VectorSpace(vector_space) => todo!(),
            MathObject::Set(set) => todo!(),
            MathObject::Function(function) => todo!(),
        };

        matches.extend(sub_matches);
        matches
    }
}

impl IsCompatible<MathObject> for MathObject {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathObject,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        todo!()
    }
}

impl Search for TheoryExpression {
    fn find_matches(
        &self,
        target: Target,
        current_id: String,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        pattern_context: &Vec<ContextEntry>,
        in_target_scope: bool,
    ) -> Vec<String> {
        todo!()
    }
}

impl IsCompatible<TheoryExpression> for TheoryExpression {
    fn is_compatible(
        &self,
        target: Target,
        target_context: &Vec<ContextEntry>,
        pattern: &TheoryExpression,
        pattern_context: &Vec<ContextEntry>,
    ) -> bool {
        todo!()
    }
}
