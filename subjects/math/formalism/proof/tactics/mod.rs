pub mod case_analysis;
pub mod implement;
pub mod search_replace;
pub mod theorem_applier;

// Re-export the items so external code can continue to use them
pub use case_analysis::{CaseAnalysisBuilder, CaseResult};
pub use implement::TacticApplicationResult;
pub use search_replace::{ExpressionPath, ReplacementSpec, SearchReplace, SearchResult};
pub use theorem_applier::{TheoremApplicationError, TheoremApplicationResult, TheoremApplier};

// Re-export only public functions from parent
pub use super::{ContextEntry, ProofForest, ProofNode};
use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::relations::MathRelation;
use crate::turn_render::Identifier;
use crate::turn_render::Section;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A tactic that can be applied to a proof state to transform it
/// Each tactic is context-aware and specifies exactly where it operates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tactic {
    // ========== CONTEXT MANIPULATION TACTICS ==========
    /// Introduce a new item into the context. This is a general-purpose tactic
    /// for adding variables, hypotheses, or definitions.
    Introduce {
        entry: ContextEntry,
        position: Option<usize>,
    },

    // ========== QUANTIFIER TACTICS ==========
    /// For a universal quantifier (∀), introduce a fresh, arbitrary variable to stand for it.
    /// This is a key step in starting a universal proof.
    IntroduceFreshVariable {
        /// Identifier of the universal quantifier (e.g., `∀x`)
        target_quantifier: Identifier,
        /// The name for the new fresh variable (e.g., `c`)
        fresh_variable_name: Identifier,
    },

    /// For an existential quantifier (∃), provide a concrete witness value.
    /// This creates a new subgoal to prove the property for the witness.
    ProvideWitness {
        /// Identifier of the existential quantifier (e.g., `∃x`)
        target_quantifier: Identifier,
        /// The witness expression to provide
        witness: MathExpression,
    },

    /// Reorder quantifiers (useful for ∀x∀y → ∀y∀x transformations, must be a legal swap)
    ReorderQuantifiers {
        /// New order specified by identifiers
        new_order: Vec<Identifier>,
    },

    /// Case analysis on a universal quantifier.
    UniversalCaseAnalysis {
        /// The universal quantifier to analyze.
        target_quantifier: Identifier,
        /// Cases to consider, which must be exhaustive. Each creates a separate subgoal.
        cases: Vec<CaseCondition>,
    },

    // ========== STATEMENT TACTICS ==========
    /// Attempts to prove the current goal *exactly* by showing it is an instance of a known theorem.
    /// This is a terminal tactic; if it succeeds, the goal is closed.
    ExactWith {
        /// The ID of the theorem to use (e.g., "addition_is_commutative").
        theorem_id: String,
        /// The mapping from the theorem's general variables to the goal's specific expressions.
        instantiation: HashMap<String, MathExpression>,
    },

    /// Rewrites a target sub-expression within the goal using a specified equality rule.
    Rewrite {
        /// The sub-expression or sub-relation within the goal to be transformed.
        /// If the theorem is an equality (`=`), this is a `MathExpression`.
        /// If the theorem is an implication (`=>`) or equivalence (`<=>`),
        /// this should be a `MathExpression::Relation`.
        target: MathExpression,
        /// The ID of the theorem that provides the rule (e.g., `a + b = b + a`).
        theorem_id: String,
        /// The mapping from the rule's variables to expressions.
        instantiation: HashMap<String, MathExpression>,
        /// The direction to apply the rule.
        /// - For `=` and `<=>`: `LeftToRight` or `RightToLeft`.
        /// - For `=>`: Only `RightToLeft` is sound (replaces conclusion with premise).
        direction: RewriteDirection,
    },

    /// When the goal is an implication `A => B`, this tactic creates a new sub-goal
    /// where `A` is added to the context as a named hypothesis and the new goal is `B`.
    /// This is a fundamental tactic used to begin proofs of implication.
    AssumeImplicationAntecedent {
        /// The name to assign to the new hypothesis (A).
        hypothesis_name: Identifier,
    },

    /// Split a conjunction in the statement (e.g., A ∧ B -> prove A, then prove B).
    SplitConjunction,

    /// Choose one disjunct to prove from a disjunction (e.g., to prove A ∨ B ∨ C, choose to prove B).
    SplitDisjunction {
        /// The target disjunction to split. Must be an `Or` variant.
        target: Box<MathRelation>,
        /// The index of the disjunct to focus on as the new goal.
        index: usize,
    },

    /// Case analysis on an expression in the statement
    StatementCaseAnalysis {
        /// The expression for case analysis.
        target: MathExpression,
        /// Cases to consider.
        cases: Vec<(String, MathExpression)>,
    },

    /// Simplify a subexpression in the statement
    Simplify {
        /// Path to the subexpression to simplify
        target_path: Vec<usize>,
        /// The original subexpression
        original_expr: MathExpression,
        /// The simplified subexpression
        simplified_expr: MathExpression,
    },

    // ========== META TACTICS ==========
    /// Apply an automated tactic.
    Auto(AutomatedTactic),

    // These don't transform the goal but affect proof structure.
    /// Apply mathematical induction on a universally quantified statement.
    /// This tactic will create two new sub-goals: one for the base case and one for the inductive step.
    Induction {
        /// A path of indices to the universally quantified sub-relation within the goal statement.
        /// This unambiguously locates the target of the induction.
        target_relation_path: TargetRelationLocation,

        /// The value to be substituted for the induction variable in the base case,
        /// represented as a MathExpression (e.g., `MathExpression::Number("0")`).
        base_case_value: MathExpression,

        /// The name to use for the induction variable in the inductive step (e.g., `k`).
        induction_variable_name: Identifier,

        /// The name to give the induction hypothesis, `P(k)`, in the inductive step's goal.
        induction_hypothesis_name: Identifier,
    },
}

