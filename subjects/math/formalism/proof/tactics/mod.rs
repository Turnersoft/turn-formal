pub mod implement;

// Re-export the items so external code can continue to use them
pub use implement::TacticApplicationResult;

// Re-export only public functions from parent
pub use super::{ContextEntry, ProofForest, ProofNode};
use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::relations::MathRelation;
use crate::turn_render::Identifier;
use crate::turn_render::RichText;
use crate::turn_render::Section;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single, logical step in a formal proof.
/// This enum includes both primitive, single-step rules of inference and
/// higher-level automated tactics that execute complex procedures.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tactic {
    //=================================================================//
    // I.   PRIMITIVE: GOAL-DIRECTED TACTICS (Introduction Rules)
    //=================================================================//
    /// To prove `A → B`, adds `A` to the context and changes the goal to `B`.
    AssumeImplicationAntecedent { with_name: Identifier },

    /// To prove `A ∧ B`, splits the goal into two sub-goals: `A` and `B`.
    SplitGoalConjunction,

    /// To prove `A ∨ B`, requires the user to choose which disjunct to prove.
    SplitGoalDisjunction { disjunct_index: usize },

    /// Performs a case analysis on a variable by replacing it with a
    /// specific new version of the object in each new subgoal.
    /// This is the most direct and type-safe way to perform a case split.
    CaseAnalysis {
        /// The identifier of the variable in the context to replace.
        on_variable: Identifier,
        /// A vector where each entry represents a new subgoal.
        /// The `MathObject` in each case will replace the original `on_variable`.
        cases: Vec<Case>,
    },

    /// To prove `∀x:T, P(x)`, splits the proof into base case and inductive step.
    /// induction is a special case of case analysis on a quantifier.
    Induction {
        variable_name: Identifier,
        hypothesis_name: Identifier,
    },

    /// To prove a goal `∃x, P(x)`, this tactic provides a concrete "witness"
    /// term `t` for `x`. The goal then becomes to prove `P(t)`.
    /// This tactic consumes the specified existential quantifier in the goal's
    /// quantifier list and substitutes the witness for the bound variable
    /// in the goal's matrix.
    ProvideWitness {
        /// The variable of the quantifier to target (e.g., `x` in `∃x`).
        target_quantifier: Identifier,
        /// The witness term `t` that is claimed to satisfy the property.
        witness: MathExpression,
    },

    //=================================================================//
    // II.  PRIMITIVE: CONTEXT-DIRECTED TACTICS (Elimination Rules)
    //=================================================================//
    /// From `H: A ∧ B`, adds `A` and `B` as new hypotheses.
    SplitAssumptionConjunction {
        target_hypothesis: Identifier,
        with_names: Vec<Identifier>,
    },

    /// From `H: A ∨ B`, splits the proof into two cases.
    SplitAssumptionDisjunction {
        target_hypothesis: Identifier,
        with_names: Vec<CaseCondition>,
    },

    //=================================================================//
    // III. PRIMITIVE: COMPLETION TACTICS
    //=================================================================//
    /// Solves goal `G` by pointing to the exact hypothesis `H: G`.
    /// this is ExactWith tactics but
    ByRelation(RelationSource),

    /// Solves goal `t = t`.
    ByReflexivity,

    /// Solves any goal by citing two contradictory hypotheses `H1: A` and `H2: ¬A`.
    ByContradiction {
        hypothesis1: Identifier,
        hypothesis2: Identifier,
    },

    /// Solves goal `G` by citing a hypothesis `H: ¬G`.
    ByGoalContradiction { conflicting_hypothesis: Identifier },

    //=================================================================//
    // IV.  PRIMITIVE: REWRITING & STRUCTURAL TACTICS
    //=================================================================//
    /// The primary workhorse. Rewrites a target using an equality or implication.
    Rewrite {
        using_rule: RelationSource,
        target: Target,
        direction: RewriteDirection,
    },

    /// Replaces a defined term with its definition.
    UnfoldDefinition {
        definition_to_unfold: Identifier,
        target: Target,
    },

    /// Gives a name to a sub-expression for clarity.
    IntroduceLetBinding {
        target_expression: Target,
        with_name: Identifier,
    },

    /// Renames a bound variable for clarity (α-conversion).
    RenameBoundVariable {
        target: Target,
        from_name: Identifier,
        to_name: Identifier,
    },

    /// Moves a hypothesis back into the goal as an implication.
    Revert { hypothesis_to_revert: Identifier },

    //=================================================================//
    // V.   AUTOMATED TACTICS (Macros)
    //=================================================================//
    /// **Automated**: Searches the context to find a hypothesis that exactly
    /// matches the goal. A convenient version of `ByAssumption`.
    SearchAssumptions,

    /// **Automated**: Searches the theorem library for a single theorem that
    /// can directly prove the current goal.
    SearchTheoremLibrary,

    /// **Automated**: A convenient combination of the above. It first calls
    /// `SearchAssumptions` and, if that fails, calls `SearchTheoremLibrary`.
    Search,

    /// **Automated**: Attempts to simplify a target expression by repeatedly
    /// applying a pre-defined set of rewrite rules (`x+0=x`, etc.).
    Simplify { target: Target },

    /// **Automated**: A general-purpose "sledgehammer" tactic that tries a
    /// sequence of other tactics to solve the goal.
    Auto {
        /// Maximum search depth.
        depth: Option<u8>,
        /// Tactics to use in the search. If empty, a default, safe set is used.
        with_tactics: Vec<Tactic>, // todo: what the heck is this?
    },

    //=================================================================//
    // VI.  META-LOGICAL TACTICS
    //=================================================================//
    /// Closes a goal `G` by pointing to a theorem that proves `¬G`.
    /// This marks the current proof branch as disproven.
    DisproveByTheorem {
        /// The ID of the theorem proving the negation of the goal.
        theorem_id: String,
    },
}

