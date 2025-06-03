use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::subjects::math::export::unified_exporter::TheoryExporter;
use crate::subjects::math::formalism::expressions::Identifier;
use crate::subjects::math::formalism::extract::Parametrizable;

//--- Imports from crate::turn_render ---
use crate::turn_render::math_node::{
    BracketSize, BracketStyle, IntegralType, MathNode, MathNodeContent, MulSymbol,
    RefinedMulOrDivOperation, RelationOperatorNode, ToTurnMath, UnaryRelationOperatorNode,
};
use crate::turn_render::section_node::{
    AbstractionMetadata, AcademicMetadata, ContentMetadata, DocumentRelationships,
    DocumentStructure, LinkTarget, MathDocument, MathematicalContent, MathematicalContentType,
    PaperType, ParagraphNode, RichTextSegment, ScientificPaperContent, Section, SectionContentNode,
    SelectableProperty, StructuredMathNode, TheoremLikeKind, ToSectionNode,
};

//--- Imports from this crate (subjects) ---
use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
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
            id: master_id,
            content: Box::new(MathNodeContent::Text("(Ω, ℱ, P)".to_string())),
        }
    }
}

impl ToTurnMath for DiscreteProbabilitySpace {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text("(Ω_discrete, ℱ, P)".to_string())),
        }
    }
}

impl ToTurnMath for ContinuousProbabilitySpace {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text("(Ω_continuous, ℱ, P)".to_string())),
        }
    }
}

impl ToTurnMath for StochasticProcess {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text("X_t".to_string())),
        }
    }
}

impl ToTurnMath for MarkovChain {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text("X_n".to_string())),
        }
    }
}

impl ToTurnMath for Martingale {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text("M_n".to_string())),
        }
    }
}

impl ToTurnMath for BrownianMotion {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text("B_t".to_string())),
        }
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
                .to_math_document(&format!("{}.stochastic_process", id_prefix)),
            ProbabilitySpace::MarkovChain(p) => p
                .core
                .core
                .to_math_document(&format!("{}.markov_chain", id_prefix)),
            ProbabilitySpace::Martingale(p) => p
                .core
                .core
                .to_math_document(&format!("{}.martingale", id_prefix)),
            ProbabilitySpace::BrownianMotion(p) => p
                .core
                .core
                .to_math_document(&format!("{}.brownian_motion", id_prefix)),
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        match self {
            ProbabilitySpace::Generic(_) => vec![RichTextSegment::Text(
                "Generic Probability Space".to_string(),
            )],
            ProbabilitySpace::Discrete(_) => vec![RichTextSegment::Text(
                "Discrete Probability Space".to_string(),
            )],
            ProbabilitySpace::Continuous(_) => vec![RichTextSegment::Text(
                "Continuous Probability Space".to_string(),
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
            _ => vec![RichTextSegment::Text("Probability Space".to_string())],
        }
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        match self {
            ProbabilitySpace::Generic(_) => vec![RichTextSegment::Text("(Ω, ℱ, P)".to_string())],
            ProbabilitySpace::Discrete(_) => {
                vec![RichTextSegment::Text("(Ω_discrete, ℱ, P)".to_string())]
            }
            ProbabilitySpace::Continuous(_) => {
                vec![RichTextSegment::Text("(Ω_continuous, ℱ, P)".to_string())]
            }
            ProbabilitySpace::StochasticProcess(_) => {
                vec![RichTextSegment::Text("X_t".to_string())]
            }
            ProbabilitySpace::MarkovChain(_) => vec![RichTextSegment::Text("X_n".to_string())],
            ProbabilitySpace::Martingale(_) => vec![RichTextSegment::Text("M_n".to_string())],
            ProbabilitySpace::BrownianMotion(_) => vec![RichTextSegment::Text("B_t".to_string())],
            _ => vec![RichTextSegment::Text("(Ω, ℱ, P)".to_string())],
        }
    }
}

// === IMPLEMENTATIONS FOR BOX<PROBABILITYSPACE> ===

impl ToTurnMath for Box<ProbabilitySpace> {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        (**self).to_turn_math(master_id)
    }
}

// === PROBABILITYEXPRESSION IMPLEMENTATIONS ===

