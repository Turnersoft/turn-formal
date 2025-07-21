use crate::subjects::math::theories::groups::definitions::{
    GenericGroup, Group, GroupExpression, GroupOperation,
};
use crate::subjects::math::theories::rings::definitions::Ring;
use crate::subjects::math::theories::zfc::definitions::Set;
use crate::{
    subjects::math::formalism::{
        expressions::{MathExpression, TheoryExpression},
        extract::Parametrizable,
        location::Located,
        objects::MathObject,
        proof::{ContextEntry, tactics::Target},
        relations::MathRelation,
    },
    turn_render::Identifier,
};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

use super::detag::TryDetag;

/// A trait for instantiating meta-variables by comparing a concrete expression (`self`) to a pattern.
/// we only map meta-variables in pattern to expression/variables in target, we will never allow pattern to have more
pub trait Instantiable: Sized {
    fn instantiate(
        &self, // The concrete target expression
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, MathExpression>;
}

/// A trait for substituting meta-variables in a template (`self`) using a pre-computed map.
pub trait Substitutable: Sized {
    fn substitute(
        &self, // The replacement template
        instantiations: &HashMap<Identifier, MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self;
}

pub trait Replace: Sized {
    fn replace(
        &self,
        current_id: &str,
        target: &Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        replacement: &MathExpression,
        pattern_and_replacement_context: &Vec<ContextEntry>,
    ) -> Self;
}

impl Replace for MathExpression {
    fn replace(
        &self,
        current_id: &str,
        target: &Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        replacement: &MathExpression,
        pattern_and_replacement_context: &Vec<ContextEntry>,
    ) -> Self {
        if current_id == target.id {
            let instantiations =
                self.instantiate(target_context, pattern, pattern_and_replacement_context);
            return replacement.substitute(&instantiations, pattern_and_replacement_context);
        }

        match self {
            MathExpression::Relation(rel) => MathExpression::Relation(Arc::new(rel.replace(
                current_id,
                target,
                target_context,
                pattern,
                replacement,
                pattern_and_replacement_context,
            ))),
            MathExpression::Object(obj) => MathExpression::Object(Arc::new(obj.replace(
                current_id,
                target,
                target_context,
                pattern,
                replacement,
                pattern_and_replacement_context,
            ))),
            MathExpression::Expression(expr) => MathExpression::Expression(expr.replace(
                current_id,
                target,
                target_context,
                pattern,
                replacement,
                pattern_and_replacement_context,
            )),
            _ => self.clone(),
        }
    }
}

impl Instantiable for MathExpression {
    fn instantiate(
        &self,
        target_context: &Vec<ContextEntry>,
        pattern: &Self,
        pattern_context: &Vec<ContextEntry>,
    ) -> HashMap<Identifier, MathExpression> {
        let mut instantiations = HashMap::new();

        instantiations
    }
}

impl Substitutable for MathExpression {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        // This is the base case for the recursion: if the expression is a meta-variable, we replace it.
        if let MathExpression::Object(obj) = self {
            if let MathObject::Group(Group::Generic(generic_group)) = &**obj {
                if let Set::Parametric { description, .. } = &generic_group.base_set {
                    if is_meta_variable(description, context) {
                        let id_key = Identifier::new_simple(description.clone());
                        if let Some(substituted_expr) = instantiations.get(&id_key) {
                            return substituted_expr.clone();
                        }
                    }
                }
            }
        }

        // Otherwise, we recurse.
        match self {
            MathExpression::Relation(rel) => {
                MathExpression::Relation(rel.substitute(instantiations, context))
            }
            MathExpression::Object(obj) => {
                MathExpression::Object(obj.substitute(instantiations, context))
            }
            MathExpression::Expression(expr) => {
                MathExpression::Expression(expr.substitute(instantiations, context))
            }
            MathExpression::ViewAs { expression, view } => MathExpression::ViewAs {
                expression: expression.substitute(instantiations, context),
                view: view.clone(),
            },
            _ => self.clone(),
        }
    }
}

impl<T: Substitutable + Clone + Debug + 'static> Substitutable for Located<T>
where
    T: TryDetag<T>,
{
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        let new_data = match &self.data {
            Parametrizable::Concrete(arc_value) => {
                let substituted_value = arc_value.substitute(instantiations, context);
                Parametrizable::Concrete(substituted_value)
            }
            Parametrizable::Variable(id) => {
                if let Some(expr) = instantiations.get(id) {
                    // Extract the correct type from the substituted expression
                    let concrete_value = TryDetag::<T>::detag(expr).clone();
                    Parametrizable::Concrete(Arc::new(concrete_value))
                } else {
                    self.data.clone()
                }
            }
        };

        Located {
            id: self.id.clone(),
            data: new_data,
        }
    }
}

