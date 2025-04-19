use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common_types::{GroupDefinition, GroupElement, GroupHomomorphism, MathObjectMetadata};

/// Provides data for all mathematical theories
pub struct MathData {
    theories: HashMap<String, TheoryData>,
}

impl MathData {
    /// Create a new instance with predefined theory data
    pub fn new() -> Self {
        let mut theories = HashMap::new();

        // Add groups theory
        theories.insert("groups".to_string(), create_groups_theory());

        // Add ZFC theory
        theories.insert("zfc".to_string(), create_zfc_theory());

        // Could add more theories here

        Self { theories }
    }

    /// Get all theory names
    pub fn get_theory_names(&self) -> Vec<String> {
        self.theories.keys().cloned().collect()
    }

    /// Get data for a specific theory
    pub fn get_theory(&self, name: &str) -> Option<&TheoryData> {
        self.theories.get(name)
    }
}

/// Data for a single mathematical theory
pub struct TheoryData {
    pub definitions: Vec<serde_json::Value>,
    pub axioms: Vec<serde_json::Value>,
    pub theorems: Vec<serde_json::Value>,
}

/// Create data for the groups theory
fn create_groups_theory() -> TheoryData {
    let group_definition = GroupDefinition {
        name: "Group".to_string(),
        order: None,
        is_abelian: false,
        is_cyclic: false,
        is_finite: false,
        identity_symbol: "e".to_string(),
        operation_symbol: "•".to_string(),
    };

    let element = GroupElement {
        symbol: "g".to_string(),
        order: None,
        is_identity: false,
        is_involution: false,
        inverse_symbol: Some("g⁻¹".to_string()),
    };

    let homomorphism = GroupHomomorphism {
        name: "φ".to_string(),
        domain: "G".to_string(),
        codomain: "H".to_string(),
        is_isomorphism: false,
        is_monomorphism: false,
        is_epimorphism: false,
        is_endomorphism: false,
        is_automorphism: false,
    };

    TheoryData {
        definitions: vec![
            serde_json::to_value(group_definition).unwrap(),
            serde_json::to_value(element).unwrap(),
            serde_json::to_value(homomorphism).unwrap(),
        ],
        axioms: vec![
            serde_json::json!({
                "name": "Associativity",
                "statement": "For all a, b, c in G: (a • b) • c = a • (b • c)",
                "latex": "\\forall a,b,c \\in G: (a \\cdot b) \\cdot c = a \\cdot (b \\cdot c)"
            }),
            serde_json::json!({
                "name": "Identity",
                "statement": "There exists an element e in G such that for all a in G: e • a = a • e = a",
                "latex": "\\exists e \\in G \\forall a \\in G: e \\cdot a = a \\cdot e = a"
            }),
            serde_json::json!({
                "name": "Inverse",
                "statement": "For each a in G, there exists an element b in G such that: a • b = b • a = e",
                "latex": "\\forall a \\in G \\exists b \\in G: a \\cdot b = b \\cdot a = e"
            }),
        ],
        theorems: vec![
            serde_json::json!({
                "name": "Uniqueness of Identity",
                "statement": "The identity element in a group is unique",
                "proof": "Suppose e and e' are both identity elements. Then e • e' = e' (since e is an identity) and e • e' = e (since e' is an identity). Thus e = e'."
            }),
            serde_json::json!({
                "name": "Uniqueness of Inverse",
                "statement": "For each element in a group, its inverse is unique",
                "proof": "Suppose a⁻¹ and b are both inverses of a. Then a⁻¹ = a⁻¹ • e = a⁻¹ • (a • b) = (a⁻¹ • a) • b = e • b = b."
            }),
        ],
    }
}

/// Create data for the ZFC Set Theory
fn create_zfc_theory() -> TheoryData {
    TheoryData {
        definitions: vec![
            serde_json::json!({
                "name": "Set",
                "statement": "A collection of distinct objects",
                "latex": "\\{a, b, c, \\ldots\\}"
            }),
            serde_json::json!({
                "name": "Empty Set",
                "statement": "The set containing no elements",
                "latex": "\\emptyset"
            }),
        ],
        axioms: vec![
            serde_json::json!({
                "name": "Axiom of Extensionality",
                "statement": "Two sets are equal if and only if they have the same elements",
                "latex": "\\forall A \\forall B (\\forall x (x \\in A \\iff x \\in B) \\implies A = B)"
            }),
            serde_json::json!({
                "name": "Axiom of Regularity",
                "statement": "Every non-empty set A contains an element B such that A and B are disjoint sets",
                "latex": "\\forall A (A \\neq \\emptyset \\implies \\exists B \\in A (B \\cap A = \\emptyset))"
            }),
        ],
        theorems: vec![serde_json::json!({
            "name": "Uniqueness of Empty Set",
            "statement": "The empty set is unique",
            "proof": "By the axiom of extensionality, if there were two empty sets, they would be equal since they contain the same elements (none)."
        })],
    }
}