/// A condition for a case in a case analysis proof
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaseCondition {
    /// Description of the case
    pub name: String,
    /// Condition/constraint for this case
    pub condition: MathRelation,
    /// Optional specific values or intervals
    pub values: Vec<MathExpression>,
}

/// The direction in which to apply an equality for a rewrite.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RewriteDirection {
    /// Use `LHS = RHS` to replace an instance of `LHS` with `RHS`.
    LeftToRight,
    /// Use `LHS = RHS` to replace an instance of `RHS` with `LHS`.
    RightToLeft,
}

/// Methods for decomposing expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecompositionMethod {
    /// Break into components
    Components,
    /// Factor out common terms
    Factor,
    /// Expand into sum of products
    Expand,
    /// Other domain-specific method
    Other(String),
}

/// Types of induction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InductionType {
    /// Mathematical induction on natural numbers
    Natural,
    /// Structural induction
    Structural,
    /// Transfinite induction
    Transfinite,
    /// Well-founded induction
    WellFounded,
    /// Other induction type
    Other(String),
}

/// Specifies a location within a `ProofGoal` for a tactic to operate on.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TargetRelationLocation {
    /// The main goal statement. The path specifies a sub-expression if Some.
    Statement(Option<MathRelation>),
    /// The expression bound to a value variable. The path specifies a sub-expression if Some.
    VariableBinding {
        variable: Identifier,
        sub_relation: Option<MathRelation>,
    },
    Quantifier {
        quantifier: Identifier,
        sub_relation: Option<MathRelation>,
    },
}

/// A set of automated tactics that perform searches or simplifications.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AutomatedTactic {
    /// Searches the theorem library to find a proof for a target sub-relation,
    /// replacing it with `True` on success.
    FindProof {
        /// The location of the sub-relation to prove.
        location: TargetRelationLocation,
    },

    /// Attempts to simplify a target expression using a set of predefined rules.
    Simplify {
        /// The location of the expression to simplify.
        location: TargetRelationLocation,
    },

    /// Checks if the goal is provable by one of the current assumptions.
    #[default]
    ByAssumption,

    /// Tries to find a contradiction within the assumptions, proving the goal `ex falso`.
    Contradiction,

    /// A recursive, best-effort tactic that tries a battery of other tactics.
    Auto {
        /// Maximum search depth.
        depth: Option<u8>,
        /// Tactics to use in the search. If empty, a default set is used.
        using: Vec<AutomatedTactic>,
    },
}

impl std::fmt::Display for Tactic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tactic::Introduce { .. } => write!(f, "Introduce"),
            Tactic::IntroduceFreshVariable { .. } => write!(f, "IntroduceFreshVariable"),
            Tactic::ProvideWitness { .. } => write!(f, "ProvideWitness"),
            Tactic::ReorderQuantifiers { .. } => write!(f, "ReorderQuantifiers"),
            Tactic::UniversalCaseAnalysis { .. } => write!(f, "UniversalCaseAnalysis"),
            Tactic::ExactWith { .. } => write!(f, "ExactWith"),
            Tactic::Rewrite { .. } => write!(f, "Rewrite"),
            Tactic::AssumeImplicationAntecedent { .. } => write!(f, "AssumeImplicationAntecedent"),
            Tactic::SplitConjunction => write!(f, "SplitConjunction"),
            Tactic::SplitDisjunction { .. } => write!(f, "SplitDisjunction"),
            Tactic::StatementCaseAnalysis { .. } => write!(f, "StatementCaseAnalysis"),
            Tactic::Simplify { .. } => write!(f, "Simplify"),
            Tactic::Auto(_) => write!(f, "Auto"),
            Tactic::Induction { .. } => write!(f, "Induction"),
        }
    }
}
