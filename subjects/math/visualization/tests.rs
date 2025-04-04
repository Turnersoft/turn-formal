// Module: src/formalize_v2/subjects/math/visualization/tests.rs
// Tests for the theorem visualization functionality

#[cfg(test)]
mod tests {
    use crate::formalize_v2::subjects::math::theorem::core::MathContext;
    use crate::formalize_v2::subjects::math::visualization::models::{
        TheoremVisualization, TheoryVisualization,
    };

    #[test]
    fn test_visualization_models() {
        // Test that we can create the basic visualization models

        // Create a theory visualization
        let theory = TheoryVisualization {
            name: "Test Theory".to_string(),
            context: MathContext::Custom("Test".to_string()),
            description: "Test description".to_string(),
            theorems: Vec::new(),
        };

        // Verify basic properties
        assert_eq!(theory.name, "Test Theory");
        assert_eq!(theory.description, "Test description");
        assert!(theory.theorems.is_empty());
    }
}