//=================================================================//
// VI.   HELPER ENUMS
//=================================================================//

/// Specifies the source of a rule for the `Rewrite` tactic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationSource {
    LocalAssumption(Identifier),
    Theorem(String, Option<usize>), // theorem id, optional index of the theorem's node to use.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Target {
    pub scope: ContextOrStatement,
    pub id: String, // target expression id. this is the id to start searching from.
    /// Optional specific indices for Vec fields (e.g., for And/Or operations)
    pub vec_indices: Option<Vec<usize>>,
    /// Whether to allow reordering of Vec elements to match pattern
    pub allow_reordering: bool,
}

impl Target {
    pub fn new(scope: ContextOrStatement, target_id: String) -> Self {
        Self {
            scope,
            id: target_id,
            vec_indices: None,
            allow_reordering: true,
        }
    }

    pub fn with_indices(scope: ContextOrStatement, target_id: String, indices: Vec<usize>) -> Self {
        Self {
            scope,
            id: target_id,
            vec_indices: Some(indices),
            allow_reordering: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContextOrStatement {
    // deals with multiple identity at once.
    Context(Vec<Identifier>),
    Statement,
    // this checks all variables and the statement
    Both,
}

/// Represents a single branch in a case analysis proof.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Case {
    /// A human-readable description for this case branch, e.g., "Case: G is Abelian".
    pub description: RichText,
    /// The new object that will replace the original variable in this branch.
    pub replacement_object: MathExpression,
}

/// A condition for a case in a case analysis proof
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaseCondition {
    /// Description of the case
    pub name: Identifier,
    /// Condition/constraint for this case
    pub condition: MathExpression,
}

/// Specifies the direction for rewriting with an equality.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RewriteDirection {
    Forward,  // LHS -> RHS
    Backward, // RHS -> LHS
}

/*
impl std::fmt::Display for Tactic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tactic::Introduce { .. } => write!(f, "Introduce"),
            Tactic::IntroduceLetBinding { .. } => write!(f, "IntroduceLetBinding"),
            Tactic::IntroduceFreshVariable { .. } => write!(f, "IntroduceFreshVariable"),
            Tactic::ProvideWitness { .. } => write!(f, "ProvideWitness"),
            Tactic::ReorderQuantifiers { .. } => write!(f, "ReorderQuantifiers"),
            Tactic::UniversalCaseAnalysis { .. } => write!(f, "UniversalCaseAnalysis"),
            Tactic::ExactWith { .. } => write!(f, "ExactWith"),
            Tactic::Rewrite { .. } => write!(f, "Rewrite"),
            Tactic::AssumeImplicationAntecedent { .. } => write!(f, "AssumeImplicationAntecedent"),
            Tactic::SplitConjunction { .. } => write!(f, "SplitConjunction"),
            Tactic::SplitDisjunction { .. } => write!(f, "SplitDisjunction"),
            Tactic::StatementCaseAnalysis { .. } => write!(f, "StatementCaseAnalysis"),
            Tactic::Simplify { .. } => write!(f, "Simplify"),
            Tactic::Auto(_) => write!(f, "Auto"),
            Tactic::Reflexivity => write!(f, "Reflexivity"),
            Tactic::Transitivity { .. } => write!(f, "Transitivity"),
            Tactic::Induction { .. } => write!(f, "Induction"),
        }
    }
}
*/

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::formalism::expressions::MathExpression;
    use crate::subjects::math::formalism::proof::tactics::implement::TacticApplicationResult;
    use crate::subjects::math::formalism::proof::{
        ContextEntry, DefinitionState, ProofGoal, Quantifier,
    };
    use crate::subjects::math::formalism::relations::{MathRelation, Quantification};
    use crate::turn_render::{Identifier, RichText};
    use std::collections::HashMap;

