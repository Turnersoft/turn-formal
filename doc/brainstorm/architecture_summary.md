# Architecture Summary: A Type-Driven Approach with Semantic Elaboration

This document outlines the core architectural principles for the Turn Formal mathematical kernel. Our approach leverages Rust's strong static type system for syntactic correctness while delegating deep, value-dependent semantic checks to a dedicated "elaboration" phase. This creates a system that is robust, user-friendly, and true to the nature of mathematical proof.

## Core Philosophy: Syntax vs. Semantics

1.  **Rust Types Model Syntax:** The primary role of our `struct`s and `enum`s is to represent the *syntactic structure* of mathematics in a well-formed way. The type system should make syntactically invalid states (e.g., a group operation with three arguments) unrepresentable.

2.  **Elaboration Models Semantics:** Deep, value-dependent properties (e.g., function domains like `x ∈ [-1, 1]` for `arcsin(x)`, or matrix dimension equality for multiplication) are not encoded directly in the static types. Instead, they are treated as **proof obligations** (preconditions) that are automatically checked and managed by a semantic analysis phase called **elaboration**.

3.  **Singular Representation:** A given mathematical concept must have one, and only one, representation within the top-level AST. This prevents ambiguity and simplifies pattern matching for tactics and rendering logic.

4.  **`Located<T>` Wrapper:** All significant AST nodes are wrapped in `Located<T>` to trace their origin (file, line, column), enabling precise error reporting and analysis.

## The `X` vs. `XExpr` Pattern: A Unified Hierarchy

Our fundamental pattern is the distinction between a mathematical "noun" and the "verb" that produces it.

*   **`X` (e.g., `GroupElement`):** An "Object" type. This is a terminal value, representing a concrete mathematical object. Examples: `Integer(5)`, `Symbol("g")`.

*   **`XExpr` (e.g., `GroupElementExpression`):** An "Expression" type. This is a recursive `enum` representing any constructive process that is guaranteed to *evaluate to* an object of type `X`.

Crucially, to maintain a single, unified hierarchy, these two are linked in a specific way:
1.  **`XExpr` must encapsulate `X`:** Every `XExpr` enum *must* have a variant that wraps a concrete `X` object (e.g., `GroupElementExpression::Element(GroupElement)`). This serves as the base case for the recursion.
2.  **`MathExpression` only contains `XExpr`:** The global `MathExpression` enum, which unifies all theories, should **only** contain the `...Expr` variants from each theory. It should *not* contain raw `X` variants. This enforces our **Singular Representation** principle, preventing a value like `g` from being represented as both `MathExpression::GroupElement(...)` and `MathExpression::GroupElementExpression(...)`.

This ensures that any part of the system can operate on a `MathExpression` without needing to handle dual representations for the same concept.

## Abstraction & Generalization: The Trait-Based Approach

A core design challenge is how to "infinitely drill down" into a concept—starting with a specific usage (e.g., `<` on numbers) and later revealing its deeper, more general nature (e.g., as a `PartialOrder`) without breaking the original usage.

### The Goal: Ergonomics and Abstraction

We need a system that supports both:
1.  **Ergonomic Usage:** Stating simple facts like `3 < 5` should be direct and intuitive (`LessThan(3, 5)`).
2.  **Abstract Reasoning:** We must be able to define and reason about abstract structures like a `PartialOrder` as a "noun" (`struct PartialOrder { set: Set, relation: OrderRelation }`) and prove general theorems about them (e.g., Transitivity).

### Rejected Designs and Their Critical Flaws

1.  **Enum as Collection:** A `PartialOrder` enum with variants like `NumberLessThan(...)` and `SetSubset(...)`.
    *   **Critical Flaw:** This is a collection, not an abstraction. It's impossible to write a general theorem (like Transitivity) that operates on `PartialOrder`, as there is no single, generic variant to target. You'd have to write a separate theorem for each variant.

2.  **Enum with Generic Variant:** An enum with a catch-all `GenericOrder { left: Box<dyn Orderable>, ... }` variant.
    *   **Critical Flaw:** This creates an "ugly seam" between the specific and generic worlds. It requires extensive boilerplate to convert between variants, incurs runtime costs from dynamic dispatch, and re-introduces the duality problem where the same concept can be represented in two different ways.

### The Chosen Architecture: Separating Structure from Semantics

The optimal solution is to decouple the syntactic structure of a relation from its semantic validity using traits.

1.  **Structure (The Generic "Noun"/"Verb"):** A single, unified `struct` or `enum` represents the concept. For example, `struct LessThan { left: MathExpression, right: MathExpression }`. This structure is generic and can hold any expression. This is what general theorems are written to operate on.

2.  **Semantics (The Trait as a "Contract"):** A `trait` (e.g., `Orderable`) defines the semantic contract. It is implemented for types where the relation is meaningful (`Number`, `Set`) and not implemented for others (`Group`).

3.  **Validation (The Elaboration Layer):** The elaboration layer acts as the semantic checker. When it encounters a `LessThan` struct, it analyzes the inner expressions. It then checks if the *types* of those expressions (e.g., `Number`) implement the required `Orderable` trait. If they don't, it raises a proof obligation or a semantic error.

This architecture provides the best of all worlds:
*   **Ergonomic:** Simple relations remain simple to write.
*   **Abstractable:** You can define structs like `PartialOrder` that contain the generic `LessThan` structure, allowing you to reason about them as objects.
*   **Safe:** The trait system, enforced by the elaboration layer, provides robust, compile-time-like semantic safety without making the core AST structures rigid and specific.
*   **Evolvable:** You can introduce new `Orderable` types in the future without changing the `LessThan` struct or any of the general theorems that operate on it.
