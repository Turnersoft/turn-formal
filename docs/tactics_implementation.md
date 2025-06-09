# Tactics Implementation - Clean Design

## Overview

This document describes the final, simplified implementation of the tactics search and replace system. The design eliminates unnecessary wrapper types and implements helper traits directly on the original `Tactic` enum.

## Design Principles

### 1. No Strategy Wrappers
- Removed all strategy wrapper types (`DirectReplacementStrategy`, etc.)
- Implement traits directly on the original `Tactic` enum
- Eliminates duplication and simplifies the codebase

### 2. Simple Trait-Based Design
```rust
/// Find targets in goals
pub trait TacticMatcher {
    fn find_targets(&self, goal: &ProofGoal) -> Vec<SearchResult>;
    fn is_applicable(&self, goal: &ProofGoal) -> bool;
    fn get_search_pattern(&self) -> MathExpression;
}

/// Apply transformations to goals
pub trait TacticApplier {
    fn apply_to_goal(&self, goal: &ProofGoal) -> TacticApplicationResult;
}

/// Combined functionality
pub trait TacticSearchReplace: TacticMatcher + TacticApplier {}
```

### 3. Direct Implementation on Tactic
```rust
impl TacticMatcher for Tactic {
    fn get_search_pattern(&self) -> MathExpression {
        match self {
            Tactic::Intro { expression, .. } => expression.clone(),
            Tactic::Substitution { target, .. } => target.clone(),
            // ... handle all tactic variants
        }
    }
}

impl TacticApplier for Tactic {
    fn apply_to_goal(&self, goal: &ProofGoal) -> TacticApplicationResult {
        match self {
            Tactic::Intro { name, expression, .. } => {
                // Direct implementation for Intro
            }
            Tactic::Substitution { target, replacement, location } => {
                // Direct implementation for Substitution
            }
            // ... handle all tactic variants
        }
    }
}
```

## Tactic Implementations

### 1. Intro Tactic
**Behavior**: Replace target expression with identifier + add variable binding

```rust
Tactic::Intro { name, expression, .. } => {
    let new_relation = SearchReplace::replace_all_in_relation(
        &goal.statement,
        expression,
        &MathExpression::Var(name.clone()),
    );
    
    let mut new_goal = goal.clone();
    new_goal.statement = new_relation;
    new_goal.value_variables.push(ValueBindedVariable {
        name: name.clone(),
        value: expression.clone(),
    });
    
    TacticApplicationResult::SingleGoal(new_goal)
}
```

### 2. Substitution Tactic
**Behavior**: Replace target with replacement expression

```rust
Tactic::Substitution { target, replacement, location } => {
    let new_relation = if let Some(location) = location {
        SearchReplace::replace_at_path_in_relation(&goal.statement, location, replacement)
    } else {
        SearchReplace::replace_all_in_relation(&goal.statement, target, replacement)
    };
    
    let mut new_goal = goal.clone();
    new_goal.statement = new_relation;
    TacticApplicationResult::SingleGoal(new_goal)
}
```

### 3. Case Analysis Tactic
**Behavior**: Create multiple goals with case assumptions

```rust
Tactic::CaseAnalysis { case_exprs, case_names, .. } => {
    let goals: Vec<ProofGoal> = case_names
        .iter()
        .zip(case_exprs.iter())
        .map(|(name, expr)| {
            let mut goal = goal.clone();
            goal.statement = MathRelation::And(vec![
                MathRelation::equal(expr.clone(), /* case condition */),
                goal.statement,
            ]);
            goal
        })
        .collect();
    TacticApplicationResult::MultipleGoals(goals)
}
```

### 4. Other Tactics
- **Apply/TheoremApplication/Rewrite**: TODO - theorem application logic
- **ChangeView**: Wrap expression with `ViewAs`
- **Decompose**: Break conjunctions/disjunctions into separate goals
- **Induction**: Create base case + inductive step goals
- **Simplify**: Apply simplification rules
- **Custom**: No-op by default

## API Usage

### Basic Application
```rust
use crate::subjects::math::formalism::proof::tactics::{TacticApplier, TacticMatcher};

// Direct trait usage
let targets = tactic.find_targets(&goal);
let result = tactic.apply_to_goal(&goal);
```

### Unified Applier
```rust
use crate::subjects::math::formalism::proof::tactics::UnifiedTacticApplier;

// Convenient wrapper methods
let result = UnifiedTacticApplier::apply_tactic(&tactic, &goal);
let is_applicable = UnifiedTacticApplier::is_applicable(&tactic, &goal);
```

### Extension Trait
```rust
use crate::subjects::math::formalism::proof::tactics::ProofGoalStrategyExt;

// Most convenient usage
let result = goal.apply_tactic_strategic(&tactic);
let applicable = goal.find_applicable_tactics(&available_tactics);
let can_apply = goal.can_apply_tactic(&tactic);
```

## Files

### Core Implementation
- `replacement_strategy.rs`: Trait definitions and `Tactic` implementations
- `unified_applier.rs`: Convenient wrapper methods and extension traits
- `search_replace.rs`: Low-level search/replace operations
- `mod.rs`: Public exports and `Tactic` enum definition

### Key Exports
```rust
pub use replacement_strategy::{
    ReplacementContext, TacticApplicationResult, TacticApplier, TacticMatcher, TacticSearchReplace,
};
```

## Benefits of This Design

### 1. Simplicity
- No intermediate strategy wrapper types
- Direct implementation on existing `Tactic` enum
- Clear separation of concerns with simple traits

### 2. Performance
- No strategy creation overhead
- Direct method dispatch
- Minimal memory allocation

### 3. Maintainability
- Single source of truth for tactic behavior
- Easy to add new tactics (just extend the match arms)
- Clear, readable implementation

### 4. Usability
- Multiple API levels (direct traits, unified applier, extension traits)
- Backward compatible with existing `Tactic` enum
- Consistent interface across all tactics

## Testing

All tactics have comprehensive test coverage:
- ✅ Search pattern generation
- ✅ Target finding
- ✅ Goal transformation
- ✅ Single goal tactics (Intro, Substitution, ChangeView, Simplify)
- ✅ Multi-goal tactics (CaseAnalysis, Decompose, Induction)
- ✅ Extension trait convenience methods
- ✅ Unified applier wrapper methods

## Future Extensions

The design supports easy addition of:
- New tactic variants (extend the match arms)
- Enhanced theorem application logic
- Custom simplification rules
- Parallel tactic application
- Tactic composition and pipelining

---

*Documentation for turn-formal mathematical proof tactics implementation*  
*Clean, trait-based design without strategy wrappers* 