    /// Test that demonstrates the comprehensive tactic application system
    #[test]
    fn test_comprehensive_tactic_application_system() {
        println!("=== Testing Comprehensive Tactic Application System ===");

        // Create a sample proof goal: ∀x: P(x) → Q(x), with hypothesis P(a) ⊢ Q(a)
        let goal = create_sample_implication_goal();

        println!("1. Testing tactic application...");

        // Test individual tactic types
        test_quantifier_tactics();
        test_structural_tactics();
        test_rewrite_tactics();
        test_automated_tactics();

        println!("✅ All tactic application tests passed!");
    }

    fn create_sample_implication_goal() -> ProofGoal {
        let x_var = Identifier::new_simple("x".to_string());
        let a_var = Identifier::new_simple("a".to_string());

        // Create P(a) → Q(a) as the goal
        let p_of_a = MathExpression::var("P_a");
        let q_of_a = MathExpression::var("Q_a");
        let implication = MathRelation::Implies(
            Box::new(MathRelation::equal(
                p_of_a.clone(),
                MathExpression::var("true"),
            )),
            Box::new(MathRelation::equal(q_of_a, MathExpression::var("true"))),
        );

        let context = vec![ContextEntry {
            name: a_var,
            ty: MathExpression::var("Element"),
            definition: DefinitionState::Abstract,
            description: Some(RichText::text("Element in domain".to_string())),
        }];

        let quantifiers = vec![Quantifier {
            variable_name: x_var,
            quantification: Quantification::Universal,
        }];

        ProofGoal {
            context,
            quantifiers,
            statement: implication,
        }
    }

    fn test_quantifier_tactics() {
        println!("2. Testing quantifier tactics...");

        let goal = create_sample_implication_goal();

        // Test universal quantifier introduction
        let fresh_var_tactic = Tactic::IntroduceFreshVariable {
            target_quantifier: Identifier::new_simple("x".to_string()),
            fresh_variable_name: Identifier::new_simple("c".to_string()),
        };

        let result = fresh_var_tactic.apply_to_goal(&goal);
        match result {
            implement::TacticApplicationResult::SingleGoal(new_goal) => {
                println!("   ✅ Universal quantifier elimination successful");
                assert!(new_goal.quantifiers.is_empty()); // Quantifier should be removed
                assert!(new_goal.context.len() > goal.context.len()); // Fresh variable added
            }
            other => panic!("Expected SingleGoal, got {:?}", other),
        }
    }

    fn test_structural_tactics() {
        println!("3. Testing structural tactics...");

        let goal = create_sample_implication_goal();

        // Test implication assumption
        let assume_tactic = Tactic::AssumeImplicationAntecedent {
            target_implication: Parametrizable::Concrete(goal.statement.clone()),
            with_name: Identifier::new_simple("h".to_string()),
        };

        let result = assume_tactic.apply_to_goal(&goal);
        match result {
            implement::TacticApplicationResult::SingleGoal(new_goal) => {
                println!("   ✅ Implication assumption successful");
                assert!(new_goal.context.len() > goal.context.len()); // Hypothesis added
            }
            other => panic!("Expected SingleGoal, got {:?}", other),
        }

        // Test conjunction splitting
        let conjunction = MathRelation::And(vec![
            MathRelation::equal(MathExpression::var("P"), MathExpression::var("true")),
            MathRelation::equal(MathExpression::var("Q"), MathExpression::var("true")),
        ]);

        let conj_goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: conjunction,
        };

        let split_tactic = Tactic::SplitConjunction {
            target_implication: Parametrizable::Concrete(conj_goal.statement.clone()),
            with_names: vec![
                Identifier::new_simple("p_true".to_string()),
                Identifier::new_simple("q_true".to_string()),
            ],
        };

