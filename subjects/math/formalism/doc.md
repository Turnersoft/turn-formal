## tactics

how to handle order of tactics, branching and then handling then combining, can they be separated or normalized?
how to apply a theorem to a branch that are already in processed by with different stages?
how to handle context in proof branches and cases?


## fully model properties in a theory
I think we can just add propertyVariant to these variant in GroupRelation. So formal mathematics is about complete modelling. So where still needs property and structure in@definitions.rs ? Adjective exist on object, adverbs exist on morphisms(which are relation, operators in group theory). Do I miss any higher order properties?


Looking at your codebase, I agree that we should add PropertyVariant enums to the GroupRelation variants like NormalizesSubgroup and CentralizesSubgroup. This would more completely model the formal mathematical language.

Here's what's still needed in definitions.rs:

1. **Adverbial modifiers for relations**:
```rust
/// Types of normalization properties
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NormalizationVariant {
    /// Standard normalization
    Standard,
    /// Local normalization (in a neighborhood)
    Local,
    /// Virtual normalization (up to finite index)
    Virtual,
    /// Strong normalization (with additional constraints)
    Strong,
    /// Weak normalization (with relaxed conditions)
    Weak,
}

/// Types of centralization properties
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CentralizationVariant {
    /// Standard centralization
    Standard,
    /// Local centralization
    Local,
    /// Virtual centralization
    Virtual,
    /// Strong centralization
    Strong,
    /// Weak centralization
    Weak,
}
```

2. **Update relation variants**:
```rust
/// An element normalizes a subgroup
NormalizesSubgroup {
    element: GroupExpression,
    subgroup: Box<Group>,
    group: Box<Group>,
    variant: NormalizationVariant,
},

/// An element centralizes a subgroup
CentralizesSubgroup {
    element: GroupExpression,
    subgroup: Box<Group>,
    group: Box<Group>,
    variant: CentralizationVariant,
},
```

3. **Higher-order properties missing**:

- **Meta-properties**: Properties about properties (e.g., "compatibility" between different properties)
```rust
pub enum MetaPropertyRelation {
    Compatible(Box<GroupProperty>, Box<GroupProperty>),
    Implies(Box<GroupProperty>, Box<GroupProperty>),
    Orthogonal(Box<GroupProperty>, Box<GroupProperty>),
}
```

- **Transformation properties**: How properties transform under morphisms
```rust
pub enum PropertyTransformationBehavior {
    Preserved,
    Reversed,
    Strengthened,
    Weakened,
    Destroyed
}
```

- **Natural equivalences**: Between different characterizations
```rust
pub enum CharacterizationEquivalence {
    EquivalentDefinitions(Vec<GroupProperty>),
    EquivalentInContext { properties: Vec<GroupProperty>, context: GroupProperty }
}
```

The formal mathematical structure follows category theory principles:
- Objects (nouns) have properties (adjectives)
- Morphisms (verbs/relations) have adverbial modifiers
- Natural transformations (morphisms between morphisms) exist
- There are meta-theorems about how these structures interact

Your insight about adding property variants to relations correctly identifies a gap in the current model's expressiveness.


## how to define al concepts (group theory)


Okay, I understand. You're looking for a general, systematic way to translate mathematical concepts into Rust types, focusing on clarity, minimal wrapping, and representing structures, properties, and operations explicitly, without needing deep domain expertise beforehand. Let's try to formulate such a strategy using Group Theory as our test case.

**Proposed General Strategy: Concept Dissection to Rust Types**

The core idea is to break down a mathematical concept by asking a series of questions and mapping the answers to Rust constructs:

1.  **What is the core "Thing" (Noun)?**
    *   Identify the central concept (e.g., "Group", "Homomorphism", "Quotient Group", "Group Action").
    *   **Rust Mapping:** This usually becomes a primary `struct` or a top-level `enum` if it represents a category with distinct variations (like `Group` itself).
    *   *Example:* `struct QuotientGroup`, `struct GroupHomomorphism`, `enum GroupAction`.

2.  **What defines it? (Components/Structure)**
    *   What other mathematical objects are intrinsically part of its definition? (e.g., A Group needs a Set and an Operation; a Quotient Group needs a Group G and a Normal Subgroup N).
    *   **Rust Mapping:** These become fields within the main `struct`. Use `Box<>` for recursive types or potentially large components. If a component is itself *constructed* in relation to the main thing (like the Normal Subgroup N *of G*), represent it using the "Operational" pattern described below (Step 4).
    *   *Example (`QuotientGroup`):* `group: Box<Group>`, `normal_subgroup: Box<OperationalGroup>`.
    *   *Example (`GroupCore`):* `base_set: Set`, `operation: GroupOperation`.

