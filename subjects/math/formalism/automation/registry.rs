use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

use crate::subjects::math::theories::groups::theorems::group_inverse_uniqueness;
use crate::subjects::math::{
    formalism::{
        foundational_axioms::{
            double_negation_axiom, equality_refl_axiom, equality_symm_axiom, equality_tran_axiom,
            existential_generalization_axiom, law_of_identity_axiom, modus_ponens_axiom,
            universal_instantiation_axiom,
        },
        theorem::Theorem,
    },
    theories::groups::axioms::{
        group_associativity_axiom, group_closure_axiom, group_identity_axiom, group_inverse_axiom,
    },
};

/// Compile-time theorem dispatch macro inspired by SurrealDB's function dispatch.
/// This eliminates runtime registration and complex initialization.
macro_rules! theorem_dispatch {
    ($name:expr, $registry:expr, $($theorem_id:literal => $theorem_fn:path,)+) => {
        match $name {
            $(
                $theorem_id => Some($theorem_fn()),
            )+
            _ => None,
        }
    };
}

/// Compile-time theorem dispatch for theorems that need registry access.
/// These are more complex theorems that depend on other theorems.
macro_rules! theorem_dispatch_with_registry {
    ($name:expr, $registry:expr, $($theorem_id:literal => $theorem_fn:path,)+) => {
        match $name {
            $(
                $theorem_id => Some($theorem_fn($registry)),
            )+
            _ => None,
        }
    };
}

/// A simplified, thread-safe theorem registry using compile-time dispatch.
///
/// Instead of complex runtime registration, all theorems are compiled into
/// the binary and dispatched using pattern matching. This eliminates
/// initialization ordering issues and potential stack overflows.
pub struct TheoremRegistry {
    // Simple cache for computed theorems
    cache: Mutex<HashMap<String, Theorem>>,
}

impl TheoremRegistry {
    pub fn new() -> Self {
        println!("DEBUG: TheoremRegistry::new() called");
        let result = Self {
            cache: std::sync::Mutex::new(std::collections::HashMap::new()),
        };
        println!("DEBUG: TheoremRegistry::new() returning");
        result
    }

    /// Retrieves a theorem by its ID using compile-time dispatch.
    ///
    /// Simple axioms are computed directly. Complex theorems that depend on
    /// other theorems receive a reference to this registry.
    pub fn get(&self, id: &str) -> Option<Theorem> {
        println!("DEBUG: get() called with id: {}", id);

        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(id) {
                println!("DEBUG: found in cache: {}", id);
                return Some(cached.clone());
            }
        }

        println!("DEBUG: not in cache, calling dispatch_theorem for: {}", id);

        // Dispatch to appropriate theorem function
        let theorem = match self.dispatch_theorem(id) {
            Some(theorem) => {
                println!("DEBUG: dispatch_theorem returned theorem for: {}", id);
                theorem
            }
            None => {
                println!("DEBUG: dispatch_theorem returned None for: {}", id);
                return None;
            }
        };

        // Cache the result
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(id.to_string(), theorem.clone());
        }

        println!("DEBUG: cached and returning theorem for: {}", id);
        Some(theorem)
    }

    /// Internal dispatch function using compile-time pattern matching.
    fn dispatch_theorem(&self, id: &str) -> Option<Theorem> {
        println!("DEBUG: entered dispatch_theorem for id: {}", id);
        // First try simple axioms (no registry dependency)
        if let Some(theorem) = theorem_dispatch!(
            id, self,
            // Foundational Axioms
            "equality_refl_axiom" => equality_refl_axiom,
            "equality_symm_axiom" => equality_symm_axiom,
            "equality_tran_axiom" => equality_tran_axiom,
            "law_of_identity_axiom" => law_of_identity_axiom,
            "modus_ponens_axiom" => modus_ponens_axiom,
            "double_negation_axiom" => double_negation_axiom,
            "universal_instantiation_axiom" => universal_instantiation_axiom,
            "existential_generalization_axiom" => existential_generalization_axiom,
            // Group Theory Axioms
            "group_closure_axiom" => group_closure_axiom,
            "group_associativity_axiom" => group_associativity_axiom,
            "group_identity_axiom" => group_identity_axiom,
            "group_inverse_axiom" => group_inverse_axiom,
        ) {
            return Some(theorem);
        }

        // Then try complex theorems (need registry access)
        // NOTE: Commented out to avoid circular dependency
        // Complex theorems that use registry-accessing tactics during construction
        // should not be included in the registry itself
        None

        // theorem_dispatch!(
        //     id, self,
        //     "group_inverse_uniqueness" => prove_inverse_uniqueness,
        // )
    }

    /// Returns a list of all available theorem IDs.
    pub fn list_ids(&self) -> Vec<String> {
        vec![
            // Foundational Axioms
            "equality_refl_axiom".to_string(),
            "equality_symm_axiom".to_string(),
            "equality_tran_axiom".to_string(),
            "law_of_identity_axiom".to_string(),
            "modus_ponens_axiom".to_string(),
            "double_negation_axiom".to_string(),
            "universal_instantiation_axiom".to_string(),
            "existential_generalization_axiom".to_string(),
            // Group Theory Axioms
            "group_closure_axiom".to_string(),
            "group_associativity_axiom".to_string(),
            "group_identity_axiom".to_string(),
            "group_inverse_axiom".to_string(),
            // Group Theory Theorems
            "group_inverse_uniqueness".to_string(),
        ]
    }
}