// Generic implementations for Arc-wrapped types
impl<T: Substitutable> Substitutable for Arc<T> {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        Arc::new((**self).substitute(instantiations, context))
    }
}

impl Substitutable for MathObject {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match self {
            MathObject::Group(group) => {
                MathObject::Group(group.substitute(instantiations, context))
            }
            MathObject::Set(set) => MathObject::Set(set.substitute(instantiations, context)),
            // Add other MathObject variants here
            _ => self.clone(),
        }
    }
}

impl Substitutable for MathRelation {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match self {
            MathRelation::Equal { left, right } => {
                // For Arc-wrapped expressions, we need to unwrap, substitute, and re-wrap
                let new_left_expr = left
                    .data
                    .unwrap(context)
                    .substitute(instantiations, context);
                let new_right_expr = right
                    .data
                    .unwrap(context)
                    .substitute(instantiations, context);

                MathRelation::Equal {
                    left: Located::new(Parametrizable::Concrete(new_left_expr)),
                    right: Located::new(Parametrizable::Concrete(new_right_expr)),
                }
            }
            MathRelation::And(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        let new_rel = r.data.unwrap(context).substitute(instantiations, context);
                        Located::new(Parametrizable::Concrete(new_rel))
                    })
                    .collect();
                MathRelation::And(new_relations)
            }
            MathRelation::Or(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        let new_rel = r.data.unwrap(context).substitute(instantiations, context);
                        Located::new(Parametrizable::Concrete(new_rel))
                    })
                    .collect();
                MathRelation::Or(new_relations)
            }
            MathRelation::Not(relation) => {
                let new_rel = relation
                    .data
                    .unwrap(context)
                    .substitute(instantiations, context);
                MathRelation::Not(Located::new(Parametrizable::Concrete(new_rel)))
            }
            MathRelation::Implies(left, right) => {
                let new_left = left
                    .data
                    .unwrap(context)
                    .substitute(instantiations, context);
                let new_right = right
                    .data
                    .unwrap(context)
                    .substitute(instantiations, context);
                MathRelation::Implies(
                    Located::new(Parametrizable::Concrete(new_left)),
                    Located::new(Parametrizable::Concrete(new_right)),
                )
            }
            MathRelation::Equivalent(left, right) => {
                let new_left = left
                    .data
                    .unwrap(context)
                    .substitute(instantiations, context);
                let new_right = right
                    .data
                    .unwrap(context)
                    .substitute(instantiations, context);
                MathRelation::Equivalent(
                    Located::new(Parametrizable::Concrete(new_left)),
                    Located::new(Parametrizable::Concrete(new_right)),
                )
            }
            _ => self.clone(),
        }
    }
}

fn is_meta_variable(name: &str, _context: &Vec<ContextEntry>) -> bool {
    // A simple heuristic: if the name is all uppercase, it's a meta-variable.
    !name.is_empty() && name.chars().all(|c| c.is_ascii_uppercase())
}

// We implement the container traits by delegating to the main `MathExpression` implementation.
// This avoids code duplication.

impl Replace for MathObject {
    fn replace(
        &self,
        current_id: &str,
        target: &Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        replacement: &MathExpression,
        pattern_and_replacement_context: &Vec<ContextEntry>,
    ) -> Self {
        let expr_wrapper = MathExpression::Object(Arc::new(self.clone()));
        let result_expr = expr_wrapper.replace(
            current_id,
            target,
            target_context,
            pattern,
            replacement,
            pattern_and_replacement_context,
        );
        if let MathExpression::Object(new_obj) = result_expr {
            (*new_obj).clone()
        } else {
            self.clone() // Fallback if the replacement is not an object, which indicates a rule logic error.
        }
    }
}

