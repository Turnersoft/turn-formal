use crate::turn_render::math_node::{MathNode, MathNodeContent};
use crate::turn_render::section_node::{
    AbstractionMetadata, LinkTarget, MathDocument, ParagraphNode, RichTextSegment, Section,
    SectionContentNode, SelectableProperty, StructuredMathContentNode, ToSectionNode,
    link_to_definition, p_text,
};

// This AbstractionLevel is for the GetAbstractionLevel trait implementations for ZFC types.
use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::zfc::set::{
    CardinalityPropertyVariant, ElementCondition, Set, SetElement, SetExpression, SetMapping,
    SetProperty, SetRelation,
};

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
            Set::Generic {
                name,
                element_description,
                properties,
            } => {
                title_text = name.as_ref().map_or_else(
                    || "Generic Set".to_string(),
                    |n| format!("Generic Set: {}", n),
                );
                if let Some(desc) = element_description {
                    content_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
                        "Elements described as: {}",
                        desc
                    ))));
                }
                for prop in properties.iter() {
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
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "Contains no elements (∅).",
                )));
            }
            Set::Singleton {
                element,
                properties,
            } => {
                title_text = format!("Singleton Set {{{:?}}}", element);
                // content_nodes.push(SectionContentNode::Paragraph(p_text(&format!("Contains exactly one element: {:?}", element))));
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
                content_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
                    "Elements: {{{}}}",
                    elements_str
                ))));
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
                content_nodes.push(SectionContentNode::Paragraph(p_text(&format!("This set is defined by a ZFC construction (e.g., Union, Intersection, PowerSet). Rendering for display level {:?} needs to be detailed.", object_formalism_level))));
            }
        }

        Section {
            id: format!("{}-set-section", id_prefix),
            title: Some(p_text(&title_text)),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathContentNode::Definition {
                    term_display: vec![RichTextSegment::Text(title_text.clone())],
                    formal_term: None, // TODO: Convert Set to local MathNode
                    label: Some(title_text.clone()),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(object_formalism_level as u8),
                        source_template_id: None,
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
            metadata: None,
            display_options: None,
        }
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        // Determine the inherent level of this Set object
        let inherent_formalism_level = self.level(); // formalism::AbstractionLevel

        let main_section = self.to_section_node(&format!("{}-main", id_prefix)); // Call with only id_prefix
        MathDocument {
            id: format!("{}-doc", id_prefix),
            title: main_section.title.as_ref().map_or_else(
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
            ),
            language: Some("en-US".to_string()),
            version: Some("1.0".to_string()),
            authors: None,
            date_published: None,
            date_modified: None,
            abstract_content: None,
            table_of_contents: None,
            body: vec![main_section],
            footnotes: None,
            glossary: None,
            bibliography: None,
            document_metadata: None,
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = match self {
            Set::Empty => "Empty Set (∅)".to_string(),
            Set::Singleton { element, .. } => format!("Singleton Set {{{:?}}}", element),
            _ => format!("Set: {}", id_prefix),
        };
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = match self {
            Set::Empty => "∅".to_string(),
            Set::Singleton { element, .. } => format!("{{{:?}}}", element),
            _ => format!("Set {}", id_prefix),
        };
        vec![link_to_definition(
            &name,
            &format!("{}-set-section", id_prefix),
            Some("ZFCSetTheory"),
        )]
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
