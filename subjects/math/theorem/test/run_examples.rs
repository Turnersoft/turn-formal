// Module: src/formalize_v2/subjects/math/theorem/test/run_examples.rs
// Command to run and demonstrate the proof builder examples

use super::advanced_example::{prove_first_isomorphism_theorem, prove_with_closures};
use super::case_builder_example::{
    prove_absolute_value_with_builder, prove_complex_theorem_with_builder,
    prove_number_theory_theorem_with_builder,
};
use super::declarative_example::{
    prove_absolute_value_declarative, prove_complex_theorem_declarative,
    prove_group_associativity_declarative,
};
use super::path_example::{prove_by_cases, prove_with_path_naming};
use super::proof_example::{
    prove_group_associativity, prove_with_bookmarks, prove_with_named_steps,
};
use super::real_world_example::{prove_fermats_little_theorem, prove_quadratic_formula};

/// Run all proof examples and display their visualization
pub fn run_all_examples() {
    println!("\n=== GROUP ASSOCIATIVITY PROOF ===\n");
    let theorem1 = prove_group_associativity();
    println!("\nProof completed with theorem: {}", theorem1.name);

    println!("\n=== PROOF WITH BOOKMARKS ===\n");
    let theorem2 = prove_with_bookmarks();
    println!("\nProof completed with theorem: {}", theorem2.name);

    println!("\n=== PROOF WITH NAMED STEPS ===\n");
    let theorem3 = prove_with_named_steps();
    println!("\nProof completed with theorem: {}", theorem3.name);

    println!("\n=== FIRST ISOMORPHISM THEOREM PROOF ===\n");
    let theorem4 = prove_first_isomorphism_theorem();
    println!("\nProof completed with theorem: {}", theorem4.name);

    println!("\n=== SYLOW THEOREM PROOF ===\n");
    let theorem5 = prove_with_closures();
    println!("\nProof completed with theorem: {}", theorem5.name);

    println!("\n=== PATH-BASED NAMING EXAMPLE ===\n");
    let theorem6 = prove_with_path_naming();
    println!("\nProof completed with theorem: {}", theorem6.name);

    println!("\n=== CASE ANALYSIS PROOF EXAMPLE ===\n");
    let theorem7 = prove_by_cases();
    println!("\nProof completed with theorem: {}", theorem7.name);

    println!("\n=== DECLARATIVE GROUP ASSOCIATIVITY PROOF ===\n");
    let theorem8 = prove_group_associativity_declarative();
    println!("\nProof completed with theorem: {}", theorem8.name);

    println!("\n=== DECLARATIVE ABSOLUTE VALUE PROOF ===\n");
    let theorem9 = prove_absolute_value_declarative();
    println!("\nProof completed with theorem: {}", theorem9.name);

    println!("\n=== DECLARATIVE COMPLEX THEOREM PROOF ===\n");
    let theorem10 = prove_complex_theorem_declarative();
    println!("\nProof completed with theorem: {}", theorem10.name);

    println!("\n=== CASE BUILDER: ABSOLUTE VALUE PROOF ===\n");
    let theorem11 = prove_absolute_value_with_builder();
    println!("\nProof completed with theorem: {}", theorem11.name);

    println!("\n=== CASE BUILDER: COMPLEX THEOREM PROOF ===\n");
    let theorem12 = prove_complex_theorem_with_builder();
    println!("\nProof completed with theorem: {}", theorem12.name);

    println!("\n=== CASE BUILDER: NUMBER THEORY PROOF ===\n");
    let theorem13 = prove_number_theory_theorem_with_builder();
    println!("\nProof completed with theorem: {}", theorem13.name);

    println!("\n=== REAL WORLD: QUADRATIC FORMULA PROOF ===\n");
    prove_quadratic_formula();

    println!("\n=== REAL WORLD: FERMAT'S LITTLE THEOREM PROOF ===\n");
    prove_fermats_little_theorem();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_all_examples() {
        run_all_examples();
        // This test just ensures everything runs without panicking
        assert!(true);
    }
}
