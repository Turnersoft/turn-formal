use serde::{Deserialize, Serialize};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::{
    AbelianPropertyVariant, FinitePropertyVariant, GenericGroup, GroupProperty,
};

use crate::turn_render::*;

impl ToTurnMath for GenericGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let base_set_str = "∅"; // Use clean notation for empty set
        let op_char = "×"; // Use clean notation for operation
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text(format!(
                "({}, {})",
                base_set_str, op_char
            ))),
        }
    }
}

impl ToSectionNode for GenericGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        // Generate a more specific title based on the id_prefix to avoid duplicates
        let title_segments = if id_prefix.contains("generic") {
            vec![RichTextSegment::Text("Generic Group".to_string())]
        } else if id_prefix.contains("general_linear") {
            vec![
                RichTextSegment::Text("General Linear Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("GL(n, F)".to_string())),
                }),
            ]
        } else if id_prefix.contains("special_linear") {
            vec![
                RichTextSegment::Text("Special Linear Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("SL(n, F)".to_string())),
                }),
            ]
        } else if id_prefix.contains("orthogonal") {
            vec![
                RichTextSegment::Text("Orthogonal Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("O(n)".to_string())),
                }),
            ]
        } else if id_prefix.contains("special_orthogonal") {
            vec![
                RichTextSegment::Text("Special Orthogonal Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("SO(n)".to_string())),
                }),
            ]
        } else if id_prefix.contains("unitary") {
            vec![
                RichTextSegment::Text("Unitary Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("U(n)".to_string())),
                }),
            ]
        } else if id_prefix.contains("special_unitary") {
            vec![
                RichTextSegment::Text("Special Unitary Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("SU(n)".to_string())),
                }),
            ]
        } else if id_prefix.contains("trivial") {
            vec![
                RichTextSegment::Text("Trivial Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("\\{e\\}".to_string())),
                }),
            ]
        } else if id_prefix.contains("modular_additive") {
            vec![
                RichTextSegment::Text("Modular Additive Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text(
                        "\\mathbb{Z}/n\\mathbb{Z}".to_string(),
                    )),
                }),
            ]
        } else if id_prefix.contains("modular_multiplicative") {
            vec![
                RichTextSegment::Text("Modular Multiplicative Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text(
                        "(\\mathbb{Z}/n\\mathbb{Z})^*".to_string(),
                    )),
                }),
            ]
        } else if id_prefix.contains("free") {
            vec![
                RichTextSegment::Text("Free Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("F_n".to_string())),
                }),
            ]
        } else if id_prefix.contains("quotient") {
            vec![
                RichTextSegment::Text("Quotient Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("G/N".to_string())),
                }),
            ]
        } else if id_prefix.contains("kernel") {
            vec![
                RichTextSegment::Text("Kernel Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("\\ker(\\phi)".to_string())),
                }),
            ]
        } else if id_prefix.contains("image") {
            vec![
                RichTextSegment::Text("Image Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("\\text{im}(\\phi)".to_string())),
                }),
            ]
        } else if id_prefix.contains("center") {
            vec![
                RichTextSegment::Text("Center Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("Z(G)".to_string())),
                }),
            ]
        } else if id_prefix.contains("generated_subgroup") {
            vec![
                RichTextSegment::Text("Generated Subgroup ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("\\langle S \\rangle".to_string())),
                }),
            ]
        } else if id_prefix.contains("normalizer") {
            vec![
                RichTextSegment::Text("Normalizer Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("N_G(H)".to_string())),
                }),
            ]
        } else if id_prefix.contains("centralizer") {
            vec![
                RichTextSegment::Text("Centralizer Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("C_G(x)".to_string())),
                }),
            ]
        } else if id_prefix.contains("commutator_subgroup") {
            vec![
                RichTextSegment::Text("Commutator Subgroup ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("[G,G]".to_string())),
                }),
            ]
        } else if id_prefix.contains("sylow_subgroup") {
            vec![
                RichTextSegment::Text("Sylow Subgroup ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("\\text{Syl}_p(G)".to_string())),
                }),
            ]
        } else if id_prefix.contains("wreath_product") {
            vec![
                RichTextSegment::Text("Wreath Product ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("G \\wr H".to_string())),
                }),
            ]
        } else if id_prefix.contains("central_product") {
            vec![
                RichTextSegment::Text("Central Product ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("G \\times_Z H".to_string())),
                }),
            ]
        } else if id_prefix.contains("pullback") {
            vec![RichTextSegment::Text("Pullback Group".to_string())]
        } else if id_prefix.contains("restriction") {
            vec![RichTextSegment::Text("Restriction Group".to_string())]
        } else if id_prefix.contains("product") {
            vec![
                RichTextSegment::Text("Product Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("G \\times H".to_string())),
                }),
            ]
        } else {
            // Fallback to the original hardcoded title
            vec![RichTextSegment::Text("Group".to_string())]
        };

        // Helper function to convert title_segments to a simple string for labels
        let title_text = title_segments
            .iter()
            .map(|seg| match seg {
                RichTextSegment::Text(t) => t.clone(),
                RichTextSegment::Math(_) => "[Math]".to_string(),
                _ => "".to_string(),
            })
            .collect::<String>();

        // Handle different abstraction levels accordingly
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                // L1 should now be handled by render_as_l1_schema, not here
                // Return a warning section that indicates the caller should use render_as_l1_schema
                let content_nodes = vec![SectionContentNode::RichText(RichText {
                    segments: vec![
                        RichTextSegment::Text("WARNING: For Level 1 schema, please use render_as_l1_schema() instead of to_section_node().".to_string()),
                    ],
                    alignment: None,
                })];

                Section {
                    id: format!("{}.main_section", id_prefix),
                    title: Some(RichText {
                        segments: title_segments,
                        alignment: None,
                    }),
                    content: content_nodes,
                    metadata: vec![
                        ("type".to_string(), "GroupBasicDefinition".to_string()),
                        ("abstraction_level".to_string(), "1".to_string()),
                        (
                            "warning".to_string(),
                            "For L1 schema, use render_as_l1_schema() instead".to_string(),
                        ),
                    ],
                    display_options: None,
                }
            }

            AbstractionLevel::Level2 => {
                // Level 2: More specific with properties but references L1
                let mut content_nodes = vec![
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "This is a group with specific properties on the set {:?}.",
                            self.base_set
                        ))],
                        alignment: None,
                    }),
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "Operation: {:?} ({:?})",
                            self.operation.operation_type, self.operation.notation
                        ))],
                        alignment: None,
                    }),
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "Identity: {:?}",
                            self.operation.identity
                        ))],
                        alignment: None,
                    }),
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "Inverse rule: {:?}",
                            self.operation.inverse
                        ))],
                        alignment: None,
                    }),
                ];

                // Add property descriptions
                if !self.props.inner.is_empty() {
                    content_nodes.push(SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text("Properties:".to_string())],
                        alignment: None,
                    }));
                    for prop in self.props.inner.iter() {
                        content_nodes.push(SectionContentNode::RichText(RichText {
                            segments: vec![
                                RichTextSegment::Text("- ".to_string()),
                                RichTextSegment::Text(format!("{:?}", prop.0)),
                            ],
                            alignment: None,
                        }));
                    }
                }

                // Reference to L1 with proper link
                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![
                        RichTextSegment::Text("For general group theory, see ".to_string()),
                        RichTextSegment::Link {
                            content: vec![RichTextSegment::Text("Group Theory".to_string())],
                            target: LinkTarget::DefinitionId {
                                term_id: format!("{}-l1-doc", id_prefix),
                                theory_context: Some("GroupTheory".to_string()),
                            },
                            tooltip: Some(format!("View definition of {}-l1-doc", id_prefix)),
                        },
                    ],
                    alignment: None,
                }));

                // Create selectable properties for L2
                let mut selectable_props = vec![];
                for variant_wrapper in self.props.inner.iter() {
                    let prop = &variant_wrapper.0;
                    match prop {
                        GroupProperty::Finite(fv) => selectable_props.push(SelectableProperty {
                            name: "Order".to_string(),
                            current_variant: format!("{:?}", fv),
                            all_variants: vec![
                                "Finite(usize)".to_string(),
                                "Infinite".to_string(),
                                "LocallyFinite".to_string(),
                            ],
                            description: Some("Order of the group.".to_string()),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                        GroupProperty::Abelian(av) => selectable_props.push(SelectableProperty {
                            name: "Commutativity".to_string(),
                            current_variant: format!("{:?}", av),
                            all_variants: vec!["Abelian".to_string(), "NonAbelian".to_string()],
                            description: Some("Is commutative?".to_string()),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                        GroupProperty::Simple(sv) => selectable_props.push(SelectableProperty {
                            name: "Simplicity".to_string(),
                            current_variant: format!("{:?}", sv),
                            all_variants: vec![
                                "Simple".to_string(),
                                "NonSimple".to_string(),
                                "QuasiSimple".to_string(),
                            ],
                            description: Some(
                                "Whether the group has normal subgroups.".to_string(),
                            ),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                        GroupProperty::Solvable(sv) => selectable_props.push(SelectableProperty {
                            name: "Solvability".to_string(),
                            current_variant: format!("{:?}", sv),
                            all_variants: vec![
                                "Solvable".to_string(),
                                "NonSolvable".to_string(),
                                "Polysolvable".to_string(),
                            ],
                            description: Some(
                                "Whether the group has a solvable series.".to_string(),
                            ),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                        GroupProperty::Nilpotent(nv) => selectable_props.push(SelectableProperty {
                            name: "Nilpotency".to_string(),
                            current_variant: format!("{:?}", nv),
                            all_variants: vec![
                                "Nilpotent(n)".to_string(),
                                "NonNilpotent".to_string(),
                            ],
                            description: Some(
                                "Whether the group has a nilpotent series.".to_string(),
                            ),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                    }
                }

                Section {
                    id: format!("{}.main_section", id_prefix),
                    title: Some(RichText {
                        segments: title_segments.clone(),
                        alignment: None,
                    }),
                    content: vec![SectionContentNode::StructuredMath(
                        StructuredMathNode::Definition {
                            term_display: RichText {
                                segments: title_segments.clone(),
                                alignment: None,
                            },
                            formal_term: Some(
                                self.to_turn_math(format!("{}-formalTerm", id_prefix)),
                            ),
                            label: Some(format!("Definition ({})", title_text)),
                            body: content_nodes,
                            abstraction_meta: Some(AbstractionMetadata {
                                level: Some(formalism_obj_level as u8),
                                source_template_id: Some(format!("{}-l1-doc", id_prefix)),
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
                    metadata: vec![
                        ("type".to_string(), "GroupBasicDefinition".to_string()),
                        ("abstraction_level".to_string(), "2".to_string()),
                    ],
                    display_options: None,
                }
            }

            AbstractionLevel::Level3 => {
                // L3 should be handled by another variant in Group enum, not by GroupBasic
                // Return a simple reference section for L3
                let title = "Group Construction (Level 3)".to_string();
                let content_nodes = vec![
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(
                            "This abstraction level (L3) should be handled by a different Group variant, not GroupBasic.".to_string(),
                        )],
                        alignment: None,
                    }),
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(
                            "Please refer to other Group constructors such as ProductGroup, QuotientGroup, etc.".to_string(),
                        )],
                        alignment: None,
                    }),
                ];

                Section {
                    id: format!("{}.main_section", id_prefix),
                    title: Some(RichText {
                        segments: vec![RichTextSegment::Text(title)],
                        alignment: None,
                    }),
                    content: content_nodes,
                    metadata: vec![
                        ("type".to_string(), "GroupBasicDefinition".to_string()),
                        ("abstraction_level".to_string(), "3".to_string()),
                    ],
                    display_options: None,
                }
            }

            AbstractionLevel::Level4 => {
                // Level 4: Concrete with explicit structure, referencing L2 and L3
                let mut content_nodes = vec![
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "Concrete group on set {:?} with explicit structure:",
                            self.base_set
                        ))],
                        alignment: None,
                    }),
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "Operation: {:?} ({:?})",
                            self.operation.operation_type, self.operation.notation
                        ))],
                        alignment: None,
                    }),
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "Identity element: {:?}",
                            self.operation.identity
                        ))],
                        alignment: None,
                    }),
                    SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(format!(
                            "Inverse rule: {:?}",
                            self.operation.inverse
                        ))],
                        alignment: None,
                    }),
                ];

                // Add elements if available and set is small
                let set_debug = format!("{:?}", self.base_set);
                if set_debug.len() < 100 {
                    content_nodes.push(SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(format!("Elements: {}", set_debug))],
                        alignment: None,
                    }));
                } else {
                    content_nodes.push(SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(
                            "Elements not explicitly enumerated (set too large)".to_string(),
                        )],
                        alignment: None,
                    }));
                }

                // Add properties with concrete values
                if !self.props.inner.is_empty() {
                    content_nodes.push(SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::Text(
                            "Properties with concrete values:".to_string(),
                        )],
                        alignment: None,
                    }));
                    for prop in self.props.inner.iter() {
                        content_nodes.push(SectionContentNode::RichText(RichText {
                            segments: vec![
                                RichTextSegment::Text("- ".to_string()),
                                RichTextSegment::Text(format!("{:?}", prop.0)),
                            ],
                            alignment: None,
                        }));
                    }
                }

                // Add references to L2 and L3 with proper links
                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![
                        RichTextSegment::Text("For the abstract specification, see ".to_string()),
                        RichTextSegment::Link {
                            content: vec![RichTextSegment::Text("Group Type".to_string())],
                            target: LinkTarget::DefinitionId {
                                term_id: format!("{}-l2-doc", id_prefix),
                                theory_context: Some("GroupTheory".to_string()),
                            },
                            tooltip: Some(format!("View definition of {}-l2-doc", id_prefix)),
                        },
                        RichTextSegment::Text(". For construction methods, see ".to_string()),
                        RichTextSegment::Link {
                            content: vec![RichTextSegment::Text("Group Constructors".to_string())],
                            target: LinkTarget::DefinitionId {
                                term_id: format!("{}-l3-doc", id_prefix),
                                theory_context: Some("GroupTheory".to_string()),
                            },
                            tooltip: Some(format!("View definition of {}-l3-doc", id_prefix)),
                        },
                        RichTextSegment::Text(".".to_string()),
                    ],
                    alignment: None,
                }));

                // Create selectable properties for L4 - same as L2 but could include more concrete details
                let mut selectable_props = vec![];
                for variant_wrapper in self.props.inner.iter() {
                    let prop = &variant_wrapper.0;
                    match prop {
                        GroupProperty::Finite(fv) => selectable_props.push(SelectableProperty {
                            name: "Order".to_string(),
                            current_variant: format!("{:?}", fv),
                            all_variants: vec![
                                "Finite(usize)".to_string(),
                                "Infinite".to_string(),
                                "LocallyFinite".to_string(),
                            ],
                            description: Some("Order of the group.".to_string()),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                        GroupProperty::Abelian(av) => selectable_props.push(SelectableProperty {
                            name: "Commutativity".to_string(),
                            current_variant: format!("{:?}", av),
                            all_variants: vec!["Abelian".to_string(), "NonAbelian".to_string()],
                            description: Some("Is commutative?".to_string()),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                        GroupProperty::Simple(sv) => selectable_props.push(SelectableProperty {
                            name: "Simplicity".to_string(),
                            current_variant: format!("{:?}", sv),
                            all_variants: vec![
                                "Simple".to_string(),
                                "NonSimple".to_string(),
                                "QuasiSimple".to_string(),
                            ],
                            description: Some(
                                "Whether the group has normal subgroups.".to_string(),
                            ),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                        GroupProperty::Solvable(sv) => selectable_props.push(SelectableProperty {
                            name: "Solvability".to_string(),
                            current_variant: format!("{:?}", sv),
                            all_variants: vec![
                                "Solvable".to_string(),
                                "NonSolvable".to_string(),
                                "Polysolvable".to_string(),
                            ],
                            description: Some(
                                "Whether the group has a solvable series.".to_string(),
                            ),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                        GroupProperty::Nilpotent(nv) => selectable_props.push(SelectableProperty {
                            name: "Nilpotency".to_string(),
                            current_variant: format!("{:?}", nv),
                            all_variants: vec![
                                "Nilpotent(n)".to_string(),
                                "NonNilpotent".to_string(),
                            ],
                            description: Some(
                                "Whether the group has a nilpotent series.".to_string(),
                            ),
                            variant_descriptions: None,
                            property_type_def_id: None,
                        }),
                    }
                }

                Section {
                    id: format!("{}.main_section", id_prefix),
                    title: Some(RichText {
                        segments: title_segments.clone(),
                        alignment: None,
                    }),
                    content: vec![SectionContentNode::StructuredMath(
                        StructuredMathNode::Definition {
                            term_display: RichText {
                                segments: title_segments.clone(),
                                alignment: None,
                            },
                            formal_term: Some(
                                self.to_turn_math(format!("{}-formalTerm", id_prefix)),
                            ),
                            label: Some(format!("Concrete Group ({})", title_text)),
                            body: content_nodes,
                            abstraction_meta: Some(AbstractionMetadata {
                                level: Some(formalism_obj_level as u8),
                                source_template_id: Some(format!("{}-l2-doc", id_prefix)),
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
                    metadata: vec![
                        ("type".to_string(), "GroupBasicDefinition".to_string()),
                        ("abstraction_level".to_string(), "4".to_string()),
                    ],
                    display_options: None,
                }
            }
        }
    }

    // fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
    //     let formalism_obj_level: AbstractionLevel = self.level();

    //     // Create different tooltip content based on abstraction level
    //     match formalism_obj_level {
    //         AbstractionLevel::Level1 => {
    //             vec![RichTextSegment::Text(
    //                 "Group Theory - Abstract algebraic structure".to_string(),
    //             )]
    //         }

    //         AbstractionLevel::Level2 => {
    //             let name = format!("Group type on set {:?}", self.base_set);

    //             let mut tooltip = vec![RichTextSegment::Text(name)];

    //             // Add properties to tooltip if any exist
    //             if !self.props.inner.is_empty() {
    //                 let props_str = self
    //                     .props
    //                     .inner
    //                     .iter()
    //                     .map(|p| format!("{:?}", p.0))
    //                     .collect::<Vec<_>>()
    //                     .join(", ");

    //                 tooltip.push(RichTextSegment::Text(format!(" ({})", props_str)));
    //             }

    //             tooltip
    //         }

    //         AbstractionLevel::Level3 => {
    //             vec![RichTextSegment::Text(
    //                 "Group Construction Method (L3 should be handled by a different Group variant)"
    //                     .to_string(),
    //             )]
    //         }

    //         AbstractionLevel::Level4 => {
    //             let name = format!("Concrete group on set {:?}", self.base_set);

    //             let mut tooltip = vec![RichTextSegment::Text(name)];

    //             // Add operation to tooltip
    //             tooltip.push(RichTextSegment::Text(format!(
    //                 " with {} operation",
    //                 match self.operation.operation_type {
    //                     crate::subjects::math::theories::groups::definitions::GroupOperationVariant::Addition => "addition",
    //                     crate::subjects::math::theories::groups::definitions::GroupOperationVariant::Multiplication => "multiplication",
    //                     crate::subjects::math::theories::groups::definitions::GroupOperationVariant::Composition => "composition",
    //                     _ => "custom",
    //                 }
    //             )));

    //             tooltip
    //         }
    //     }
    // }

    // fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
    //     let formalism_obj_level: AbstractionLevel = self.level();

    //     match formalism_obj_level {
    //         AbstractionLevel::Level1 => {
    //             vec![RichTextSegment::Link {
    //                 content: vec![RichTextSegment::Text("Group Theory".to_string())],
    //                 target: LinkTarget::DefinitionId {
    //                     term_id: format!("{}-l1-doc", id_prefix),
    //                     theory_context: Some("GroupTheory".to_string()),
    //                 },
    //                 tooltip: Some(format!("View definition of {}-l1-doc", id_prefix)),
    //             }]
    //         }

    //         AbstractionLevel::Level2 => {
    //             let name = format!("Group Type on {:?}", self.base_set);

    //             vec![RichTextSegment::Link {
    //                 content: vec![RichTextSegment::Text(name)],
    //                 target: LinkTarget::DefinitionId {
    //                     term_id: format!("{}-l2-doc", id_prefix),
    //                     theory_context: Some("GroupTheory".to_string()),
    //                 },
    //                 tooltip: Some(format!("View definition of {}-l2-doc", id_prefix)),
    //             }]
    //         }

    //         AbstractionLevel::Level3 => {
    //             vec![RichTextSegment::Link {
    //                 content: vec![RichTextSegment::Text("Group Constructor".to_string())],
    //                 target: LinkTarget::DefinitionId {
    //                     term_id: format!("{}-l3-doc", id_prefix),
    //                     theory_context: Some("GroupTheory".to_string()),
    //                 },
    //                 tooltip: Some(format!("View definition of {}-l3-doc", id_prefix)),
    //             }]
    //         }

    //         AbstractionLevel::Level4 => {
    //             let name = format!("Concrete Group on {:?}", self.base_set);

    //             vec![RichTextSegment::Link {
    //                 content: vec![RichTextSegment::Text(name)],
    //                 target: LinkTarget::DefinitionId {
    //                     term_id: format!("{}-l4-doc", id_prefix),
    //                     theory_context: Some("GroupTheory".to_string()),
    //                 },
    //                 tooltip: Some(format!("View definition of {}-l4-doc", id_prefix)),
    //             }]
    //         }
    //     }
    // }

    // Fix the render_as_l1_schema method to use new Section structure
    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "Group Theory".to_string();

        // Create the comprehensive L1 document
        let mut content_nodes = vec![SectionContentNode::RichText(RichText {
            segments: vec![RichTextSegment::Text(
                "A group is an algebraic structure consisting of a set with a binary operation that satisfies four fundamental properties: closure, associativity, identity, and invertibility.".to_string(),
            )],
            alignment: None,
        })];

        // Create subsections as SectionContentNode::SubSection
        let definition_section = Section {
            id: format!("{}.definition_section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Definition".to_string())],
                alignment: None,
            }),
            content: vec![
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "A group (G, *) consists of a set G and a binary operation * such that:"
                            .to_string(),
                    )],
                    alignment: None,
                }),
                // Closure axiom using structured format
                SectionContentNode::MathNode {
                    math: self.create_closure_axiom(&format!("{}-closure", id_prefix)),
                    label: Some("Closure".to_string()),
                    caption: None,
                },
                // Associativity axiom using structured format
                SectionContentNode::MathNode {
                    math: self.create_associativity_axiom(&format!("{}-assoc", id_prefix)),
                    label: Some("Associativity".to_string()),
                    caption: None,
                },
                // Identity axiom using structured format
                SectionContentNode::MathNode {
                    math: self.create_identity_axiom(&format!("{}-identity", id_prefix)),
                    label: Some("Identity Element".to_string()),
                    caption: None,
                },
                // Inverse axiom using structured format
                SectionContentNode::MathNode {
                    math: self.create_inverse_axiom(&format!("{}-inverse", id_prefix)),
                    label: Some("Inverse Elements".to_string()),
                    caption: None,
                },
            ],
            metadata: vec![],
            display_options: None,
        };

        let properties_section = Section {
            id: format!("{}.properties_section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Properties".to_string())],
                alignment: None,
            }),
            content: vec![
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text("Groups can have various properties:".to_string())],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Abelian/Commutative: If a * b = b * a for all a, b ∈ G".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Finite/Infinite: Whether the group has a finite or infinite number of elements".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Simple: If the group has no proper normal subgroups".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Solvable: If the group has a subnormal series with abelian factors".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Nilpotent: If the lower central series terminates at the identity".to_string(),
                    )],
                    alignment: None,
                }),
            ],
            metadata: vec![],
            display_options: None,
        };

        let examples_section = Section {
            id: format!("{}.examples_section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Examples".to_string())],
                alignment: None,
            }),
            content: vec![
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "Common examples of groups include:".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Integers under addition: (Z, +)".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Non-zero rational numbers under multiplication: (Q\\{0}, ×)".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Symmetric groups: S_n (permutations of n elements)".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Cyclic groups: Z_n or C_n (integers modulo n)".to_string(),
                    )],
                    alignment: None,
                }),
                SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "- Matrix groups: GL(n, F) (n×n invertible matrices over a field F)"
                            .to_string(),
                    )],
                    alignment: None,
                }),
            ],
            metadata: vec![],
            display_options: None,
        };

        // Add subsections
        content_nodes.push(SectionContentNode::SubSection(Box::new(definition_section)));
        content_nodes.push(SectionContentNode::SubSection(Box::new(properties_section)));
        content_nodes.push(SectionContentNode::SubSection(Box::new(examples_section)));

        Section {
            id: format!("{}.main_section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title)],
                alignment: None,
            }),
            content: content_nodes,
            metadata: vec![
                ("type".to_string(), "GroupBasicDefinition".to_string()),
                ("abstraction_level".to_string(), "1".to_string()),
            ],
            display_options: None,
        }
    }

    fn render_as_l1_schema_document(&self, id_prefix: &str) -> MathDocument {
        // Create the main L1 section
        let main_section = self.render_as_l1_schema(&format!("{}-main", id_prefix));

        MathDocument {
            id: format!("{}-l1-doc", id_prefix),
            content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
            title: "Basic Group".to_string(),
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
            abstract_content: Some(Section {
                id: format!("{}-l1-abstract", id_prefix),
                title: None,
                        content: vec![SectionContentNode::RichText(RichText {
                            segments: vec![RichTextSegment::Text(
                                "A comprehensive document covering the general theory of groups, including definitions, properties, and examples.".to_string(),
                            )],
                            alignment: None,
                        })],
                metadata: vec![],
                display_options: None,
            }),
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
}

impl ToMathDocument for GenericGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let formalism_obj_level: AbstractionLevel = self.level();

        // Create different documents based on abstraction level
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                // Use L1 schema rendering for L1 groups
                self.render_as_l1_schema_document(id_prefix)
            }

            AbstractionLevel::Level2 => {
                // L2: Specific group type document
                let main_section = self.to_section_node(&format!("{}-main", id_prefix));

                MathDocument {
                    id: format!("{}-l2-doc", id_prefix),
                    content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                        title: format!(
                            "Group Type: {}",
                            main_section
                                .title
                                .as_ref()
                                .map_or("Group".to_string(), |p| {
                                    p.segments
                                        .iter()
                                        .map(|s| match s {
                                            RichTextSegment::Text(t) => t.clone(),
                                            RichTextSegment::StyledText { text, .. } => {
                                                text.clone()
                                            }
                                            _ => "".to_string(),
                                        })
                                        .collect::<String>()
                                })
                        ),
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
                            abstract_content: Some(Section {
                                id: format!("{}-abstract", id_prefix),
                                title: None,
                                content: vec![SectionContentNode::RichText(RichText {
                                    segments: vec![RichTextSegment::Text(format!(
                                        "A document describing a specific group type on set {:?} with its properties.",
                                        self.base_set
                                    ))],
                                    alignment: None,
                                })],
                                metadata: vec![],
                                display_options: None,
                            }),
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

            AbstractionLevel::Level3 => {
                // L3: Should be handled by a different Group variant, provide reference document
                let dummy_section = Section {
                    id: format!("{}-l3-reference", id_prefix),
                    title: Some(RichText {
                        segments: vec![RichTextSegment::Text("Group Constructor (Level 3)".to_string())],
                        alignment: None,
                    }),
                    content: vec![
                        SectionContentNode::RichText(RichText {
                            segments: vec![RichTextSegment::Text(
                                "This abstraction level (L3) should be handled by a different Group variant, not GroupBasic.".to_string(),
                            )],
                            alignment: None,
                        }),
                        SectionContentNode::RichText(RichText {
                            segments: vec![RichTextSegment::Text(
                                "Please refer to other Group constructors such as ProductGroup, QuotientGroup, etc.".to_string(),
                            )],
                            alignment: None,
                        }),
                    ],
                    metadata: vec![],
                    display_options: None,
                };

                MathDocument {
                    id: format!("{}-l3-doc", id_prefix),
                    content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                        title: "Group Constructor Reference".to_string(),
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
                            abstract_content: Some(Section {
                                id: format!("{}-l3-abstract", id_prefix),
                                title: None,
                                content: vec![SectionContentNode::RichText(RichText {
                                    segments: vec![RichTextSegment::Text(
                                        "A reference document for group construction methods."
                                            .to_string(),
                                    )],
                                    alignment: None,
                                })],
                                metadata: vec![],
                                display_options: None,
                            }),
                            table_of_contents: None,
                            body: vec![dummy_section],
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

            AbstractionLevel::Level4 => {
                // L4: Concrete group instance
                let main_section = self.to_section_node(&format!("{}-main", id_prefix));

                MathDocument {
                    id: format!("{}-l4-doc", id_prefix),
                    content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                        title: main_section
                            .title
                            .as_ref()
                            .map_or("Group".to_string(), |p| {
                                p.segments
                                    .iter()
                                    .map(|s| match s {
                                        RichTextSegment::Text(t) => t.clone(),
                                        RichTextSegment::StyledText { text, .. } => text.clone(),
                                        _ => "".to_string(),
                                    })
                                    .collect::<String>()
                            }),
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
                            abstract_content: Some(Section {
                                id: format!("{}-l4-abstract", id_prefix),
                                title: None,
                                content: vec![SectionContentNode::RichText(RichText {
                                    segments: vec![RichTextSegment::Text(format!(
                                        "A document describing a concrete group instance on set {:?} with explicit elements and structure.",
                                        self.base_set
                                    ))],
                                    alignment: None,
                                })],
                                metadata: vec![],
                                display_options: None,
                            }),
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
        }
    }
}

// Add helper methods for creating axiom MathNodes
impl GenericGroup {
    /// Create the closure axiom: ∀ a, b ∈ G : a * b ∈ G
    fn create_closure_axiom(&self, node_id: &str) -> MathNode {
        use crate::turn_render::math_node::{
            MathNodeContent, QuantificationNode, RefinedMulOrDivOperation, RelationOperatorNode,
        };

        // Create variables a and b
        let a_el = MathNode {
            id: format!("{}_el_a", node_id),
            content: Box::new(MathNodeContent::Text("a".to_string())),
        };

        let b_el = MathNode {
            id: format!("{}_el_b", node_id),
            content: Box::new(MathNodeContent::Text("b".to_string())),
        };

        let g_set = MathNode {
            id: format!("{}_set_g", node_id),
            content: Box::new(MathNodeContent::Text("G".to_string())),
        };

        // Create a * b using CustomFunction
        let ab = MathNode {
            id: format!("{}_ab", node_id),
            content: Box::new(MathNodeContent::FunctionCall {
                name: Box::new(MathNode {
                    id: format!("{}_func", node_id),
                    content: Box::new(MathNodeContent::Identifier(Identifier {
                        body: "∘".to_string(),
                        pre_script: None,
                        mid_script: None,
                        post_script: None,
                        primes: 0,
                        is_function: false,
                    })),
                }),
                parameters: vec![a_el.clone(), b_el.clone()],
            }),
        };

        // Create a * b ∈ G using Relationship
        let result_membership = MathNode {
            id: format!("{}_result", node_id),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(ab),
                rhs: Box::new(g_set.clone()),
                operator: RelationOperatorNode::ElementOf,
            }),
        };

        // Create a, b ∈ G using Relationship with comma-separated variables
        let var_list = MathNode {
            id: format!("{}_var_list", node_id),
            content: Box::new(MathNodeContent::Multiplications {
                terms: vec![
                    (RefinedMulOrDivOperation::None, a_el),
                    (
                        RefinedMulOrDivOperation::None,
                        MathNode {
                            id: format!("{}_comma", node_id),
                            content: Box::new(MathNodeContent::Identifier(Identifier {
                                body: ",".to_string(),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            })),
                        },
                    ),
                    (RefinedMulOrDivOperation::None, b_el),
                ],
            }),
        };

        let membership = MathNode {
            id: format!("{}_membership", node_id),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(var_list),
                rhs: Box::new(g_set),
                operator: RelationOperatorNode::ElementOf,
            }),
        };

        // Create the full quantified expression
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::QuantifiedExpression {
                quantifier: QuantificationNode::Universal,
                variables: vec![],
                domain: Some(Box::new(membership)),
                predicate: Some(Box::new(result_membership)),
            }),
        }
    }

    /// Create the associativity axiom: ∀ a, b, c ∈ G : (a * b) * c = a * (b * c)
    fn create_associativity_axiom(&self, node_id: &str) -> MathNode {
        use crate::turn_render::math_node::{
            MathNodeContent, QuantificationNode, RelationOperatorNode,
        };

        // Create variables
        let a_el = MathNode {
            id: format!("{}_el_a", node_id),
            content: Box::new(MathNodeContent::Text("a".to_string())),
        };

        let b_el = MathNode {
            id: format!("{}_el_b", node_id),
            content: Box::new(MathNodeContent::Text("b".to_string())),
        };

        let c_el = MathNode {
            id: format!("{}_el_c", node_id),
            content: Box::new(MathNodeContent::Text("c".to_string())),
        };

        let g_set = MathNode {
            id: format!("{}_set_g", node_id),
            content: Box::new(MathNodeContent::Text("G".to_string())),
        };

        // Create a * b using CustomFunction
        let ab = MathNode {
            id: format!("{}_ab", node_id),
            content: Box::new(MathNodeContent::FunctionCall {
                name: Box::new(MathNode {
                    id: format!("{}_func", node_id),
                    content: Box::new(MathNodeContent::Identifier(Identifier {
                        body: "∘".to_string(),
                        pre_script: None,
                        mid_script: None,
                        post_script: None,
                        primes: 0,
                        is_function: false,
                    })),
                }),
                parameters: vec![a_el.clone(), b_el.clone()],
            }),
        };

        // Create (a * b) * c - simplified as text
        let left_assoc = MathNode {
            id: format!("{}_left", node_id),
            content: Box::new(MathNodeContent::Text("(a ∘ b) ∘ c".to_string())),
        };

        // Create b * c - simplified as text
        let bc = MathNode {
            id: format!("{}_bc", node_id),
            content: Box::new(MathNodeContent::Text("b ∘ c".to_string())),
        };

        // Create a * (b * c) - simplified as text
        let right_assoc = MathNode {
            id: format!("{}_right", node_id),
            content: Box::new(MathNodeContent::Text("a ∘ (b ∘ c)".to_string())),
        };

        // Create equation
        let equation = MathNode {
            id: format!("{}_eq", node_id),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(left_assoc),
                rhs: Box::new(right_assoc),
                operator: RelationOperatorNode::Equal,
            }),
        };

        // Create a, b, c ∈ G
        let var_list = MathNode {
            id: format!("{}_var_list", node_id),
            content: Box::new(MathNodeContent::Multiplications {
                terms: vec![
                    (RefinedMulOrDivOperation::None, a_el),
                    (
                        RefinedMulOrDivOperation::None,
                        MathNode {
                            id: format!("{}_comma1", node_id),
                            content: Box::new(MathNodeContent::Text(",".to_string())),
                        },
                    ),
                    (RefinedMulOrDivOperation::None, b_el),
                    (
                        RefinedMulOrDivOperation::None,
                        MathNode {
                            id: format!("{}_comma2", node_id),
                            content: Box::new(MathNodeContent::Text(",".to_string())),
                        },
                    ),
                    (RefinedMulOrDivOperation::None, c_el),
                ],
            }),
        };

        let membership = MathNode {
            id: format!("{}_membership", node_id),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(var_list),
                rhs: Box::new(g_set),
                operator: RelationOperatorNode::ElementOf,
            }),
        };

        // Create the full quantified expression
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::QuantifiedExpression {
                quantifier: QuantificationNode::Universal,
                variables: vec![],
                domain: Some(Box::new(membership)),
                predicate: Some(Box::new(equation)),
            }),
        }
    }

    /// Create the identity axiom: ∃ e ∈ G : ∀ a ∈ G : e * a = a * e = a
    fn create_identity_axiom(&self, node_id: &str) -> MathNode {
        // Simplified implementation to avoid compilation errors
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Text(
                "∃ e ∈ G : ∀ a ∈ G : e ∘ a = a ∘ e = a".to_string(),
            )),
        }
    }

    /// Create the inverse axiom: ∀ a ∈ G : ∃ a⁻¹ ∈ G : a * a⁻¹ = a⁻¹ * a = e
    fn create_inverse_axiom(&self, node_id: &str) -> MathNode {
        // Simplified implementation to avoid compilation errors
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Text(
                "∀ a ∈ G : ∃ a⁻¹ ∈ G : a ∘ a⁻¹ = a⁻¹ ∘ a = e".to_string(),
            )),
        }
    }
}
