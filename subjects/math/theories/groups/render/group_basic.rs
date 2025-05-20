use serde::{Deserialize, Serialize};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::{
    AbelianPropertyVariant, FinitePropertyVariant, GroupBasic, GroupProperty,
};
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::section_node::{
    AbstractionMetadata, LinkTarget, MathDocument, ParagraphNode, RichTextSegment, Section,
    SectionContentNode, SelectableProperty, StructuredMathContentNode, ToSectionNode, p_text,
};

impl ToTurnMath for GroupBasic {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let base_set_str = format!("{:?}", self.base_set);
        let op_char = match &self.operation.notation {
            crate::subjects::math::theories::groups::definitions::GroupNotation::Infix(s) => {
                format!("{:?}", s)
            }
            crate::subjects::math::theories::groups::definitions::GroupNotation::Function(_s) => {
                "op".to_string()
            }
            crate::subjects::math::theories::groups::definitions::GroupNotation::Juxtaposition => {
                "".to_string()
            }
        };
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text(format!(
                "({}, {})",
                base_set_str, op_char
            ))),
        }
    }
}

impl ToSectionNode for GroupBasic {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("Group on set {:?}", self.base_set);

        // Handle different abstraction levels accordingly
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                // L1 should now be handled by render_as_l1_schema, not here
                // Return a warning section that indicates the caller should use render_as_l1_schema
                let content_nodes = vec![SectionContentNode::Paragraph(p_text(
                    "WARNING: For Level 1 schema, please use render_as_l1_schema() instead of to_section_node().",
                ))];

