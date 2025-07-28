use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use crate::subjects::math::export::unified_exporter::TheoryExporter;

use crate::subjects::math::formalism::extract::Parametrizable;

//--- Imports from crate::turn_render ---
use crate::turn_render::ToMathDocument;
use crate::turn_render::math_node::{
    BracketSize, BracketStyle, IntegralType, MathNode, MathNodeContent, MulSymbol,
    RefinedMulOrDivOperation, RelationOperatorNode, ScriptNode, ToTurnMath,
    UnaryRelationOperatorNode,
};
use crate::turn_render::*;

//--- Imports from this crate (subjects) ---
use crate::subjects::math::formalism::traits::abstraction_level::{
    AbstractionLevel, GetAbstractionLevel,
};
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::fields::definitions::Field;
use crate::subjects::math::theories::topology::definitions::{TopologicalSpace, Topology};
use crate::subjects::math::theories::zfc::definitions::Set;

//--- Imports from probability definitions ---
use crate::subjects::math::theories::probability::definitions::{
    BrownianMotion, ConditionalProbabilitySpace, ContinuousProbabilitySpace,
    DiscreteProbabilitySpace, Distribution, Event, GenericProbabilitySpace, MarkovChain,
    Martingale, ProbabilityExpression, ProbabilityMeasure, ProbabilityRelation, ProbabilitySpace,
    RandomVariable, SigmaAlgebra, StochasticProcess,
};

use crate::subjects::math::theories::probability::theorems::all_probability_theorems;

// Include render modules (to be created later)
pub mod continuous_probability;
pub mod discrete_probability;
pub mod probability_basic;
pub mod stochastic_processes;
pub mod tests;

// === TOMATH IMPLEMENTATIONS ===

impl ToTurnMath for ProbabilitySpace {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            ProbabilitySpace::Generic(p) => p.to_turn_math(master_id),
            ProbabilitySpace::Discrete(p) => p.to_turn_math(master_id),
            ProbabilitySpace::Continuous(p) => p.core.to_turn_math(master_id),
            ProbabilitySpace::Product(p) => p.core.to_turn_math(master_id),
            ProbabilitySpace::Conditional(p) => p.core.to_turn_math(master_id),
            ProbabilitySpace::StochasticProcess(p) => p.core.to_turn_math(master_id),
            ProbabilitySpace::MarkovChain(p) => p.core.core.to_turn_math(master_id),
            ProbabilitySpace::Martingale(p) => p.core.core.to_turn_math(master_id),
            ProbabilitySpace::BrownianMotion(p) => p.core.core.to_turn_math(master_id),
        }
    }
}

impl ToTurnMath for GenericProbabilitySpace {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Arc::new(MathNodeContent::Bracketed {
                inner: Arc::new(MathNode {
                    id: format!("{}-tuple", master_id),
                    content: Arc::new(MathNodeContent::Multiplications {
                        terms: vec![
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}-omega", master_id),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "Ω".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                                },
                            ),
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}-sigma", master_id),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "ℱ".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                                },
                            ),
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}-measure", master_id),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "P".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                                },
                            ),
                        ],
                    }),
                }),
                style: BracketStyle::Round,
                size: BracketSize::Normal,
            }),
        }
    }
}

impl ToTurnMath for DiscreteProbabilitySpace {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Arc::new(MathNodeContent::Bracketed {
                inner: Arc::new(MathNode {
                    id: format!("{}-tuple", master_id),
                    content: Arc::new(MathNodeContent::Multiplications {
                        terms: vec![
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}-omega-discrete", master_id),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "Ω".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: Some(ScriptNode {
                                            subscripts: vec![MathNode {
                                                id: format!("{}-discrete-sub", master_id),
                                                content: Arc::new(MathNodeContent::Identifier(
                                                    Identifier {
                                                        body: "discrete".to_string(),
                                                        pre_script: None,
                                                        mid_script: None,
                                                        post_script: None,
                                                        primes: 0,
                                                        is_function: false,
                                                    },
                                                )),
                                            }],
                                            superscripts: vec![],
                                        }),
                                        primes: 0,
                                        is_function: false,
                                    })),
                                },
                            ),
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}-sigma", master_id),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "ℱ".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                                },
                            ),
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}-measure", master_id),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "P".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                                },
                            ),
                        ],
                    }),
                }),
                style: BracketStyle::Round,
                size: BracketSize::Normal,
            }),
        }
    }
}

