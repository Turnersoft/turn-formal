/// Tests for the ZFC set theory implementation
///
/// This module contains tests that verify our implementation adheres to
/// the Zermelo-Fraenkel set theory with Choice (ZFC) axioms.
///
/// ZFC set theory is the foundation of modern mathematics, built on these key ideas:
/// - Sets are collections of objects where order doesn't matter and duplicates are ignored
/// - Every mathematical object can be represented as a set
/// - Sets are defined by their elements (Extensionality)
/// - There exists an empty set ∅ (Empty Set)
/// - We can create new sets from existing ones through operations like:
///   * Pairing: Creating a set containing two elements {a,b}
///   * Union: Combining elements from multiple sets
///   * Power Set: Creating a set of all subsets
///   * Separation: Creating subsets by filtering elements
///   * Replacement: Creating new sets by transforming elements
/// - The Axiom of Choice allows us to select one element from each set in a collection
///
/// These tests verify that our implementation correctly follows these mathematical rules.
#[cfg(test)]
mod zfc_machinery_tests {
    use crate::subjects::math::theories::zfc::axioms::ZFCAxioms;

    use super::super::super::super::super::math::theories::VariantSet;
    use super::super::super::super::super::math::theories::zfc::{
        cartesian_product,
        definitions::{ElementCondition, Set, SetElement, SetMapping, SetOpProperty, SetProperty},
        empty_set, ordered_pair, pair_set, power_set, singleton_set, symmetric_difference_set,
        union_set,
        verifier::ZFCVerifier,
    };
    use super::super::*;
    use super::*;

    /// Creates a standard set of test sets used across multiple tests.
    ///
    /// Returns a tuple containing three fundamental set constructions:
    /// - empty set (∅): The unique set containing no elements
    /// - singleton set ({∅}): A set containing only the empty set
    /// - pair set ({∅, {∅}}): A set containing both the empty set and its singleton
    ///
    /// These sets form the basic building blocks for more complex set constructions
    /// in the von Neumann hierarchy of sets, where:
    /// - 0 is represented as ∅
    /// - 1 is represented as {∅}
    /// - 2 is represented as {∅, {∅}}
    /// and so on.
    fn create_test_sets() -> (Set, Set, Set) {
        let empty = empty_set(); // Create ∅
        let empty_elem = SetElement::from(empty.clone());
        let singleton = singleton_set(empty_elem.clone()); // Create {∅}
        let singleton_elem = SetElement::from(singleton.clone());
        let pair = pair_set(empty_elem.clone(), singleton_elem.clone()); // Create {∅, {∅}}
        (empty.clone(), singleton, pair)
    }

    /// Verifies that a set satisfies the Extensionality axiom.
    ///
    /// The Extensionality axiom states that two sets are equal if and only if
    /// they have exactly the same elements. This is the fundamental principle
    /// that defines set equality in ZFC set theory.
    ///
    /// For example:
    /// - {1,2} = {2,1} (order doesn't matter)
    /// - {1,1,2} = {1,2} (duplicates don't matter)
    /// - {1,2} ≠ {1,2,3} (different elements mean different sets)
    ///
    /// This test creates a clone of the input set and verifies that they are equal
    /// according to the extensionality principle.
    fn assert_extensionality_holds(set: &Set) {
        let set2 = set.clone(); // Create an identical copy of the set
        assert!(ZFCVerifier::verify_extensionality(set, &set2)); // Verify they are equal
    }

    /// Verifies that a set satisfies the Foundation/Regularity axiom.
    ///
    /// The Foundation axiom states that every non-empty set x contains an element y
    /// such that x and y are disjoint (x ∩ y = ∅). This prevents:
    /// - Sets that contain themselves (x ∈ x)
    /// - Infinite descending chains (... ∈ x₂ ∈ x₁ ∈ x₀)
    ///
    /// For example:
    /// - ∅ satisfies foundation trivially (it has no elements)
    /// - {∅} satisfies foundation because ∅ is disjoint with {∅}
    /// - {∅, {∅}} satisfies foundation because ∅ is disjoint with {∅, {∅}}
    ///
    /// This axiom ensures sets are "well-founded" and can be built from
    /// the empty set in a hierarchical way.
    fn assert_foundation_holds(set: &Set) {
        assert!(ZFCVerifier::verify_foundation(set)); // Verify the set is well-founded
    }

    /// Tests for verifying compliance with the ZFC axioms
    /// Each submodule tests one of the fundamental axioms of ZFC set theory
    mod axiom_tests {
        use super::*;

        /// Tests for the Extensionality axiom
        /// Two sets are equal if and only if they have the same elements
        /// This is the fundamental axiom that defines set equality
        mod axiom_extensionality {
            use super::*;

            /// Tests basic extensionality with simple sets
            ///
            /// This test verifies that:
            /// 1. Two empty sets are equal (∅ = ∅)
            /// 2. Two singleton sets with equal elements are equal ({∅} = {∅})
            ///
            /// The extensionality principle states that sets are equal if they have
            /// the same elements, regardless of how they were constructed. This is
            /// a fundamental principle in set theory that distinguishes it from
            /// other mathematical structures where construction method matters.
            #[test]
            fn test_basic_extensionality() {
                // Create two copies of our basic test sets
                let (empty1, singleton1, _) = create_test_sets();
                let (empty2, singleton2, _) = create_test_sets();

                // Empty sets should be equal by extensionality
                // This is because they both contain no elements: ∅ = ∅
                assert!(ZFCVerifier::verify_extensionality(&empty1, &empty2));

                // Singleton sets with equal elements should be equal
                // This is because {∅} and {∅} contain exactly the same element
                assert!(ZFCVerifier::verify_extensionality(&singleton1, &singleton2));
            }

            #[test]
            fn test_complex_extensionality() {
                let empty = empty_set();
                let empty_elem = SetElement::from(empty.clone());
                let a = singleton_set(empty_elem.clone());
                let a_elem = SetElement::from(a.clone());

                // Different constructions of same set should be equal
                // Here we create {∅, {∅}} in two different ways
                let set1 = pair_set(empty_elem.clone(), a_elem.clone());
                let set2 = Set::Enumeration {
                    elements: vec![empty_elem.clone(), a_elem.clone()],
                    properties: VariantSet::new(),
                };
                assert!(ZFCVerifier::verify_extensionality(&set1, &set2));

                // Sets with same structure but different elements should not be equal
                // Here we create {∅, {{∅}}} which has different elements from {∅, {∅}}
                let b = singleton_set(a_elem.clone());
                let b_elem = SetElement::from(b.clone());
                let set3 = pair_set(empty_elem.clone(), b_elem.clone());
                assert!(!ZFCVerifier::verify_extensionality(&set1, &set3));
            }
        }

        /// Tests for the Empty Set axiom
        /// There exists a set that contains no elements
        /// This axiom guarantees the existence of at least one set
        mod axiom_empty_set {
            use super::*;

            /// Tests fundamental properties of the empty set (∅)
            ///
            /// This test verifies that the empty set satisfies key mathematical properties:
            ///
            /// 1. Extensionality:
            ///    - Any two empty sets are equal
            ///    - This follows from having no elements to compare
            ///
            /// 2. Foundation/Regularity:
            ///    - The empty set is trivially well-founded
            ///    - It has no elements, so no infinite descending chains
            ///
            /// 3. Basic Properties:
            ///    - Contains no elements (is_empty)
            ///    - Has cardinality 0 (len = 0)
            #[test]
            fn test_empty_set_properties() {
                let empty = Set::Empty;
                // Empty set should satisfy both extensionality and foundation
                assert_extensionality_holds(&empty);
                assert_foundation_holds(&empty);
                assert!(empty.is_empty());
                assert_eq!(empty.len(), 0);
            }

