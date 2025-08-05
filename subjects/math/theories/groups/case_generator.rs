use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::objects::MathObject;
use crate::subjects::math::formalism::proof::tactics::Case;
use crate::subjects::math::theories::VariantSet;
use crate::turn_render::{RichText, RichTextSegment};
use std::sync::Arc;

use super::definitions::*;

impl Group {
    pub fn to_math_expression(&self) -> MathExpression {
        MathExpression::Object(Arc::new(MathObject::Group(self.clone())))
    }

    /// Performs a case split on the Abelian property.
    /// Returns a vector of `Case` structs, one for each branch of the proof.
    pub fn case_core_abelian(&self) -> Result<Vec<Case>, String> {
        // Define the exhaustive variants for the property.
        let variants = vec![
            AbelianPropertyVariant::Abelian,
            AbelianPropertyVariant::NonAbelian,
        ];

        let cases = variants
            .into_iter()
            .map(|variant| {
                // 1. Create a human-readable description for the case.
                let description_text = match &variant {
                    AbelianPropertyVariant::Abelian => "Case: The group is Abelian".to_string(),
                    AbelianPropertyVariant::NonAbelian => {
                        "Case: The group is non-Abelian".to_string()
                    }
                };
                let description = RichText {
                    segments: vec![RichTextSegment::Text(description_text)],
                    alignment: None,
                };

                // 2. Create the modified group object for this case.
                let mut new_group = self.clone();
                let props = new_group.get_mut_core_properties();
                props.insert(GroupProperty::Abelian(variant));
                let replacement_object = new_group.to_math_expression();

                // 3. Return the fully-formed Case struct.
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(cases)
    }

    /// Performs a case split on the Finite property.
    pub fn case_core_finite(&self) -> Result<Vec<Case>, String> {
        let variants = vec![
            FinitePropertyVariant::Finite(0), // Placeholder for "some n"
            FinitePropertyVariant::Infinite,
            FinitePropertyVariant::LocallyFinite,
        ];

        let cases = variants
            .into_iter()
            .map(|variant| {
                let description_text = match &variant {
                    FinitePropertyVariant::Finite(_) => "Case: The group is finite".to_string(),
                    FinitePropertyVariant::Infinite => "Case: The group is infinite".to_string(),
                    FinitePropertyVariant::LocallyFinite => {
                        "Case: The group is locally finite".to_string()
                    }
                };
                let description = RichText::text(description_text);
                let mut new_group = self.clone();
                new_group
                    .get_mut_core_properties()
                    .insert(GroupProperty::Finite(variant));
                let replacement_object = new_group.to_math_expression();
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(cases)
    }

    /// Performs a case split on the Simple property.
    pub fn case_core_simple(&self) -> Result<Vec<Case>, String> {
        let variants = vec![
            SimplePropertyVariant::Simple,
            SimplePropertyVariant::NonSimple,
            SimplePropertyVariant::QuasiSimple,
        ];

        let cases = variants
            .into_iter()
            .map(|variant| {
                let description_text = match &variant {
                    SimplePropertyVariant::Simple => "Case: The group is simple".to_string(),
                    SimplePropertyVariant::NonSimple => "Case: The group is not simple".to_string(),
                    SimplePropertyVariant::QuasiSimple => {
                        "Case: The group is quasi-simple".to_string()
                    }
                };
                let description = RichText::text(description_text);
                let mut new_group = self.clone();
                new_group
                    .get_mut_core_properties()
                    .insert(GroupProperty::Simple(variant));
                let replacement_object = new_group.to_math_expression();
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(cases)
    }

    /// Performs a case split on the Solvable property.
    pub fn case_core_solvable(&self) -> Result<Vec<Case>, String> {
        let variants = vec![
            SolvablePropertyVariant::Solvable,
            SolvablePropertyVariant::NonSolvable,
            SolvablePropertyVariant::Polysolvable,
        ];

        let cases = variants
            .into_iter()
            .map(|variant| {
                let description_text = match &variant {
                    SolvablePropertyVariant::Solvable => "Case: The group is solvable".to_string(),
                    SolvablePropertyVariant::NonSolvable => {
                        "Case: The group is not solvable".to_string()
                    }
                    SolvablePropertyVariant::Polysolvable => {
                        "Case: The group is polysolvable".to_string()
                    }
                };
                let description = RichText::text(description_text);
                let mut new_group = self.clone();
                new_group
                    .get_mut_core_properties()
                    .insert(GroupProperty::Solvable(variant));
                let replacement_object = new_group.to_math_expression();
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(cases)
    }

    /// Performs a case split on the Nilpotent property.
    pub fn case_core_nilpotent(&self) -> Result<Vec<Case>, String> {
        let variants = vec![
            NilpotentPropertyVariant::Nilpotent(0), // Placeholder for "some class c"
            NilpotentPropertyVariant::NonNilpotent,
        ];

        let cases = variants
            .into_iter()
            .map(|variant| {
                let description_text = match &variant {
                    NilpotentPropertyVariant::Nilpotent(_) => {
                        "Case: The group is nilpotent".to_string()
                    }
                    NilpotentPropertyVariant::NonNilpotent => {
                        "Case: The group is not nilpotent".to_string()
                    }
                };
                let description = RichText::text(description_text);
                let mut new_group = self.clone();
                new_group
                    .get_mut_core_properties()
                    .insert(GroupProperty::Nilpotent(variant));
                let replacement_object = new_group.to_math_expression();
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(cases)
    }
}

impl TopologicalGroup {
    pub fn to_math_expression(&self) -> MathExpression {
        MathExpression::Object(Arc::new(MathObject::Group(Group::Topological(
            self.clone(),
        ))))
    }

    pub fn case_compact(&self) -> Result<Vec<Case>, String> {
        let variants = vec![
            CompactPropertyVariant::Compact,
            CompactPropertyVariant::LocallyCompact,
            CompactPropertyVariant::NonCompact,
        ];
        let cases = variants
            .into_iter()
            .map(|variant| {
                let description_text = match &variant {
                    CompactPropertyVariant::Compact => "Case: The group is compact".to_string(),
                    CompactPropertyVariant::NonCompact => {
                        "Case: The group is non-compact".to_string()
                    }
                    CompactPropertyVariant::LocallyCompact => {
                        "Case: The group is locally compact".to_string()
                    }
                };
                let description = RichText {
                    segments: vec![RichTextSegment::Text(description_text)],
                    alignment: None,
                };
                let mut new_group = self.clone();

                new_group
                    .props
                    .insert(TopologicalGroupProperty::Compact(variant));
                let replacement_object = new_group.to_math_expression();
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(cases)
    }

    pub fn case_connected(&self) -> Result<Vec<Case>, String> {
        let variants = vec![
            ConnectedPropertyVariant::Connected,
            ConnectedPropertyVariant::SimplyConnected,
            ConnectedPropertyVariant::TotallyDisconnected,
            ConnectedPropertyVariant::LocallyConnected,
            ConnectedPropertyVariant::LocallySimplyConnected,
        ];
        let cases = variants
            .into_iter()
            .map(|variant| {
                let description_text = match &variant {
                    ConnectedPropertyVariant::Connected => {
                        "Case: The group is connected".to_string()
                    }
                    ConnectedPropertyVariant::SimplyConnected => {
                        "Case: The group is simply connected".to_string()
                    }
                    ConnectedPropertyVariant::TotallyDisconnected => {
                        "Case: The group is totally disconnected".to_string()
                    }
                    ConnectedPropertyVariant::LocallyConnected => {
                        "Case: The group is locally connected".to_string()
                    }
                    ConnectedPropertyVariant::LocallySimplyConnected => {
                        "Case: The group is locally simply connected".to_string()
                    }
                };
                let description = RichText::text(description_text);
                let mut new_group = self.clone();

                new_group
                    .props
                    .insert(TopologicalGroupProperty::Connected(variant));
                let replacement_object = new_group.to_math_expression();
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(cases)
    }

    pub fn case_metrizable(&self) -> Result<Vec<Case>, String> {
        let variants = vec![
            MetrizablePropertyVariant::Metrizable,
            MetrizablePropertyVariant::NonMetrizable,
        ];
        let cases = variants
            .into_iter()
            .map(|variant| {
                let description_text = match &variant {
                    MetrizablePropertyVariant::Metrizable => {
                        "Case: The group is metrizable".to_string()
                    }
                    MetrizablePropertyVariant::NonMetrizable => {
                        "Case: The group is non-metrizable".to_string()
                    }
                };
                let description = RichText::text(description_text);
                let mut new_group = self.clone();

                new_group
                    .props
                    .insert(TopologicalGroupProperty::Metrizable(variant));
                let replacement_object = new_group.to_math_expression();
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(cases)
    }
}

impl LieGroup {
    pub fn to_math_expression(&self) -> MathExpression {
        MathExpression::Object(Arc::new(MathObject::Group(Group::Lie(self.clone()))))
    }

    pub fn case_semisimple(&self) -> Result<Vec<Case>, String> {
        let variants = vec![
            SemisimplePropertyVariant::Semisimple,
            SemisimplePropertyVariant::NonSemisimple,
            SemisimplePropertyVariant::Split,
        ];
        let cases = variants
            .into_iter()
            .map(|variant| {
                let description_text = match &variant {
                    SemisimplePropertyVariant::Semisimple => {
                        "Case: The group is semisimple".to_string()
                    }
                    SemisimplePropertyVariant::NonSemisimple => {
                        "Case: The group is not semisimple".to_string()
                    }
                    SemisimplePropertyVariant::Split => {
                        "Case: The group is split semisimple".to_string()
                    }
                };
                let description = RichText::text(description_text);
                let mut new_group = self.clone();

                new_group
                    .props
                    .insert(LieGroupProperty::Semisimple(variant));
                let replacement_object = new_group.to_math_expression();
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(cases)
    }

    pub fn case_reductive(&self) -> Result<Vec<Case>, String> {
        let variants = vec![
            ReductivePropertyVariant::Reductive,
            ReductivePropertyVariant::NonReductive,
        ];
        let cases = variants
            .into_iter()
            .map(|variant| {
                let description_text = match &variant {
                    ReductivePropertyVariant::Reductive => {
                        "Case: The group is reductive".to_string()
                    }
                    ReductivePropertyVariant::NonReductive => {
                        "Case: The group is not reductive".to_string()
                    }
                };
                let description = RichText::text(description_text);
                let mut new_group = self.clone();

                new_group.props.insert(LieGroupProperty::Reductive(variant));
                let replacement_object = new_group.to_math_expression();
                Ok(Case {
                    description,
                    replacement_object,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(cases)
    }
}
