use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    thread,
};

use crate::subjects::math::{
    formalism::{
        foundational_axioms::{
            double_negation_axiom, equality_refl_axiom, equality_symm_axiom, equality_tran_axiom,
            existential_generalization_axiom, law_of_identity_axiom, modus_ponens_axiom,
            universal_instantiation_axiom,
        },
        group_identity_theorem_2,
        theorem::Theorem,
    },
    theories::groups::axioms::{
        group_associativity_axiom, group_closure_axiom, group_identity_axiom,
        group_identity_theorem, group_inverse_axiom, test_theorem, test_theorem_2,
    },
};

/// Represents the state of a theorem in the registry.
/// This allows for lazy, on-demand proving.
enum TheoremEntry {
    /// The proof has not been run yet. Stores the function to run it.
    Thunk(Box<dyn Fn() -> Theorem + Send + Sync>),
    /// The theorem is currently being proven by a thread. This helps detect cycles.
    Proving,
    /// The theorem has been successfully proven and its result is cached.
    Proven(Theorem),
}

/// A thread-safe, lazily-initialized registry for theorems.
///
/// Theorems are registered as functions ("thunks") and are only proven when
/// they are first requested. The results are then cached for future lookups.
/// This approach prevents stack overflows from recursive initialization and
/// avoids unnecessary computation by only proving theorems as needed.
pub struct TheoremRegistry {
    // Each entry is behind its own lock to allow for fine-grained, concurrent proving.
    // The map itself is also behind a lock to allow for thread-safe registration.
    theorems: Mutex<HashMap<String, Arc<Mutex<TheoremEntry>>>>,
}

impl TheoremRegistry {
    pub fn new() -> Self {
        Self {
            theorems: Mutex::new(HashMap::new()),
        }
    }

    /// Registers a theorem's ID and the function that can prove it.
    /// The function is not executed until the theorem is requested.
    pub fn register(&self, id: String, proof_fn: Box<dyn Fn() -> Theorem + Send + Sync>) {
        self.theorems
            .lock()
            .unwrap()
            .insert(id, Arc::new(Mutex::new(TheoremEntry::Thunk(proof_fn))));
    }

    /// Retrieves a theorem by its ID.
    ///
    /// If the theorem has not been proven yet, this will trigger its proof function,
    /// cache the result, and then return it. If the proof is already running in
    /// another thread, this will block until it's done.
    ///
    /// # Panics
    /// Panics if a circular dependency is detected (e.g., Theorem A's proof
    /// requires Theorem B, and Theorem B's proof requires Theorem A).
    pub fn get(&self, id: &str) -> Option<Theorem> {
        // Find the entry for the requested theorem. We lock the entire map only for the
        // duration of this lookup, then release it.
        let entry_arc = {
            let theorems = self.theorems.lock().unwrap();
            theorems.get(id)?.clone()
        };

        // Lock this specific theorem's entry.
        // Other theorems can be proven in parallel.
        let mut entry_guard = entry_arc.lock().unwrap();

        // Check the state of the theorem.
        match &*entry_guard {
            TheoremEntry::Proven(theorem) => {
                // It's already proven, just return a clone.
                return Some(theorem.clone());
            }
            TheoremEntry::Proving => {
                // If we hit this, it means we have a cyclic dependency!
                // A -> B -> A. This is a fatal logic error in the proofs.
                panic!(
                    "Cyclic dependency detected when trying to prove theorem: {}",
                    id
                );
            }
            TheoremEntry::Thunk(_) => {
                // The theorem needs to be proven.
                // Take the proof function out, replacing it with `Proving`.
                let thunk = match std::mem::replace(&mut *entry_guard, TheoremEntry::Proving) {
                    TheoremEntry::Thunk(thunk) => thunk,
                    _ => unreachable!(), // Should not happen due to the match guard
                };

                // IMPORTANT: Drop the lock *before* running the proof.
                // This allows other threads to access other theorems and prevents deadlocks.
                drop(entry_guard);

                test_theorem();
                test_theorem_2();
                group_identity_theorem();
                group_identity_theorem_2();
                group_identity_axiom();

                // Now, run the potentially long-running proof function.
                let proven_theorem = thunk();

                // Re-acquire the lock to cache the result.
                let mut entry_guard = entry_arc.lock().unwrap();
                *entry_guard = TheoremEntry::Proven(proven_theorem.clone());

                return Some(proven_theorem);
            }
        }
    }