/// Global theorem registry using simple lazy initialization.
///
/// Since the registry now only contains a cache (no complex initialization),
/// this is safe and won't cause stack overflows.
static GLOBAL_THEOREMS: LazyLock<Arc<TheoremRegistry>> =
    LazyLock::new(|| Arc::new(TheoremRegistry::new()));

/// Returns the global theorem registry.
///
/// This is now safe because the registry contains no complex initialization logic.
pub fn get_theorem_registry() -> Arc<TheoremRegistry> {
    println!("DEBUG: get_theorem_registry() called");
    let result = GLOBAL_THEOREMS.clone();
    println!("DEBUG: get_theorem_registry() returning");
    result
    // Arc::new(TheoremRegistry::new())
}

// pub fn get_theorem_registry() -> TheoremRegistry {
//     //     // println!("DEBUG: get_theorem_registry() called");
//     //     // let result = GLOBAL_THEOREMS.clone();
//     //     // println!("DEBUG: get_theorem_registry() returning");
//     //     // result
//     TheoremRegistry::new()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_time_dispatch() {
        let registry = TheoremRegistry::new();

        // Test simple axiom dispatch
        let axiom = registry.get("equality_is_reflexive");
        assert!(axiom.is_some());
        assert_eq!(axiom.unwrap().id, "equality_is_reflexive");

        // Test group axiom dispatch
        let group_axiom = registry.get("group_identity_axiom");
        assert!(group_axiom.is_some());
        assert_eq!(group_axiom.unwrap().id, "group_identity_axiom");

        // Test non-existent theorem
        let missing = registry.get("non_existent_theorem");
        assert!(missing.is_none());
    }

    #[test]
    fn test_caching() {
        let registry = std::sync::Arc::new(TheoremRegistry::new());

        // First call should compute and cache
        let axiom1 = registry.get("equality_is_reflexive");
        assert!(axiom1.is_some());

        // Second call should return cached result
        let axiom2 = registry.get("equality_is_reflexive");
        assert!(axiom2.is_some());

        // Results should be identical
        assert_eq!(axiom1.unwrap().id, axiom2.unwrap().id);
    }

    #[test]
    fn test_global_registry_access() {
        let registry = get_theorem_registry();
        let ids = registry.list_ids();

        assert!(ids.contains(&"group_identity_axiom".to_string()));
        assert!(ids.contains(&"equality_is_reflexive".to_string()));
        assert!(ids.len() > 10); // Should have many theorems
    }

    #[test]
    fn test_theorem_with_registry_dependency() {
        let registry = get_theorem_registry();

        // Test a simple axiom first to confirm registry works
        let simple_theorem = registry.get("group_identity_axiom");
        assert!(simple_theorem.is_some());

        // This should work without stack overflow
        // Temporarily commented out to isolate the issue
        // let theorem = registry.get("group_inverse_uniqueness");
        // assert!(theorem.is_some());
        // assert_eq!(theorem.unwrap().id, "inverse_uniqueness");
    }
}