impl ToTurnMath for ProbabilityExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            ProbabilityExpression::EventProbability { .. } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("P(A)".to_string())),
            },
            ProbabilityExpression::ConditionalProbability { .. } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("P(A|B)".to_string())),
            },
            ProbabilityExpression::ExpectedValue { .. } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("E[X]".to_string())),
            },
            ProbabilityExpression::Variance { .. } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("Var(X)".to_string())),
            },
            ProbabilityExpression::Covariance { .. } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("Cov(X,Y)".to_string())),
            },
            _ => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("prob_expr".to_string())),
            },
        }
    }
}

// === PROBABILITYRELATION IMPLEMENTATIONS ===

impl ToTurnMath for ProbabilityRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            ProbabilityRelation::EventsAreIndependent { .. } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(MathNode {
                        id: format!("{}-eventA", master_id),
                        content: Box::new(MathNodeContent::Text("A".to_string())),
                    }),
                    operator: RelationOperatorNode::Equal,
                    rhs: Box::new(MathNode {
                        id: format!("{}-eventB", master_id),
                        content: Box::new(MathNodeContent::Text("B".to_string())),
                    }),
                }),
            },
            ProbabilityRelation::RandomVariablesAreIndependent { .. } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(MathNode {
                        id: format!("{}-varX", master_id),
                        content: Box::new(MathNodeContent::Text("X".to_string())),
                    }),
                    operator: RelationOperatorNode::Equal,
                    rhs: Box::new(MathNode {
                        id: format!("{}-varY", master_id),
                        content: Box::new(MathNodeContent::Text("Y".to_string())),
                    }),
                }),
            },
            ProbabilityRelation::HasDistribution { .. } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(MathNode {
                        id: format!("{}-var", master_id),
                        content: Box::new(MathNodeContent::Text("X".to_string())),
                    }),
                    operator: RelationOperatorNode::Similar,
                    rhs: Box::new(MathNode {
                        id: format!("{}-dist", master_id),
                        content: Box::new(MathNodeContent::Text("F".to_string())),
                    }),
                }),
            },
            _ => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("prob_relation".to_string())),
            },
        }
    }
}

