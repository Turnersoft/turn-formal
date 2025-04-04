// Module: src/formalize_v2/subjects/math/theorem/test/minimal_case_test.rs
// Minimal test for case analysis builder

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::formalize_v2::subjects::math::theorem::core::ProofState;
use crate::formalize_v2::subjects::math::theorem::expressions::MathExpression;
use crate::formalize_v2::subjects::math::theorem::proof::{
    CaseAnalysisBuilder, ProofBranch, ProofForest, ProofStatus, Tactic, TheoremBuilder,
};
use crate::formalize_v2::subjects::math::theorem::relations::MathRelation;

// Implement Default for ProofState for testing
impl Default for ProofState {
    fn default() -> Self {
        ProofState {
            quantifier: Vec::new(),
            value_variables: Vec::new(),
            statement: MathRelation::custom("Test statement".to_string(), Vec::new()),
            path: Some("p0".to_string()),
            justification: Some("initial state".to_string()),
        }
    }
}

/// Test that the case analysis builder works
#[test]
fn test_case_analysis_builder() {
    // Create a forest and a branch
    let forest = Rc::new(RefCell::new(ProofForest::new()));

    // Add an initial node to the forest
    let root_id = {
        let mut forest_borrow = forest.borrow_mut();
        forest_borrow.add_node(
            None,
            Default::default(), // Using default ProofState
            None,
            "Initial node".to_string(),
            ProofStatus::InProgress,
        )
    };

    // Create a branch
    let branch = ProofBranch {
        node_id: root_id,
        forest: forest.clone(),
        path_id: "p0".to_string(),
    };

    // Use the case analysis builder
    let case_result = branch
        .case_analysis()
        .on_variable("x")
        .case("x > 0", |branch| {
            // Inside case scope
            let c1 = branch.tactics_intro("Case x > 0", 1);
            c1.should_complete()
        })
        .case("x = 0", |branch| {
            // Another case scope
            let c2 = branch.tactics_intro("Case x = 0", 1);
            c2.should_complete()
        })
        .case("x < 0", |branch| {
            // Third case scope
            let c3 = branch.tactics_intro("Case x < 0", 1);

            // Branch within a case
            let c3_1 = c3.branch();
            let c3_1_1 = c3_1.tactics_intro("Alternative in case x < 0", 2);

            c3.should_complete()
        })
        .build();

    // Print paths for debugging
    println!("Case paths:");
    for (i, case) in case_result.cases.iter().enumerate() {
        println!("Case {} path: {}", i, case.get_path_name());
    }

    // Check that the case result has the right structure
    assert_eq!(case_result.cases.len(), 3);
    assert_eq!(case_result.parent_path, "p0");

    // Check that the parent branch is properly set up
    assert!(case_result.parent_branch.path_id.contains("cases"));

    // Check that each case has the right path structure
    let case_paths = vec![
        case_result.cases[0].get_path_name(),
        case_result.cases[1].get_path_name(),
        case_result.cases[2].get_path_name(),
    ];

    // Check that cases have different paths but all contain case indicators
    for path in &case_paths {
        assert!(path.contains("c"));
    }
    assert_ne!(case_paths[0], case_paths[1]);
    assert_ne!(case_paths[0], case_paths[2]);
    assert_ne!(case_paths[1], case_paths[2]);

    // Verify the forest structure
    let forest_visualization = branch.visualize_forest();
    println!("Forest visualization:\n{}", forest_visualization);

    // Check that the forest contains the expected elements
    assert!(forest_visualization.contains("p0"));
    assert!(forest_visualization.contains("Case x > 0"));
    assert!(forest_visualization.contains("Case x = 0"));
    assert!(forest_visualization.contains("Case x < 0"));
    assert!(forest_visualization.contains("Alternative in case x < 0"));
}