                Section {
                    id: format!("{}-groupbasic-section", id_prefix),
                    title: Some(p_text(&title)),
                    content: content_nodes,
                    sub_sections: vec![],
                    metadata: Some(vec![
                        ("type".to_string(), "GroupBasicDefinition".to_string()),
                        ("abstraction_level".to_string(), "1".to_string()),
                        (
                            "warning".to_string(),
                            "For L1 schema, use render_as_l1_schema() instead".to_string(),
                        ),
                    ]),
                    display_options: None,
                }
            }

            AbstractionLevel::Level2 => {
                // Level 2: More specific with properties but references L1
                let mut content_nodes = vec![
                    SectionContentNode::Paragraph(p_text(&format!(
                        "This is a group with specific properties on the set {:?}.",
                        self.base_set
                    ))),
                    SectionContentNode::Paragraph(p_text(&format!(
                        "Operation: {:?} ({:?})",
                        self.operation.operation_type, self.operation.notation
                    ))),
                    SectionContentNode::Paragraph(p_text(&format!(
                        "Identity: {:?}",
                        self.operation.identity
                    ))),
                    SectionContentNode::Paragraph(p_text(&format!(
                        "Inverse rule: {:?}",
                        self.operation.inverse
                    ))),
                ];

                // Add property descriptions
                if !self.props.inner.is_empty() {
                    content_nodes.push(SectionContentNode::Paragraph(p_text("Properties:")));
                    for prop in self.props.inner.iter() {
                        content_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
                            "- {:?}",
                            prop.0
                        ))));
                    }
                }

                // Reference to L1 with proper link
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![
                        RichTextSegment::Text("For general group theory, see ".to_string()),
                        crate::turn_render::section_node::link_to_definition(
                            "Group Theory",
                            &format!("{}-l1-doc", id_prefix),
                            Some("GroupTheory"),
                        ),
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
                    id: format!("{}-groupbasic-section", id_prefix),
                    title: Some(p_text(&title)),
                    content: vec![SectionContentNode::StructuredMath(
                        StructuredMathContentNode::Definition {
                            term_display: vec![RichTextSegment::Text(title.clone())],
                            formal_term: Some(
                                self.to_turn_math(format!("{}-formalTerm", id_prefix)),
                            ),
                            label: Some(format!("Definition ({})", title)),
                            body: content_nodes,
                            abstraction_meta: Some(AbstractionMetadata {
                                level: Some(formalism_obj_level as u8),
                                source_template_id: Some(format!("{}-l1-doc", id_prefix)),
                                specified_parameters: None,
                                universally_quantified_properties: None,
                            }),
                            selectable_properties: if selectable_props.is_empty() {
                                None
                            } else {
                                Some(selectable_props)
                            },
                        },
                    )],
                    sub_sections: vec![],
                    metadata: Some(vec![
                        ("type".to_string(), "GroupBasicDefinition".to_string()),
                        ("abstraction_level".to_string(), "2".to_string()),
                    ]),
                    display_options: None,
                }
            }

            AbstractionLevel::Level3 => {
                // L3 should be handled by another variant in Group enum, not by GroupBasic
                // Return a simple reference section for L3
                let title = "Group Construction (Level 3)".to_string();
                let content_nodes = vec![
                    SectionContentNode::Paragraph(p_text(
                        "This abstraction level (L3) should be handled by a different Group variant, not GroupBasic.",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "Please refer to other Group constructors such as ProductGroup, QuotientGroup, etc.",
                    )),
                ];

                Section {
                    id: format!("{}-groupbasic-section", id_prefix),
                    title: Some(p_text(&title)),
                    content: content_nodes,
                    sub_sections: vec![],
                    metadata: Some(vec![
                        ("type".to_string(), "GroupBasicDefinition".to_string()),
                        ("abstraction_level".to_string(), "3".to_string()),
                    ]),
                    display_options: None,
                }
            }

            AbstractionLevel::Level4 => {
                // Level 4: Concrete with explicit structure, referencing L2 and L3
                let mut content_nodes = vec![
                    SectionContentNode::Paragraph(p_text(&format!(
                        "Concrete group on set {:?} with explicit structure:",
                        self.base_set
                    ))),
                    SectionContentNode::Paragraph(p_text(&format!(
                        "Operation: {:?} ({:?})",
                        self.operation.operation_type, self.operation.notation
                    ))),
                    SectionContentNode::Paragraph(p_text(&format!(
                        "Identity element: {:?}",
                        self.operation.identity
                    ))),
                    SectionContentNode::Paragraph(p_text(&format!(
                        "Inverse rule: {:?}",
                        self.operation.inverse
                    ))),
                ];

                // Add elements if available and set is small
                let set_debug = format!("{:?}", self.base_set);
                if set_debug.len() < 100 {
                    content_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
                        "Elements: {}",
                        set_debug
                    ))));
                } else {
                    content_nodes.push(SectionContentNode::Paragraph(p_text(
                        "Elements not explicitly enumerated (set too large)",
                    )));
                }

                // Add properties with concrete values
                if !self.props.inner.is_empty() {
                    content_nodes.push(SectionContentNode::Paragraph(p_text(
                        "Properties with concrete values:",
                    )));
                    for prop in self.props.inner.iter() {
                        content_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
                            "- {:?}",
                            prop.0
                        ))));
                    }
                }

                // Add references to L2 and L3 with proper links
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![
                        RichTextSegment::Text("For the abstract specification, see ".to_string()),
                        crate::turn_render::section_node::link_to_definition(
                            "Group Type",
                            &format!("{}-l2-doc", id_prefix),
                            Some("GroupTheory"),
                        ),
                        RichTextSegment::Text(". For construction methods, see ".to_string()),
                        crate::turn_render::section_node::link_to_definition(
                            "Group Constructors",
                            &format!("{}-l3-doc", id_prefix),
                            Some("GroupTheory"),
                        ),
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
                    id: format!("{}-groupbasic-section", id_prefix),
                    title: Some(p_text(&title)),
                    content: vec![SectionContentNode::StructuredMath(
                        StructuredMathContentNode::Definition {
                            term_display: vec![RichTextSegment::Text(title.clone())],
                            formal_term: Some(
                                self.to_turn_math(format!("{}-formalTerm", id_prefix)),
                            ),
                            label: Some(format!("Concrete Group ({})", title)),
                            body: content_nodes,
                            abstraction_meta: Some(AbstractionMetadata {
                                level: Some(formalism_obj_level as u8),
                                source_template_id: Some(format!("{}-l2-doc", id_prefix)),
                                specified_parameters: None,
                                universally_quantified_properties: None,
                            }),
                            selectable_properties: if selectable_props.is_empty() {
                                None
                            } else {
                                Some(selectable_props)
                            },
                        },
                    )],
                    sub_sections: vec![],
                    metadata: Some(vec![
                        ("type".to_string(), "GroupBasicDefinition".to_string()),
                        ("abstraction_level".to_string(), "4".to_string()),
                    ]),
                    display_options: None,
                }
            }
        }
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let formalism_obj_level: AbstractionLevel = self.level();

        // Create different documents based on abstraction level
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                // L1 should now be handled by render_as_l1_schema_document, not here
                // Return a warning document that indicates the caller should use render_as_l1_schema_document
                let warning_section = Section {
                    id: format!("{}-warning-section", id_prefix),
                    title: Some(p_text("Warning")),
                    content: vec![SectionContentNode::Paragraph(p_text(
                        "For Level 1 schema document, please use render_as_l1_schema_document() instead of to_math_document().",
                    ))],
                    sub_sections: vec![],
                    metadata: None,
                    display_options: None,
                };

                MathDocument {
                    id: format!("{}-warning-doc", id_prefix),
                    title: "Warning: Incorrect Method Used".to_string(),
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    authors: None,
                    date_published: None,
                    date_modified: None,
                    abstract_content: Some(vec![SectionContentNode::Paragraph(p_text(
                        "WARNING: For Level 1 schema document, please use render_as_l1_schema_document() instead of to_math_document().",
                    ))]),
                    table_of_contents: None,
                    body: vec![warning_section],
                    footnotes: None,
                    glossary: None,
                    bibliography: None,
                    document_metadata: Some(vec![(
                        "warning".to_string(),
                        "For L1 schema, use render_as_l1_schema_document() instead".to_string(),
                    )]),
                }
            }

            AbstractionLevel::Level2 => {
                // L2: Specific group type document
                let main_section = self.to_section_node(&format!("{}-main", id_prefix));

                MathDocument {
                    id: format!("{}-l2-doc", id_prefix),
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
                                        RichTextSegment::StyledText { text, .. } => text.clone(),
                                        _ => "".to_string(),
                                    })
                                    .collect::<String>()
                            })
                    ),
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    authors: None,
                    date_published: None,
                    date_modified: None,
                    abstract_content: Some(vec![SectionContentNode::Paragraph(p_text(&format!(
                        "A document describing a specific group type on set {:?} with its properties.",
                        self.base_set
                    )))]),
                    table_of_contents: None,
                    body: vec![main_section],
                    footnotes: None,
                    glossary: None,
                    bibliography: None,
                    document_metadata: Some(vec![
                        ("abstraction_level".to_string(), "2".to_string()),
                        ("references_l1".to_string(), format!("{}-l1-doc", id_prefix)),
                    ]),
                }
            }

            AbstractionLevel::Level3 => {
                // L3: Should be handled by a different Group variant, provide reference document
                let dummy_section = Section {
                    id: format!("{}-l3-reference", id_prefix),
                    title: Some(p_text("Group Constructor (Level 3)")),
                    content: vec![
                        SectionContentNode::Paragraph(p_text(
                            "This abstraction level (L3) should be handled by a different Group variant, not GroupBasic.",
                        )),
                        SectionContentNode::Paragraph(p_text(
                            "Please refer to other Group constructors such as ProductGroup, QuotientGroup, etc.",
                        )),
                    ],
                    sub_sections: vec![],
                    metadata: None,
                    display_options: None,
                };

                MathDocument {
                    id: format!("{}-l3-doc", id_prefix),
                    title: "Group Constructor Reference".to_string(),
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    authors: None,
                    date_published: None,
                    date_modified: None,
                    abstract_content: Some(vec![SectionContentNode::Paragraph(p_text(
                        "A reference document for group construction methods.",
                    ))]),
                    table_of_contents: None,
                    body: vec![dummy_section],
                    footnotes: None,
                    glossary: None,
                    bibliography: None,
                    document_metadata: Some(vec![
                        ("abstraction_level".to_string(), "3".to_string()),
                        (
                            "warning".to_string(),
                            "L3 should be implemented by other Group variants".to_string(),
                        ),
                    ]),
                }
            }

            AbstractionLevel::Level4 => {
                // L4: Concrete group instance
                let main_section = self.to_section_node(&format!("{}-main", id_prefix));

                MathDocument {
                    id: format!("{}-l4-doc", id_prefix),
                    title: format!(
                        "Concrete Group: {}",
                        main_section
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
                            })
                    ),
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    authors: None,
                    date_published: None,
                    date_modified: None,
                    abstract_content: Some(vec![SectionContentNode::Paragraph(p_text(&format!(
                        "A document describing a concrete group instance on set {:?} with explicit elements and structure.",
                        self.base_set
                    )))]),
                    table_of_contents: None,
                    body: vec![main_section],
                    footnotes: None,
                    glossary: None,
                    bibliography: None,
                    document_metadata: Some(vec![
                        ("abstraction_level".to_string(), "4".to_string()),
                        ("references_l2".to_string(), format!("{}-l2-doc", id_prefix)),
                        ("references_l3".to_string(), format!("{}-l3-doc", id_prefix)),
                    ]),
                }
            }
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let formalism_obj_level: AbstractionLevel = self.level();

        // Create different tooltip content based on abstraction level
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                vec![RichTextSegment::Text(
                    "Group Theory - Abstract algebraic structure".to_string(),
                )]
            }

            AbstractionLevel::Level2 => {
                let name = format!("Group type on set {:?}", self.base_set);

                let mut tooltip = vec![RichTextSegment::Text(name)];

                // Add properties to tooltip if any exist
                if !self.props.inner.is_empty() {
                    let props_str = self
                        .props
                        .inner
                        .iter()
                        .map(|p| format!("{:?}", p.0))
                        .collect::<Vec<_>>()
                        .join(", ");

                    tooltip.push(RichTextSegment::Text(format!(" ({})", props_str)));
                }

                tooltip
            }

            AbstractionLevel::Level3 => {
                vec![RichTextSegment::Text(
                    "Group Construction Method (L3 should be handled by a different Group variant)"
                        .to_string(),
                )]
            }

            AbstractionLevel::Level4 => {
                let name = format!("Concrete group on set {:?}", self.base_set);

                let mut tooltip = vec![RichTextSegment::Text(name)];

                // Add operation to tooltip
                tooltip.push(RichTextSegment::Text(format!(
                    " with {} operation",
                    match self.operation.operation_type {
                        crate::subjects::math::theories::groups::definitions::GroupOperationVariant::Addition => "addition",
                        crate::subjects::math::theories::groups::definitions::GroupOperationVariant::Multiplication => "multiplication",
                        crate::subjects::math::theories::groups::definitions::GroupOperationVariant::Composition => "composition",
                        _ => "custom",
                    }
                )));

                tooltip
            }
        }
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let formalism_obj_level: AbstractionLevel = self.level();

        // Create different reference nodes based on abstraction level
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                vec![crate::turn_render::section_node::link_to_definition(
                    "Group Theory",
                    &format!("{}-l1-doc", id_prefix),
                    Some("GroupTheory"),
                )]
            }

            AbstractionLevel::Level2 => {
                let name = format!("Group Type on {:?}", self.base_set);

                vec![crate::turn_render::section_node::link_to_definition(
                    &name,
                    &format!("{}-l2-doc", id_prefix),
                    Some("GroupTheory"),
                )]
            }

            AbstractionLevel::Level3 => {
                vec![crate::turn_render::section_node::link_to_definition(
                    "Group Constructor",
                    &format!("{}-l3-doc", id_prefix),
                    Some("GroupTheory"),
                )]
            }

            AbstractionLevel::Level4 => {
                let name = format!("Concrete Group on {:?}", self.base_set);

                vec![crate::turn_render::section_node::link_to_definition(
                    &name,
                    &format!("{}-l4-doc", id_prefix),
                    Some("GroupTheory"),
                )]
            }
        }
    }

    // Add the implementation for render_as_l1_schema
    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "Group Theory".to_string();

        // Create the comprehensive L1 document with multiple subsections
        let content_nodes = vec![SectionContentNode::Paragraph(p_text(
            "A group is an algebraic structure consisting of a set with a binary operation that satisfies four fundamental properties: closure, associativity, identity, and invertibility.",
        ))];

        let sub_sections = vec![
            // Definition Section
            Section {
                id: format!("{}-definition-section", id_prefix),
                title: Some(p_text("Definition")),
                content: vec![
                    SectionContentNode::Paragraph(p_text(
                        "A group (G, *) consists of a set G and a binary operation * such that:",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "1. Closure: For all a, b ∈ G, a * b ∈ G",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "2. Associativity: For all a, b, c ∈ G, (a * b) * c = a * (b * c)",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "3. Identity: There exists an element e ∈ G such that for all a ∈ G, e * a = a * e = a",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "4. Inverse: For each a ∈ G, there exists an element a⁻¹ ∈ G such that a * a⁻¹ = a⁻¹ * a = e",
                    )),
                ],
                sub_sections: vec![],
                metadata: None,
                display_options: None,
            },
            // Properties Section
            Section {
                id: format!("{}-properties-section", id_prefix),
                title: Some(p_text("Properties")),
                content: vec![
                    SectionContentNode::Paragraph(p_text("Groups can have various properties:")),
                    SectionContentNode::Paragraph(p_text(
                        "- Abelian/Commutative: If a * b = b * a for all a, b ∈ G",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "- Finite/Infinite: Whether the group has a finite or infinite number of elements",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "- Simple: If the group has no proper normal subgroups",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "- Solvable: If the group has a subnormal series with abelian factors",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "- Nilpotent: If the lower central series terminates at the identity",
                    )),
                ],
                sub_sections: vec![],
                metadata: None,
                display_options: None,
            },
            // Examples Section
            Section {
                id: format!("{}-examples-section", id_prefix),
                title: Some(p_text("Examples")),
                content: vec![
                    SectionContentNode::Paragraph(p_text("Common examples of groups include:")),
                    SectionContentNode::Paragraph(p_text("- Integers under addition: (Z, +)")),
                    SectionContentNode::Paragraph(p_text(
                        "- Non-zero rational numbers under multiplication: (Q\\{0}, ×)",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "- Symmetric groups: S_n (permutations of n elements)",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "- Cyclic groups: Z_n or C_n (integers modulo n)",
                    )),
                    SectionContentNode::Paragraph(p_text(
                        "- Matrix groups: GL(n, F) (n×n invertible matrices over a field F)",
                    )),
                ],
                sub_sections: vec![],
                metadata: None,
                display_options: None,
            },
        ];

        Section {
            id: format!("{}-groupbasic-section", id_prefix),
            title: Some(p_text(&title)),
            content: content_nodes,
            sub_sections: sub_sections,
            metadata: Some(vec![
                ("type".to_string(), "GroupBasicDefinition".to_string()),
                ("abstraction_level".to_string(), "1".to_string()),
            ]),
            display_options: None,
        }
    }

    fn render_as_l1_schema_document(&self, id_prefix: &str) -> MathDocument {
        // Create the main L1 section
        let main_section = self.render_as_l1_schema(&format!("{}-main", id_prefix));

        MathDocument {
            id: format!("{}-l1-doc", id_prefix),
            title: "Group Theory".to_string(),
            language: Some("en-US".to_string()),
            version: Some("1.0".to_string()),
            authors: None,
            date_published: None,
            date_modified: None,
            abstract_content: Some(vec![SectionContentNode::Paragraph(p_text(
                "A comprehensive document covering the general theory of groups, including definitions, properties, and examples.",
            ))]),
            table_of_contents: None,
            body: vec![main_section],
            footnotes: None,
            glossary: None,
            bibliography: None,
            document_metadata: Some(vec![
                ("abstraction_level".to_string(), "1".to_string()),
                ("math_subject".to_string(), "Abstract Algebra".to_string()),
            ]),
        }
    }
}
