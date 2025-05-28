use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::formalism::theorem::Theorem;
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::fields::definitions::Field;
use crate::subjects::math::theories::groups::definitions::{
    AlternatingGroup, CenterGroup, CentralProductGroup, CentralizerGroup, CommutatorSubgroup,
    CyclicGroup, DihedralGroup, FreeGroup, GeneralLinearGroup, GeneratedSubgroup, GenericGroup,
    Group, GroupElement, ImageGroup, KernelGroup, LieGroup, ModularAdditiveGroup,
    ModularMultiplicativeGroup, NormalizerGroup, OrthogonalGroup, ProductGroup, ProductOperation,
    PullbackGroup, QuotientGroup, RestrictionGroup, SpecialLinearGroup, SpecialOrthogonalGroup,
    SpecialUnitaryGroup, SylowSubgroup, SymmetricGroup, TopologicalGroup, TrivialGroup,
    UnitaryGroup, WreathProductGroup,
};
use crate::subjects::math::theories::theorems::{
    prove_abelian_squared_criterion, prove_example_chaining_theorems, prove_inverse_product_rule,
    prove_inverse_uniqueness, prove_lagrange_theorem, prove_theorem_extraction_example,
};
use crate::subjects::math::theories::topology::definitions::{TopologicalSpace, Topology};
use crate::subjects::math::theories::zfc::set::Set;
use crate::turn_render::section_node::{MathematicalContent, ToSectionNode};

/// Content manifest for efficient loading across ALL theories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentManifest {
    pub theories: Vec<TheoryManifest>,
    pub total_items: usize,
    pub generated_at: String,
    pub version: String,
}

/// Theory-level manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoryManifest {
    pub theory_id: String,
    pub theory_name: String,
    pub files: Vec<ContentFile>,
    pub item_count: usize,
}

/// Individual content file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentFile {
    pub file_path: String,
    pub content_type: String, // "l1_definitions", "l3_constructors", "theorems"
    pub item_count: usize,
    pub items: Vec<String>, // List of content IDs in this file
}

/// Content bundle for specific theory and abstraction level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentBundle {
    pub theory_name: String,
    pub content_type: String,
    pub version: String,
    pub exported_at: String,
    pub content: Vec<MathematicalContent>,
}

/// **THEORY EXPORTER TRAIT** - Generic interface for theory exports
pub trait TheoryExporter {
    fn theory_id(&self) -> &str;
    fn theory_name(&self) -> &str;
    fn generate_definitions(&self) -> Vec<MathematicalContent>;
    fn generate_theorems(&self) -> Vec<MathematicalContent>;
}

/// Group Theory Exporter Implementation
pub struct GroupTheoryExporter;

impl TheoryExporter for GroupTheoryExporter {
    fn theory_id(&self) -> &str {
        "group_theory"
    }
    fn theory_name(&self) -> &str {
        "Group Theory"
    }

    fn generate_definitions(&self) -> Vec<MathematicalContent> {
        let mut content = Vec::new();

        // Generate sample instances for L1 definitions using our enhanced implementations

        // Basic group
        let basic_group = GenericGroup::default();
        content.push(basic_group.to_math_document("group_theory.basic.l1"));

        // Topological group with our enhanced content
        let topological_group = TopologicalGroup {
            core: GenericGroup::default(),
            topology: TopologicalSpace {
                base_set: Set::empty(),
                topology: Topology {
                    properties: VariantSet::new(),
                },
                properties: vec![],
            },
            props: VariantSet::new(),
        };
        content.push(topological_group.to_math_document("group_theory.topological.l1"));

        // Cyclic group (Z/12Z as example)
        let cyclic_group = CyclicGroup {
            core: GenericGroup::default(),
            generator: GroupElement::Integer(1),
            order: Some(12),
        };
        content.push(cyclic_group.to_math_document("group_theory.cyclic.l1"));

        // Symmetric group (S_4 as example)
        let symmetric_group = SymmetricGroup {
            core: GenericGroup::default(),
            degree: 4,
        };
        content.push(symmetric_group.to_math_document("group_theory.symmetric.l1"));

        // Lie group
        let lie_group = LieGroup {
            core: GenericGroup::default(),
            topology: TopologicalSpace {
                base_set: Set::empty(),
                topology: Topology {
                    properties: VariantSet::new(),
                },
                properties: vec![],
            },
            charts: vec!["chart1".to_string()],
            props: VariantSet::new(),
        };
        content.push(lie_group.to_math_document("group_theory.lie.l1"));

        content
    }