/// Test nested case analysis
#[test]
fn test_nested_case_analysis() {
    // Create a forest and a branch
    let forest = Rc::new(RefCell::new(ProofForest::new()));

    // Add an initial node to the forest
    let root_id = {
        let mut forest_borrow = forest.borrow_mut();
        forest_borrow.add_node(
            None,
            Default::default(), // Using default ProofState
            None,
            "Initial node".to_string(),
            ProofStatus::InProgress,
        )
    };

    // Create a branch
    let branch = ProofBranch {
        node_id: root_id,
        forest: forest.clone(),
        path_id: "p0".to_string(),
    };

    // Use case analysis with nested cases
    let case_result = branch
        .case_analysis()
        .on_variable("n")
        .case("n divisible by 4", |branch| {
            let c1 = branch.tactics_intro("n = 4k", 1);

            // Nested case analysis
            let subcase_result = c1
                .case_analysis()
                .on_variable("k")
                .case("k is even", |subcase| {
                    let sc1 = subcase.tactics_intro("k = 2m", 1);
                    sc1.should_complete()
                })
                .case("k is odd", |subcase| {
                    let sc2 = subcase.tactics_intro("k = 2m + 1", 1);
                    sc2.should_complete()
                })
                .build();

            subcase_result.parent_branch.should_complete()
        })
        .case("n mod 4 = 1", |branch| {
            let c2 = branch.tactics_intro("n = 4k + 1", 1);
            c2.should_complete()
        })
        .case("n mod 4 = 2", |branch| {
            let c3 = branch.tactics_intro("n = 4k + 2", 1);
            c3.should_complete()
        })
        .case("n mod 4 = 3", |branch| {
            let c4 = branch.tactics_intro("n = 4k + 3", 1);
            c4.should_complete()
        })
        .build();

    // Print paths for debugging
    println!("Case paths:");
    for (i, case) in case_result.cases.iter().enumerate() {
        println!("Case {} path: {}", i, case.get_path_name());
    }

    // Check that the case result has the right structure
    assert_eq!(case_result.cases.len(), 4);

    // Check the forest structure
    let forest_visualization = branch.visualize_forest();
    println!("Forest visualization:\n{}", forest_visualization);

    // Check for the presence of expected nodes in the forest
    assert!(forest_visualization.contains("n = 4k"));
    assert!(forest_visualization.contains("k = 2m"));
    assert!(forest_visualization.contains("k = 2m + 1"));
    assert!(forest_visualization.contains("n = 4k + 1"));
    assert!(forest_visualization.contains("n = 4k + 2"));
    assert!(forest_visualization.contains("n = 4k + 3"));
}

/// Test case analysis with ad hoc branching
#[test]
fn test_case_analysis_with_branching() {
    // Create a forest and a branch
    let forest = Rc::new(RefCell::new(ProofForest::new()));

    // Add an initial node to the forest
    let root_id = {
        let mut forest_borrow = forest.borrow_mut();
        forest_borrow.add_node(
            None,
            Default::default(), // Using default ProofState
            None,
            "Initial node".to_string(),
            ProofStatus::InProgress,
        )
    };

    // Create a branch
    let branch = ProofBranch {
        node_id: root_id,
        forest: forest.clone(),
        path_id: "p0".to_string(),
    };

    // Use case analysis with branches inside cases
    let case_result = branch
        .case_analysis()
        .on_expression("triangle type")
        .case("equilateral", |branch| {
            let c1 = branch.tactics_intro("All sides equal", 1);

            // Create a branch within this case
            let c1_alt = c1.branch();
            let c1_alt_1 = c1_alt.tactics_intro("Alternative approach", 2);
            let c1_alt_2 = c1_alt_1.tactics_intro("Using a different theorem", 3);

            // Continue with main path
            let c1_1 = c1.tactics_intro("Using standard approach", 2);
            c1_1.should_complete()
        })
        .case("isosceles", |branch| {
            let c2 = branch.tactics_intro("Two sides equal", 1);
            c2.should_complete()
        })
        .case("scalene", |branch| {
            let c3 = branch.tactics_intro("No sides equal", 1);
            c3.should_complete()
        })
        .build();

    // Print paths for debugging
    println!("Case paths:");
    for (i, case) in case_result.cases.iter().enumerate() {
        println!("Case {} path: {}", i, case.get_path_name());
    }

    // Check the forest structure
    let forest_visualization = branch.visualize_forest();
    println!("Forest visualization:\n{}", forest_visualization);

    // Check for the presence of expected nodes in the forest
    assert!(forest_visualization.contains("All sides equal"));
    assert!(forest_visualization.contains("Alternative approach"));
    assert!(forest_visualization.contains("Using a different theorem"));
    assert!(forest_visualization.contains("Using standard approach"));
    assert!(forest_visualization.contains("Two sides equal"));
    assert!(forest_visualization.contains("No sides equal"));
}
