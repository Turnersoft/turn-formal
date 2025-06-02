use serde::{Deserialize, Serialize};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::GroupProperty;
use crate::subjects::math::theories::groups::definitions::{
    CompactPropertyVariant, ConnectedPropertyVariant, MetrizablePropertyVariant, TopologicalGroup,
    TopologicalGroupProperty,
};
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::section_node::{
    AbstractionMetadata, AcademicMetadata, ContentMetadata, DocumentRelationships,
    DocumentStructure, LinkTarget, MathDocument, MathematicalContentType, PaperType, ParagraphNode,
    RichTextSegment, ScientificPaperContent, Section, SectionContentNode, SelectableProperty,
    StructuredMathContentNode, ToSectionNode,
};

impl ToTurnMath for TopologicalGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Display as (G, τ) to show the group and its topology
        let group_str = format!("{:?}", self.core.base_set);
        let topology_str = format!("{:?}", self.topology.base_set);

        let suffix = if group_str == topology_str {
            String::new()
        } else {
            format!(" on {}", topology_str)
        };

        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text(format!(
                "({}, τ{})",
                group_str, suffix
            ))),
        }
    }
}

impl ToSectionNode for TopologicalGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        // Create title with abstract notation
        let title_segments = vec![
            RichTextSegment::Text("Topological Group ".to_string()),
            RichTextSegment::Math(MathNode {
                id: format!("{}-title-math", id_prefix),
                content: Box::new(MathNodeContent::Text("(G, \\tau)".to_string())),
            }),
        ];

        // Helper function to convert title_segments to a simple string for labels
        let title_text = title_segments.iter().map(|seg| match seg {
            RichTextSegment::Text(t) => t.clone(),
            RichTextSegment::Math(_) => "[Math]".to_string(),
            _ => "".to_string(),
        }).collect::<String>();

        // Create comprehensive content nodes with rich mathematical explanations
        let mut content_nodes = vec![
            // Mathematical introduction
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "A topological group is a mathematical structure that combines the algebraic structure of a group with the geometric structure of a topology in a compatible way.".to_string(),
                )],
                alignment: None,
            }),
            
            // Formal definition
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Formally, a topological group G is a group (G, ·, e, ⁻¹) equipped with a topology such that the group operations are continuous functions.".to_string(),
                )],
                alignment: None,
            }),

            // Reference to underlying group structure with detailed explanation
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![
                    RichTextSegment::Text("Underlying algebraic structure: ".to_string()),
                    RichTextSegment::Link {
                        content: vec![RichTextSegment::Text("Group Theory".to_string())],
                        target: LinkTarget::DefinitionId {
                            term_id: "group_theory.def.generic_group.main_section".to_string(),
                            theory_context: Some("GroupTheory".to_string()),
                        },
                        tooltip: Some("View the underlying group structure with closure, associativity, identity, and inverses".to_string()),
                    },
                    RichTextSegment::Text(" — provides the algebraic framework with binary operation, identity element, and inverses.".to_string()),
                ],
                alignment: None,
            }),

            // Reference to topology structure with detailed explanation
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![
                    RichTextSegment::Text("Underlying geometric structure: ".to_string()),
                    RichTextSegment::Link {
                        content: vec![RichTextSegment::Text("Topological Space".to_string())],
                        target: LinkTarget::DefinitionId {
                            term_id: format!("{}-topology-section", id_prefix),
                            theory_context: Some("Topology".to_string()),
                        },
                        tooltip: Some("View the underlying topological space with open sets, continuity, and convergence".to_string()),
                    },
                    RichTextSegment::Text(" — provides the geometric framework with open sets, neighborhoods, and continuity.".to_string()),
                ],
                alignment: None,
            }),

            // Compatibility conditions - the key requirement
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Compatibility Requirements: The algebra and geometry must be compatible through continuity of group operations:".to_string(),
                )],
                alignment: None,
            }),
            
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "• Multiplication μ: G × G → G, (x,y) ↦ x·y must be continuous".to_string(),
                )],
                alignment: None,
            }),
            
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "• Inversion ι: G → G, x ↦ x⁻¹ must be continuous".to_string(),
                )],
                alignment: None,
            }),

            // Current instance details
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "This specific topological group is constructed on the set {:?}:",
                    self.core.base_set
                ))],
                alignment: None,
            }),
        ];

        // Add properties from the topology if any
        if !self.topology.topology.properties.inner.is_empty() {
            content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Topological Properties of the underlying space:".to_string())],
                alignment: None,
            }));
            
            let topology_props = self
                .topology
                .topology
                .properties
                .inner
                .iter()
                .map(|p| format!("• {:?} — affects convergence, compactness, and continuity properties", p.0))
                .collect::<Vec<_>>();

            for prop_desc in topology_props {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(prop_desc)],
                    alignment: None,
                }));
            }
        }

        // Add detailed group properties explanations
        if !self.core.props.inner.is_empty() {
            content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Algebraic Properties of the underlying group:".to_string())],
                alignment: None,
            }));
            
            for prop in self.core.props.inner.iter() {
                let prop_description = match &prop.0 {
                    GroupProperty::Finite(fv) => {
                        format!("• Finiteness: {:?} — determines cardinality and affects topology (finite groups have discrete topology)", fv)
                    }, 
                    GroupProperty::Abelian(av) => {
                        format!("• Commutativity: {:?} — when xy = yx for all elements, enables harmonic analysis techniques", av)
                    },
                    GroupProperty::Simple(sv) => {
                        format!("• Simplicity: {:?} — absence of proper normal subgroups, important for classification", sv)
                    },
                    GroupProperty::Solvable(sv) => {
                        format!("• Solvability: {:?} — existence of composition series with abelian factors, related to Galois theory", sv)
                    },
                    GroupProperty::Nilpotent(nv) => {
                        format!("• Nilpotency: {:?} — termination of lower central series, ensures nice geometric properties", nv)
                    },
                };
                
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(prop_description)],
                    alignment: None,
                }));
            }
        }

        // Add topological group specific properties with detailed mathematical explanations
        if !self.props.inner.is_empty() {
            content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Topological Group Specific Properties:".to_string())],
                alignment: None,
            }));
            
            for variant_wrapper in self.props.inner.iter() {
                let prop = &variant_wrapper.0;
                let prop_description = match prop {
                    TopologicalGroupProperty::Compact(cv) => {
                        format!("• Compactness: {:?} — every open cover has finite subcover. Enables Haar measure existence and Peter-Weyl theorem applications.", cv)
                    },
                    TopologicalGroupProperty::Connected(cv) => {
                        format!("• Connectedness: {:?} — cannot be written as union of disjoint non-empty open sets. Connected components form important subgroups.", cv)
                    },
                    TopologicalGroupProperty::Metrizable(mv) => {
                        format!("• Metrizability: {:?} — topology induced by metric. Enables distance-based analysis and uniform structures.", mv)
                    },
                };
                
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(prop_description)],
                    alignment: None,
                }));
            }
        }

        // Add abstraction level specific content with rich mathematical explanations
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                content_nodes.extend(vec![
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Level 1 (General Schema): This represents the abstract mathematical concept of topological groups, providing the foundational framework that all specific topological groups must satisfy.".to_string(),
                        )],
                        alignment: None,
                    }),
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Classical Examples: ℝ (real numbers under addition), S¹ (circle group), GL(n,ℝ) (general linear group), SL(n,ℝ) (special linear group), O(n) (orthogonal group), U(n) (unitary group).".to_string(),
                        )],
                        alignment: None,
                    }),
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Applications: Lie group theory, harmonic analysis, representation theory, gauge theory in physics, and symmetric spaces in differential geometry.".to_string(),
                        )],
                        alignment: None,
                    }),
                ]);
            }
            AbstractionLevel::Level2 => {
                content_nodes.extend(vec![
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Level 2 (Specific Type): This represents a particular class of topological groups with defined algebraic and topological properties that constrain the structure significantly.".to_string(),
                        )],
                        alignment: None,
                    }),
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Structure Theory: The interplay between group and topological properties determines existence of Haar measure, Peter-Weyl decomposition, and representation-theoretic properties.".to_string(),
                        )],
                        alignment: None,
                    }),
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Key Results: Compact groups admit finite-dimensional unitary representations, connected groups have universal covers, and locally compact groups possess Haar measure.".to_string(),
                        )],
                        alignment: None,
                    }),
                ]);
            }
            AbstractionLevel::Level3 => {
                content_nodes.extend(vec![
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Level 3 (Constructor): This represents the constructive process of building a topological group by equipping a group with a compatible topology, or conversely, by imposing group structure on a topological space.".to_string(),
                        )],
                        alignment: None,
                    }),
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Construction Verification: Must verify that multiplication μ: G×G → G and inversion ι: G → G are continuous maps with respect to the chosen topology.".to_string(),
                        )],
                        alignment: None,
                    }),
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Topology Constraints: Not every topology on a group makes it a topological group. The continuity requirements impose significant restrictions on admissible topologies.".to_string(),
                        )],
                        alignment: None,
                    }),
                ]);
            }
            AbstractionLevel::Level4 => {
                content_nodes.extend(vec![
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Level 4 (Concrete Instance): This represents a fully specified topological group with explicit elements, operation tables, topology basis, and verified continuity of all operations.".to_string(),
                        )],
                        alignment: None,
                    }),
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Computational Aspects: All group operations can be computed explicitly, topological properties like convergence and compactness can be verified algorithmically.".to_string(),
                        )],
                        alignment: None,
                    }),
                    SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(
                            "Concrete Examples: Z/nZ with discrete topology, ℝⁿ with standard topology and addition, matrix groups with subspace topology from embedding in M(n,ℝ).".to_string(),
                        )],
                        alignment: None,
                    }),
                ]);
            }
        };

        // Add mathematical importance and applications
        content_nodes.extend(vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Mathematical Significance: Topological groups bridge algebra and geometry, providing the foundation for modern harmonic analysis, representation theory, and differential geometry.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Key Theorems: Haar's theorem (existence of invariant measure), Peter-Weyl theorem (decomposition of representations), and the structure theory of locally compact groups.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Physical Applications: Gauge theories, crystal structures, particle physics symmetries, and quantum mechanical systems all rely heavily on topological group theory.".to_string(),
                )],
                alignment: None,
            }),
        ]);

        // Create selectable properties
        let mut selectable_props = vec![];

        // Add properties from the core group if any
        if !self.core.props.inner.is_empty() {
            for prop in self.core.props.inner.iter() {
                selectable_props.push(SelectableProperty {
                    name: format!("Group: {:?}", prop.0),
                    current_variant: format!("{:?}", prop.0),
                    all_variants: vec![format!("{:?}", prop.0)],
                    description: Some("Core group property".to_string()),
                    variant_descriptions: None,
                    property_type_def_id: None,
                });
            }
        }

        // Add topological group properties if any
        for variant_wrapper in self.props.inner.iter() {
            let prop = &variant_wrapper.0;
            match prop {
                TopologicalGroupProperty::Compact(cv) => {
                    selectable_props.push(SelectableProperty {
                        name: "Compactness".to_string(),
                        current_variant: format!("{:?}", cv),
                        all_variants: vec![
                            "Compact".to_string(),
                            "NonCompact".to_string(),
                            "LocallyCompact".to_string(),
                        ],
                        description: Some(
                            "Compactness property of the topological group".to_string(),
                        ),
                        variant_descriptions: None,
                        property_type_def_id: None,
                    });
                }
                TopologicalGroupProperty::Connected(cv) => {
                    selectable_props.push(SelectableProperty {
                        name: "Connectedness".to_string(),
                        current_variant: format!("{:?}", cv),
                        all_variants: vec![
                            "Connected".to_string(),
                            "SimplyConnected".to_string(),
                            "TotallyDisconnected".to_string(),
                            "LocallyConnected".to_string(),
                            "LocallySimplyConnected".to_string(),
                        ],
                        description: Some(
                            "Connectedness property of the topological group".to_string(),
                        ),
                        variant_descriptions: None,
                        property_type_def_id: None,
                    });
                }
                TopologicalGroupProperty::Metrizable(mv) => {
                    selectable_props.push(SelectableProperty {
                        name: "Metrizability".to_string(),
                        current_variant: format!("{:?}", mv),
                        all_variants: vec!["Metrizable".to_string(), "NonMetrizable".to_string()],
                        description: Some(
                            "Metrizability property of the topological group".to_string(),
                        ),
                        variant_descriptions: None,
                        property_type_def_id: None,
                    });
                }
            }
        }

        Section {
            id: format!("{}.main_section", id_prefix),
            title: Some(ParagraphNode {
                segments: title_segments,
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathContentNode::Definition {
                    term_display: vec![RichTextSegment::Text(title_text.clone())],
                    formal_term: Some(self.to_turn_math(format!("{}-formalTerm", id_prefix))),
                    label: Some(format!("Definition ({})", title_text)),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(formalism_obj_level as u8),
                        source_template_id: None,
                        specified_parameters: vec![],
                        universally_quantified_properties: vec![],
                    }),
                    selectable_properties: if selectable_props.is_empty() {
                        vec![]
                    } else {
                        selectable_props
                    },
                },
            )],
            metadata: vec![("type".to_string(), "TopologicalGroupDefinition".to_string())],
            display_options: None,
        }
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));
        let title = main_section.title.as_ref().map_or_else(
            || "Topological Group Document".to_string(),
            |p| {
                p.segments
                    .iter()
                    .map(|s| match s {
                        RichTextSegment::Text(t) => t.clone(),
                        RichTextSegment::StyledText { text, .. } => text.clone(),
                        _ => "".to_string(),
                    })
                    .collect::<String>()
            },
        );

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
                title,
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
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let mut tooltip_text = format!("Topological Group on {:?}", self.core.base_set);

        // Add primary topological properties if any
        if !self.props.inner.is_empty() {
            // Pick the most important property to show in tooltip
            if let Some(prop) = self.props.inner.iter().next() {
                tooltip_text.push_str(&format!(" ({})", format!("{:?}", prop.0)));
            }
        }

        vec![RichTextSegment::Text(tooltip_text)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "(G, τ)";
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name.to_string())],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}.main_section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}.main_section",
                id_prefix
            )),
        }]
    }
}