impl ToTurnMath for ContinuousProbabilitySpace {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Arc::new(MathNodeContent::Bracketed {
                inner: Arc::new(MathNode {
                    id: format!("{}-tuple", master_id),
                    content: Arc::new(MathNodeContent::Multiplications {
                        terms: vec![
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}-omega-continuous", master_id),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "Ω".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: Some(ScriptNode {
                                            subscripts: vec![MathNode {
                                                id: format!("{}-continuous-sub", master_id),
                                                content: Arc::new(MathNodeContent::Identifier(
                                                    Identifier {
                                                        body: "continuous".to_string(),
                                                        pre_script: None,
                                                        mid_script: None,
                                                        post_script: None,
                                                        primes: 0,
                                                        is_function: false,
                                                    },
                                                )),
                                            }],
                                            superscripts: vec![],
                                        }),
                                        primes: 0,
                                        is_function: false,
                                    })),
                                },
                            ),
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}-sigma", master_id),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "ℱ".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                                },
                            ),
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}-measure", master_id),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "P".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                                },
                            ),
                        ],
                    }),
                }),
                style: BracketStyle::Round,
                size: BracketSize::Normal,
            }),
        }
    }
}

impl ToTurnMath for StochasticProcess {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Arc::new(MathNodeContent::Identifier(Identifier {
                body: "X_t".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            })),
        }
    }
}

impl ToTurnMath for MarkovChain {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        self.core.core.to_turn_math(master_id)
    }
}

impl ToTurnMath for Martingale {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        self.core.core.to_turn_math(master_id)
    }
}

impl ToTurnMath for BrownianMotion {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        self.core.core.to_turn_math(master_id)
    }
}

// === TOSECTIONNODE IMPLEMENTATIONS ===

impl ToSectionNode for ProbabilitySpace {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        match self {
            ProbabilitySpace::Generic(p) => p.to_section_node(id_prefix),
            ProbabilitySpace::Discrete(p) => p.to_section_node(id_prefix),
            ProbabilitySpace::Continuous(p) => p.core.to_section_node(id_prefix),
            ProbabilitySpace::Product(p) => p.core.to_section_node(id_prefix),
            ProbabilitySpace::Conditional(p) => p.core.to_section_node(id_prefix),
            ProbabilitySpace::StochasticProcess(p) => p.core.to_section_node(id_prefix),
            ProbabilitySpace::MarkovChain(p) => p.core.core.to_section_node(id_prefix),
            ProbabilitySpace::Martingale(p) => p.core.core.to_section_node(id_prefix),
            ProbabilitySpace::BrownianMotion(p) => p.core.core.to_section_node(id_prefix),
        }
    }
}

impl ToMathDocument for ProbabilitySpace {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        match self {
            ProbabilitySpace::Generic(p) => {
                let level = p.level();
                if level == AbstractionLevel::Level1 {
                    p.render_as_l1_schema_document(&format!("{}.generic", id_prefix))
                } else {
                    p.to_math_document(&format!("{}.generic", id_prefix))
                }
            }
            ProbabilitySpace::Discrete(p) => p.to_math_document(&format!("{}.discrete", id_prefix)),
            ProbabilitySpace::Continuous(p) => p
                .core
                .to_math_document(&format!("{}.continuous", id_prefix)),
            ProbabilitySpace::Product(p) => {
                p.core.to_math_document(&format!("{}.product", id_prefix))
            }
            ProbabilitySpace::Conditional(p) => p
                .core
                .to_math_document(&format!("{}.conditional", id_prefix)),
            ProbabilitySpace::StochasticProcess(p) => p
                .core
                .to_math_document(&format!("{}.stochastic", id_prefix)),
            ProbabilitySpace::MarkovChain(p) => p
                .core
                .core
                .to_math_document(&format!("{}.markov", id_prefix)),
            ProbabilitySpace::Martingale(p) => p
                .core
                .core
                .to_math_document(&format!("{}.martingale", id_prefix)),
            ProbabilitySpace::BrownianMotion(p) => p
                .core
                .core
                .to_math_document(&format!("{}.brownian", id_prefix)),
        }
    }
}

