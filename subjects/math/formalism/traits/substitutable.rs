use crate::subjects::math::theories::groups::definitions::{
    GenericGroup, Group, GroupExpression, GroupHomomorphism, GroupOperation,
};
use crate::subjects::math::theories::rings::definitions::Ring;
use crate::subjects::math::theories::zfc::definitions::Set;
use crate::{
    subjects::math::formalism::{
        expressions::{MathExpression, TheoryExpression},
        extract::Parametrizable,
        interpretation::TypeViewOperator,
        location::Located,
        objects::MathObject,
        proof::{ContextEntry, tactics::Target},
        relations::MathRelation,
        traits::{
            collect_identifier::CollectIdentifier, debug::ShortDebug, detag::TryDetag,
            search::Search,
        },
    },
    turn_render::Identifier,
};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

use super::instantiable::InstantiationType;

/// A trait for substituting meta-variables in a template (`self`) using a pre-computed map.
pub trait Substitutable<U: 'static + Clone + Debug>: Sized {
    fn substitute(
        &self, // The replacement template
        instantiations: &HashMap<Identifier, InstantiationType>,
        target: &Located<U>,
        context: &Vec<ContextEntry>,
    ) -> Self;
}

impl<U: 'static + Clone + Debug + Search> Substitutable<U> for MathExpression {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, InstantiationType>,
        target: &Located<U>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        // This is the base case for the recursion: if the expression is a meta-variable, we replace it.
        // if let MathExpression::Object(obj) = self {
        //     if let MathObject::Group(Group::Generic(generic_group)) = &**obj {
        //         if let Set::Parametric { description, .. } = &generic_group.base_set {
        //             if is_meta_variable(description, context) {
        //                 let id_key = Identifier::new_simple(description.clone());
        //                 if let Some(substituted_expr) = instantiations.get(&id_key) {
        //                     return substituted_expr.clone();
        //                 }
        //             }
        //         }
        //     }
        // }
        // todo: the above is completely wrong.

        // Otherwise, we recurse.
        match self {
            MathExpression::Relation(rel) => {
                MathExpression::Relation(rel.substitute(instantiations, target, context))
            }
            MathExpression::Object(obj) => {
                MathExpression::Object(obj.substitute(instantiations, target, context))
            }
            MathExpression::Expression(expr) => {
                MathExpression::Expression(expr.substitute(instantiations, target, context))
            }
            MathExpression::ViewAs { expression, view } => MathExpression::ViewAs {
                expression: expression.substitute(instantiations, target, context),
                view: view.clone(),
            },
            _ => self.clone(),
        }
    }
}

impl<T: Substitutable<U> + Clone + Debug + 'static, U: 'static + Clone + Debug + Search>
    Substitutable<U> for Located<T>
where
    T: TryDetag<T>,
{
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, InstantiationType>,
        target: &Located<U>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match &self.data {
            Parametrizable::Concrete(arc_value) => {
                let substituted_value = arc_value.substitute(instantiations, target, context);

                Located {
                    id: self.id.clone(),
                    data: Parametrizable::Concrete(substituted_value),
                }
            }
            Parametrizable::Variable(id) => {
                if let Some(substituted_name) = instantiations.get(id) {
                    match substituted_name {
                        InstantiationType::Identifier(identifier) => Located {
                            id: self.id.clone(),
                            data: Parametrizable::Variable(identifier.clone()),
                        },
                        InstantiationType::LocatedId(id) => {
                            if let Some(located_expr) =
                                target.data.unwrap(context).get_located::<T>(id.clone())
                            {
                                located_expr
                            } else {
                                // If lookup fails, return the original variable unchanged
                                println!(
                                    "DEBUG: lookup failed for {} in target: {:#?}",
                                    id, target
                                );
                                self.clone()
                            }
                        }
                    }
                } else {
                    // Variable not found in instantiations - return unchanged
                    self.clone()
                }
            }
        }
    }
}

// Generic implementations for Arc-wrapped types
impl<T: Substitutable<U>, U: 'static + Clone + Debug> Substitutable<U> for Arc<T> {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, InstantiationType>,
        target: &Located<U>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        Arc::new((**self).substitute(instantiations, target, context))
    }
}

impl<U: 'static + Clone + Debug + Search> Substitutable<U> for MathObject {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, InstantiationType>,
        target: &Located<U>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match &self {
            MathObject::Group(group) => {
                MathObject::Group(group.substitute(instantiations, target, context))
            }
            MathObject::Set(set) => {
                MathObject::Set(set.substitute(instantiations, target, context))
            }
            // Add other MathObject variants here
            _ => self.clone(),
        }
    }
}

impl<U: 'static + Clone + Debug + Search> Substitutable<U> for MathRelation {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, InstantiationType>,
        target: &Located<U>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match &self {
            MathRelation::Equal { left, right } => {
                // For Arc-wrapped expressions, we need to unwrap, substitute, and re-wrap
                let new_left_expr =
                    left.data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                let new_right_expr =
                    right
                        .data
                        .unwrap(context)
                        .substitute(instantiations, target, context);

                MathRelation::Equal {
                    left: Located::new_concrete(new_left_expr),
                    right: Located::new_concrete(new_right_expr),
                }
            }
            MathRelation::And(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        let new_rel =
                            r.data
                                .unwrap(context)
                                .substitute(instantiations, target, context);
                        Located::new_concrete(new_rel)
                    })
                    .collect();
                MathRelation::And(new_relations)
            }
            MathRelation::Or(relations) => {
                let new_relations = relations
                    .iter()
                    .map(|r| {
                        let new_rel =
                            r.data
                                .unwrap(context)
                                .substitute(instantiations, target, context);
                        Located::new_concrete(new_rel)
                    })
                    .collect();
                MathRelation::Or(new_relations)
            }
            MathRelation::Not(relation) => {
                let new_rel =
                    relation
                        .data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                MathRelation::Not(Located::new_concrete(new_rel))
            }
            MathRelation::Implies(left, right) => {
                let new_left =
                    left.data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                let new_right =
                    right
                        .data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                MathRelation::Implies(
                    Located::new_concrete(new_left),
                    Located::new_concrete(new_right),
                )
            }
            MathRelation::Equivalent(left, right) => {
                let new_left =
                    left.data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                let new_right =
                    right
                        .data
                        .unwrap(context)
                        .substitute(instantiations, target, context);
                MathRelation::Equivalent(
                    Located::new_concrete(new_left),
                    Located::new_concrete(new_right),
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

// Implementation for TheoryExpression Substitutable
impl<U: 'static + Clone + Debug + Search> Substitutable<U> for TheoryExpression {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, InstantiationType>,
        target: &Located<U>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match self {
            TheoryExpression::Group(group_expr) => {
                TheoryExpression::Group(group_expr.substitute(instantiations, target, context))
            }
            _ => self.clone(), // TODO: Implement for other theory expressions
        }
    }
}