impl ProbabilityRelation {
    pub fn to_structured_probability_relation(&self) -> String {
        match self {
            ProbabilityRelation::EventsAreIndependent { .. } => "A ⊥ B (independence)".to_string(),
            ProbabilityRelation::RandomVariablesAreIndependent { .. } => {
                "X ⊥ Y (independence)".to_string()
            }
            ProbabilityRelation::HasDistribution { .. } => "X ~ F (distribution)".to_string(),
            _ => format!("{:?}", self),
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

    fn export_theory_overview(&self) -> MathematicalContent {
        // Get ALL actual exported content to maximize references
        let all_definitions = self.export_object_definitions(self.generate_object_definitions());
        let all_expressions =
            self.export_expression_definitions(self.generate_expression_definitions());
        let all_relations = self.export_relation_definitions(self.generate_relation_definitions());
        let all_theorems = self.export_theorems();

        // Extract real IDs from ALL exported content
        let definition_links: Vec<String> = all_definitions
            .iter()
            .map(|content| content.id.clone())
            .collect();

        let expression_links: Vec<String> = all_expressions
            .iter()
            .map(|content| content.id.clone())
            .collect();

        let relation_links: Vec<String> = all_relations
            .iter()
            .map(|content| content.id.clone())
            .collect();

        let theorem_links: Vec<String> = all_theorems
            .iter()
            .map(|content| content.id.clone())
            .collect();

        // Organize definitions by category using actual IDs
        let fundamental_links: Vec<String> = definition_links
            .iter()
            .filter(|id| id.contains("generic") || id.contains("discrete"))
            .cloned()
            .collect();

        let continuous_links: Vec<String> = definition_links
            .iter()
            .filter(|id| id.contains("continuous"))
            .cloned()
            .collect();

        let process_links: Vec<String> = definition_links
            .iter()
            .filter(|id| {
                id.contains("stochastic_process")
                    || id.contains("markov_chain")
                    || id.contains("martingale")
                    || id.contains("brownian_motion")
            })
            .cloned()
            .collect();

        // ALL content links for comprehensive navigation
        let all_content_links: Vec<String> = [
            definition_links.clone(),
            expression_links.clone(),
            relation_links.clone(),
            theorem_links.clone(),
        ]
        .concat();

        MathematicalContent {
            id: "probability_theory.overview".to_string(),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
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
                    keywords: vec![
                        "probability".to_string(),
                        "random variables".to_string(),
                        "stochastic processes".to_string(),
                    ],
                },
                structure: DocumentStructure {
                    abstract_content: Some(Section {
                        id: "probability_theory.overview.abstract".to_string(),
                        title: None,
                        content: vec![SectionContentNode::Paragraph(ParagraphNode {
                            segments: vec![RichTextSegment::Text(
                                "Probability theory provides a rigorous mathematical framework for analyzing randomness and uncertainty. This comprehensive overview covers probability spaces, random variables, distributions, and stochastic processes.".to_string()
                            )],
                            alignment: None,
                        })],
                        metadata: vec![],
                        display_options: None,
                    }),
                    table_of_contents: None,
                    body: vec![
                        Section {
                            id: "probability_theory.overview.fundamental".to_string(),
                            title: Some(ParagraphNode {
                                segments: vec![RichTextSegment::Text("Fundamental Concepts".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::Paragraph(ParagraphNode {
                                    segments: vec![RichTextSegment::Text("Basic probability spaces and measures.".to_string())],
                                    alignment: None,
                                }),
                                SectionContentNode::StructuredMath(
                                    StructuredMathNode::Definition {
                                        term_display: vec![RichTextSegment::Text("Fundamental Concepts".to_string())],
                                        formal_term: None,
                                        label: Some("Fundamental probability concepts".to_string()),
                                        body: vec![SectionContentNode::Paragraph(ParagraphNode {
                                            segments: vec![RichTextSegment::Text("Basic probability spaces and measures.".to_string())],
                                            alignment: None,
                                        })],
                                        abstraction_meta: None,
                                        selectable_properties: vec![],
                                    }
                                ),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "probability_theory.overview.continuous".to_string(),
                            title: Some(ParagraphNode {
                                segments: vec![RichTextSegment::Text("Continuous Probability".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::Paragraph(ParagraphNode {
                                    segments: vec![RichTextSegment::Text("Continuous probability spaces and measure theory.".to_string())],
                                    alignment: None,
                                }),
                                SectionContentNode::StructuredMath(
                                    StructuredMathNode::Definition {
                                        term_display: vec![RichTextSegment::Text("Continuous Probability".to_string())],
                                        formal_term: None,
                                        label: Some("Continuous probability theory".to_string()),
                                        body: vec![SectionContentNode::Paragraph(ParagraphNode {
                                            segments: vec![RichTextSegment::Text("Continuous probability spaces and measure theory.".to_string())],
                                            alignment: None,
                                        })],
                                        abstraction_meta: None,
                                        selectable_properties: vec![],
                                    }
                                ),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "probability_theory.overview.processes".to_string(),
                            title: Some(ParagraphNode {
                                segments: vec![RichTextSegment::Text("Stochastic Processes".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::Paragraph(ParagraphNode {
                                    segments: vec![RichTextSegment::Text("Time-dependent random phenomena and processes.".to_string())],
                                    alignment: None,
                                }),
                                SectionContentNode::StructuredMath(
                                    StructuredMathNode::Definition {
                                        term_display: vec![RichTextSegment::Text("Stochastic Processes".to_string())],
                                        formal_term: None,
                                        label: Some("Stochastic processes and Markov chains".to_string()),
                                        body: vec![SectionContentNode::Paragraph(ParagraphNode {
                                            segments: vec![RichTextSegment::Text("Time-dependent random phenomena and processes.".to_string())],
                                            alignment: None,
                                        })],
                                        abstraction_meta: None,
                                        selectable_properties: vec![],
                                    }
                                ),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                    ],
                    footnotes: vec![],
                    glossary: vec![],
                    bibliography: vec![],
                },
                relationships: DocumentRelationships {
                    parent_documents: vec![],
                    child_documents: all_content_links,
                    related_concepts: vec![],
                    dependency_graph: None,
                    cross_references: vec![],
                },
            }),
        }
    }

    fn export_definitions(&self) -> Vec<MathematicalContent> {
        let mut definitions = Vec::new();
        definitions.extend(self.export_object_definitions(self.generate_object_definitions()));
        definitions
            .extend(self.export_expression_definitions(self.generate_expression_definitions()));
        definitions.extend(self.export_relation_definitions(self.generate_relation_definitions()));
        definitions
    }

    fn export_theorems(&self) -> Vec<MathematicalContent> {
        let theorems = all_probability_theorems();
        theorems
            .into_iter()
            .map(|theorem| theorem.to_math_document(&theorem.id))
            .collect()
    }

    fn export_object_definitions(
        &self,
        objects: Vec<ProbabilitySpace>,
    ) -> Vec<MathematicalContent> {
        objects
            .into_iter()
            .enumerate()
            .map(|(i, space)| space.to_math_document(&format!("probability_theory.object_{}", i)))
            .collect()
    }

    fn generate_object_definitions(&self) -> Vec<ProbabilitySpace> {
        vec![
            ProbabilitySpace::Generic(GenericProbabilitySpace::default()),
            // Add other probability spaces as needed
        ]
    }

    fn generate_expression_definitions(&self) -> Vec<ProbabilityExpression> {
        vec![
            ProbabilityExpression::EventProbability {
                event: Parametrizable::Variable(Identifier::Name("A".to_string(), 1)),
                probability_space: Parametrizable::Variable(Identifier::Name("Ω".to_string(), 1)),
            },
            ProbabilityExpression::ExpectedValue {
                variable: Parametrizable::Variable(Identifier::Name("X".to_string(), 1)),
            },
        ]
    }

    fn generate_relation_definitions(&self) -> Vec<ProbabilityRelation> {
        vec![ProbabilityRelation::EventsAreIndependent {
            events: vec![
                Parametrizable::Variable(Identifier::Name("A".to_string(), 1)),
                Parametrizable::Variable(Identifier::Name("B".to_string(), 1)),
            ],
            probability_space: Parametrizable::Variable(Identifier::Name("Ω".to_string(), 1)),
        }]
    }

    fn export_expression_definitions(
        &self,
        expressions: Vec<ProbabilityExpression>,
    ) -> Vec<MathematicalContent> {
        expressions
            .into_iter()
            .enumerate()
            .map(|(i, expr)| {
                let main_section = Section {
                    id: format!("probability_theory.expression_{}.main", i),
                    title: Some(ParagraphNode {
                        segments: vec![RichTextSegment::Text("Expression".to_string())],
                        alignment: None,
                    }),
                    content: vec![SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(format!(
                            "Probability expression: {:?}",
                            expr
                        ))],
                        alignment: None,
                    })],
                    metadata: vec![],
                    display_options: None,
                };

                MathematicalContent {
                    id: format!("probability_theory.expression_{}", i),
                    content_type: MathematicalContentType::ScientificPaper(
                        ScientificPaperContent {
                            title: "Probability Expression".to_string(),
                            paper_type: PaperType::Research,
                            venue: Some("Mathematical Expressions".to_string()),
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
                                venue: Some("Mathematical Expressions".to_string()),
                                doi: None,
                                keywords: vec!["probability".to_string(), "expression".to_string()],
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
                                dependency_graph: None,
                                cross_references: vec![],
                            },
                        },
                    ),
                }
            })
            .collect()
    }

    fn export_relation_definitions(
        &self,
        relations: Vec<ProbabilityRelation>,
    ) -> Vec<MathematicalContent> {
        relations
            .into_iter()
            .enumerate()
            .map(|(i, relation)| {
                let main_section = Section {
                    id: format!("probability_theory.relation_{}.main", i),
                    title: Some(ParagraphNode {
                        segments: vec![RichTextSegment::Text("Relation".to_string())],
                        alignment: None,
                    }),
                    content: vec![SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(format!(
                            "Probability relation: {:?}",
                            relation
                        ))],
                        alignment: None,
                    })],
                    metadata: vec![],
                    display_options: None,
                };

                MathematicalContent {
                    id: format!("probability_theory.relation_{}", i),
                    content_type: MathematicalContentType::ScientificPaper(
                        ScientificPaperContent {
                            title: "Probability Relation".to_string(),
                            paper_type: PaperType::Research,
                            venue: Some("Mathematical Relations".to_string()),
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
                                venue: Some("Mathematical Relations".to_string()),
                                doi: None,
                                keywords: vec!["probability".to_string(), "relation".to_string()],
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
                                dependency_graph: None,
                                cross_references: vec![],
                            },
                        },
                    ),
                }
            })
            .collect()
    }
}