impl ProbabilitySpace {
    pub fn get_id(&self) -> String {
        match self {
            ProbabilitySpace::Generic(_) => "generic_probability_space".to_string(),
            ProbabilitySpace::Discrete(_) => "discrete_probability_space".to_string(),
            ProbabilitySpace::Continuous(_) => "continuous_probability_space".to_string(),
            ProbabilitySpace::Product(_) => "product_probability_space".to_string(),
            ProbabilitySpace::Conditional(_) => "conditional_probability_space".to_string(),
            ProbabilitySpace::StochasticProcess(_) => "stochastic_process".to_string(),
            ProbabilitySpace::MarkovChain(_) => "markov_chain".to_string(),
            ProbabilitySpace::Martingale(_) => "martingale".to_string(),
            ProbabilitySpace::BrownianMotion(_) => "brownian_motion".to_string(),
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        match self {
            ProbabilitySpace::Generic(_) => vec![RichTextSegment::Text(
                "Generic Probability Space".to_string(),
            )],
            ProbabilitySpace::Discrete(_) => {
                vec![RichTextSegment::Text(
                    "Discrete Probability Space".to_string(),
                )]
            }
            ProbabilitySpace::Continuous(_) => vec![RichTextSegment::Text(
                "Continuous Probability Space".to_string(),
            )],
            ProbabilitySpace::Product(_) => {
                vec![RichTextSegment::Text(
                    "Product Probability Space".to_string(),
                )]
            }
            ProbabilitySpace::Conditional(_) => vec![RichTextSegment::Text(
                "Conditional Probability Space".to_string(),
            )],
            ProbabilitySpace::StochasticProcess(_) => {
                vec![RichTextSegment::Text("Stochastic Process".to_string())]
            }
            ProbabilitySpace::MarkovChain(_) => {
                vec![RichTextSegment::Text("Markov Chain".to_string())]
            }
            ProbabilitySpace::Martingale(_) => {
                vec![RichTextSegment::Text("Martingale".to_string())]
            }
            ProbabilitySpace::BrownianMotion(_) => {
                vec![RichTextSegment::Text("Brownian Motion".to_string())]
            }
        }
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        match self {
            ProbabilitySpace::Generic(_) => vec![RichTextSegment::Text("(Ω, ℱ, P)".to_string())],
            ProbabilitySpace::Discrete(_) => {
                vec![RichTextSegment::Text("(Ωd, ℱd, Pd)".to_string())]
            }
            ProbabilitySpace::Continuous(_) => {
                vec![RichTextSegment::Text("(Ωc, ℱc, Pc)".to_string())]
            }
            ProbabilitySpace::Product(_) => {
                vec![RichTextSegment::Text("Product Space".to_string())]
            }
            ProbabilitySpace::Conditional(_) => {
                vec![RichTextSegment::Text("Conditional Space".to_string())]
            }
            ProbabilitySpace::StochasticProcess(_) => {
                vec![RichTextSegment::Text("Stochastic Process".to_string())]
            }
            ProbabilitySpace::MarkovChain(_) => {
                vec![RichTextSegment::Text("Markov Chain".to_string())]
            }
            ProbabilitySpace::Martingale(_) => {
                vec![RichTextSegment::Text("Martingale".to_string())]
            }
            ProbabilitySpace::BrownianMotion(_) => {
                vec![RichTextSegment::Text("Brownian Motion".to_string())]
            }
        }
    }
}

impl ProbabilityExpression {
    pub fn get_id(&self) -> String {
        match self {
            ProbabilityExpression::EventProbability { .. } => "event_probability".to_string(),
            ProbabilityExpression::ConditionalProbability { .. } => {
                "conditional_probability".to_string()
            }
            ProbabilityExpression::ExpectedValue { .. } => "expected_value".to_string(),
            ProbabilityExpression::Variance { .. } => "variance".to_string(),
            ProbabilityExpression::Covariance { .. } => "covariance".to_string(),
            ProbabilityExpression::Moment { .. } => "moment".to_string(),
            ProbabilityExpression::CharacteristicFunction { .. } => {
                "characteristic_function".to_string()
            }
            ProbabilityExpression::MomentGeneratingFunction { .. } => {
                "moment_generating_function".to_string()
            }
            _ => "probability_expression".to_string(),
        }
    }
}

impl ProbabilityRelation {
    pub fn get_id(&self) -> String {
        match self {
            ProbabilityRelation::EventsAreIndependent { .. } => "events_independent".to_string(),
            ProbabilityRelation::RandomVariablesAreIndependent { .. } => {
                "random_variables_independent".to_string()
            }
            ProbabilityRelation::HasDistribution { .. } => "has_distribution".to_string(),
            ProbabilityRelation::IdenticallyDistributed { .. } => {
                "identically_distributed".to_string()
            }
            ProbabilityRelation::ConvergesTo { .. } => "converges_to".to_string(),
            _ => "probability_relation".to_string(),
        }
    }
}

impl ToTurnMath for Box<ProbabilitySpace> {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        self.as_ref().to_turn_math(master_id)
    }
}

