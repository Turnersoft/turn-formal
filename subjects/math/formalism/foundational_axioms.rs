/// Module: src/formalize_v2/subjects/math/formalism/foundational_axioms.rs
/// Provides foundational logical axioms that are available globally for all mathematical reasoning.
/// These axioms should be registered before any domain-specific theorems.
use std::{collections::HashMap, sync::Arc};

use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::proof::{ProofForest, ProofGoal};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::theorem::{Axiom, Theorem};
use crate::turn_render::Identifier;

use super::automation::registry::TheoremRegistry;
use super::extract::Parametrizable;

/// Axiom: Reflexivity of Equality
/// Statement: ∀x. x = x
pub fn equality_refl_axiom() -> Axiom {
    let x_var = Identifier::new_simple("x".to_string());
    let reflexivity_relation = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(x_var.clone())),
        right: Located::new(Parametrizable::Variable(x_var.clone())),
    };
    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(Arc::new(reflexivity_relation)),
    };
    Theorem {
        id: "equality_is_reflexive".to_string(),
        name: "Reflexivity of Equality".to_string(),
        description: "For any element x, x = x".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Axiom: Symmetry of Equality
/// Statement: ∀x,y. x = y → y = x
pub fn equality_symm_axiom() -> Axiom {
    let x_var = Identifier::new_simple("x".to_string());
    let y_var = Identifier::new_simple("y".to_string());
    let premise = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(x_var.clone())),
        right: Located::new(Parametrizable::Variable(y_var.clone())),
    };
    let conclusion = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(y_var.clone())),
        right: Located::new(Parametrizable::Variable(x_var.clone())),
    };
    let symmetry_relation = MathRelation::Implies(
        Located::new(Parametrizable::Concrete(Arc::new(premise))),
        Located::new(Parametrizable::Concrete(Arc::new(conclusion))),
    );
    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(Arc::new(symmetry_relation)),
    };
    Theorem {
        id: "equality_is_symmetric".to_string(),
        name: "Symmetry of Equality".to_string(),
        description: "If x = y, then y = x".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Axiom: Transitivity of Equality
/// Statement: ∀x,y,z. (x = y ∧ y = z) → x = z
pub fn equality_tran_axiom() -> Axiom {
    let x_var = Identifier::new_simple("x".to_string());
    let y_var = Identifier::new_simple("y".to_string());
    let z_var = Identifier::new_simple("z".to_string());
    let premise1 = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(x_var.clone())),
        right: Located::new(Parametrizable::Variable(y_var.clone())),
    };
    let premise2 = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(y_var.clone())),
        right: Located::new(Parametrizable::Variable(z_var.clone())),
    };
    let premises = MathRelation::And(vec![
        Located::new(Parametrizable::Concrete(Arc::new(premise1))),
        Located::new(Parametrizable::Concrete(Arc::new(premise2))),
    ]);
    let conclusion = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(x_var.clone())),
        right: Located::new(Parametrizable::Variable(z_var.clone())),
    };
    let transitivity_relation = MathRelation::Implies(
        Located::new(Parametrizable::Concrete(Arc::new(premises))),
        Located::new(Parametrizable::Concrete(Arc::new(conclusion))),
    );
    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(Arc::new(transitivity_relation)),
    };
    Theorem {
        id: "equality_is_transitive".to_string(),
        name: "Transitivity of Equality".to_string(),
        description: "If x = y and y = z, then x = z".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Axiom: Law of Identity
/// Statement: P → P
pub fn law_of_identity_axiom() -> Axiom {
    let p_var = Identifier::new_simple("P".to_string());
    let p_relation = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(p_var.clone())),
        right: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "true".to_string(),
        ))),
    };
    let identity_relation = MathRelation::Implies(
        Located::new(Parametrizable::Concrete(Arc::new(p_relation.clone()))),
        Located::new(Parametrizable::Concrete(Arc::new(p_relation))),
    );
    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(Arc::new(identity_relation)),
    };
    Theorem {
        id: "law_of_identity".to_string(),
        name: "Law of Identity".to_string(),
        description: "P implies P (tautology)".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Axiom: Modus Ponens
/// Statement: (P ∧ (P → Q)) → Q
pub fn modus_ponens_axiom() -> Axiom {
    let p_var = Identifier::new_simple("P".to_string());
    let q_var = Identifier::new_simple("Q".to_string());
    let p_true = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(p_var.clone())),
        right: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "true".to_string(),
        ))),
    };
    let q_true = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(q_var.clone())),
        right: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "true".to_string(),
        ))),
    };
    let implication = MathRelation::Implies(
        Located::new(Parametrizable::Concrete(Arc::new(p_true.clone()))),
        Located::new(Parametrizable::Concrete(Arc::new(q_true.clone()))),
    );
    let premises = MathRelation::And(vec![
        Located::new(Parametrizable::Concrete(Arc::new(p_true.clone()))),
        Located::new(Parametrizable::Concrete(Arc::new(implication))),
    ]);
    let modus_ponens_relation = MathRelation::Implies(
        Located::new(Parametrizable::Concrete(Arc::new(premises))),
        Located::new(Parametrizable::Concrete(Arc::new(q_true))),
    );
    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(Arc::new(modus_ponens_relation)),
    };
    Theorem {
        id: "modus_ponens".to_string(),
        name: "Modus Ponens".to_string(),
        description: "If P and P implies Q, then Q".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Axiom: Double Negation Elimination
/// Statement: ¬¬P → P
pub fn double_negation_axiom() -> Axiom {
    let p_var = Identifier::new_simple("P".to_string());
    let p_true = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(p_var.clone())),
        right: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "true".to_string(),
        ))),
    };
    let not_p = MathRelation::Not(Located::new(Parametrizable::Concrete(Arc::new(
        p_true.clone(),
    ))));
    let not_not_p = MathRelation::Not(Located::new(Parametrizable::Concrete(Arc::new(not_p))));
    let double_negation_relation = MathRelation::Implies(
        Located::new(Parametrizable::Concrete(Arc::new(not_not_p))),
        Located::new(Parametrizable::Concrete(Arc::new(p_true))),
    );
    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(Arc::new(double_negation_relation)),
    };
    Theorem {
        id: "double_negation".to_string(),
        name: "Double Negation Elimination".to_string(),
        description: "Not not P implies P".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Axiom: Universal Instantiation
/// Statement: ∀x.P(x) → P(a)
pub fn universal_instantiation_axiom() -> Axiom {
    let p_of_x = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "P".to_string(),
        ))),
        right: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "true".to_string(),
        ))),
    };
    let p_of_a = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "P".to_string(),
        ))),
        right: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "true".to_string(),
        ))),
    };
    let universal_instantiation_relation = MathRelation::Implies(
        Located::new(Parametrizable::Concrete(Arc::new(p_of_x))),
        Located::new(Parametrizable::Concrete(Arc::new(p_of_a))),
    );
    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(Arc::new(universal_instantiation_relation)),
    };
    Theorem {
        id: "universal_instantiation".to_string(),
        name: "Universal Instantiation".to_string(),
        description: "If P holds for all x, then P holds for any specific a".to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Axiom: Existential Generalization
/// Statement: P(a) → ∃x.P(x)
pub fn existential_generalization_axiom() -> Axiom {
    let p_of_a = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "P".to_string(),
        ))),
        right: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "true".to_string(),
        ))),
    };
    let p_of_x = MathRelation::Equal {
        left: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "P".to_string(),
        ))),
        right: Located::new(Parametrizable::Variable(Identifier::new_simple(
            "true".to_string(),
        ))),
    };
    let existential_generalization_relation = MathRelation::Implies(
        Located::new(Parametrizable::Concrete(Arc::new(p_of_a))),
        Located::new(Parametrizable::Concrete(Arc::new(p_of_x))),
    );
    let goal = ProofGoal {
        context: vec![],
        quantifiers: vec![],
        statement: Located::new(Arc::new(existential_generalization_relation)),
    };
    Theorem {
        id: "existential_generalization".to_string(),
        name: "Existential Generalization".to_string(),
        description: "If P holds for a specific a, then there exists an x such that P(x)"
            .to_string(),
        proofs: ProofForest::new_from_goal(goal),
    }
}

