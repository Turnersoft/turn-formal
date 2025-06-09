# Tactics Strategy System Documentation

## Overview

This document describes the comprehensive search and replace strategy system implemented for the mathematical proof tactics in the turn-formal Rust system. The new design organizes all proof tactics into logical strategy groups based on their replacement behavior patterns.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Strategy Types](#strategy-types)
3. [Core Traits](#core-traits)
4. [API Reference](#api-reference)
5. [Usage Examples](#usage-examples)
6. [Migration Guide](#migration-guide)
7. [Testing](#testing)
8. [Design Decisions](#design-decisions)

## Architecture Overview

The new system replaces the previous ad-hoc search and replace logic with a systematic, trait-based design that cleanly separates concerns:

```
Tactic Enum → Strategy Factory → Unified Strategy → Core Traits → Search/Replace Engine
```

### Key Components

- **Tactic Enum**: Original proof tactics (unchanged interface)
- **Strategy Types**: 6 organized strategy patterns covering all tactics
- **Core Traits**: `TargetMatcher`, `ReplacementTransformer`, `ResultProcessor`
- **Unified Applier**: Single entry point for strategy application
- **Search/Replace Engine**: Low-level expression manipulation

### Files Modified/Created

- `subjects/math/formalism/proof/tactics/replacement_strategy.rs` (NEW - 941 lines)
- `subjects/math/formalism/proof/tactics/unified_applier.rs` (NEW - 287 lines)
- `subjects/math/formalism/proof/tactics/search_replace.rs` (ENHANCED)
- `subjects/math/formalism/proof/tactics/mod.rs` (UPDATED)

## Strategy Types

### 1. Direct Replacement Strategy

**Used by**: `Intro`, `Substitution`

**Pattern**: Replace subexpression with something else

```rust
pub struct DirectReplacementStrategy {
    pub target_pattern: MathExpression,
    pub replacement: MathExpression,
    pub replacement_type: DirectReplacementType,
    pub location: Option<Vec<usize>>,
}

pub enum DirectReplacementType {
    WithIdentifier(Identifier),  // Intro: add variable binding
    WithExpression,              // Substitution: direct replacement
}
```

**Behavior**:
- `Intro`: Replaces target with identifier + adds variable binding to goal
- `Substitution`: Replaces target with expression (optionally at specific location)

### 2. Theorem-Based Strategy

**Used by**: `Apply`, `TheoremApplication`, `Rewrite`

**Pattern**: Apply theorem/equation to transform expressions

```rust
pub struct TheoremBasedStrategy {
    pub theorem_id: Option<String>,
    pub equation: Option<MathExpression>,
    pub target_pattern: Option<MathExpression>,
    pub direction: Option<RewriteDirection>,
    pub require_exact_match: bool,
    pub instantiations: HashMap<Identifier, MathExpression>,
}
```

**Behavior**:
- `Apply`/`TheoremApplication`: Look up theorem and apply with type matching
- `Rewrite`: Apply equation with specified direction (left-to-right or right-to-left)

### 3. View Transformation Strategy

**Used by**: `ChangeView`

**Pattern**: Change interpretation of mathematical object

```rust
pub struct ViewTransformationStrategy {
    pub target_object: MathExpression,
    pub new_view: TypeViewOperator,
}
```

**Behavior**: Wraps target object with `MathExpression::ViewAs { expression, view }`

### 4. Multi-Goal Strategy

**Used by**: `Decompose`, `CaseAnalysis`, `Induction`

**Pattern**: Break single goal into multiple subgoals

```rust
pub struct MultiGoalStrategy {
    pub decomposition_type: MultiGoalType,
    pub target_expression: Option<MathExpression>,
}

pub enum MultiGoalType {
    LogicalDecomposition { method: DecompositionMethod },
    CaseAnalysis { cases: Vec<(String, MathExpression)> },
    Induction { variable: Identifier, induction_type: InductionType },
}
```

**Behavior**:
- `Decompose`: Break conjunctions/disjunctions into separate goals
- `CaseAnalysis`: Create multiple goals with case assumptions
- `Induction`: Generate base case + inductive step goals

### 5. Simplification Strategy

**Used by**: `Simplify`

**Pattern**: Apply simplification rules to expressions

```rust
pub struct SimplificationStrategy {
    pub target_expression: MathExpression,
    pub hints: Option<Vec<String>>,
    pub simplification_rules: Vec<SimplificationRule>,
}

pub struct SimplificationRule {
    pub pattern: MathExpression,
    pub replacement: MathExpression,
    pub condition: Option<MathExpression>,
}
```

**Behavior**: Applies sequence of pattern → replacement transformations

### 6. Custom Strategy

**Used by**: `Custom`

**Pattern**: User-defined behavior

```rust
pub struct CustomStrategy {
    pub name: String,
    pub args: Vec<String>,
    pub behavior: CustomBehavior,
}

pub enum CustomBehavior {
    DelegateToStrategy(Box<TacticStrategy>),
    CustomTransform,
    NoOp,
}
```

**Behavior**: Flexible dispatch to other strategies or custom logic

## Core Traits

### TargetMatcher

Defines how to find applicable targets for each strategy:

```rust
pub trait TargetMatcher {
    fn exact_match(&self, expression: &MathExpression) -> bool;
    fn type_match(&self, expression: &MathExpression) -> bool;
    fn pattern_match(&self, expression: &MathExpression, pattern: &MathExpression) 
        -> Option<HashMap<Identifier, MathExpression>>;
    fn decomposable(&self, expression: &MathExpression) -> bool;
    fn simplifiable(&self, expression: &MathExpression) -> bool;
}
```

### ReplacementTransformer

Defines how to transform matched targets:

```rust
pub trait ReplacementTransformer {
    fn transform(&self, target: &MathExpression, context: &ReplacementContext) 
        -> Result<MathExpression, String>;
    fn transform_relation(&self, target: &MathRelation, context: &ReplacementContext) 
        -> Result<MathRelation, String>;
}
```

### ResultProcessor

Defines how to process transformation results into final goals:

```rust
pub trait ResultProcessor {
    fn process_result(&self, transformed: MathExpression, context: &ReplacementContext) 
        -> TacticApplicationResult;
    fn process_relation_result(&self, transformed: MathRelation, context: &ReplacementContext) 
        -> TacticApplicationResult;
}
```

### TacticSearchReplace

Unified interface combining all behaviors:

```rust
pub trait TacticSearchReplace: TargetMatcher + ReplacementTransformer + ResultProcessor {
    fn find_targets(&self, goal: &ProofGoal) -> Vec<SearchResult>;
    fn apply_tactic(&self, goal: &ProofGoal) -> TacticApplicationResult;
    fn get_search_pattern(&self) -> MathExpression;
}
```

## API Reference

### UnifiedTacticApplier

Main entry point for strategy application:

```rust
impl UnifiedTacticApplier {
    pub fn apply_tactic(tactic: &Tactic, goal: &ProofGoal) -> TacticApplicationResult;
    pub fn apply_strategy(strategy: TacticStrategy, goal: &ProofGoal) -> TacticApplicationResult;
    pub fn find_applicable_targets(tactic: &Tactic, goal: &ProofGoal) -> Vec<SearchResult>;
    pub fn is_applicable(tactic: &Tactic, goal: &ProofGoal) -> bool;
    pub fn get_applicable_tactics(goal: &ProofGoal, available_tactics: &[Tactic]) -> Vec<Tactic>;
}
```

### ProofGoalStrategyExt

Extension trait for convenient usage:

```rust
pub trait ProofGoalStrategyExt {
    fn apply_tactic_strategic(&self, tactic: &Tactic) -> TacticApplicationResult;
    fn find_applicable_tactics(&self, available_tactics: &[Tactic]) -> Vec<Tactic>;
    fn can_apply_tactic(&self, tactic: &Tactic) -> bool;
}
```

### TacticApplicationResult

Result type for tactic applications:

```rust
pub enum TacticApplicationResult {
    SingleGoal(ProofGoal),
    MultipleGoals(Vec<ProofGoal>),
    NoChange,
    Error(String),
}
```

### Factory Function

```rust
pub fn create_strategy_from_tactic(tactic: &Tactic) -> Option<TacticStrategy>;
```

## Usage Examples

### Basic Tactic Application

```rust
use crate::subjects::math::formalism::proof::tactics::{
    UnifiedTacticApplier, Tactic, ProofGoalStrategyExt
};

// Method 1: Direct application
let result = UnifiedTacticApplier::apply_tactic(&tactic, &goal);

// Method 2: Extension trait (more convenient)
let result = goal.apply_tactic_strategic(&tactic);

match result {
    TacticApplicationResult::SingleGoal(new_goal) => {
        println!("Tactic succeeded: {}", new_goal.format());
    }
    TacticApplicationResult::MultipleGoals(goals) => {
        println!("Created {} subgoals", goals.len());
    }
    TacticApplicationResult::NoChange => {
        println!("Tactic not applicable");
    }
    TacticApplicationResult::Error(msg) => {
        println!("Error: {}", msg);
    }
}
```

### Finding Applicable Tactics

```rust
let available_tactics = vec![
    Tactic::Intro { /* ... */ },
    Tactic::Substitution { /* ... */ },
    Tactic::ChangeView { /* ... */ },
];

// Find which tactics can be applied to current goal
let applicable = goal.find_applicable_tactics(&available_tactics);
println!("Can apply {} out of {} tactics", applicable.len(), available_tactics.len());

// Check individual tactic
if goal.can_apply_tactic(&some_tactic) {
    let result = goal.apply_tactic_strategic(&some_tactic);
    // ...
}
```

### Strategy Creation and Application

```rust
// Create strategy from tactic
let strategy = create_strategy_from_tactic(&tactic).unwrap();

// Apply strategy directly
let result = UnifiedTacticApplier::apply_strategy(strategy, &goal);

// Find targets for strategy
let targets = match create_strategy_from_tactic(&tactic) {
    Some(strategy) => UnifiedTacticApplier::find_strategy_targets(strategy, &goal),
    None => Vec::new(),
};
```

### Intro Tactic Example

```rust
let intro_tactic = Tactic::Intro {
    name: Identifier::Name("t".to_string(), 0),
    expression: MathExpression::Var(Identifier::Name("x".to_string(), 0)),
    view: None,
};

let result = goal.apply_tactic_strategic(&intro_tactic);
// Result: x replaced with t, variable binding t = x added to goal
```

### Case Analysis Example

```rust
let case_analysis = Tactic::CaseAnalysis {
    target_expr: some_expression,
    case_exprs: vec![case1_expr, case2_expr],
    case_names: vec!["Case 1".to_string(), "Case 2".to_string()],
};

let result = goal.apply_tactic_strategic(&case_analysis);
// Result: Multiple goals, each with case assumption added
```

## Migration Guide

### From Old System

**Before** (scattered search/replace logic):
```rust
// Direct manipulation
let new_relation = SearchReplace::replace_all_in_relation(&relation, &pattern, &replacement);
let new_goal = ProofGoal { statement: new_relation, ..goal };
```

**After** (strategic approach):
```rust
// Strategic application
let substitution = Tactic::Substitution { target: pattern, replacement, location: None };
let result = goal.apply_tactic_strategic(&substitution);
```

### Key Changes

1. **Centralized Logic**: All replacement logic now goes through strategy system
2. **Type Safety**: Strategy dispatch ensures correct behavior for each tactic type
3. **Consistent Interface**: All tactics use same application pattern
4. **Better Testing**: Strategy-level testing enables comprehensive coverage

### Backward Compatibility

- Original `Tactic` enum unchanged - no breaking changes to existing code
- `SearchReplace` module still available for low-level operations
- Legacy functions preserved where possible

## Testing

### Strategy-Level Tests

Each strategy type has comprehensive tests:

```rust
#[test]
fn test_direct_replacement_strategy() {
    let strategy = DirectReplacementStrategy { /* ... */ };
    let result = strategy.apply_tactic(&goal);
    // Verify behavior...
}
```

### Integration Tests

Full tactic application tests:

```rust
#[test]
fn test_all_strategy_types_covered() {
    // Verify every tactic can create a strategy
    for tactic in all_tactics {
        let strategy = create_strategy_from_tactic(&tactic);
        assert!(strategy.is_some());
    }
}
```

### Test Coverage

- ✅ All 11 tactic types can create strategies
- ✅ Strategy application produces expected results
- ✅ Target finding works correctly
- ✅ Error handling for invalid applications
- ✅ Multi-goal strategies create appropriate subgoals

## Design Decisions

### Why Strategy Pattern?

1. **Separation of Concerns**: Each strategy handles one type of replacement behavior
2. **Extensibility**: Easy to add new tactics or modify existing behavior
3. **Type Safety**: Rust's type system ensures correct strategy dispatch
4. **Testability**: Strategy-level testing provides better coverage

### Strategy Organization Rationale

- **Direct Replacement**: Simple find-and-replace operations
- **Theorem-Based**: Complex transformations requiring theorem lookup/matching
- **View Transformation**: Type system operations (wrapping/unwrapping)
- **Multi-Goal**: Operations that naturally create multiple proof branches
- **Simplification**: Rule-based transformations
- **Custom**: Escape hatch for user-defined behavior

### Trait Design

- **TargetMatcher**: Separates "what to find" logic
- **ReplacementTransformer**: Separates "how to transform" logic  
- **ResultProcessor**: Separates "how to handle results" logic
- **TacticSearchReplace**: Provides unified interface

### Performance Considerations

- Strategy creation is lightweight (mostly data structure construction)
- Search operations use existing efficient `SearchReplace` engine
- No runtime overhead compared to direct approach
- Strategy dispatch is compile-time resolved

### Future Extensions

The design supports easy addition of:
- New strategy types for emerging tactic patterns
- Enhanced pattern matching (unification, higher-order matching)
- Parallel strategy application
- Strategy composition and pipelining
- Custom search algorithms per strategy type

## Troubleshooting

### Common Issues

1. **"No strategy available for tactic"**: 
   - Ensure `create_strategy_from_tactic` handles your tactic variant
   - Check that all required fields are provided

2. **"Target not found"**:
   - Verify search pattern matches expression structure
   - Check if target matcher implementation is appropriate

3. **"Transformation failed"**:
   - Review transformer logic for edge cases
   - Ensure replacement expression is valid

### Debug Tips

```rust
// Check if tactic can create strategy
if let Some(strategy) = create_strategy_from_tactic(&tactic) {
    // Check if targets can be found
    let targets = strategy.find_targets(&goal);
    if targets.is_empty() {
        println!("No targets found for strategy");
    }
} else {
    println!("No strategy available for tactic: {:?}", tactic);
}
```

---

*Generated for turn-formal mathematical proof system*  
*Last updated: 2024* 