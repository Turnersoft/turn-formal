# Tactics Strategy System - Quick Reference

## Strategy Mapping

| Tactic | Strategy Type | Key Behavior |
|--------|---------------|--------------|
| `Intro` | DirectReplacement | Replace target with identifier + add variable binding |
| `Substitution` | DirectReplacement | Replace target with expression |
| `Apply` | TheoremBased | Apply theorem with type matching |
| `TheoremApplication` | TheoremBased | Apply theorem with pattern matching |
| `Rewrite` | TheoremBased | Apply equation with direction |
| `ChangeView` | ViewTransformation | Wrap with `ViewAs` |
| `Decompose` | MultiGoal | Break conjunction/disjunction into subgoals |
| `CaseAnalysis` | MultiGoal | Create case-specific subgoals |
| `Induction` | MultiGoal | Generate base case + inductive step |
| `Simplify` | Simplification | Apply simplification rules |
| `Custom` | Custom | User-defined behavior |

## Quick API Usage

### Apply Tactic (Recommended)
```rust
let result = goal.apply_tactic_strategic(&tactic);
```

### Check Applicability
```rust
if goal.can_apply_tactic(&tactic) {
    let result = goal.apply_tactic_strategic(&tactic);
}
```

### Find Applicable Tactics
```rust
let applicable = goal.find_applicable_tactics(&available_tactics);
```

### Handle Results
```rust
match result {
    TacticApplicationResult::SingleGoal(new_goal) => { /* ... */ }
    TacticApplicationResult::MultipleGoals(goals) => { /* ... */ }
    TacticApplicationResult::NoChange => { /* ... */ }
    TacticApplicationResult::Error(msg) => { /* ... */ }
}
```

## Strategy Pattern Overview

```
┌─────────────┐    ┌──────────────────┐    ┌─────────────────┐
│    Tactic   │───▶│ Strategy Factory │───▶│ Strategy Object │
└─────────────┘    └──────────────────┘    └─────────────────┘
                                                    │
                                                    ▼
                   ┌─────────────────────────────────────────────┐
                   │            Core Traits                      │
                   │  ┌─────────────────────────────────────┐    │
                   │  │ TargetMatcher                       │    │
                   │  │ ReplacementTransformer              │    │
                   │  │ ResultProcessor                     │    │
                   │  │ TacticSearchReplace                 │    │
                   │  └─────────────────────────────────────┘    │
                   └─────────────────────────────────────────────┘
                                                    │
                                                    ▼
                           ┌─────────────────────────────┐
                           │    TacticApplicationResult  │
                           └─────────────────────────────┘
```

## Key Files

- `replacement_strategy.rs` - Strategy definitions and implementations
- `unified_applier.rs` - Main API and convenience methods  
- `search_replace.rs` - Low-level search/replace engine
- `mod.rs` - Public exports and tactic enum

## Common Patterns

### Create Custom Strategy
```rust
let custom_strategy = CustomStrategy {
    name: "my_strategy".to_string(),
    args: vec!["arg1".to_string()],
    behavior: CustomBehavior::NoOp,
};
```

### Batch Apply Tactics
```rust
for tactic in tactics {
    match goal.apply_tactic_strategic(&tactic) {
        TacticApplicationResult::SingleGoal(new_goal) => {
            goal = new_goal;
            break;
        }
        _ => continue,
    }
}
```

### Debug Strategy Creation
```rust
if let Some(strategy) = create_strategy_from_tactic(&tactic) {
    println!("Strategy created: {:?}", strategy);
} else {
    println!("No strategy for tactic: {:?}", tactic);
}
```

---

*Quick reference for turn-formal tactics strategy system* 