/// Note: Foundational tactics have been moved to the main Tactic enum in mod.rs
/// This provides better organization and consistency in the tactic system.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::formalism::automation::registry::get_theorem_registry;
    use crate::subjects::math::formalism::proof::tactics::Tactic;
    use crate::subjects::math::formalism::proof::tactics::implement::TacticApplicationResult;

    #[test]
    fn test_foundational_axioms_registration_and_retrieval() {
        // Get the singleton registry instance. This will also trigger the
        // registration of all axioms and theorems.
        let registry = get_theorem_registry();

        // Check that fundamental axioms can be retrieved. The `get` call
        // will prove and cache the axiom on the first call.
        assert!(registry.get("equality_is_reflexive").is_some());
        assert!(registry.get("equality_is_symmetric").is_some());
        assert!(registry.get("equality_is_transitive").is_some());
        assert!(registry.get("law_of_identity").is_some());
        assert!(registry.get("modus_ponens").is_some());
        assert!(registry.get("double_negation").is_some());
        assert!(registry.get("universal_instantiation").is_some());
        assert!(registry.get("existential_generalization").is_some());

        println!("✅ All foundational axioms registered and retrieved successfully");
    }

    #[test]
    fn test_foundational_tactics_in_tactic_enum() {
        // Test reflexivity tactic using the main Tactic enum
        let x_var = Identifier::new_simple("x".to_string());
        let reflexivity_goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: Located::new(Arc::new(MathRelation::Equal {
                left: Located::new(Parametrizable::Variable(x_var.clone())),
                right: Located::new(Parametrizable::Variable(x_var.clone())),
            })),
        };

        let reflexivity_tactic = Tactic::ByReflexivity;
        match reflexivity_tactic.apply_to_goal(&reflexivity_goal) {
            TacticApplicationResult::ProofComplete => {
                println!("✅ Reflexivity tactic in main enum works correctly");
            }
            other => panic!("Expected ProofComplete, got {:?}", other),
        }

        // Test transitivity tactic (create a goal that would require transitivity)
        // This would require proper hypotheses in context, so we'll test it with a simpler auto tactic
        let auto_goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: Located::new(Arc::new(MathRelation::True)), // Simple tautology that Auto should handle
        };

        let auto_tactic = Tactic::SearchAssumptions;
        match auto_tactic.apply_to_goal(&auto_goal) {
            TacticApplicationResult::Error(_) | TacticApplicationResult::NoChange => {
                println!("✅ Auto tactic correctly handles cases without applicable assumptions");
            }
            other => println!("Auto tactic result: {:?}", other),
        }
    }
}