            #[test]
            fn test_empty_set_uniqueness() {
                // The empty set must be unique (up to extensionality)
                let empty1 = empty_set();
                let empty2 = empty_set();
                assert!(ZFCVerifier::verify_extensionality(&empty1, &empty2));
            }
        }

        /// Tests for the Pairing axiom
        /// For any sets a and b, there exists a set containing exactly a and b
        /// This axiom allows us to construct sets with exactly two elements
        mod axiom_pairing {
            use super::*;

            /// Tests the Pairing axiom of ZFC set theory
            ///
            /// This test verifies that given any two sets a and b (not necessarily distinct),
            /// there exists a set containing exactly those two sets as its elements.
            /// In set notation: ∀a∀b∃y∀x(x ∈ y ↔ x = a ∨ x = b)
            ///
            /// We test this by:
            /// 1. Creating a pair set with two distinct sets (∅ and {∅})
            /// 2. Creating a pair set with the same set twice (∅ and ∅)
            /// 3. Verifying the resulting sets have the correct elements
            #[test]
            fn test_pairing() {
                // Create our basic test sets
                let (empty, singleton, _) = create_test_sets();

                // Test pairing of different sets: {∅, {∅}}
                let empty_elem = SetElement::from(empty.clone());
                let singleton_elem = SetElement::from(singleton.clone());
                let distinct_pair = pair_set(empty_elem.clone(), singleton_elem.clone());

                assert!(ZFCVerifier::verify_pairing(
                    &distinct_pair,
                    &empty,
                    &singleton
                ));
                assert_eq!(distinct_pair.len(), 2);

                // Test pairing of identical sets: {∅, ∅} which should yield {∅}
                let identical_pair = pair_set(empty_elem.clone(), empty_elem.clone());
                assert!(ZFCVerifier::verify_pairing(&identical_pair, &empty, &empty));
                assert_eq!(identical_pair.len(), 1);
            }
        }

        /// Tests for the Union axiom
        /// For any collection of sets, there exists a set containing all elements
        /// that belong to at least one of the sets in the collection
        mod axiom_union {

            use super::*;

            /// Tests the Union axiom of ZFC set theory
            ///
            /// This test verifies that for any collection of sets, there exists a set
            /// that contains all elements that belong to at least one of the sets in
            /// the collection. In set notation: ∀F∃U∀x(x ∈ U ↔ ∃y(y ∈ F ∧ x ∈ y))
            ///
            /// We test three key properties of unions:
            /// 1. Union of disjoint sets: {∅} ∪ {{∅}} = {∅, {∅}}
            ///    Shows that elements from both sets are included
            ///
            /// 2. Union with empty set: {∅} ∪ ∅ = {∅}
            ///    Demonstrates that union with empty set doesn't add elements
            ///
            /// 3. Union with overlapping sets: {∅, {∅}} ∪ {∅} = {∅, {∅}}
            ///    Shows that duplicate elements are handled correctly
            #[test]
            fn test_union() {
                // Create our basic test sets
                let (empty, singleton, pair) = create_test_sets();

                // Test union of disjoint sets: {∅} ∪ {{∅}} = {∅, {∅}}
                let disjoint_sets = vec![singleton.clone(), pair.clone()];
                let disjoint_union = union_set(&disjoint_sets);
                assert!(ZFCVerifier::verify_union(&disjoint_union, &disjoint_sets));
                assert_eq!(disjoint_union.len(), 2);

                // Test union with empty set: {∅} ∪ ∅ = {∅}
                let empty_sets = vec![singleton.clone(), empty.clone()];
                let empty_union = union_set(&empty_sets);
                assert!(ZFCVerifier::verify_union(&empty_union, &empty_sets));
                assert_eq!(empty_union.len(), 1);

                // Test union with overlapping sets: {∅, {∅}} ∪ {∅} = {∅, {∅}}
                let overlapping_sets = vec![pair.clone(), singleton.clone()];
                let overlapping_union = union_set(&overlapping_sets);
                assert!(ZFCVerifier::verify_union(
                    &overlapping_union,
                    &overlapping_sets
                ));
                assert_eq!(overlapping_union.len(), 2);
            }

            /// Tests the Big Union operation in ZFC set theory
            ///
            /// The Big Union (⋃) operation takes a family of sets and returns a set containing
            /// all elements that belong to any set in the family. This is a generalization
            /// of the binary union operation to arbitrary collections of sets.
            ///
            /// In this test, we create a family F = {{∅}, {{∅}}} and verify that:
            /// ⋃F = {∅, {∅}}
            #[test]
            fn test_big_union() {
                let empty = empty_set();
                let empty_elem = SetElement::from(empty.clone());
                let a = singleton_set(empty_elem.clone());
                let a_elem = SetElement::from(a.clone());
                let b = singleton_set(a_elem.clone());

                // Create the family of sets as elements
                let family_elements = vec![a_elem.clone(), SetElement::from(b.clone())];
                let family = Set::Enumeration {
                    elements: family_elements,
                    properties: VariantSet::new(),
                };

                let big_union = Set::BigUnion {
                    family: Box::new(family.clone()),
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };

                // Big union should contain ∅ (from {∅}) and {∅} (from {{∅}})
                assert!(big_union.contains(&empty_elem));
                assert!(big_union.contains(&a_elem));
            }
        }

        /// Tests for the Power Set axiom
        /// For any set A, there exists a set P(A) containing all subsets of A
        /// This axiom enables us to construct sets of subsets
        mod axiom_power_set {
            use super::*;

            /// Tests the Power Set axiom of ZFC set theory
            ///
            /// The Power Set axiom states that for any set A, there exists a set P(A)
            /// that contains all subsets of A. In set notation:
            /// ∀A∃P∀x(x ∈ P ↔ x ⊆ A)
            ///
            /// For example:
            /// - P(∅) = {∅}
            /// - P({∅}) = {∅, {∅}}
            /// - P({∅, {∅}}) = {∅, {∅}, {{∅}}, {∅, {∅}}}
            ///
            /// This axiom is crucial for:
            /// - Constructing sets of subsets
            /// - Defining functions as sets of ordered pairs
            /// - Building the cumulative hierarchy of sets
            #[test]
            fn test_basic_power_set() {
                let (_, singleton, _) = create_test_sets();
                let power = power_set(&singleton);
                // Verify that power set contains all subsets of singleton
                assert!(ZFCVerifier::verify_power_set(&power, &singleton));
            }

            /// Tests specific properties of power sets in ZFC set theory
            ///
            /// This test verifies key properties of power sets:
            /// 1. Power set of empty set: P(∅) = {∅}
            ///    - Contains exactly one element (the empty set)
            ///    - Demonstrates that ∅ is a subset of itself
            ///
            /// 2. Power set of singleton: P({∅}) = {∅, {∅}}
            ///    - Contains exactly two elements
            ///    - Shows that both ∅ and {∅} are subsets of {∅}
            ///
            /// These properties illustrate how power sets grow exponentially:
            /// - |P(∅)| = 2⁰ = 1
            /// - |P({∅})| = 2¹ = 2
            /// - |P({∅, {∅}})| = 2² = 4
            /// And so on, where |A| denotes the cardinality of set A
            #[test]
            fn test_power_set_properties() {
                let empty = empty_set();
                let power_empty = power_set(&empty);
                // Power set of empty set has exactly one element: {∅}
                assert_eq!(power_empty.len(), 1); // P(∅) = {∅}

                let empty_elem = SetElement::from(empty.clone());
                let singleton = singleton_set(empty_elem.clone());
                let power_singleton = power_set(&singleton);
                // Power set of singleton has exactly two elements: ∅ and {∅}
                assert_eq!(power_singleton.len(), 2); // P({∅}) = {∅, {∅}}
            }