    fn generate_theorems(&self) -> Vec<MathematicalContent> {
        let mut content = vec![
            prove_inverse_uniqueness().to_math_document("group_theory.inverse_uniqueness"),
            prove_inverse_product_rule().to_math_document("group_theory.inverse_product_rule"),
            prove_abelian_squared_criterion()
                .to_math_document("group_theory.abelian_squared_criterion"),
            prove_lagrange_theorem().to_math_document("group_theory.lagrange_theorem"),
            prove_example_chaining_theorems()
                .to_math_document("group_theory.example_chaining_theorems"),
            prove_theorem_extraction_example()
                .to_math_document("group_theory.theorem_extraction_example"),
        ];

        // Generate theorem examples using our enhanced content
        // For now, we can create placeholder theorems or use actual theorem implementations if available

        // Placeholder for Haar measure theorem
        // content.insert(
        //     "group_theory.haar_measure_theorem".to_string(),
        //     // Would need theorem implementation
        // );

        // For now, return empty until we have theorem implementations
        content
    }
}

/// **COMPREHENSIVE EXPORTER FOR ALL MATHEMATICAL THEORIES**
/// This discovers available theories dynamically and exports them
pub struct UnifiedExporter;

impl UnifiedExporter {
    /// **MAIN EXPORT FUNCTION** - Discovers and exports ALL available theories
    pub fn export_all_theories_to_directory(output_dir: &str) -> Result<()> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;

        let mut manifest = ContentManifest {
            theories: vec![],
            total_items: 0,
            generated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            version: "1.0.0".to_string(),
        };

        // **GENERIC THEORY DISCOVERY** - Add available theory exporters here
        let available_theories: Vec<Box<dyn TheoryExporter>> = vec![
            Box::new(GroupTheoryExporter),
            // Add other theories when they become available:
            // Box::new(FieldTheoryExporter),
            // Box::new(NumberTheoryExporter),
            // Box::new(ZFCTheoryExporter),
        ];

        // Export each discovered theory
        for theory_exporter in available_theories {
            let theory_manifest =
                Self::export_theory_to_files(output_dir, theory_exporter.as_ref())?;
            manifest.total_items += theory_manifest.item_count;
            manifest.theories.push(theory_manifest);
        }

        // Write master manifest file
        let manifest_path = Path::new(output_dir).join("manifest.json");
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        fs::write(manifest_path, manifest_json)?;

        println!(
            "✅ All theories exported: {} items across {} theories in {}",
            manifest.total_items,
            manifest.theories.len(),
            output_dir
        );
        Ok(())
    }

    /// Generic method to export any theory using the TheoryExporter trait
    fn export_theory_to_files(
        output_dir: &str,
        theory: &dyn TheoryExporter,
    ) -> Result<TheoryManifest> {
        let mut theory_manifest = TheoryManifest {
            theory_id: theory.theory_id().to_string(),
            theory_name: theory.theory_name().to_string(),
            files: vec![],
            item_count: 0,
        };

        // Export L1 Definitions
        let definition_content = theory.generate_definitions();
        if !definition_content.is_empty() {
            let filename = format!("{}.definitions.json", theory.theory_id());
            Self::write_content_bundle(
                output_dir,
                &filename,
                theory.theory_name(),
                "definitions",
                definition_content.clone(),
            )?;

            theory_manifest.files.push(ContentFile {
                file_path: filename,
                content_type: "definitions".to_string(),
                item_count: definition_content.len(),
                items: definition_content.iter().map(|c| c.id.clone()).collect(),
            });
            theory_manifest.item_count += definition_content.len();
        }

        // Export Theorems
        let theorems_content = theory.generate_theorems();
        if !theorems_content.is_empty() {
            let filename = format!("{}.theorems.json", theory.theory_id());
            Self::write_content_bundle(
                output_dir,
                &filename,
                theory.theory_name(),
                "theorems",
                theorems_content.clone(),
            )?;

            theory_manifest.files.push(ContentFile {
                file_path: filename,
                content_type: "theorems".to_string(),
                item_count: theorems_content.len(),
                items: theorems_content.iter().map(|c| c.id.clone()).collect(),
            });
            theory_manifest.item_count += theorems_content.len();
        }

        Ok(theory_manifest)
    }

    /// Helper function to write content bundles
    fn write_content_bundle(
        output_dir: &str,
        filename: &str,
        theory_name: &str,
        content_type: &str,
        content: Vec<MathematicalContent>,
    ) -> Result<()> {
        let bundle = ContentBundle {
            theory_name: theory_name.to_string(),
            content_type: content_type.to_string(),
            version: "1.0.0".to_string(),
            exported_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            content,
        };

        let file_path = Path::new(output_dir).join(filename);
        fs::write(&file_path, serde_json::to_string_pretty(&bundle)?)?;
        Ok(())
    }

    // /// Legacy single-file export for backwards compatibility
    // pub fn write_to_file(path: &str) -> Result<()> {
    //     // Use the group theory exporter for legacy compatibility
    //     let group_exporter = GroupTheoryExporter;
    //     let export = LegacyMathExport {
    //         version: "1.0.0".to_string(),
    //         exported_at: std::time::SystemTime::now()
    //             .duration_since(std::time::UNIX_EPOCH)
    //             .unwrap()
    //             .as_secs()
    //             .to_string(),
    //         content: group_exporter.generate_l1_definitions(),
    //     };
    //     let json = serde_json::to_string_pretty(&export)?;
    //     fs::write(path, json)?;
    //     Ok(())
    // }
}