    pub fn list_ids(&self) -> Vec<String> {
        self.theorems.lock().unwrap().keys().cloned().collect()
    }
}

static GLOBAL_THEOREMS: OnceLock<Arc<TheoremRegistry>> = OnceLock::new();

pub fn get_theorem_registry() -> Arc<TheoremRegistry> {
    GLOBAL_THEOREMS
        .get_or_init(|| {
            let mut registry = TheoremRegistry::new();

            // --- Foundational Axioms ---
            {
                registry.register(
                    "equality_is_reflexive".to_string(),
                    Box::new(equality_refl_axiom),
                );
                registry.register(
                    "equality_is_symmetric".to_string(),
                    Box::new(equality_symm_axiom),
                );
                registry.register(
                    "equality_is_transitive".to_string(),
                    Box::new(equality_tran_axiom),
                );
                registry.register(
                    "law_of_identity".to_string(),
                    Box::new(law_of_identity_axiom),
                );
                registry.register("modus_ponens".to_string(), Box::new(modus_ponens_axiom));
                registry.register(
                    "double_negation".to_string(),
                    Box::new(double_negation_axiom),
                );
                registry.register(
                    "universal_instantiation".to_string(),
                    Box::new(universal_instantiation_axiom),
                );
                registry.register(
                    "existential_generalization".to_string(),
                    Box::new(existential_generalization_axiom),
                );
            }

            // --- Group Theory Axioms ---
            {
                registry.register(
                    "group_closure_axiom".to_string(),
                    Box::new(group_closure_axiom),
                );
                registry.register(
                    "group_associativity_axiom".to_string(),
                    Box::new(group_associativity_axiom),
                );
                registry.register(
                    "group_identity_axiom".to_string(),
                    Box::new(group_identity_axiom),
                );
                registry.register(
                    "group_inverse_axiom".to_string(),
                    Box::new(group_inverse_axiom),
                );
            }

            Arc::new(registry)
        })
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::formalism::{
        proof::{ProofForest, ProofGoal},
        theorem::Axiom,
    };

    #[test]
    fn test_lazy_proving_and_caching() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let mut registry = TheoremRegistry::new();
        let call_count = Arc::new(AtomicUsize::new(0));

        let call_count_clone = call_count.clone();
        let proof_fn = move || -> Axiom {
            call_count_clone.fetch_add(1, Ordering::SeqCst);
            equality_refl_axiom()
        };

        registry.register("test_axiom".to_string(), Box::new(proof_fn));

        // The proof function should not have been called yet.
        assert_eq!(call_count.load(Ordering::SeqCst), 0);

        // First call: should trigger the proof function.
        let axiom1 = registry.get("test_axiom");
        assert!(axiom1.is_some());
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // Second call: should return the cached result without calling the function again.
        let axiom2 = registry.get("test_axiom");
        assert!(axiom2.is_some());
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // Verify the theorems are identical.
        assert_eq!(axiom1.unwrap().id, axiom2.unwrap().id);
    }

    #[test]
    #[should_panic(expected = "Cyclic dependency detected")]
    fn test_cyclic_dependency_detection() {
        let registry = Arc::new(TheoremRegistry::new());

        let registry_clone_a = registry.clone();
        registry.register(
            "cycle_a".to_string(),
            Box::new(move || {
                println!("Proving A, requires B...");
                registry_clone_a.get("cycle_b"); // Depends on B
                equality_refl_axiom()
            }),
        );

        let registry_clone_b = registry.clone();
        registry.register(
            "cycle_b".to_string(),
            Box::new(move || {
                println!("Proving B, requires A...");
                registry_clone_b.get("cycle_a"); // Depends on A
                equality_refl_axiom()
            }),
        );

        // This should panic.
        registry.get("cycle_a");
    }
}