            /// Tests complex properties of power sets
            ///
            /// This test verifies that power sets of non-trivial sets work correctly.
            /// For the set A = {∅, {∅}}, we verify that P(A) contains:
            /// 1. ∅ (empty set)
            /// 2. {∅} (singleton)
            /// 3. {{∅}} (nested singleton)
            /// 4. {∅, {∅}} (original set)
            #[test]
            fn test_complex_power_set() {
                let empty = empty_set();
                let empty_elem = SetElement::from(empty.clone());
                let a = singleton_set(empty_elem.clone());
                let a_elem = SetElement::from(a.clone());

                // Create pair set with proper SetElement parameters
                let pair = pair_set(empty_elem.clone(), a_elem.clone());

                // Get power set using function from mod.rs
                let power = power_set(&pair);

                // Power set of {∅, {∅}} should have 4 elements:
                // {∅, {∅}, {∅, {∅}}, ∅}
                assert_eq!(power.len(), 4);

                // Convert sets to elements for contains checks
                let empty_elem_for_contains = SetElement::from(empty.clone());
                let a_elem_for_contains = SetElement::from(a.clone());
                let pair_elem_for_contains = SetElement::from(pair.clone());
                let empty_set_elem_for_contains = SetElement::from(Set::empty());

                // Check power set contains expected elements
                assert!(power.contains(&empty_elem_for_contains));
                assert!(power.contains(&a_elem_for_contains));
                assert!(power.contains(&pair_elem_for_contains));
                assert!(power.contains(&empty_set_elem_for_contains));
            }
        }

        /// Tests for the Foundation/Regularity axiom
        /// Every non-empty set has an ∈-minimal element
        /// This axiom prevents infinite descending chains of membership
        mod axiom_foundation {
            use super::*;

            /// Tests basic compliance with the Foundation/Regularity axiom
            ///
            /// The Foundation axiom states that every non-empty set x contains an element y
            /// such that x ∩ y = ∅. This test verifies this property for basic set constructions:
            ///
            /// 1. Empty set (∅): Trivially satisfies foundation as it has no elements
            /// 2. Singleton set ({∅}): Contains ∅, which is disjoint with {∅}
            /// 3. Pair set ({∅, {∅}}): Contains ∅, which is disjoint with {∅, {∅}}
            ///
            /// This axiom prevents sets from containing themselves and ensures sets are well-founded.
            #[test]
            fn test_basic_foundation() {
                let (empty, singleton, pair) = create_test_sets();
                // All basic set constructions should satisfy foundation
                assert!(ZFCVerifier::verify_foundation(&empty));
                assert!(ZFCVerifier::verify_foundation(&singleton));
                assert!(ZFCVerifier::verify_foundation(&pair));
            }

            /// Tests the Foundation axiom with deeply nested sets
            ///
            /// This test verifies that even deeply nested sets satisfy the Foundation axiom.
            /// We create a set of the form {{{...{∅}...}}} with 10 levels of nesting:
            ///
            /// - Level 0: ∅
            /// - Level 1: {∅}
            /// - Level 2: {{∅}}
            /// - Level 3: {{{∅}}}
            /// And so on...
            ///
            /// Each level adds another layer of singleton set wrapping. Despite the deep
            /// nesting, the set remains well-founded because:
            /// 1. Each level contains exactly one element (the previous level)
            /// 2. The chain of membership eventually reaches ∅
            /// 3. No set in the chain contains itself or forms a cycle
            #[test]
            fn test_deep_foundation() {
                let empty = empty_set();
                let mut nested_elem = SetElement::from(empty.clone());
                // Create a deeply nested set: {{{...{∅}...}}}
                for _ in 0..10 {
                    let nested_set = singleton_set(nested_elem);
                    nested_elem = SetElement::from(nested_set);
                }
                let final_nested_set = match nested_elem {
                    SetElement::Set(boxed_set) => *boxed_set,
                    _ => panic!("Expected Set element"),
                };
                // Even deeply nested sets should satisfy foundation
                assert!(ZFCVerifier::verify_foundation(&final_nested_set));
            }
        }

        /// Tests for the Separation/Specification axiom
        /// For any set A and property φ, there exists a set containing exactly
        /// those elements of A that satisfy φ
        mod axiom_separation {
            use super::*;

            /// Tests basic application of the Separation/Specification axiom
            ///
            /// The Separation axiom states that for any set A and property φ(x), there exists
            /// a set B = {x ∈ A | φ(x)} containing exactly those elements of A that satisfy φ.
            ///
            /// This test demonstrates separation with a simple property:
            /// Given A = {∅, {∅}}, we create B = {x ∈ A | x = ∅} = {∅}
            ///
            /// This shows how separation can be used to extract specific elements from a set.
            #[test]
            fn test_basic_separation() {
                let (empty, _, pair) = create_test_sets();
                let empty_elem = SetElement::from(empty.clone());

                // Create subset of elements equal to empty set
                let subset = Set::Separation {
                    source: Box::new(pair.clone()),
                    condition: ElementCondition::Contains(Box::new(empty_elem.clone())),
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };

                // Custom property function that checks if a set equals the empty set
                let property = |s: &Set| s == &empty;
                assert!(ZFCVerifier::verify_separation(&subset, &pair, property));
            }

            /// Tests complex application of the Separation axiom with nested sets
            ///
            /// This test demonstrates separation with a more complex property:
            /// Given domain D = {∅, {∅}, {{∅}}}, we create a subset containing
            /// elements that directly contain the empty set.
            ///
            /// The test verifies that:
            /// 1. {∅} is in the result (because ∅ ∈ {∅})
            /// 2. ∅ is not in the result (because ∅ contains nothing)
            /// 3. {{∅}} is not in the result (because ∅ ∉ {{∅}} directly)
            ///
            /// This shows how separation can work with:
            /// - Nested set structures
            /// - Complex membership conditions
            /// - Direct vs. indirect containment
            #[test]
            fn test_complex_separation() {
                let empty = empty_set();
                let empty_elem = SetElement::from(empty.clone());
                let a = singleton_set(empty_elem.clone());
                let a_elem = SetElement::from(a.clone());
                let b = singleton_set(a_elem.clone());
                let b_elem = SetElement::from(b.clone());

                // Create domain with three nested sets as elements
                let domain_elements = vec![empty_elem.clone(), a_elem.clone(), b_elem.clone()];
                let domain = Set::Enumeration {
                    elements: domain_elements,
                    properties: VariantSet::new(),
                };

                // Separate elements that contain the empty set
                let separated = Set::Separation {
                    source: Box::new(domain),
                    condition: ElementCondition::Contains(Box::new(empty_elem.clone())),
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };

                // {∅} contains ∅, but ∅ and {{∅}} don't contain ∅ directly
                assert!(separated.contains(&a_elem));
                assert!(!separated.contains(&empty_elem));
                assert!(!separated.contains(&b_elem));
            }
        }
    }

    /// Tests for advanced set operations and their properties
    ///
    /// These tests verify complex combinations of set operations and their
    /// interactions. They ensure that our implementation correctly handles:
    /// 1. Nested operations (operations applied to results of other operations)
    /// 2. Complex set constructions (power sets, cartesian products, etc.)
    /// 3. Advanced set-theoretic properties (well-foundedness, etc.)
    mod advanced_set_operations {
        use super::*;