impl ToTurnMath for ProbabilityExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            ProbabilityExpression::EventProbability { event, .. } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::FunctionCall {
                    name: Arc::new(MathNode {
                        id: format!("{}-prob-name", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "P".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: true,
                        })),
                    }),
                    parameters: vec![MathNode {
                        id: format!("{}-event", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "A".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        })),
                    }],
                }),
            },
            ProbabilityExpression::ConditionalProbability {
                event,
                conditioning_event,
                ..
            } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::FunctionCall {
                    name: Arc::new(MathNode {
                        id: format!("{}-prob-name", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "P".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: true,
                        })),
                    }),
                    parameters: vec![MathNode {
                        id: format!("{}-conditional", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "A|B".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        })),
                    }],
                }),
            },
            ProbabilityExpression::ExpectedValue { variable } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::FunctionCall {
                    name: Arc::new(MathNode {
                        id: format!("{}-exp-name", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "E".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: true,
                        })),
                    }),
                    parameters: vec![MathNode {
                        id: format!("{}-rv", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "X".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        })),
                    }],
                }),
            },
            ProbabilityExpression::Variance { variable } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::FunctionCall {
                    name: Arc::new(MathNode {
                        id: format!("{}-var-name", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "Var".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: true,
                        })),
                    }),
                    parameters: vec![MathNode {
                        id: format!("{}-rv", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "X".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        })),
                    }],
                }),
            },
            _ => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Identifier(Identifier {
                    body: "ProbExpr".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                })),
            },
        }
    }
}

impl ToTurnMath for ProbabilityRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            ProbabilityRelation::EventsAreIndependent { events, .. } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                    lhs: Arc::new(MathNode {
                        id: format!("{}-lhs", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "P(A∩B)".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        })),
                    }),
                    rhs: Arc::new(MathNode {
                        id: format!("{}-rhs", master_id),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: "P(A)P(B)".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        })),
                    }),
                    operator: RelationOperatorNode::Equal,
                }),
            },
            ProbabilityRelation::RandomVariablesAreIndependent { variables, .. } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Text(format!(
                    "{} random variables are independent",
                    variables.len()
                ))),
            },
            _ => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Identifier(Identifier {
                    body: "ProbRel".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                })),
            },
        }
    }
}

