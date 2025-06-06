use leptos::attr::Src;

use super::super::relations::MathRelation;
use super::expressions::ToStructuredFormat;
use crate::turn_render::*;
use std::string::String;

impl ToTurnMath for MathRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            MathRelation::Equal { left, right, .. } => {
                let lhs = left.to_turn_math(format!("{}_left", master_id));
                let rhs = right.to_turn_math(format!("{}_right", master_id));

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        operator: RelationOperatorNode::Equal,
                    }),
                }
            }
            MathRelation::And(relations) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: format!("∧({} relations)", relations.len()),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            MathRelation::Or(relations) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: format!("∨({} relations)", relations.len()),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            MathRelation::Not(relation) => {
                let inner_id = format!("{}_inner", master_id);
                let neg_id = format!("{}_neg", master_id);
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::UnaryPrefixOperation {
                        parameter: Box::new(relation.to_turn_math(inner_id)),
                        operator: Box::new(MathNode {
                            id: neg_id,
                            content: Box::new(MathNodeContent::Identifier {
                                body: "¬".to_string(),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        }),
                    }),
                }
            }
            MathRelation::Implies(antecedent, consequent) => {
                let lhs = antecedent.to_turn_math(format!("{}_ante", master_id));
                let rhs = consequent.to_turn_math(format!("{}_cons", master_id));

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        operator: RelationOperatorNode::Implies,
                    }),
                }
            }
            MathRelation::Equivalent(left, right) => {
                let lhs = left.to_turn_math(format!("{}_left", master_id));
                let rhs = right.to_turn_math(format!("{}_right", master_id));

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        operator: RelationOperatorNode::Iff,
                    }),
                }
            }
            MathRelation::NumberTheory(_) => todo!(),
            MathRelation::SetTheory(sr) => todo!(),
            MathRelation::GroupTheory(gr) => gr.to_turn_math(master_id),
            MathRelation::RingTheory(rr) => todo!(),
            MathRelation::TopologyTheory(tr) => todo!(),
            MathRelation::CategoryTheory(cr) => todo!(),
            MathRelation::Todo { name, .. } => todo!(),
        }
    }
}

impl ToStructuredFormat for MathRelation {
    type Output = String;

    fn to_structured(&self) -> Self::Output {
        match self {
            MathRelation::Equal { .. } => "Equality".to_string(),
            MathRelation::And(_) => "Conjunction".to_string(),
            MathRelation::Or(_) => "Disjunction".to_string(),
            MathRelation::Not(_) => "Negation".to_string(),
            MathRelation::Implies(_, _) => "Implication".to_string(),
            MathRelation::Equivalent(_, _) => "Equivalence".to_string(),
            MathRelation::NumberTheory(_) => "Number Theory Relation".to_string(),
            MathRelation::SetTheory(_) => "Set Theory Relation".to_string(),
            MathRelation::GroupTheory(_) => "Group Theory Relation".to_string(),
            MathRelation::RingTheory(_) => "Ring Theory Relation".to_string(),
            MathRelation::TopologyTheory(_) => "Topology Theory Relation".to_string(),
            MathRelation::CategoryTheory(_) => "Category Theory Relation".to_string(),
            MathRelation::Todo { name, .. } => format!("Relation TODO: {}", name),
        }
    }
}