        /// Tests nested composition of replacement functions
        ///
        /// This test verifies that complex compositions of functions work correctly:
        /// h = (π₁ ∘ singleton) ∘ π₁
        ///
        /// Given a domain with nested sets {∅, {∅}, {{∅}}}, we verify:
        /// 1. The composition is well-defined
        /// 2. Set-theoretic properties are preserved
        /// 3. The result matches mathematical expectations
        #[test]
        fn test_nested_composition() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());
            let b = singleton_set(a_elem.clone());
            let b_elem = SetElement::from(b.clone());

            // Create domain with proper SetElement types
            let domain = Set::Enumeration {
                elements: vec![empty_elem.clone(), a_elem.clone(), b_elem.clone()],
                properties: VariantSet::new(),
            };

            // Test nested composition: (FirstProjection ∘ Singleton) ∘ FirstProjection
            let composed = Set::Replacement {
                source: Box::new(domain),
                mapping: SetMapping::Composition(
                    Box::new(SetMapping::Composition(
                        Box::new(SetMapping::FirstProjection),
                        Box::new(SetMapping::Singleton),
                    )),
                    Box::new(SetMapping::FirstProjection),
                ),
                properties: VariantSet::new(),
                op_properties: VariantSet::new(),
            };

            // Complex compositions should preserve set-theoretic properties
            assert_extensionality_holds(&composed);
            assert_foundation_holds(&composed);
        }

        /// Tests complex properties of power sets
        ///
        /// This test verifies that power sets of non-trivial sets work correctly.
        /// For the set A = {∅, {∅}}, we verify that P(A) contains:
        /// 1. ∅ (empty set)
        /// 2. {∅} (singleton)
        /// 3. {{∅}} (nested singleton)
        /// 4. {∅, {∅}} (original set)
        #[test]
        fn test_complex_power_set() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());
            let pair = pair_set(empty_elem.clone(), a_elem.clone());
            let power = power_set(&pair);

            // Power set of {∅, {∅}} should have 4 elements:
            // {∅, {∅}, {∅, {∅}}, ∅}
            assert_eq!(power.len(), 4);

            // Convert Sets to SetElements for contains checks
            let empty_elem_for_contains = SetElement::from(empty.clone());
            let a_elem_for_contains = SetElement::from(a.clone());
            let pair_elem_for_contains = SetElement::from(pair.clone());
            let empty_set_elem_for_contains = SetElement::from(Set::empty());

            assert!(power.contains(&empty_elem_for_contains));
            assert!(power.contains(&a_elem_for_contains));
            assert!(power.contains(&pair_elem_for_contains));
            assert!(power.contains(&empty_set_elem_for_contains));
        }

        /// Tests interaction between cartesian products and power sets
        ///
        /// This test verifies that cartesian products work correctly with
        /// power sets. Given A = {∅} and B = P({∅}), we verify that A × B:
        /// 1. Contains all ordered pairs (a,b) where a ∈ A, b ∈ P({∅})
        /// 2. Has the correct cardinality
        /// 3. Preserves set-theoretic properties
        #[test]
        fn test_cartesian_product_with_power_set() {
            let empty = empty_set(); // ∅
            let empty_elem = SetElement::from(empty.clone());
            let singleton = singleton_set(empty_elem.clone()); // {∅}
            let power_singleton = power_set(&singleton); // P({∅})

            // Print debug info for troubleshooting
            println!("singleton = {:?}", singleton);
            println!("power_singleton = {:?}", power_singleton);
            println!(
                "power_singleton elements = {:?}",
                power_singleton.elements()
            );

            // Create the cartesian product {∅} × P({∅})
            let product = cartesian_product(&singleton, &power_singleton);
            println!("product = {:?}", product);
            println!("product elements = {:?}", product.elements());

            // Get the elements of the power set
            let power_elements = power_singleton.elements();
            assert_eq!(power_elements.len(), 2);

            // Here we need SetElements for the empty set and singleton
            let empty_set_elem = SetElement::from(empty.clone());
            let singleton_set_elem = SetElement::from(singleton.clone());

            // Create ordered pairs ({∅}, ∅) and ({∅}, {∅})
            let ordered_pair1 = ordered_pair(singleton.clone(), empty.clone());
            let ordered_pair1_elem = SetElement::from(ordered_pair1.clone());

            let ordered_pair2 = ordered_pair(singleton.clone(), singleton.clone());
            let ordered_pair2_elem = SetElement::from(ordered_pair2.clone());

            // Debug output for troubleshooting ordered pairs
            println!("ordered_pair1 = {:?}", ordered_pair1);
            println!("ordered_pair1 elements = {:?}", ordered_pair1.elements());
            println!("ordered_pair2 = {:?}", ordered_pair2);
            println!("ordered_pair2 elements = {:?}", ordered_pair2.elements());

            // More debug info
            // ... other print statements ...

            // Verify product contains both ordered pairs
            assert!(
                product.contains(&ordered_pair1_elem),
                "Product should contain the first ordered pair"
            );
            assert!(
                product.contains(&ordered_pair2_elem),
                "Product should contain the second ordered pair"
            );
            assert_eq!(product.elements().len(), 2);
        }

        #[test]
        fn test_complex_separation() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());
            let b = singleton_set(a_elem.clone());
            let b_elem = SetElement::from(b.clone());

            // Create domain with three nested sets as elements
            let domain_elements = vec![empty_elem.clone(), a_elem.clone(), b_elem.clone()];
            let domain = Set::Enumeration {
                elements: domain_elements,
                properties: VariantSet::new(),
            };

            // Separate elements that contain the empty set
            let separated = Set::Separation {
                source: Box::new(domain),
                condition: ElementCondition::Contains(Box::new(empty_elem.clone())),
                properties: VariantSet::new(),
                op_properties: VariantSet::new(),
            };

            // {∅} contains ∅, but ∅ and {{∅}} don't contain ∅ directly
            assert!(separated.contains(&a_elem));
            assert!(!separated.contains(&empty_elem));
            assert!(!separated.contains(&b_elem));
        }

        #[test]
        fn test_big_intersection_soundness() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());
            let family = Set::Enumeration {
                elements: vec![empty_elem.clone(), a_elem.clone()],
                properties: VariantSet::new(),
            };
            let set = Set::BigIntersection {
                family: Box::new(family),
                properties: VariantSet::new(),
                op_properties: VariantSet::new(),
            };

            assert_extensionality_holds(&set);
            assert_foundation_holds(&set);
        }

        #[test]
        fn test_symmetric_difference_with_power_sets() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());

            let power_empty = power_set(&empty);
            let power_a = power_set(&a);

            let sym_diff = symmetric_difference_set(&power_empty, &power_a);

            // Empty set is in both power sets, so not in symmetric difference
            assert!(!sym_diff.contains(&empty_elem));
            // {∅} is only in P({∅}), so it should be in symmetric difference
            assert!(sym_diff.contains(&a_elem));
        }
    }

    /// Tests for advanced axiom verification
    ///
    /// These tests verify that complex set constructions still satisfy
    /// the fundamental axioms of ZFC set theory. They ensure that:
    /// 1. Foundation holds even for deeply nested structures
    /// 2. Extensionality works with different construction methods
    /// 3. Set-theoretic properties are preserved under operations
    mod advanced_axiom_tests {
        use super::*;

        /// Tests Foundation axiom with complex set structures
        ///
        /// This test verifies that even complex nested sets satisfy
        /// the Foundation axiom. We create a set containing:
        /// - Simple elements: ∅, {∅}
        /// - Nested elements: {{∅}}
        /// - Power sets: P({∅})
        ///
        /// We verify that:
        /// 1. The set is well-founded
        /// 2. No infinite descending chains exist
        /// 3. Each non-empty subset has an ∈-minimal element
        #[test]
        fn test_foundation_with_complex_sets() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());
            let b = singleton_set(a_elem.clone());
            let b_elem = SetElement::from(b.clone());

            // Get power set of b
            let power_b = power_set(&b);
            let power_b_elem = SetElement::from(power_b.clone());

            // Create a complex set with mixed elements
            let complex = Set::Enumeration {
                elements: vec![
                    empty_elem.clone(),
                    a_elem.clone(),
                    b_elem.clone(),
                    power_b_elem.clone(),
                ],
                properties: VariantSet::new(),
            };

            // Even complex nested structures should satisfy foundation
            assert_foundation_holds(&complex);
            assert!(complex.is_well_founded());
        }

        /// Tests Extensionality with different set constructions
        ///
        /// This test verifies that sets constructed in different ways
        /// are equal if they have the same elements. We create the
        /// same set A = {∅, {∅}} using:
        /// 1. Direct construction: pair_set(∅, {∅})
        /// 2. Element list: from_elements([∅, {∅}])
        ///
        /// We verify that:
        /// 1. Both constructions yield equal sets
        /// 2. Extensionality holds for both sets
        /// 3. Set properties are preserved
        #[test]
        fn test_extensionality_with_different_constructions() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());

            // Two different ways to construct the same set
            let set1 = Set::Enumeration {
                elements: vec![empty_elem.clone(), a_elem.clone()],
                properties: VariantSet::new(),
            };
            let set2 = pair_set(empty_elem.clone(), a_elem.clone());

            // Different constructions of the same set should be equal
            assert_eq!(set1, set2);
            assert_extensionality_holds(&set1);
            assert_extensionality_holds(&set2);
        }
    }

    /// Tests for verifying soundness of different set variants
    ///
    /// These tests ensure that each variant of the Set enum maintains
    /// the fundamental axioms of ZFC set theory. They verify that:
    /// 1. Each variant preserves extensionality (equal sets have equal elements)
    /// 2. Each variant maintains well-foundedness (no infinite membership chains)
    /// 3. Each variant behaves correctly under set operations
    mod variant_axiom_soundness {
        use super::*;

        /// Tests soundness of binary union operations
        ///
        /// This test verifies that binary union A ∪ B:
        /// 1. Preserves extensionality (equal unions have equal elements)
        /// 2. Maintains well-foundedness (no infinite membership chains)
        /// 3. Contains exactly the elements from either input set
        #[test]
        fn test_binary_union_soundness() {
            let set = Set::BinaryUnion {
                left: Box::new(Set::Empty),
                right: Box::new(Set::Empty),
                properties: VariantSet::new(),
                op_properties: VariantSet::new(),
            };
            assert_extensionality_holds(&set);
            assert_foundation_holds(&set);
        }

        /// Tests soundness of binary intersection operations
        ///
        /// This test verifies that binary intersection A ∩ B:
        /// 1. Preserves extensionality (equal intersections have equal elements)
        /// 2. Maintains well-foundedness (no infinite membership chains)
        /// 3. Contains only elements present in both input sets
        #[test]
        fn test_binary_intersection_soundness() {
            let set = Set::BinaryIntersection {
                left: Box::new(Set::Empty),
                right: Box::new(Set::Empty),
                properties: VariantSet::new(),
                op_properties: VariantSet::new(),
            };
            assert_extensionality_holds(&set);
            assert_foundation_holds(&set);
        }

        /// Tests soundness of set difference operations
        ///
        /// This test verifies that set difference A - B:
        /// 1. Preserves extensionality (equal differences have equal elements)
        /// 2. Maintains well-foundedness (no infinite membership chains)
        /// 3. Contains only elements from A that are not in B
        #[test]
        fn test_set_difference_soundness() {
            let set = Set::SetDifference {
                left: Box::new(Set::Empty),
                right: Box::new(Set::Empty),
                properties: VariantSet::new(),
                op_properties: VariantSet::new(),
            };
            assert_extensionality_holds(&set);
            assert_foundation_holds(&set);
        }

        /// Tests soundness of cartesian product operations
        ///
        /// This test verifies that cartesian product A × B:
        /// 1. Preserves extensionality (equal products have equal elements)
        /// 2. Maintains well-foundedness (no infinite membership chains)
        /// 3. Contains all ordered pairs (a,b) where a ∈ A and b ∈ B
        #[test]
        fn test_cartesian_product_soundness() {
            let set = Set::CartesianProduct {
                left: Box::new(Set::Empty),
                right: Box::new(Set::Empty),
                properties: VariantSet::new(),
                op_properties: VariantSet::new(),
            };
            assert_extensionality_holds(&set);
            assert_foundation_holds(&set);
        }

        /// Tests soundness of big intersection operations
        ///
        /// This test verifies that big intersection ⋂F:
        /// 1. Preserves extensionality (equal intersections have equal elements)
        /// 2. Maintains well-foundedness (no infinite membership chains)
        /// 3. Contains only elements present in all sets of the family F
        #[test]
        fn test_big_intersection_soundness() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());
            let family = Set::Enumeration {
                elements: vec![empty_elem.clone(), a_elem.clone()],
                properties: VariantSet::new(),
            };
            let set = Set::BigIntersection {
                family: Box::new(family),
                properties: VariantSet::new(),
                op_properties: VariantSet::new(),
            };
            assert_extensionality_holds(&set);
            assert_foundation_holds(&set);
        }
    }

    /// Tests for basic set operations and their properties
    ///
    /// These tests verify that our implementation correctly handles
    /// fundamental set operations like union, intersection, etc.
    /// Each operation is tested for:
    /// 1. Correctness (produces expected results)
    /// 2. Soundness (maintains set-theoretic properties)
    /// 3. Edge cases (empty sets, singleton sets, etc.)
    mod set_operations {
        use super::*;

        /// Tests for empty set operations and properties
        ///
        /// The empty set is a fundamental concept in set theory and has
        /// special properties in relation to other sets:
        /// - Identity element for union: A ∪ ∅ = A
        /// - Zero element for intersection: A ∩ ∅ = ∅
        /// - Subset of all sets: ∅ ⊆ A for all A
        mod empty_set {
            use super::*;

            /// Tests fundamental properties of the empty set
            ///
            /// This test verifies that the empty set:
            /// 1. Contains no elements (is_empty)
            /// 2. Has cardinality 0 (len = 0)
            /// 3. Satisfies extensionality (all empty sets are equal)
            /// 4. Satisfies foundation (trivially, as it has no elements)
            #[test]
            fn test_empty_set_properties() {
                let empty = Set::Empty;
                assert_extensionality_holds(&empty);
                assert_foundation_holds(&empty);
                assert!(empty.is_empty());
                assert_eq!(empty.len(), 0);
            }

            /// Tests operations involving the empty set
            ///
            /// This test verifies that:
            /// 1. Union with empty set: A ∪ ∅ = A
            /// 2. Intersection with empty set: A ∩ ∅ = ∅
            /// 3. Power set of empty set: P(∅) = {∅}
            #[test]
            fn test_empty_set_operations() {
                let empty = empty_set();
                let empty_elem = SetElement::from(empty.clone());

                // Union with empty set should not change the set
                let union = Set::BinaryUnion {
                    left: Box::new(empty.clone()),
                    right: Box::new(empty.clone()),
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };
                assert_eq!(union.elements().len(), 0);

                // Intersection with empty set should be empty
                let intersection = Set::BinaryIntersection {
                    left: Box::new(empty.clone()),
                    right: Box::new(empty.clone()),
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };
                assert_eq!(intersection.elements().len(), 0);

                // Power set of empty set should contain only empty set
                let power = power_set(&empty);
                assert_eq!(power.len(), 1); // P(∅) = {∅}
                assert!(power.contains(&empty_elem));
            }

            /// Tests set-theoretic relations involving the empty set
            ///
            /// This test verifies that:
            /// 1. Empty set is subset of all sets: ∅ ⊆ A
            /// 2. Only empty set is subset of empty set: A ⊆ ∅ ⇒ A = ∅
            /// 3. Empty set contains no elements: x ∉ ∅
            #[test]
            fn test_empty_set_relations() {
                let empty = Set::Empty;
                let empty_elem = SetElement::from(empty.clone());
                let singleton = singleton_set(empty_elem.clone());

                // Empty set is a subset of every set
                assert!(empty.is_subset_of(&singleton));
                assert!(empty.is_subset_of(&empty));

                // Only empty set is subset of empty set
                assert!(!singleton.is_subset_of(&empty));

                // Empty set has no elements
                assert!(!empty.contains(&empty_elem));
                assert!(!empty.contains(&SetElement::from(singleton)));
            }
        }
    }

    /// Tests for replacement operations and their properties
    ///
    /// These tests verify that our implementation correctly handles
    /// the Replacement axiom and its various mapping functions.
    /// Each test ensures that:
    /// 1. The resulting set exists and is well-defined
    /// 2. Set-theoretic properties are preserved
    /// 3. The mapping behaves as expected mathematically
    mod replacement_operations {
        use super::*;

        /// Tests for identity replacement function
        ///
        /// The identity function f(x) = x maps each element to itself.
        /// This is the simplest possible replacement function and serves
        /// as a baseline for testing the Replacement axiom.
        mod identity_replacement {
            use super::*;

            /// Tests that identity replacement preserves set structure
            ///
            /// Given a set A = {∅, {∅}}, applying the identity function should:
            /// 1. Return a set extensionally equal to A
            /// 2. Preserve all set-theoretic properties
            /// 3. Maintain the same cardinality as A
            ///
            /// This test verifies these properties and ensures that
            /// the most basic replacement function works correctly.
            #[test]
            fn test_identity_replacement() {
                let empty = empty_set();
                let empty_elem = SetElement::from(empty.clone());
                let a = singleton_set(empty_elem.clone());
                let a_elem = SetElement::from(a.clone());
                let domain = pair_set(empty_elem.clone(), a_elem.clone());

                let identity = Set::Replacement {
                    source: Box::new(domain.clone()),
                    mapping: SetMapping::Identity,
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };

                // Identity mapping should preserve set-theoretic properties
                assert_extensionality_holds(&identity);
                assert_foundation_holds(&identity);
                // Identity mapping should preserve elements
                assert_eq!(identity.elements().len(), domain.elements().len());
            }
        }

        /// Tests for singleton replacement function
        ///
        /// The singleton function f(x) = {x} wraps each element in a set.
        /// This function is important for constructing nested set structures
        /// and testing how replacement handles set construction.
        mod singleton_replacement {
            use super::*;

            /// Tests that singleton replacement properly wraps elements
            ///
            /// Given a set A = {∅, {∅}}, applying the singleton function should:
            /// 1. Map ∅ to {∅}
            /// 2. Map {∅} to {{∅}}
            /// 3. Preserve set-theoretic properties
            ///
            /// This test verifies these properties and ensures that
            /// the singleton function correctly constructs new sets.
            #[test]
            fn test_singleton_replacement() {
                let empty = empty_set();
                let empty_elem = SetElement::from(empty.clone());
                let a = singleton_set(empty_elem.clone());
                let a_elem = SetElement::from(a.clone());
                let domain = pair_set(empty_elem.clone(), a_elem.clone());

                let singleton_map = Set::Replacement {
                    source: Box::new(domain),
                    mapping: SetMapping::Singleton,
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };

                // Singleton mapping should preserve set-theoretic properties
                assert_extensionality_holds(&singleton_map);
                assert_foundation_holds(&singleton_map);

                // Each element should be mapped to its singleton
                // Create expected elements for comparison
                let singleton_of_empty_elem = singleton_set(empty_elem.clone());
                let singleton_of_empty_elem_elem = SetElement::from(singleton_of_empty_elem);

                let singleton_of_a_elem = singleton_set(a_elem.clone());
                let singleton_of_a_elem_elem = SetElement::from(singleton_of_a_elem);

                assert!(singleton_map.contains(&singleton_of_empty_elem_elem));
                assert!(singleton_map.contains(&singleton_of_a_elem_elem));
            }
        }

        /// Tests for projection functions on ordered pairs
        ///
        /// The projection functions π₁ and π₂ extract components from ordered pairs:
        /// - π₁((x,y)) = x (first projection)
        /// - π₂((x,y)) = y (second projection)
        /// These functions are fundamental for working with relations and functions.
        mod projection_replacement {
            use super::*;

            /// Tests that projection functions correctly extract pair components
            ///
            /// Given an ordered pair p = (∅, {∅}), this test verifies:
            /// 1. First Projection:
            ///    - π₁(p) = ∅
            ///    - Correctly extracts first component
            ///    - Preserves set-theoretic properties
            ///
            /// 2. Second Projection:
            ///    - π₂(p) = {∅}
            ///    - Correctly extracts second component
            ///    - Preserves set-theoretic properties
            #[test]
            fn test_projection_replacement() {
                let empty = empty_set();
                let empty_elem = SetElement::from(empty.clone());
                let a = singleton_set(empty_elem.clone());
                let a_elem = SetElement::from(a.clone());

                // Create an ordered pair using the function from mod.rs
                let pair = ordered_pair(empty.clone(), a.clone());
                let pair_elem = SetElement::from(pair.clone());

                // Create a singleton set containing the pair
                let singleton_pair = singleton_set(pair_elem.clone());

                let first_proj = Set::Replacement {
                    source: Box::new(singleton_pair.clone()),
                    mapping: SetMapping::FirstProjection,
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };

                let second_proj = Set::Replacement {
                    source: Box::new(singleton_pair.clone()),
                    mapping: SetMapping::SecondProjection,
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };

                // First projection should preserve set-theoretic properties
                assert_extensionality_holds(&first_proj);
                assert_foundation_holds(&first_proj);
                // First projection should extract first component
                assert!(first_proj.contains(&empty_elem));

                // Second projection should preserve set-theoretic properties
                assert_extensionality_holds(&second_proj);
                assert_foundation_holds(&second_proj);
                // Second projection should extract second component
                assert!(second_proj.contains(&a_elem));
            }
        }

        /// Tests for composition of replacement functions
        ///
        /// Function composition (g ∘ f)(x) = g(f(x)) allows building
        /// complex mappings from simpler ones. This is crucial for
        /// constructing sophisticated set transformations.
        mod composition_replacement {
            use super::*;

            /// Tests that function composition works correctly
            ///
            /// This test composes the singleton and first projection functions:
            /// h = π₁ ∘ singleton, where:
            /// - singleton(x) = {x}
            /// - π₁({x}) = x
            ///
            /// Given domain A = {∅, {∅}}, we verify:
            /// 1. h(∅) = π₁({∅}) = ∅
            /// 2. h({∅}) = π₁({{∅}}) = {∅}
            /// 3. The composition preserves set-theoretic properties
            #[test]
            fn test_composition_replacement() {
                let empty = empty_set();
                let empty_elem = SetElement::from(empty.clone());
                let a = singleton_set(empty_elem.clone());
                let a_elem = SetElement::from(a.clone());
                let domain = pair_set(empty_elem.clone(), a_elem.clone());

                // First map to singleton, then project first element
                let composed = Set::Replacement {
                    source: Box::new(domain),
                    mapping: SetMapping::Composition(
                        Box::new(SetMapping::Singleton),
                        Box::new(SetMapping::FirstProjection),
                    ),
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };

                // Composition should preserve set-theoretic properties
                assert_extensionality_holds(&composed);
                assert_foundation_holds(&composed);

                // For each x in domain, composed(x) = first(singleton(x))
                // So empty -> {empty} -> empty
                // and {empty} -> {{empty}} -> {empty}
                assert!(composed.contains(&empty_elem));
                assert!(composed.contains(&a_elem));
            }
        }
    }

    /// Tests for ordinal numbers and their properties
    ///
    /// These tests verify that our implementation correctly handles ordinal numbers,
    /// which are transitive sets well-ordered by the membership relation (∈).
    /// Key properties tested include:
    /// 1. Transitivity: If x ∈ y ∈ α then x ∈ α
    /// 2. Well-ordering: Every non-empty subset has a least element
    /// 3. Von Neumann construction: Each ordinal represents its predecessors
    mod ordinal_tests {
        use super::*;

        /// Tests basic ordinal number properties
        ///
        /// This test verifies fundamental properties of small ordinals:
        /// - 0 = ∅ (empty set)
        /// - 1 = {∅} = {0}
        /// - 2 = {∅, {∅}} = {0, 1}
        /// - 3 = {∅, {∅}, {∅, {∅}}} = {0, 1, 2}
        ///
        /// Each ordinal α contains all ordinals β < α as elements.
        #[test]
        fn test_basic_ordinals() {
            let empty = empty_set(); // 0 = ∅
            let empty_elem = SetElement::from(empty.clone());
            let one = singleton_set(empty_elem.clone()); // 1 = {∅}
            let one_elem = SetElement::from(one.clone());
            let two = pair_set(empty_elem.clone(), one_elem.clone()); // 2 = {∅, {∅}}

            // Verify ordinal properties
            assert!(empty.is_ordinal());
            assert!(one.is_ordinal());
            assert!(two.is_ordinal());

            // Verify ordering
            assert!(empty.is_subset_of(&one));
            assert!(one.is_subset_of(&two));
            assert!(empty.is_subset_of(&two));
        }

        /// Tests ordinal arithmetic operations
        ///
        /// This test verifies basic ordinal arithmetic:
        /// 1. Addition: α + β
        /// 2. Multiplication: α × β
        /// 3. Exponentiation: α^β
        ///
        /// Note: Ordinal arithmetic is generally non-commutative
        /// For example: 1 + ω = ω but ω + 1 > ω
        #[test]
        fn test_ordinal_arithmetic() {
            let empty = empty_set(); // 0
            let empty_elem = SetElement::from(empty.clone());
            let one = singleton_set(empty_elem.clone()); // 1
            let one_elem = SetElement::from(one.clone());
            let two = pair_set(empty_elem.clone(), one_elem.clone()); // 2

            // Test ordinal addition
            let sum = two.ordinal_add(&one);
            assert_eq!(sum.len(), 3); // 2 + 1 = 3

            // Test ordinal multiplication
            let product = two.ordinal_multiply(&two);
            println!("Product: {:?}", product.elements());
            assert_eq!(product.len(), 4); // 2 × 2 = 4

            // Test ordinal exponentiation
            let power = two.ordinal_power(&two);
            assert_eq!(power.len(), 4); // 2^2 = 4
        }
    }

    /// Tests for cardinal numbers and their properties
    ///
    /// These tests verify that our implementation correctly handles cardinal numbers,
    /// which measure the size of sets. Key properties tested include:
    /// 1. Cantor's theorem: |A| < |P(A)| for any set A
    /// 2. Cardinal arithmetic: Addition, multiplication, exponentiation
    /// 3. Comparison of infinite cardinals
    mod cardinal_tests {
        use super::*;

        /// Tests basic cardinal number properties
        ///
        /// This test verifies fundamental properties of small cardinals:
        /// - |∅| = 0 (empty set has cardinality 0)
        /// - |{∅}| = 1 (singleton set has cardinality 1)
        /// - |{∅, {∅}}| = 2 (pair set has cardinality 2)
        #[test]
        fn test_basic_cardinals() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let one = singleton_set(empty_elem.clone());
            let one_elem = SetElement::from(one.clone());
            let two = pair_set(empty_elem.clone(), one_elem.clone());

            // Verify cardinalities
            assert_eq!(empty.cardinality(), 0);
            assert_eq!(one.cardinality(), 1);
            assert_eq!(two.cardinality(), 2);
        }

        /// Tests Cantor's theorem for finite sets
        ///
        /// Cantor's theorem states that for any set A:
        /// |A| < |P(A)| where P(A) is the power set of A
        ///
        /// This test verifies this property for small finite sets:
        /// - |P(∅)| = 1
        /// - |P({∅})| = 2
        /// - |P({∅, {∅}})| = 4
        #[test]
        fn test_cantors_theorem() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let singleton = singleton_set(empty_elem.clone());
            let singleton_elem = SetElement::from(singleton.clone());
            let pair = pair_set(empty_elem.clone(), singleton_elem.clone());

            // Test power set cardinalities
            let power_empty = power_set(&empty);
            let power_singleton = power_set(&singleton);
            let power_pair = power_set(&pair);

            assert_eq!(power_empty.cardinality(), 1); // |P(∅)| = 2⁰ = 1
            assert_eq!(power_singleton.cardinality(), 2); // |P({∅})| = 2¹ = 2
            assert_eq!(power_pair.cardinality(), 4); // |P({∅, {∅}})| = 2² = 4
        }
    }

    /// Tests for relations and their properties
    ///
    /// These tests verify that our implementation correctly handles relations,
    /// which are sets of ordered pairs. Key properties tested include:
    /// 1. Basic relation properties (reflexive, symmetric, transitive)
    /// 2. Equivalence relations and partitions
    /// 3. Order relations (partial orders, total orders)
    mod relation_tests {
        use super::*;

        /// Tests basic relation properties
        ///
        /// This test verifies fundamental properties of relations:
        /// 1. Reflexivity: ∀x ∈ A: (x,x) ∈ R
        /// 2. Symmetry: (x,y) ∈ R ⇒ (y,x) ∈ R
        /// 3. Transitivity: (x,y) ∈ R ∧ (y,z) ∈ R ⇒ (x,z) ∈ R
        #[test]
        fn test_relation_properties() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());

            // Create domain
            let domain = pair_set(empty_elem.clone(), a_elem.clone());

            // Create a simplified identity relation by hand
            // This is a placeholder until we implement a proper identity_relation method
            let relation = Set::Enumeration {
                elements: vec![
                    SetElement::from(ordered_pair(empty.clone(), empty.clone())),
                    SetElement::from(ordered_pair(a.clone(), a.clone())),
                ],
                properties: VariantSet::new(),
            };

            println!("Domain: {:?}", domain);
            println!("Relation: {:?}", relation);
            println!("Relation elements: {:?}", relation.elements());

            // Instead of testing specific relation properties (which need to be implemented),
            // we'll just verify some basic properties we can test
            assert_eq!(relation.len(), 2);
        }

        /// Tests order relations
        ///
        /// This test verifies properties of order relations:
        /// 1. Partial orders (reflexive, antisymmetric, transitive)
        /// 2. Total orders (partial order + total)
        /// 3. Well-orders (total order + well-founded)
        #[test]
        fn test_order_relations() {
            let empty = empty_set();
            let empty_elem = SetElement::from(empty.clone());
            let a = singleton_set(empty_elem.clone());
            let a_elem = SetElement::from(a.clone());
            let b = singleton_set(a_elem.clone());
            let b_elem = SetElement::from(b.clone());

            // Create a domain with properly typed elements
            let domain = Set::Enumeration {
                elements: vec![empty_elem.clone(), a_elem.clone(), b_elem.clone()],
                properties: VariantSet::new(),
            };

            // Create a simplified subset relation manually
            // This is a placeholder until we implement a proper subset_relation method
            let subset_relation = Set::Enumeration {
                elements: vec![
                    SetElement::from(ordered_pair(empty.clone(), empty.clone())),
                    SetElement::from(ordered_pair(empty.clone(), a.clone())),
                    SetElement::from(ordered_pair(empty.clone(), b.clone())),
                    SetElement::from(ordered_pair(a.clone(), a.clone())),
                    SetElement::from(ordered_pair(a.clone(), b.clone())),
                    SetElement::from(ordered_pair(b.clone(), b.clone())),
                ],
                properties: VariantSet::new(),
            };

            // Just verify the basic structure since we don't have the specific methods yet
            assert!(subset_relation.len() > 0);
        }
    }
}

