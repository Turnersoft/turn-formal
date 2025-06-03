we are creating a formal proof language for math(not working with type theory directly) with tactics. 
It is foundation-agnostic.
The building block of this language is not type or term, it is math definitions and math connectives as well as logic connectives. Note, they are all non-meta! Meaning they can all be converted to a single formalism like topos theory, cubical type theory, category theory, etc. 
The reason of this design is that we are starting not from the ground up, but from the top level or perhapse the most superficial level of math, that is math expressed in natural language. The design start with some rust struct and enum that covered only the possible chain of properties and properties modifier on any given math object. 
This way, we can check whether a given math object configuration is used or not. We can in theory achieve pattern matching on any math object configuration. This may be the most efficient way to tie theorem and math types (type means object with properties) together. Therefore we can achieve absolute coverability of theorems the search tactics.
No matter how complext a math proposition is, it is always a tree structure, therefore we can loop through each node to search for theorems that can be applied, and try all possible combination of tactics to proof the theorems.
To proof a arbitrary math proposition is like finding integral, where risch algorithm is used to try many steps heuristically.

So the question is, how to integrate these math definitions into the proof language? We should first convert the object&property definition into string. things like "locally.compact,globally.connected.topological_space". then put it in a proposition with logical connectives like "and", "or", "not", "implies", "iff". For example, "locally.compact,globally.connected.topological_space and (P or Q) -> R" is a proposition.

Then we can use the proposition to search for theorems that can be applied.

The tactics are like the rules of a game, they are applied to the proposition to change it.

# efficient theorem search
once we have some "proven" theorems, we can search on them. But how can we manage the theorems? Effectively, we are searching for similar pattern in many tree structure when we looping through the current tree. what is the most efficient way of achieveing this?


remember, unlike lean where definitions needs to be constructed formally before use, our system provide all possible definition configuration before hand!. So that our theorems can use these definitions directly.

So we will have different classes of theorems.
1. simplest: equivalence of math object configuration: metric_space{compact} <=> metric_space{complete, totally_bounded}, 
2. math equality: sin(x)^2 + cos(x)^2 = 1
3. theorem containing complex connectives and math conditions

The problem is type-checking possible definitions(object with only correct properties and construction) (this will greatly help non-mathematician developer construct theorems).
But most importantly, we need theorems to be matchable to user-input math expressions and tactics.

## Comparison with Lean
### Lean's tactic system has several key features:
- Metaprogramming: Lean allows defining new tactics using its metaprogramming framework
- Elaboration: Lean has a powerful elaboration system for type inference
- Contextual Automation: Lean tactics can inspect and use the local context
- Structured Proofs: Lean supports both tactic-based and term-based proofs
- 
### Your system can adopt these strengths while maintaining your higher-level approach:
- Use Rust's trait system instead of metaprogramming for extending tactics
- Leverage your existing type system for elaboration
- Maintain rich proof contexts that tactics can inspect
- Support both low-level tactics and high-level theorem application