        let result = split_tactic.apply_to_goal(&conj_goal);
        match result {
            implement::TacticApplicationResult::MultiGoal(goals) => {
                println!(
                    "   ✅ Conjunction splitting successful: {} subgoals",
                    goals.len()
                );
                assert_eq!(goals.len(), 2);
            }
            other => panic!("Expected MultiGoal, got {:?}", other),
        }
    }

    fn test_rewrite_tactics() {
        println!("4. Testing rewrite tactics...");

        // Test simplification tactic
        let complex_expr = MathExpression::var("complex_expr");
        let goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: MathRelation::equal(complex_expr.clone(), MathExpression::var("value")),
        };

        let simplify_tactic = Tactic::Simplify {
            target: Target {
                scope: ContextOrStatement::Statement,
                target: complex_expr.clone(),
            },
        };

        let result = simplify_tactic.apply_to_goal(&goal);
        match result {
            implement::TacticApplicationResult::SingleGoal(_new_goal) => {
                println!("   ✅ Simplification successful");
                // The expression should be simplified in the new goal
            }
            other => panic!("Expected SingleGoal, got {:?}", other),
        }
    }

    fn test_automated_tactics() {
        println!("5. Testing automated tactics...");

        // Create a goal that can be solved by assumption
        let assumption_goal = ProofGoal {
            context: vec![ContextEntry {
                name: Identifier::new_simple("h".to_string()),
                ty: MathExpression::Relation(Box::new(MathRelation::equal(
                    MathExpression::var("P"),
                    MathExpression::var("true"),
                ))),
                definition: DefinitionState::Abstract,
                description: Some(RichText::text("Hypothesis: P = true".to_string())),
            }],
            quantifiers: vec![],
            statement: MathRelation::equal(MathExpression::var("P"), MathExpression::var("true")),
        };

        let auto_tactic = Tactic::Auto(AutomatedTactic::ByAssumption);
        let result = auto_tactic.apply_to_goal(&assumption_goal);

        match result {
            implement::TacticApplicationResult::ProofComplete => {
                println!("   ✅ Auto tactic by assumption successful");
            }
            implement::TacticApplicationResult::Error(e) => {
                println!(
                    "   ℹ️  Auto tactic could not find proof (expected behavior): {}",
                    e
                );
            }
            other => println!("   ℹ️  Auto tactic result: {:?}", other),
        }
    }

    #[test]
    fn test_rewrite_with_hypothesis() {
        println!("=== Testing Rewrite Tactic with Hypotheses ===");

        // Create a goal with a hypothesis that can be used for rewriting
        let goal = ProofGoal {
            context: vec![ContextEntry {
                name: Identifier::new_simple("hyp_gh1_eq_e".to_string()),
                ty: MathExpression::Relation(Box::new(MathRelation::equal(
                    MathExpression::var("g * h1"),
                    MathExpression::var("e"),
                ))),
                definition: DefinitionState::Abstract,
                description: Some(RichText::text("Hypothesis: g * h1 = e".to_string())),
            }],
            quantifiers: vec![],
            statement: MathRelation::equal(
                MathExpression::var("g * h1"),
                MathExpression::var("other_expr"),
            ),
        };

        // Test rewrite tactic using the hypothesis
        let rewrite_tactic = Tactic::Rewrite {
            target: Target {
                scope: ContextOrStatement::Statement,
                target: MathExpression::var("g * h1").to_string(),
            },
            using_rule: RewriteSource::LocalAssumption(Identifier::new_simple("hyp_gh1_eq_e".to_string())),
            direction: RewriteDirection::LeftToRight,
        };

        let result = rewrite_tactic.apply_to_goal(&goal);
        match result {
            implement::TacticApplicationResult::SingleGoal(new_goal) => {
                println!("   ✅ Rewrite with hypothesis successful");
                // The "g * h1" should be replaced with "e" in the new goal
                println!("   Original statement: {:?}", goal.statement);
                println!("   Rewritten statement: {:?}", new_goal.statement);
            }
            implement::TacticApplicationResult::Error(msg) => {
                println!("   ❌ Rewrite failed: {}", msg);
                // This might fail due to expression matching - that's expected
            }
            other => println!("   Unexpected result: {:?}", other),
        }

        println!("✅ Hypothesis rewrite test completed!");
    }

    #[test]
    fn test_exact_with_hypothesis() {
        println!("=== Testing Exact Tactic with Hypotheses ===");

        // Create a goal that exactly matches a hypothesis
        let goal = ProofGoal {
            context: vec![ContextEntry {
                name: Identifier::new_simple("hyp_p_eq_q".to_string()),
                ty: MathExpression::Relation(Box::new(MathRelation::equal(
                    MathExpression::var("P"),
                    MathExpression::var("Q"),
                ))),
                definition: DefinitionState::Abstract,
                description: Some(RichText::text("Hypothesis: P = Q".to_string())),
            }],
            quantifiers: vec![],
            statement: MathRelation::equal(MathExpression::var("P"), MathExpression::var("Q")),
        };

        // Test exact tactic using the hypothesis
        let exact_tactic = Tactic::ExactWith {
            theorem_or_local_assumption: TheoremOrLocalAssumption::LocalAssumption(
                Identifier::new_simple("hyp_p_eq_q".to_string()),
            ),
            instantiation: HashMap::new(),
        };

        let result = exact_tactic.apply_to_goal(&goal);
        match result {
            implement::TacticApplicationResult::ProofComplete => {
                println!("   ✅ Exact with hypothesis successful - proof complete!");
            }
            implement::TacticApplicationResult::Error(msg) => {
                println!("   ❌ Exact failed: {}", msg);
                // This might fail due to hypothesis lookup - that's expected
            }
            other => println!("   Unexpected result: {:?}", other),
        }

        println!("✅ Hypothesis exact test completed!");
    }
}
*/