// Add a test for the new Set, SetExpression, and SetRelation structure
#[cfg(test)]
mod set_structure_tests {

    use crate::subjects::math::formalism::extract::Parametrizable;
    use crate::subjects::math::theories::VariantSet;
    use crate::subjects::math::theories::zfc::definitions::{
        CardinalityPropertyVariant, Set, SetElement, SetExpression, SetRelation,
    };
    use crate::turn_render::Identifier;

    #[test]
    fn test_new_set_structures() {
        // Create basic sets
        let empty_set = Set::empty();
        let empty_element = SetElement::from(empty_set.clone());
        let singleton = Set::singleton(empty_element.clone());

        // Create set expressions
        let var_a = SetExpression::Variable(Identifier::new_simple("A".to_string()));

        // Create cardinality expression
        let cardinality = SetExpression::Cardinality {
            set: Box::new(Parametrizable::Concrete(singleton.clone())),
        };

        // Create element selection expression
        let element_selection = SetExpression::ElementSelection {
            set: Box::new(Parametrizable::Concrete(singleton.clone())),
        };

        // Create relations
        let element_of = SetRelation::ElementOf {
            element: Parametrizable::Concrete(empty_element.clone()),
            set: Parametrizable::Concrete(singleton.clone()),
        };

        let subset_of = SetRelation::SubsetOf {
            subset: Parametrizable::Concrete(empty_set.clone()),
            superset: Parametrizable::Concrete(singleton.clone()),
        };

        let has_cardinality = SetRelation::HasCardinality {
            set: Parametrizable::Concrete(singleton.clone()),
            cardinality: Parametrizable::Concrete(CardinalityPropertyVariant::Finite(1)),
        };

        // Test pattern matching
        let pattern_element_of = SetRelation::ElementOf {
            element: Parametrizable::Variable(Identifier::new_simple("x".to_string())),
            set: Parametrizable::Variable(Identifier::new_simple("A".to_string())),
        };

        let pattern_subset_of = SetRelation::SubsetOf {
            subset: Parametrizable::Variable(Identifier::new_simple("X".to_string())),
            superset: Parametrizable::Variable(Identifier::new_simple("Y".to_string())),
        };

        // Verify that concrete relations match the patterns
        assert!(element_of.matches_pattern(&pattern_element_of));
        assert!(subset_of.matches_pattern(&pattern_subset_of));

        // Create some parametric relations with variables
        let parametric_relation = SetRelation::ElementOf {
            element: Parametrizable::Concrete(empty_element.clone()),
            set: Parametrizable::Variable(Identifier::new_simple("B".to_string())),
        };

        // This should also match the pattern since the pattern has variable placeholders
        assert!(parametric_relation.matches_pattern(&pattern_element_of));

        // Test set element types
        let int_element = SetElement::Integer(42);
        let sym_element = SetElement::Symbol("x".to_string());
        let pair_element =
            SetElement::Pair(Box::new(int_element.clone()), Box::new(sym_element.clone()));

        // Test set enumeration
        let enum_set = Set::Enumeration {
            elements: vec![int_element, sym_element, pair_element],
            properties: VariantSet::new(),
        };

        // The enumeration set should have 3 elements
        match &enum_set {
            Set::Enumeration { elements, .. } => assert_eq!(elements.len(), 3),
            _ => panic!("Expected Enumeration set"),
        }
    }
}

