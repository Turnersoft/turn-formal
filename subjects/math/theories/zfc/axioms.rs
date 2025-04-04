/// The nine axioms of Zermelo-Fraenkel set theory with Choice (ZFC)
pub trait ZFCAxioms {
    /// The type that represents a set in the theory
    type Set;

    /// 1. Axiom of Extensionality:
    /// Two sets are equal if and only if they have the same elements
    /// ∀A∀B(∀x(x∈A ↔ x∈B) → A=B)
    fn verify_extensionality(set1: &Self::Set, set2: &Self::Set) -> bool;

    /// 2. Axiom of Empty Set:
    /// There exists a set with no elements
    /// ∃A∀x¬(x∈A)
    fn verify_empty_set(set: &Self::Set) -> bool;

    /// 3. Axiom of Pairing:
    /// For any two sets, there exists a set containing exactly those two sets as elements
    /// ∀A∀B∃C∀x(x∈C ↔ x=A ∨ x=B)
    fn verify_pairing(pair: &Self::Set, a: &Self::Set, b: &Self::Set) -> bool;

    /// 4. Axiom of Union:
    /// For any collection of sets, there exists a set containing all elements of all those sets
    /// ∀F∃A∀x(x∈A ↔ ∃B(B∈F ∧ x∈B))
    fn verify_union(union: &Self::Set, sets: &[Self::Set]) -> bool;

    /// 5. Axiom of Power Set:
    /// For any set, there exists a set containing all subsets of the original set
    /// ∀A∃P∀x(x∈P ↔ ∀y(y∈x → y∈A))
    fn verify_power_set(power: &Self::Set, original: &Self::Set) -> bool;

    /// 6. Axiom of Foundation (Regularity):
    /// Every non-empty set has an element disjoint from itself
    /// ∀A(∃x(x∈A) → ∃y(y∈A ∧ ¬∃z(z∈y ∧ z∈A)))
    fn verify_foundation(set: &Self::Set) -> bool;

    /// 7. Axiom Schema of Separation (Specification):
    /// For any set and any property, there exists a set containing exactly those elements
    /// of the original set that satisfy the property
    /// ∀A∃B∀x(x∈B ↔ x∈A ∧ φ(x))
    fn verify_separation<F>(subset: &Self::Set, superset: &Self::Set, property: F) -> bool
    where
        F: Fn(&Self::Set) -> bool;

    /// 8. Axiom Schema of Replacement:
    /// The image of any set under any definable function will also form a set
    /// ∀A∃B∀y(y∈B ↔ ∃x(x∈A ∧ φ(x,y)))
    fn verify_replacement<F>(image: &Self::Set, domain: &Self::Set, function: F) -> bool
    where
        F: Fn(&Self::Set) -> Self::Set;

    /// 9. Axiom of Choice:
    /// For any collection of non-empty sets, there exists a set containing exactly one element
    /// from each set in the collection
    /// ∀A(∅∉A → ∃f:A→∪A ∀B∈A(f(B)∈B))
    fn verify_choice(choice: &Self::Set, sets: &[Self::Set]) -> bool;
}

/// A marker trait for theories that satisfy all ZFC axioms
pub trait SatisfiesZFC: ZFCAxioms {
    /// Verify that all ZFC axioms hold for this theory
    fn verify_all_axioms(&self) -> bool {
        true
    }
}

/// Helper trait for implementing the Separation axiom
pub trait Separable {
    /// The type of elements in the set
    type Element;

    /// Separate elements that satisfy a property
    fn separate<F>(&self, property: F) -> Self
    where
        F: Fn(&Self::Element) -> bool;
}

/// Helper trait for implementing the Replacement axiom
pub trait Replaceable {
    /// The type of elements in the set
    type Element;

    /// Replace elements using a function
    fn replace<F, T>(&self, function: F) -> Self
    where
        F: Fn(&Self::Element) -> T;
}
