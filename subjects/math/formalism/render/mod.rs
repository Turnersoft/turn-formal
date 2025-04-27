use crate::turn_render::MathNode;

pub mod expressions;
pub mod math_object;
pub mod relations;
pub mod theorem;

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::super::super::super::super::{
        subjects::math::formalism::{
            core::{ProofGoal, Theorem},
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
            goal: ProofGoal {
                quantifier: vec![],
                value_variables: vec![],
                statement: MathRelation::custom("IsTrue".to_string(), vec![]),
            },
            proofs: ProofForest::new(),
        };

        // Convert to MathNode
        let node = theorem.to_turn_math("test_id".to_string());

        // Basic validation
        if let MathNodeContent::Theorem {
            name,
            description,
            goal: _,
            proofs: _,
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
        let state = ProofGoal {
            quantifier: vec![],
            value_variables: vec![],
            statement: MathRelation::custom("IsTrue".to_string(), vec![]),
        };

        // Convert to MathNode
        let node = state.to_turn_math("test_id".to_string());

        // Basic validation
        if let MathNodeContent::ProofGoal {
            statement: _,
            quantifiers,
            variables,
        } = &*node.content
        {
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
        let state = ProofGoal {
            quantifier: vec![],
            value_variables: vec![],
            statement: MathRelation::custom("IsTrue".to_string(), vec![]),
        };

        forest.add_node(ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: None,
            children: vec![],
            state,
            tactic: None,
            status: ProofStatus::InProgress,
        });

        // Convert to MathNode
        let node = forest.to_turn_math("test_id".to_string());

        // Basic validation
        if let MathNodeContent::ProofForest { roots } = &*node.content {
            assert_eq!(roots.len(), 1); // One root node
        } else {
            panic!("Expected ProofForest content");
        }
    }
}