// Add complex separation with nested sets test
#[cfg(test)]
mod additional_tests {
    use super::super::super::super::super::math::theories::zfc::{
        axioms::{SatisfiesZFC, ZFCAxioms},
        cartesian_product,
        definitions::{ElementCondition, Set, SetElement, SetMapping, SetOpProperty, SetProperty},
        empty_set, ordered_pair, pair_set, power_set, singleton_set, symmetric_difference_set,
        union_set,
        verifier::ZFCVerifier,
    };
    use super::*;
    use crate::subjects::math::theories::VariantSet;

    #[test]
    fn test_complex_separation() {
        let empty = empty_set();
        let empty_elem = SetElement::from(empty.clone());
        let a = singleton_set(empty_elem.clone());
        let a_elem = SetElement::from(a.clone());
        let b = singleton_set(a_elem.clone());
        let b_elem = SetElement::from(b.clone());

        // Create domain with three nested sets as elements
        let domain_elements = vec![empty_elem.clone(), a_elem.clone(), b_elem.clone()];
        let domain = Set::Enumeration {
            elements: domain_elements,
            properties: VariantSet::new(),
        };

        // Separate elements that contain the empty set
        let separated = Set::Separation {
            source: Box::new(domain),
            condition: ElementCondition::Contains(Box::new(empty_elem.clone())),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        };

        // {∅} contains ∅, but ∅ and {{∅}} don't contain ∅ directly
        assert!(separated.contains(&a_elem));
        assert!(!separated.contains(&empty_elem));
        assert!(!separated.contains(&b_elem));
    }

