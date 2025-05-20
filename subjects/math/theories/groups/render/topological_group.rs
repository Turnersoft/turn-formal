use serde::{Deserialize, Serialize};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::{
    CompactPropertyVariant, ConnectedPropertyVariant, MetrizablePropertyVariant, TopologicalGroup,
    TopologicalGroupProperty,
};
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::section_node::{
    AbstractionMetadata, MathDocument, ParagraphNode, RichTextSegment, Section, SectionContentNode,
    SelectableProperty, StructuredMathContentNode, ToSectionNode, p_text,
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

        // Create title from core group and topology
        let title = format!("Topological Group on {:?}", self.core.base_set);

        // Create content nodes with group and topology information
        let mut content_nodes = vec![
            SectionContentNode::Paragraph(p_text(&format!(
                "Core Group: Group on set {:?} with {:?} operation",
                self.core.base_set, self.core.operation.operation_type
            ))),
            SectionContentNode::Paragraph(p_text(&format!(
                "Topology: {:?}",
                self.topology.base_set
            ))),
        ];

        // Add properties from the topology if any
        if !self.topology.topology.properties.inner.is_empty() {
            let topology_props = self
                .topology
                .topology
                .properties
                .inner
                .iter()
                .map(|p| format!("{:?}", p.0))
                .collect::<Vec<_>>()
                .join(", ");

            content_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
                "Topology Properties: {}",
                topology_props
            ))));
        }

        // Add abstraction level specific content
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "This is L1: A general schema for any topological group.",
                )));
            }
            AbstractionLevel::Level2 => {
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "This is L2: A specific type of topological group with defined properties.",
                )));
            }
            AbstractionLevel::Level3 => {
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "This is L3: A constructor for building a topological group from a group and a topology."
                )));
            }
            AbstractionLevel::Level4 => {
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "This is L4: A concrete topological group with fully specified group, topology, and elements."
                )));
            }
        };

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
            id: format!("{}-topologicalgroup-section", id_prefix),
            title: Some(p_text(&title)),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathContentNode::Definition {
                    term_display: vec![RichTextSegment::Text(title.clone())],
                    formal_term: Some(self.to_turn_math(format!("{}-formalTerm", id_prefix))),
                    label: Some(format!("Definition ({})", title)),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(formalism_obj_level as u8),
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
            metadata: Some(vec![(
                "type".to_string(),
                "TopologicalGroupDefinition".to_string(),
            )]),
            display_options: None,
        }
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));

        MathDocument {
            id: format!("{}-doc", id_prefix),
            title: main_section.title.as_ref().map_or_else(
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
        let name = format!("Topological Group on {:?}", self.core.base_set);

        vec![crate::turn_render::section_node::link_to_definition(
            &name,
            &format!("{}-topologicalgroup-section", id_prefix),
            Some("GroupTheory"),
        )]
    }
}