impl Replace for MathRelation {
    fn replace(
        &self,
        current_id: &str,
        target: &Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        replacement: &MathExpression,
        pattern_and_replacement_context: &Vec<ContextEntry>,
    ) -> Self {
        // If this node is the target, we perform the replacement at the MathExpression level.
        if current_id == target.id {
            let expr_wrapper = MathExpression::Relation(Arc::new(self.clone()));
            let instantiations =
                expr_wrapper.instantiate(target_context, pattern, pattern_and_replacement_context);
            let result_expr =
                replacement.substitute(&instantiations, pattern_and_replacement_context);
            if let MathExpression::Relation(new_rel) = result_expr {
                return (*new_rel).clone();
            }
            return self.clone(); // Fallback on type mismatch
        }

        // Otherwise, we recurse into the children.
        match self {
            MathRelation::Equal { left, right } => {
                let new_left_expr = left.data.unwrap(target_context);
                let new_right_expr = right.data.unwrap(target_context);
                // during a replace, creating Located<> is important, we need to track the before and after so that we can
                let new_left = Located {
                    id: left.id.clone(),
                    data: Parametrizable::Concrete(Arc::new(new_left_expr.replace(
                        &left.id,
                        target,
                        target_context,
                        pattern,
                        replacement,
                        pattern_and_replacement_context,
                    ))),
                };
                let new_right = Located {
                    id: right.id.clone(),
                    data: Parametrizable::Concrete(Arc::new(new_right_expr.replace(
                        &right.id,
                        target,
                        target_context,
                        pattern,
                        replacement,
                        pattern_and_replacement_context,
                    ))),
                };
                MathRelation::Equal {
                    left: new_left,
                    right: new_right,
                }
            }
            MathRelation::Implies(left, right) => {
                let new_left = left.data.unwrap(target_context);
                let new_right = right.data.unwrap(target_context);

                let new_left = new_left.replace(
                    current_id,
                    target,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                );
                let new_right = new_right.replace(
                    current_id,
                    target,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                );
                MathRelation::Implies(
                    Located::new(Parametrizable::Concrete(Arc::new(new_left))),
                    Located::new(Parametrizable::Concrete(Arc::new(new_right))),
                )
            }
            MathRelation::Equivalent(left, right) => {
                let new_left = left.data.unwrap(target_context);
                let new_right = right.data.unwrap(target_context);

                let new_left = Located::new(Parametrizable::Concrete(Arc::new(new_left.replace(
                    current_id,
                    target,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                ))));
                let new_right =
                    Located::new(Parametrizable::Concrete(Arc::new(new_right.replace(
                        current_id,
                        target,
                        target_context,
                        pattern,
                        replacement,
                        pattern_and_replacement_context,
                    ))));
                MathRelation::Equivalent(new_left, new_right)
            }
            MathRelation::And(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        Located::new(Parametrizable::Concrete(Arc::new(
                            r.data.unwrap(target_context).replace(
                                current_id,
                                target,
                                target_context,
                                pattern,
                                replacement,
                                pattern_and_replacement_context,
                            ),
                        )))
                    })
                    .collect();
                MathRelation::And(new_relations)
            }
            MathRelation::Or(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        Located::new(Parametrizable::Concrete(Arc::new(
                            r.data.unwrap(target_context).replace(
                                current_id,
                                target,
                                target_context,
                                pattern,
                                replacement,
                                pattern_and_replacement_context,
                            ),
                        )))
                    })
                    .collect();
                MathRelation::Or(new_relations)
            }
            MathRelation::Not(relation) => {
                let new_relation = Located::new(Parametrizable::Concrete(Arc::new(
                    relation.data.unwrap(target_context).replace(
                        current_id,
                        target,
                        target_context,
                        pattern,
                        replacement,
                        pattern_and_replacement_context,
                    ),
                )));
                MathRelation::Not(new_relation)
            }
            MathRelation::True => MathRelation::True,
            MathRelation::False => MathRelation::False,
            MathRelation::NumberTheory(number_theory_relation) => todo!(),
            MathRelation::SetTheory(set_relation) => todo!(),
            MathRelation::GroupTheory(group_relation) => {
                MathRelation::GroupTheory(group_relation.replace(
                    current_id,
                    target,
                    target_context,
                    pattern,
                    replacement,
                    pattern_and_replacement_context,
                ))
            }
            MathRelation::RingTheory(ring_relation) => todo!(),
            MathRelation::TopologyTheory(topology_relation) => todo!(),
            MathRelation::CategoryTheory(category_relation) => todo!(),
            MathRelation::ProbabilityTheory(probability_relation) => todo!(),
        }
    }
}

impl Replace for TheoryExpression {
    fn replace(
        &self,
        _current_id: &str,
        _target: &Target,
        _target_context: &Vec<ContextEntry>,
        _pattern: &MathExpression,
        _replacement: &MathExpression,
        _pattern_and_replacement_context: &Vec<ContextEntry>,
    ) -> Self {
        self.clone()
    }
}

impl Substitutable for TheoryExpression {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match self {
            TheoryExpression::Group(g) => {
                TheoryExpression::Group(g.substitute(instantiations, context))
            }
            _ => self.clone(),
        }
    }
}
