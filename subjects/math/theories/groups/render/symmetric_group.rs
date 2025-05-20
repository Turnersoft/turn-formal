use serde::{Deserialize, Serialize};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::SymmetricGroup;
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::section_node::{
    AbstractionMetadata, MathDocument, ParagraphNode, RichTextSegment, Section,
    SectionContentNode, SelectableProperty, StructuredMathContentNode, ToSectionNode, p_text,
};

impl ToTurnMath for SymmetricGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Standard notation for symmetric group is S_n
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text(format!("S_{}", self.degree))),
        }
    }
}

impl ToSectionNode for SymmetricGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        
        // Create title
        let title = format!("Symmetric Group S_{}", self.degree);
        
        // Create content nodes
        let mut content_nodes = vec![
            SectionContentNode::Paragraph(p_text(&format!(
                "Degree: {}",
                self.degree
            ))),
            SectionContentNode::Paragraph(p_text(&format!(
                "Order: {}",
                factorial(self.degree)
            ))),
        ];
        
        // Add core group information
        content_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
            "Base set: {:?}",
            self.core.base_set
        ))));
        
        content_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
            "Operation: {:?} ({})",
            self.core.operation.operation_type,
            match self.core.operation.notation {
                crate::subjects::math::theories::groups::definitions::GroupNotation::Infix(ref symbol) => 
                    format!("{:?}", symbol),
                crate::subjects::math::theories::groups::definitions::GroupNotation::Function(ref name) => 
                    name.clone(),
                crate::subjects::math::theories::groups::definitions::GroupNotation::Juxtaposition => 
                    "juxtaposition".to_string(),
            }
        ))));
        
        // Add abstraction level specific content
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "This is L1: A general schema for any symmetric group."
                )));
            },
            AbstractionLevel::Level2 => {
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "This is L2: A specific type of symmetric group with defined properties."
                )));
                
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "A symmetric group S_n consists of all permutations on n elements. \
                    It is a non-abelian group for n ≥ 3."
                )));
            },
            AbstractionLevel::Level3 => {
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "This is L3: A constructor for building a symmetric group from its degree."
                )));
            },
            AbstractionLevel::Level4 => {
                content_nodes.push(SectionContentNode::Paragraph(p_text(
                    "This is L4: A concrete symmetric group with fully specified elements."
                )));
                
                // For small degree, list the elements
                if self.degree <= 3 {
                    content_nodes.push(SectionContentNode::Paragraph(p_text(
                        "Elements: "
                    )));
                    
                    if self.degree == 1 {
                        content_nodes.push(SectionContentNode::Paragraph(p_text(
                            "e (identity)"
                        )));
                    } else if self.degree == 2 {
                        content_nodes.push(SectionContentNode::Paragraph(p_text(
                            "e (identity), (1 2)"
                        )));
                    } else if self.degree == 3 {
                        content_nodes.push(SectionContentNode::Paragraph(p_text(
                            "e (identity), (1 2), (1 3), (2 3), (1 2 3), (1 3 2)"
                        )));
                    }
                }
            },
        };
        
        // Create selectable properties
        let mut selectable_props = vec![];
        
        // Add properties from core group if any
        if !self.core.props.inner.is_empty() {
            for prop in self.core.props.inner.iter() {
                selectable_props.push(SelectableProperty {
                    name: format!("{:?}", prop.0),
                    current_variant: format!("{:?}", prop.0),
                    all_variants: vec![format!("{:?}", prop.0)],
                    description: Some("Group property".to_string()),
                    variant_descriptions: None,
                    property_type_def_id: None,
                });
            }
        }
        
        // Add specific symmetric group properties
        if self.degree >= 3 {
            selectable_props.push(SelectableProperty {
                name: "Abelian".to_string(),
                current_variant: "NonAbelian".to_string(),
                all_variants: vec!["Abelian".to_string(), "NonAbelian".to_string()],
                description: Some("Commutativity property".to_string()),
                variant_descriptions: None,
                property_type_def_id: None,
            });
        } else {
            selectable_props.push(SelectableProperty {
                name: "Abelian".to_string(),
                current_variant: "Abelian".to_string(),
                all_variants: vec!["Abelian".to_string(), "NonAbelian".to_string()],
                description: Some("Commutativity property".to_string()),
                variant_descriptions: None,
                property_type_def_id: None,
            });
        }
        
        // Always finite
        selectable_props.push(SelectableProperty {
            name: "Order".to_string(),
            current_variant: format!("Finite({})", factorial(self.degree)),
            all_variants: vec!["Finite(n)".to_string(), "Infinite".to_string()],
            description: Some("Order of the group (number of elements)".to_string()),
            variant_descriptions: None,
            property_type_def_id: None,
        });
        
        // Simple property
        if self.degree >= 5 {
            selectable_props.push(SelectableProperty {
                name: "Simple".to_string(),
                current_variant: "Simple".to_string(),
                all_variants: vec!["Simple".to_string(), "NonSimple".to_string()],
                description: Some("Simplicity property (no normal subgroups)".to_string()),
                variant_descriptions: None,
                property_type_def_id: None,
            });
        } else {
            selectable_props.push(SelectableProperty {
                name: "Simple".to_string(),
                current_variant: "NonSimple".to_string(),
                all_variants: vec!["Simple".to_string(), "NonSimple".to_string()],
                description: Some("Simplicity property (no normal subgroups)".to_string()),
                variant_descriptions: None,
                property_type_def_id: None,
            });
        }
        
        Section {
            id: format!("{}-symmetricgroup-section", id_prefix),
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
            metadata: Some(vec![("type".to_string(), "SymmetricGroupDefinition".to_string())]),
            display_options: None,
        }
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));
        
        MathDocument {
            id: format!("{}-doc", id_prefix),
            title: main_section.title.as_ref().map_or_else(
                || "Symmetric Group Document".to_string(),
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
        let tooltip_text = format!("Symmetric Group S_{} (order {})", 
            self.degree, 
            factorial(self.degree)
        );
        
        vec![RichTextSegment::Text(tooltip_text)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("Symmetric Group S_{}", self.degree);
        
        vec![crate::turn_render::section_node::link_to_definition(
            &name,
            &format!("{}-symmetricgroup-section", id_prefix),
            Some("GroupTheory"),
        )]
    }
}

// Helper function to calculate factorial for small numbers
fn factorial(n: usize) -> u64 {
    if n == 0 {
        return 1;
    }
    
    let mut result: u64 = 1;
    for i in 1..=n {
        result *= i as u64;
    }
    
    result
} 