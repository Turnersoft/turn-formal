#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::export::unified_exporter::TheoryExporter;
    use crate::subjects::math::theories::probability::definitions::*;
    use crate::turn_render::section_node::ToSectionNode;

    #[test]
    fn test_generic_probability_space_rendering() {
        let space = GenericProbabilitySpace::default();
        let section = space.to_section_node("test_id");
        assert_eq!(section.id, "test_id.main");
    }

    #[test]
    fn test_probability_theory_exporter() {
        let exporter =
            crate::subjects::math::theories::probability::render::ProbabilityTheoryExporter;
        assert_eq!(exporter.theory_id(), "probability_theory");
        assert_eq!(exporter.theory_name(), "Probability Theory");
    }
}