3.  **How can we describe it? (Properties/Adjectives/Adverbs)**
    *   What are its intrinsic characteristics or qualities? (e.g., A Group can be *abelian*, *finite*, *simple*; an Action can be *transitive*, *free*).
    *   **Rust Mapping:**
        *   **Categorical/Mutually Exclusive:** Use specific `enum` variants (e.g., `FinitePropertyVariant::Finite(u32)` vs `FinitePropertyVariant::Infinite`). Group related properties into specific enums (e.g., `GroupProperty`, `QuotientProperty`, `GroupActionProperty`).
        *   **Quantitative:** Use direct fields (e.g., `order: usize`, `dimension: u32`).
        *   **Boolean Flags:** Use `bool` fields for simple yes/no properties (e.g., `is_maximal: bool` *within a construction*).
        *   **Collections:** Attach a `VariantSet<PropertyName>` field to the main struct to hold a set of applicable properties (e.g., `QuotientGroup` has `quotient_props: VariantSet<QuotientProperty>`).
    *   *Focus:* Attach properties directly related to *this specific concept*. Properties of its *components* belong to the components' definitions.

4.  **How is it *made* or *derived*? (Construction/Operations - Verbs)**
    *   Is this concept typically defined *by an operation* on other concepts? (e.g., Quotient Group G/N, Product Group GxH, Subgroup generated by {g1, g2}, Normal Subgroup N <| G).
    *   **Rust Mapping:** Use an "Operational" struct pattern. Create a dedicated struct (e.g., `OperationalGroup`, `ConstructedSet`) containing:
        *   `sources`: A list (`Vec<Box<ConceptType>>`) of the input objects.
        *   `construction`: An `enum` (e.g., `GroupConstruction`, `SetConstruction`) detailing the *specific operation* used. Variants represent the verbs (e.g., `Quotient`, `DirectProduct`, `Generated`, `NormalSubgroup`, `Intersection`).
        *   `construction_props`: A `VariantSet` for properties *of the construction itself* (if any). Any parameters defining the construction (like `is_maximal` for `NormalSubgroup`) belong as fields *within* the relevant `construction` enum variant.
    *   *Benefit:* This avoids cluttering the main concept enum (like `Group`) with every possible construction method. Specific group types like `QuotientGroup` or `ProductGroup` can still exist for common cases, but their components (like the normal subgroup) might be represented operationally.

5.  **What does it *do* or how does it *relate*? (Actions/Relations - Verbs)**
    *   Does the concept *act* on something else? (e.g., Group acts on Set).
        *   **Rust Mapping:** Define an `Action` struct/enum (e.g., `GroupAction`). It holds the actor, the acted-upon, and properties of the action (`VariantSet<ActionProperty>`).
    *   Does it *relate* to other concepts? (e.g., G is isomorphic to H, N is a subgroup of G).
        *   **Rust Mapping:** Define a `Relation` enum (e.g., `GroupRelation`). Variants represent the relationship type and hold references (`Box<>`) to the related concepts.
    *   Can you perform operations *within* it? (e.g., element multiplication `g*h`, inverse `g⁻¹`, kernel `Ker(φ)`).
        *   **Rust Mapping:** Define an `Expression` enum (e.g., `GroupExpression`). Variants represent operations/terms and hold necessary references/operands.

**Applying to `QuotientGroup G/N` again:**

1.  **Thing:** `QuotientGroup` -> `struct QuotientGroup`.
2.  **Components:** Needs `Group` (G) and `Normal Subgroup` (N). N is *constructed* relative to G. -> `group: Box<Group>`, `normal_subgroup: Box<OperationalGroup>`.
3.  **Properties:** Quotient groups can have properties related to simplicity or the projection map. -> `quotient_props: VariantSet<QuotientProperty>`.
4.  **Construction:** `QuotientGroup` itself is a variant in the main `Group` enum. But its component `normal_subgroup` is constructed. -> The `OperationalGroup` in `normal_subgroup` field has `sources: vec![Box::new(group)]`, `construction: GroupConstruction::NormalSubgroup { is_maximal: ..., ... }`.
5.  **Relations:** `GroupRelation::IsQuotientOf { quotient: Box<Group::Quotient(...)>, group: Box<Group>, normal_subgroup: Box<OperationalGroup> }`.

**Benefits of this Strategy:**

*   **Systematic:** Provides a repeatable checklist for dissecting concepts.
*   **Focus on Structure:** Prioritizes representing the mathematical definition's components and properties.
*   **Explicit Operations/Constructions:** Treats operations, constructions, actions, and relations as first-class concepts with their own types.
*   **Reduces Wrappers:** Uses specific structs for common concepts and the "Operational" pattern for constructed ones, keeping the top-level enums cleaner. `QuotientGroup` directly holds the `OperationalGroup` for N, rather than `Group::Quotient(Box<Group>, Box<Group::Operational(...)>)`.
*   **Domain Agnostic (Hopefully):** The questions (Thing, Components, Properties, Construction, Actions/Relations) seem general enough to apply to other areas like Linear Algebra (Vector Space, Linear Map, Subspace, Quotient Space, Tensor Product) or Topology (Topological Space, Continuous Map, Subspace Topology, Product Topology).

This strategy emphasizes creating clear, dedicated types for each distinct aspect of a mathematical concept, aiming for a flatter, more explicit structure where possible.