impl ProbabilityRelation {
    pub fn to_structured_probability_relation(&self) -> String {
        // This is a placeholder. A more comprehensive implementation would be needed.
        match self {
            ProbabilityRelation::EventsAreIndependent { .. } => {
                "Events are independent".to_string()
            }
            ProbabilityRelation::RandomVariablesAreIndependent { .. } => {
                "Random variables are independent".to_string()
            }
            _ => "Probability relation".to_string(),
        }
    }
}

/// Probability Theory Exporter Implementation
pub struct ProbabilityTheoryExporter;

impl TheoryExporter<ProbabilitySpace, ProbabilityExpression, ProbabilityRelation>
    for ProbabilityTheoryExporter
{
    fn theory_id(&self) -> &str {
        "probability_theory"
    }

    fn theory_name(&self) -> &str {
        "Probability Theory"
    }

    fn export_theory_overview(&self) -> MathDocument {
        let id_prefix = "probability_theory.overview";
        let main_section = Section {
            id: format!("{}.fundamental", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Fundamental Concepts".to_string())],
                alignment: None,
            }),
            content: vec![
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "Basic probability spaces and measures.".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::StructuredMath(StructuredMathNode::Definition {
                    term_display: RichText {
                        segments: vec![RichTextSegment::Text("Fundamental Concepts".to_string())],
                        alignment: None,
                    },
                    formal_term: None,
                    label: Some("Fundamental probability concepts".to_string()),
                    body: vec![SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(
                            "Basic probability spaces and measures.".to_string(),
                        )],
                        alignment: None,
                    })],
                    abstraction_meta: None,
                    selectable_properties: vec![],
                }),
            ],
            metadata: vec![],
            display_options: None,
        };

        let continuous_section = Section {
            id: format!("{}.continuous", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Continuous Probability".to_string())],
                alignment: None,
            }),
            content: vec![
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "Continuous probability spaces and measure theory.".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::StructuredMath(StructuredMathNode::Definition {
                    term_display: RichText {
                        segments: vec![RichTextSegment::Text("Continuous Probability".to_string())],
                        alignment: None,
                    },
                    formal_term: None,
                    label: Some("Continuous probability theory".to_string()),
                    body: vec![SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(
                            "Continuous probability spaces and measure theory.".to_string(),
                        )],
                        alignment: None,
                    })],
                    abstraction_meta: None,
                    selectable_properties: vec![],
                }),
            ],
            metadata: vec![],
            display_options: None,
        };

        let processes_section = Section {
            id: format!("{}.processes", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Stochastic Processes".to_string())],
                alignment: None,
            }),
            content: vec![
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "Time-dependent random phenomena and processes.".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::StructuredMath(StructuredMathNode::Definition {
                    term_display: RichText {
                        segments: vec![RichTextSegment::Text("Stochastic Processes".to_string())],
                        alignment: None,
                    },
                    formal_term: None,
                    label: Some("Stochastic processes and Markov chains".to_string()),
                    body: vec![SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(
                            "Time-dependent random phenomena and processes.".to_string(),
                        )],
                        alignment: None,
                    })],
                    abstraction_meta: None,
                    selectable_properties: vec![],
                }),
            ],
            metadata: vec![],
            display_options: None,
        };

        MathDocument {
            id: id_prefix.to_string(),
            content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                title: "Probability Theory".to_string(),
                paper_type: PaperType::Survey,
                venue: Some("Mathematical Survey".to_string()),
                peer_reviewed: true,
                content_metadata: ContentMetadata {
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    created_at: None,
                    last_modified: None,
                    content_hash: None,
                },
                academic_metadata: AcademicMetadata {
                    authors: vec!["Turn-Formal System".to_string()],
                    date_published: None,
                    date_modified: None,
                    venue: Some("Mathematical Survey".to_string()),
                    doi: None,
                    keywords: vec!["probability".to_string(), "random variables".to_string(), "stochastic processes".to_string()],
                },
                structure: DocumentStructure {
                    abstract_content: Some(Section {
                        id: format!("{}.abstract", id_prefix),
                        title: None,
                        content: vec![SectionContentNode::RichText(RichText {
                            segments: vec![RichTextSegment::Text(
                                "Probability theory provides a rigorous mathematical framework for analyzing randomness and uncertainty. This comprehensive overview covers probability spaces, random variables, distributions, and stochastic processes.".to_string(),
                            )],
                            alignment: None,
                        })],
                        metadata: vec![],
                        display_options: None,
                    }),
                    table_of_contents: None,
                    body: vec![main_section, continuous_section, processes_section],
                    footnotes: vec![],
                    glossary: vec![],
                    bibliography: vec![],
                },
                relationships: DocumentRelationships {
                    parent_documents: vec![],
                    child_documents: vec![
                        "probability_theory.object_0.generic.schema.doc".to_string(),
                        "probability_theory.expression_0".to_string(),
                        "probability_theory.expression_1".to_string(),
                        "probability_theory.relation_0".to_string(),
                        "probability.weak_law_of_large_numbers-doc".to_string(),
                        "probability.strong_law_of_large_numbers-doc".to_string(),
                        "probability.central_limit_theorem-doc".to_string(),
                        "probability.bayes_theorem-doc".to_string(),
                        "probability.law_of_total_probability-doc".to_string(),
                        "probability.chebyshev_inequality-doc".to_string(),
                        "probability.markov_inequality-doc".to_string(),
                        "probability.jensen_inequality-doc".to_string(),
                        "probability.martingale_convergence-doc".to_string(),
                        "probability.optional_stopping-doc".to_string(),
                        "probability.kolmogorov_three_series-doc".to_string(),
                        "probability.glivenko_cantelli-doc".to_string(),
                    ],
                    related_concepts: vec![],
                    cross_references: vec![],
                    dependency_graph: None,
                },
            }),
        }
    }

    fn export_definitions(&self) -> Vec<MathDocument> {
        let mut definitions = vec![];

        // Generate object definitions
        let objects = self.generate_object_definitions();
        definitions.extend(self.export_object_definitions(objects));

        // Generate expression definitions
        let expressions = self.generate_expression_definitions();
        definitions.extend(self.export_expression_definitions(expressions));

        // Generate relation definitions
        let relations = self.generate_relation_definitions();
        definitions.extend(self.export_relation_definitions(relations));

        definitions
    }

    fn export_theorems(&self) -> Vec<MathDocument> {
        all_probability_theorems()
            .into_iter()
            .map(|thm| {
                let id = thm.id.clone();
                thm.to_math_document(&id)
            })
            .collect()
    }

    fn export_object_definitions(&self, objects: Vec<ProbabilitySpace>) -> Vec<MathDocument> {
        objects
            .into_iter()
            .map(|obj| obj.to_math_document(&obj.get_id()))
            .collect()
    }

    fn generate_object_definitions(&self) -> Vec<ProbabilitySpace> {
        vec![ProbabilitySpace::Generic(GenericProbabilitySpace::default())]
    }

    fn generate_expression_definitions(&self) -> Vec<ProbabilityExpression> {
        vec![
            ProbabilityExpression::EventProbability {
                event: Parametrizable::Variable(Identifier::new_simple("A".to_string())),
                probability_space: Parametrizable::Variable(Identifier::new_simple(
                    "Ω".to_string(),
                )),
            },
            ProbabilityExpression::ExpectedValue {
                variable: Parametrizable::Variable(Identifier::new_simple("X".to_string())),
            },
        ]
    }

    fn generate_relation_definitions(&self) -> Vec<ProbabilityRelation> {
        vec![ProbabilityRelation::EventsAreIndependent {
            events: vec![
                Parametrizable::Variable(Identifier::new_simple("A".to_string())),
                Parametrizable::Variable(Identifier::new_simple("B".to_string())),
            ],
            probability_space: Parametrizable::Variable(Identifier::new_simple("Ω".to_string())),
        }]
    }

    fn export_expression_definitions(
        &self,
        expressions: Vec<ProbabilityExpression>,
    ) -> Vec<MathDocument> {
        expressions
            .into_iter()
            .map(|expr| {
                let id = expr.get_id();
                let main_section = Section {
                    id: format!("{}-main", id),
                    title: Some(RichText {
                        segments: vec![RichTextSegment::Text("Expression".to_string())],
                        alignment: None,
                    }),
                    content: vec![SectionContentNode::StructuredMath(
                        StructuredMathNode::Definition {
                            term_display: RichText {
                                segments: vec![RichTextSegment::Math(
                                    expr.to_turn_math(id.clone()),
                                )],
                                alignment: None,
                            },
                            formal_term: None,
                            label: Some("Probability Expression".to_string()),
                            body: vec![],
                            abstraction_meta: None,
                            selectable_properties: vec![],
                        },
                    )],
                    metadata: vec![],
                    display_options: None,
                };
                MathDocument {
                    id,
                    content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                        title: "Probability Expression".to_string(),
                        paper_type: PaperType::Research,
                        venue: None,
                        peer_reviewed: false,
                        content_metadata: ContentMetadata {
                            language: Some("en-US".to_string()),
                            version: Some("1.0".to_string()),
                            created_at: None,
                            last_modified: None,
                            content_hash: None,
                        },
                        academic_metadata: AcademicMetadata {
                            authors: vec![],
                            date_published: None,
                            date_modified: None,
                            venue: None,
                            doi: None,
                            keywords: vec![],
                        },
                        structure: DocumentStructure {
                            abstract_content: None,
                            table_of_contents: None,
                            body: vec![main_section],
                            footnotes: vec![],
                            glossary: vec![],
                            bibliography: vec![],
                        },
                        relationships: DocumentRelationships {
                            parent_documents: vec![],
                            child_documents: vec![],
                            related_concepts: vec![],
                            cross_references: vec![],
                            dependency_graph: None,
                        },
                    }),
                }
            })
            .collect()
    }

    fn export_relation_definitions(
        &self,
        relations: Vec<ProbabilityRelation>,
    ) -> Vec<MathDocument> {
        relations
            .into_iter()
            .map(|rel| {
                let id = rel.get_id();
                let main_section = Section {
                    id: format!("{}-main", id),
                    title: Some(RichText {
                        segments: vec![RichTextSegment::Text("Relation".to_string())],
                        alignment: None,
                    }),
                    content: vec![SectionContentNode::StructuredMath(
                        StructuredMathNode::Definition {
                            term_display: RichText {
                                segments: vec![RichTextSegment::Math(rel.to_turn_math(id.clone()))],
                                alignment: None,
                            },
                            formal_term: None,
                            label: Some("Probability Relation".to_string()),
                            body: vec![],
                            abstraction_meta: None,
                            selectable_properties: vec![],
                        },
                    )],
                    metadata: vec![],
                    display_options: None,
                };
                MathDocument {
                    id,
                    content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                        title: "Probability Relation".to_string(),
                        paper_type: PaperType::Research,
                        venue: None,
                        peer_reviewed: false,
                        content_metadata: ContentMetadata {
                            language: Some("en-US".to_string()),
                            version: Some("1.0".to_string()),
                            created_at: None,
                            last_modified: None,
                            content_hash: None,
                        },
                        academic_metadata: AcademicMetadata {
                            authors: vec![],
                            date_published: None,
                            date_modified: None,
                            venue: None,
                            doi: None,
                            keywords: vec![],
                        },
                        structure: DocumentStructure {
                            abstract_content: None,
                            table_of_contents: None,
                            body: vec![main_section],
                            footnotes: vec![],
                            glossary: vec![],
                            bibliography: vec![],
                        },
                        relationships: DocumentRelationships {
                            parent_documents: vec![],
                            child_documents: vec![],
                            related_concepts: vec![],
                            cross_references: vec![],
                            dependency_graph: None,
                        },
                    }),
                }
            })
            .collect()
    }
}
