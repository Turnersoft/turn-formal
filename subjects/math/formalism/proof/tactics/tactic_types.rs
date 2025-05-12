use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::subjects::math::formalism::expressions::{Identifier, MathExpression};
use crate::subjects::math::formalism::interpretation::TypeViewOperator;

/// A tactic that can be applied to a proof state to transform it
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tactic {
    /// Introduce a new variable or hypothesis
    Intro {
        /// Name to give the introduced variable
        name: Identifier,
        /// Expression for the variable
        expression: MathExpression,
        /// Optional view operator
        view: Option<TypeViewOperator>,
    },
    /// Apply a hypothesis or theorem to the goal
    Apply {
        /// ID of the theorem or hypothesis to apply
        theorem_id: String,
        /// Variable instantiations
        instantiation: HashMap<String, MathExpression>,
        /// Target expression to apply to (if any)
        target_expr: Option<MathExpression>,
    },
    /// Substitute an expression for another in a proof
    Substitution {
        /// The expression to substitute
        target: MathExpression,
        /// What to replace it with
        replacement: MathExpression,
        /// Location within the expression
        location: Option<Vec<usize>>,
    },
    /// Change the view of a mathematical object
    ChangeView {
        /// The object to change view of
        object: MathExpression,
        /// The new view to use
        view: TypeViewOperator,
    },
    /// Application of a previously proven theorem
    TheoremApplication {
        /// ID of the theorem to apply
        theorem_id: String,
        /// Variable instantiations for the theorem parameters
        instantiation: HashMap<Identifier, MathExpression>,
        /// Target expression within the goal to focus the application on
        target_expr: Option<MathExpression>,
    },
    /// Decompose an expression into its components
    Decompose {
        /// The target expression to decompose
        target: MathExpression,
        /// Method of decomposition
        method: DecompositionMethod,
    },
    /// Simplify an expression
    Simplify {
        /// The target expression to simplify
        target: MathExpression,
        /// Optional hints for simplification
        hints: Option<Vec<String>>,
    },
    /// Apply mathematical induction
    Induction {
        /// Variable to induct on
        name: Identifier,
        /// Type of induction to apply
        induction_type: InductionType,
        /// Optional induction schema
        schema: Option<MathExpression>,
    },
    /// Custom tactic defined by the user
    Custom {
        /// Name of the custom tactic
        name: String,
        /// Arguments for the custom tactic
        args: Vec<String>,
    },
    /// Case analysis on an expression
    CaseAnalysis {
        /// Target expression for case analysis
        target_expr: MathExpression,
        /// Expressions for each case
        case_exprs: Vec<MathExpression>,
        /// Names for each case
        case_names: Vec<String>,
    },
    /// Rewrite using an equation
    Rewrite {
        /// Target expression to rewrite
        target_expr: MathExpression,
        /// Equation to use for rewriting
        equation_expr: MathExpression,
        /// Direction of rewriting
        direction: RewriteDirection,
        /// Location within the expression
        location: Option<Vec<usize>>,
    },
    /// Branch to create a new proof path
    Branch {
        /// Description of the new proof path
        description: String,
    },
    /// Case analysis case
    Case {
        /// Parent case node ID
        parent_id: String,
        /// Case expression
        case_expr: MathExpression,
        /// Case name
        case_name: String,
    },
}

/// Direction for rewriting expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RewriteDirection {
    /// Left to right (replace left side with right side)
    LeftToRight,
    /// Right to left (replace right side with left side)
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
