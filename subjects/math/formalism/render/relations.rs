use super::super::extract::Parametrizable;
use super::super::location::Located;
use super::super::relations::MathRelation;
use crate::turn_render::*;
use std::{string::String, sync::Arc};

impl ToTurnMath for MathRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            MathRelation::Equal { left, right, .. } => {
                let lhs = left.data.to_turn_math(left.id.clone());
                let rhs = right.data.to_turn_math(right.id.clone());

                MathNode {
                    id: master_id,
                    content: Arc::new(MathNodeContent::Relationship {
                        lhs: Arc::new(lhs),
                        rhs: Arc::new(rhs),
                        operator: RelationOperatorNode::Equal,
                    }),
                }
            }
            MathRelation::And(relations) => {
                if relations.len() == 0 {
                    panic!("And relation vec with no elements inside");
                } else if relations.len() == 1 {
                    // Single relation, just return it directly
                    relations[0].data.to_turn_math(relations[0].id.clone())
                } else {
                    MathNode {
                        id: master_id.clone(),
                        content: Arc::new(MathNodeContent::And(
                            relations
                                .iter()
                                .map(|r| r.data.to_turn_math(r.id.clone()))
                                .collect(),
                        )),
                    }
                }
            }
            MathRelation::Or(relations) => {
                if relations.len() == 0 {
                    panic!("And relation vec with no elements inside");
                } else if relations.len() == 1 {
                    // Single relation, just return it directly
                    relations[0].data.to_turn_math(relations[0].id.clone())
                } else {
                    MathNode {
                        id: master_id.clone(),
                        content: Arc::new(MathNodeContent::Or(
                            relations
                                .iter()
                                .map(|r| r.data.to_turn_math(r.id.clone()))
                                .collect(),
                        )),
                    }
                }
            }
            MathRelation::Not(relation) => {
                let inner_id = format!("{}_inner", master_id);
                MathNode {
                    id: master_id,
                    content: Arc::new(MathNodeContent::UnaryPrefixOperation {
                        parameter: Arc::new(relation.data.to_turn_math(relation.id.clone())),
                        operator: Arc::new(MathNode {
                            id: "unique id for this operator".to_string(),
                            content: Arc::new(MathNodeContent::Identifier(Identifier {
                                body: "¬".to_string(),
                                pre_script: None,
                                mid_script: None, // TODO: add mid script
                                post_script: None,
                                primes: 0,
                                is_function: false, // TODO: add is function
                            })),
                        }),
                    }),
                }
            }
            MathRelation::Implies(antecedent, consequent) => {
                let lhs = antecedent.data.to_turn_math(antecedent.id.clone());
                let rhs = consequent.data.to_turn_math(consequent.id.clone());

                MathNode {
                    id: master_id,
                    content: Arc::new(MathNodeContent::Relationship {
                        lhs: Arc::new(lhs),
                        rhs: Arc::new(rhs),
                        operator: RelationOperatorNode::Implies,
                    }),
                }
            }
            MathRelation::Equivalent(left, right) => {
                let lhs = left.data.to_turn_math(left.id.clone());
                let rhs = right.data.to_turn_math(right.id.clone());

                MathNode {
                    id: master_id,
                    content: Arc::new(MathNodeContent::Relationship {
                        lhs: Arc::new(lhs),
                        rhs: Arc::new(rhs),
                        operator: RelationOperatorNode::Iff,
                    }),
                }
            }
            MathRelation::NumberTheory(nr) => nr.to_turn_math(master_id),
            MathRelation::SetTheory(sr) => todo!(),
            MathRelation::GroupTheory(gr) => gr.to_turn_math(master_id),
            MathRelation::RingTheory(rr) => todo!(),
            MathRelation::TopologyTheory(tr) => todo!(),
            MathRelation::CategoryTheory(cr) => todo!(),
            MathRelation::ProbabilityTheory(pr) => pr.to_turn_math(master_id),

            MathRelation::True => MathNode {
                id: master_id,
                content: Arc::new(MathNodeContent::Identifier(Identifier {
                    body: "True".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                })),
            },
            MathRelation::False => MathNode {
                id: master_id,
                content: Arc::new(MathNodeContent::Identifier(Identifier {
                    body: "False".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                })),
            },
            MathRelation::ProbabilityTheory(prob_rel) => MathNode {
                id: "prob_rel".to_string(),
                content: Arc::new(MathNodeContent::Text("Probability Relation".to_string())),
            },
            _ => MathNode {
                id: "unknown_rel".to_string(),
                content: Arc::new(MathNodeContent::Text("Unknown Relation".to_string())),
            },
        }
    }
}

impl ToLogicalNode for MathRelation {
    fn to_logical_node(&self) -> LogicalNode {
        match self {
            MathRelation::And(relations) => {
                LogicalNode::And(
                    relations
                        .iter()
                        .map(|r| {
                            // Use safe rendering to avoid infinite recursion
                            match &r.data {
                                Parametrizable::Concrete(arc) => {
                                    // For concrete values, render the actual relation
                                    arc.to_logical_node()
                                }
                                Parametrizable::Variable(id) => {
                                    // For variables, just render the identifier
                                    LogicalNode::Atomic(MathNode {
                                        id: r.id.clone(),
                                        content: Arc::new(MathNodeContent::Identifier(id.clone())),
                                    })
                                }
                            }
                        })
                        .collect(),
                )
            }
            MathRelation::Or(relations) => {
                LogicalNode::Or(
                    relations
                        .iter()
                        .map(|r| {
                            // Use safe rendering to avoid infinite recursion
                            match &r.data {
                                Parametrizable::Concrete(arc) => {
                                    // For concrete values, render the actual relation
                                    arc.to_logical_node()
                                }
                                Parametrizable::Variable(id) => {
                                    // For variables, just render the identifier
                                    LogicalNode::Atomic(MathNode {
                                        id: r.id.clone(),
                                        content: Arc::new(MathNodeContent::Identifier(id.clone())),
                                    })
                                }
                            }
                        })
                        .collect(),
                )
            }

            _ => LogicalNode::Atomic(self.to_turn_math("".to_string())),
        }
    }
}
