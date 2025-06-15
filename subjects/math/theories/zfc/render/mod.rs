// This AbstractionLevel is for the GetAbstractionLevel trait implementations for ZFC types.
use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::zfc::definitions::{
    CardinalityPropertyVariant, ElementCondition, Set, SetElement, SetExpression, SetMapping,
    SetProperty, SetRelation,
};
use crate::turn_render::*;

impl ToSectionNode for Set {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let object_formalism_level = self.level(); // formalism::AbstractionLevel
        // Decide display_level based on object_formalism_level or other context if needed.
        // For now, let's assume display_level is the same as object's inherent level for simplicity,
        // This assumes a direct mapping or that they are structurally identical and can be cast.

        let mut title_text = format!("Set: {}", id_prefix);
        let mut content_nodes = vec![];
        let mut selectable_props = vec![];

        match self {
            Set::Generic(gs) => {
                for prop in gs.properties.iter() {
                    let (prop_name, current_variant, all_variants) = property_to_selectable(prop);
                    selectable_props.push(SelectableProperty {
                        name: prop_name,
                        current_variant,
                        all_variants,
                        description: Some("A property of this set.".to_string()), // TODO: Specific descriptions
                        variant_descriptions: None,                               // TODO
                        property_type_def_id: None,                               // TODO
                    });
                }
            }
            Set::Empty => {
                title_text = "The Empty Set".to_string();
                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "Contains no elements (∅).".to_string(),
                    )],
                    alignment: None,
                }));
            }
            Set::Singleton {
                element,
                properties,
            } => {
                title_text = format!("Singleton Set {{{:?}}}", element);
                // content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                //     segments: vec![RichTextSegment::Text(format!("Contains exactly one element: {:?}", element))],
                //     alignment: None,
                // }));
                for prop in properties.iter() {
                    let (prop_name, current_variant, all_variants) = property_to_selectable(prop);
                    selectable_props.push(SelectableProperty {
                        name: prop_name,
                        current_variant,
                        all_variants,
                        description: Some("Property of the singleton set.".to_string()),
                        variant_descriptions: None,
                        property_type_def_id: None,
                    });
                }
            }
            Set::Enumeration {
                elements,
                properties,
            } => {
                title_text = "Enumerated Set".to_string();
                let elements_str = elements
                    .iter()
                    .map(|e| format!("{:?}", e))
                    .collect::<Vec<_>>()
                    .join(", ");
                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(format!(
                        "Elements: {{{}}}",
                        elements_str
                    ))],
                    alignment: None,
                }));
                for prop in properties.iter() {
                    let (prop_name, current_variant, all_variants) = property_to_selectable(prop);
                    selectable_props.push(SelectableProperty {
                        name: prop_name,
                        current_variant,
                        all_variants,
                        description: None,
                        variant_descriptions: None,
                        property_type_def_id: None,
                    });
                }
            }
            // TODO: Implement for all other Set variants, detailing their construction based on display_level
            _ => {
                title_text = format!("Set defined by construction: {:?}", self)
                    .chars()
                    .take(50)
                    .collect(); // Fallback title
                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(format!("This set is defined by a ZFC construction (e.g., Union, Intersection, PowerSet). Rendering for display level {:?} needs to be detailed.", object_formalism_level))],
                    alignment: None,
                }));
            }
        }

        // Create the section with all the content
        Section {
            id: format!("{}-set-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title_text.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
                    term_display: RichText {
                        segments: vec![RichTextSegment::Text(title_text.clone())],
                        alignment: None,
                    },
                    formal_term: None,
                    label: Some(format!("Definition ({})", title_text)),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(object_formalism_level as u8),
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
            metadata: vec![("type".to_string(), "ZFCSetDefinition".to_string())],
            display_options: None,
        }
    }

    // fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
    //     let name = match self {
    //         Set::Empty => "Empty Set (∅)".to_string(),
    //         Set::Singleton { element, .. } => format!("Singleton Set {{{:?}}}", element),
    //         _ => format!("Set: {}", id_prefix),
    //     };
    //     vec![RichTextSegment::Text(name)]
    // }

    // fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
    //     let name = match self {
    //         Set::Empty => "∅".to_string(),
    //         Set::Singleton { element, .. } => format!("{{{:?}}}", element),
    //         _ => format!("Set {}", id_prefix),
    //     };
    //     vec![RichTextSegment::Link {
    //         content: vec![RichTextSegment::Text(name)],
    //         target: LinkTarget::DefinitionId {
    //             term_id: format!("{}-section", id_prefix),
    //             theory_context: Some("ZFCSetTheory".to_string()),
    //         },
    //         tooltip: Some(format!("View definition of {}-section", id_prefix)),
    //     }]
    // }
}

impl ToMathDocument for Set {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        // Determine the inherent level of this Set object
        let inherent_formalism_level = self.level(); // formalism::AbstractionLevel

        let main_section = self.to_section_node(&format!("{}-main", id_prefix));
        let title = main_section.title.as_ref().map_or_else(
            || "Set Document".to_string(),
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
            content_type: MathDocumentType::WikiPage(WikiPageContent {
                title,
                theory_domain: "ZFC Set Theory".to_string(),
                completeness_level: CompletenessLevel::Basic,
                maintainer: None,
                content_metadata: ContentMetadata {
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    created_at: None,
                    last_modified: None,
                    content_hash: None,
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
}

// Helper to convert SetProperty to parts for SelectableProperty
fn property_to_selectable(prop: &SetProperty) -> (String, String, Vec<String>) {
    match prop {
        SetProperty::Cardinality(cv) => (
            "Cardinality".to_string(),
            format!("{:?}", cv),
            vec![
                "Finite(usize)".to_string(),
                "CountablyInfinite".to_string(),
                "ContinuumSize".to_string(),
                "LargerCardinal(usize)".to_string(),
            ],
        ),
        SetProperty::IsEmpty(b) => (
            "Is Empty".to_string(),
            b.to_string(),
            vec!["true".to_string(), "false".to_string()],
        ),
        SetProperty::IsFinite(b) => (
            "Is Finite".to_string(),
            b.to_string(),
            vec!["true".to_string(), "false".to_string()],
        ),
        SetProperty::IsCountable(b) => (
            "Is Countable".to_string(),
            b.to_string(),
            vec!["true".to_string(), "false".to_string()],
        ),
        SetProperty::IsWellOrdered(b) => (
            "Is Well-Ordered".to_string(),
            b.to_string(),
            vec!["true".to_string(), "false".to_string()],
        ),
        SetProperty::IsTransitive(b) => (
            "Is Transitive".to_string(),
            b.to_string(),
            vec!["true".to_string(), "false".to_string()],
        ),
        SetProperty::IsOrdinal(b) => (
            "Is Ordinal".to_string(),
            b.to_string(),
            vec!["true".to_string(), "false".to_string()],
        ),
        SetProperty::IsCardinal(b) => (
            "Is Cardinal".to_string(),
            b.to_string(),
            vec!["true".to_string(), "false".to_string()],
        ),
        SetProperty::IsReflexive(b) => (
            "Is Reflexive".to_string(),
            b.to_string(),
            vec!["true".to_string(), "false".to_string()],
        ),
        SetProperty::IsSymmetric(b) => (
            "Is Symmetric".to_string(),
            b.to_string(),
            vec!["true".to_string(), "false".to_string()],
        ),
    }
}

// TODO: Implement GetAbstractionLevel and ToSectionNode for SetElement, SetProperty (as a main definition if needed), etc.
