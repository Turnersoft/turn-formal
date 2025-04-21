use crate::turn_render::MathNode;

pub mod expressions;
pub mod math_object;
pub mod relations;
pub mod theorem;

#[cfg(test)]
mod tests {
    use crate::{
        subjects::math::formalism::{
            core::{ProofState, Theorem},
            proof::{ProofForest, ProofNode, ProofStatus},
            relations::MathRelation,
        },
        turn_render::{MathNode, MathNodeContent, ToTurnMath},
    };
    use std::collections::HashMap;

    #[test]
    fn test_theorem_to_turn_math() {
        // Create a simple theorem
        let theorem = Theorem {
            id: "test_theorem".to_string(),
            name: "Test Theorem".to_string(),
            description: "A test theorem for rendering".to_string(),
            initial_proof_state: ProofState {
                quantifier: vec![],
                value_variables: vec![],
                statement: MathRelation::custom("IsTrue".to_string(), vec![]),
                path: Some("p0".to_string()),
                justification: None,
            },
        };

        // Convert to MathNode
        let node = theorem.to_turn_math("test_id".to_string());

        // Basic validation
        if let MathNodeContent::Theorem {
            name,
            description,
            initial_proof_state: _,
        } = &*node.content
        {
            assert_eq!(name, "Test Theorem");
            assert_eq!(description, "A test theorem for rendering");
        } else {
            panic!("Expected Theorem content");
        }
    }

    #[test]
    fn test_proof_state_to_turn_math() {
        // Create a simple proof state
        let state = ProofState {
            quantifier: vec![],
            value_variables: vec![],
            statement: MathRelation::custom("IsTrue".to_string(), vec![]),
            path: Some("p0".to_string()),
            justification: Some("By assumption".to_string()),
        };

        // Convert to MathNode
        let node = state.to_turn_math("test_id".to_string());

        // Basic validation
        if let MathNodeContent::ProofState {
            path,
            justification,
            statement: _,
            quantifiers,
            variables,
        } = &*node.content
        {
            assert_eq!(path, &Some("p0".to_string()));
            assert_eq!(justification, &Some("By assumption".to_string()));
            assert!(quantifiers.is_empty());
            assert!(variables.is_empty());
        } else {
            panic!("Expected ProofState content");
        }
    }

    #[test]
    fn test_proof_forest_to_turn_math() {
        // Create a simple proof forest
        let mut forest = ProofForest::new();

        // Add a root node
        let state = ProofState {
            quantifier: vec![],
            value_variables: vec![],
            statement: MathRelation::custom("IsTrue".to_string(), vec![]),
            path: Some("p0".to_string()),
            justification: None,
        };

        forest.add_node(
            None,
            state.clone(),
            None,
            "Initial state".to_string(),
            ProofStatus::InProgress,
        );

        // Convert to MathNode
        let node = forest.to_turn_math("test_id".to_string());

        // Basic validation
        if let MathNodeContent::ProofForest {
            summary,
            roots,
            bookmarks,
        } = &*node.content
        {
            assert!(!summary.is_empty());
            assert_eq!(roots.len(), 1); // One root node
            assert!(bookmarks.is_empty());
        } else {
            panic!("Expected ProofForest content");
        }
    }
}