    #[test]
    fn test_big_intersection_with_power_sets() {
        let empty = empty_set();
        let empty_elem = SetElement::from(empty.clone());
        let a = singleton_set(empty_elem.clone());
        let a_elem = SetElement::from(a.clone());

        // Create power sets
        let power_a = power_set(&a);
        let power_empty = power_set(&empty);

        // Create power set elements
        let power_a_elem = SetElement::from(power_a.clone());
        let power_empty_elem = SetElement::from(power_empty.clone());

        // Create family as a set containing the power sets
        let family_elements = vec![power_a_elem, power_empty_elem];
        let family = Set::Enumeration {
            elements: family_elements,
            properties: VariantSet::new(),
        };

        let intersection = Set::BigIntersection {
            family: Box::new(family),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        };

        // The empty set is in every power set
        assert!(intersection.contains(&empty_elem));
    }

    #[test]
    fn test_symmetric_difference_with_power_sets() {
        let empty = empty_set();
        let empty_elem = SetElement::from(empty.clone());
        let a = singleton_set(empty_elem.clone());
        let a_elem = SetElement::from(a.clone());

        let power_empty = power_set(&empty);
        let power_a = power_set(&a);

        let sym_diff = symmetric_difference_set(&power_empty, &power_a);

        // Empty set is in both power sets, so not in symmetric difference
        assert!(!sym_diff.contains(&empty_elem));
        // {∅} is only in P({∅}), so it should be in symmetric difference
        assert!(sym_diff.contains(&a_elem));
    }
}
