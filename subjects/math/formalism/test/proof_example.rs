// Module: src/formalize_v2/subjects/math/theorem/test/proof_example.rs
// Examples of using the proof builder to prove theorems with branching

use std::collections::HashMap;

use crate::subjects::math::formalism::{
    expressions::MathExpression,
    proof::{
        ProofForest, ProofGoal,
        tactics::{RewriteDirection, Tactic},
    },
    relations::MathRelation,
    theorem::Theorem,
};
use crate::turn_render::Identifier;

/// Helper function to create a variable expression
fn var(name: &str) -> MathExpression {
    MathExpression::var(name)
}

/// Creates a simple theorem for testing: a = b
fn create_simple_equality_theorem() -> Theorem {
    let goal = ProofGoal {
        statement: MathRelation::equal(var("a"), var("b")),
        quantifiers: vec![],
        value_variables: vec![],
    };
    let proofs = ProofForest::new_from_goal(goal);
    Theorem {
        id: "simple_equality".to_string(),
        name: "Simple Equality".to_string(),
        description: "A simple equality theorem for testing.".to_string(),
        proofs,
    }
}

/// Creates a theorem for testing: forall x, P(x) -> Q(x)
fn create_simple_implication_theorem_with_context() -> Theorem {
    // Use simple equality relations instead of Todo variants
    let p_of_x = MathRelation::equal(var("P"), var("x"));
    let q_of_x = MathRelation::equal(var("Q"), var("x"));
    let goal = ProofGoal {
        statement: MathRelation::Implies(Box::new(p_of_x), Box::new(q_of_x)),
        quantifiers: vec![],
        value_variables: vec![],
    };
    let proofs = ProofForest::new_from_goal(goal);
    Theorem {
        id: "simple_implication".to_string(),
        name: "Simple Implication".to_string(),
        description: "A simple implication theorem for testing.".to_string(),
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_proof_forest() {
        let theorem = create_simple_equality_theorem();
        let forest = theorem.proofs;
        assert_eq!(forest.len(), 1);
        assert_eq!(forest.roots.len(), 1);
        let root_node = forest.get_root().unwrap();
        assert_eq!(
            root_node.state.statement,
            MathRelation::equal(var("a"), var("b"))
        );
    }

    #[test]
    fn test_apply_tactic_assume_implication() {
        let theorem = create_simple_implication_theorem_with_context();
        let mut forest = theorem.proofs;
        let root_node = forest.get_root().unwrap().clone();

        let tactic = Tactic::AssumeImplicationAntecedent {
            hypothesis_name: Identifier::new_simple("H".to_string()),
        };
        let new_node = root_node.apply_tactic(tactic, &mut forest);

        assert_eq!(new_node.parent, Some(root_node.id));
        assert_eq!(forest.len(), 2);
        let child_node = forest.get_node(&new_node.id).unwrap();
        assert_eq!(child_node.state.value_variables.len(), 1);
        // Just check that we have a hypothesis variable - exact structure depends on Identifier implementation
        assert!(!child_node.state.value_variables.is_empty());
    }

    #[test]
    fn test_apply_tactic_rewrite() {
        // Register the theorem so the tactic can find it.
        let equality_theorem = create_simple_equality_theorem();
        crate::subjects::math::formalism::proof::TheoremRegistry::register_globally(
            equality_theorem,
        );

        let goal_to_prove = ProofGoal {
            statement: MathRelation::equal(var("a"), var("c")),
            quantifiers: vec![],
            value_variables: vec![],
        };
        let mut forest = ProofForest::new_from_goal(goal_to_prove);
        let root_node = forest.get_root().unwrap().clone();

        let tactic = Tactic::Rewrite {
            target: var("a"),
            theorem_id: "simple_equality".to_string(),
            instantiation: {
                let mut map = HashMap::new();
                map.insert("a".to_string(), var("a"));
                map.insert("b".to_string(), var("b"));
                map
            },
            direction: RewriteDirection::LeftToRight,
        };
        let new_node = root_node.apply_tactic(tactic, &mut forest);

        let expected_statement = MathRelation::equal(var("b"), var("c"));
        assert_eq!(new_node.state.statement, expected_statement);
    }
}
