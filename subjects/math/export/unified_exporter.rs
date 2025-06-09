use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::formalism::expressions::Identifier;
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::theorem::Theorem;
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::fields::definitions::{Field, FieldBasic};
use crate::subjects::math::theories::groups::definitions::{Group, GroupExpression, GroupRelation};

use crate::subjects::math::theories::groups::render::GroupTheoryExporter;
use crate::subjects::math::theories::probability::render::ProbabilityTheoryExporter;
// use crate::subjects::math::theories::groups::theorems::{
//     prove_abelian_squared_criterion, prove_example_chaining_theorems, prove_inverse_product_rule,
//     prove_inverse_uniqueness, prove_lagrange_theorem, prove_theorem_extraction_example,
// };
use crate::subjects::math::theories::topology::definitions::{TopologicalSpace, Topology};
use crate::subjects::math::theories::zfc::definitions::Set;
use crate::turn_render::*;

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
    pub content: Vec<MathDocument>,
}

/// **THEORY EXPORTER TRAIT** - Generic interface for theory exports
pub trait TheoryExporter<O, E, R> {
    fn theory_id(&self) -> &str;
    fn theory_name(&self) -> &str;

    // **MAIN THEORY ENTRANCE PAGE** - serves as the theory overview and navigation hub
    fn export_theory_overview(&self) -> MathDocument;

    // definitions - separate from the overview
    fn export_definitions(&self) -> Vec<MathDocument>;
    // intermediate steps to prepare type instances
    fn generate_object_definitions(&self) -> Vec<O>;
    fn generate_expression_definitions(&self) -> Vec<E>;
    fn generate_relation_definitions(&self) -> Vec<R>;

    // work on the type instances for export
    fn export_object_definitions(&self, objects: Vec<O>) -> Vec<MathDocument>;
    fn export_expression_definitions(&self, expressions: Vec<E>) -> Vec<MathDocument>;
    fn export_relation_definitions(&self, relations: Vec<R>) -> Vec<MathDocument>;
    // theorems - separate from the overview
    fn export_theorems(&self) -> Vec<MathDocument>;
}

/// **TYPE-ERASED THEORY EXPORTER** - Allows working with different theory types
/// This trait erases the type parameters to enable dynamic dispatch
pub trait AnyTheoryExporter {
    fn theory_id(&self) -> &str;
    fn theory_name(&self) -> &str;
    fn export_theory_overview(&self) -> MathDocument;
    fn export_definitions(&self) -> Vec<MathDocument>;
    fn export_theorems(&self) -> Vec<MathDocument>;
}

/// **WRAPPER STRUCT** - Wraps any TheoryExporter to implement AnyTheoryExporter
pub struct TheoryExporterWrapper<T, O, E, R>
where
    T: TheoryExporter<O, E, R>,
{
    exporter: T,
    _phantom: std::marker::PhantomData<(O, E, R)>,
}

impl<T, O, E, R> TheoryExporterWrapper<T, O, E, R>
where
    T: TheoryExporter<O, E, R>,
{
    pub fn new(exporter: T) -> Self {
        Self {
            exporter,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, O, E, R> AnyTheoryExporter for TheoryExporterWrapper<T, O, E, R>
where
    T: TheoryExporter<O, E, R>,
{
    fn theory_id(&self) -> &str {
        self.exporter.theory_id()
    }

    fn theory_name(&self) -> &str {
        self.exporter.theory_name()
    }

    fn export_theory_overview(&self) -> MathDocument {
        self.exporter.export_theory_overview()
    }

    fn export_definitions(&self) -> Vec<MathDocument> {
        self.exporter.export_definitions()
    }

    fn export_theorems(&self) -> Vec<MathDocument> {
        self.exporter.export_theorems()
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
        // Now we can use different theory types thanks to the AnyTheoryExporter trait
        let available_theories: Vec<Box<dyn AnyTheoryExporter>> = vec![
            Box::new(TheoryExporterWrapper::new(GroupTheoryExporter)),
            // Add probability theory exporter
            Box::new(TheoryExporterWrapper::new(ProbabilityTheoryExporter)),
            // Add other theories when they become available:
            // Box::new(TheoryExporterWrapper::new(FieldTheoryExporter)),
            // Box::new(TheoryExporterWrapper::new(NumberTheoryExporter)),
            // Box::new(TheoryExporterWrapper::new(ZFCTheoryExporter)),

            // **EXAMPLE**: To add a field theory exporter, you would:
            // 1. Create a FieldTheoryExporter struct
            // 2. Implement TheoryExporter<Field, FieldExpression, FieldRelation> for it
            // 3. Add it here: Box::new(TheoryExporterWrapper::new(FieldTheoryExporter))
            // The export_theory_to_files function will work automatically!
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

    /// **GENERIC METHOD** - Export any theory using the type-erased AnyTheoryExporter trait
    /// This now works for ALL theory types, not just group theory
    fn export_theory_to_files(
        output_dir: &str,
        theory: &dyn AnyTheoryExporter,
    ) -> Result<TheoryManifest> {
        let mut theory_manifest = TheoryManifest {
            theory_id: theory.theory_id().to_string(),
            theory_name: theory.theory_name().to_string(),
            files: vec![],
            item_count: 0,
        };

        // **THEORY OVERVIEW** - Main entrance page for the theory
        let theory_overview = theory.export_theory_overview();
        let overview_filename = format!("{}.overview.json", theory.theory_id());
        Self::write_content_bundle(
            output_dir,
            &overview_filename,
            theory.theory_name(),
            "theory_overview",
            vec![theory_overview.clone()],
        )?;

        theory_manifest.files.push(ContentFile {
            file_path: overview_filename,
            content_type: "theory_overview".to_string(),
            item_count: 1,
            items: vec![theory_overview.id],
        });
        theory_manifest.item_count += 1;

        // Export Definitions - separate from overview
        let definition_content = theory.export_definitions();
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

        // Export Theorems - separate from overview
        let theorems_content = theory.export_theorems();
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
        content: Vec<MathDocument>,
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
