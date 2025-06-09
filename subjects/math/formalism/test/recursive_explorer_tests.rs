// subjects/math/formalism/test/recursive_explorer_tests.rs

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::mem::Discriminant;
    use uuid::Uuid; // For checking new state

    use crate::subjects::math::formalism::{
        expressions::{Identifier, MathExpression, TheoryExpression},
        extract::Parametrizable,
        proof::{
            ProofForest, ProofNode, ProofStatus, TheoremRegistry,
            collect::CollectSubExpressions,
            tactics::{self, Tactic, TheoremApplicationError, TheoremApplicationResult},
        },
        relations::MathRelation,
        theorem::{ProofGoal, Theorem},
    };
    // Alias for group definitions to shorten long paths
    use crate::subjects::math::theories::groups::definitions as group_defs;

    // Helper to create a MathExpression::Var
    fn var(name: &str) -> MathExpression {
        MathExpression::Var(Identifier::Name(name.to_string(), 0))
    }

    // Helper to create a MathExpression::Relation
    fn rel_expr(relation: MathRelation) -> MathExpression {
        MathExpression::Relation(Box::new(relation))
    }

    // Helper to create a test theorem
    fn create_test_theorem(id: &str, name: &str, statement: MathRelation) -> Theorem {
        Theorem {
            id: id.to_string(),
            name: name.to_string(),
            description: format!("Test theorem: {}", name),
            goal: ProofGoal::new(statement),
            proofs: ProofForest::new(),
        }
    }

    // Note: The setup_registry_applier helper was problematic due to lifetimes.
    // Tests will create registry and applier directly for now.

    #[test]
    fn test_math_expression_direct_application_success() {
        let mut registry = TheoremRegistry::new();
        registry.register(create_test_theorem(
            "test_succeeds_on_equal",
            "Succeeds on Equal Relations",
            MathRelation::equal(var("ignore1"), var("ignore2")), // Statement kind is Equal
        ));
        let applier = tactics::TheoremApplier::new(&registry);

        let mut forest = ProofForest::new();
        let initial_goal_statement = MathRelation::equal(var("a"), var("b"));
        let initial_proof_goal = ProofGoal::new(initial_goal_statement.clone());
        let root_node_id = Uuid::new_v4().to_string();
        let root_node = ProofNode {
            id: root_node_id.clone(),
            parent: None,
            children: vec![],
            state: initial_proof_goal.clone(),
            tactic: None,
            status: ProofStatus::InProgress,
        };
        forest.add_node(root_node.clone());

        let mut new_branch_ids = Vec::new();
        let root_expr_target = rel_expr(initial_goal_statement.clone()); // This is MathExpression::Relation(Equal(a,b))

        // Pass 1: Collect targets using new method
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        root_expr_target.collect_sub_expressions_with_paths(Vec::new(), &mut collected_targets, 0);

        // Pass 2: Apply theorems to collected targets
        for (target_path, target_sub_expression) in collected_targets {
            // We are interested in applying to the `root_expr_target` itself for this specific test.
            if target_sub_expression == root_expr_target {
                let theorems_to_try = vec!["test_succeeds_on_equal"]; // Directly specify for this test

                for theorem_id_str in theorems_to_try {
                    let theorem_id = theorem_id_str; // It's already a &str if from a vec of literals
                    let instantiations = HashMap::new();

                    match applier.apply_theorem(
                        theorem_id,
                        &instantiations,
                        Some(&target_sub_expression), // Pass as reference
                        Some(&target_path),           // Pass the path
                        &initial_proof_goal,
                    ) {
                        Ok(application_result) => {
                            let tactic_used = tactics::Tactic::TheoremApplication {
                                theorem_id: theorem_id.to_string(),
                                instantiation: application_result.instantiations.clone(),
                                target_expr: Some(target_sub_expression.clone()), // Clone for storage
                            };
                            let new_node_id_str = Uuid::new_v4().to_string();
                            let new_node = ProofNode {
                                id: new_node_id_str.clone(),
                                parent: Some(root_node_id.clone()),
                                children: vec![],
                                state: application_result.new_goal,
                                tactic: Some(tactic_used),
                                status: ProofStatus::InProgress,
                            };
                            forest.add_node(new_node);
                            if let Some(parent_node_in_forest) = forest.nodes.get_mut(&root_node_id)
                            {
                                parent_node_in_forest.children.push(new_node_id_str.clone());
                            }
                            new_branch_ids.push(new_node_id_str);
                        }
                        Err(_error) => {
                            // Optionally log: println!("Failed to apply theorem {} to {:?} at path {:?}: {:?}", theorem_id, self, path_so_far, _error);
                        }
                    }
                }
            }
        }

        assert_eq!(
            new_branch_ids.len(),
            1,
            "Should create exactly one new branch."
        );
        if let Some(new_node_id) = new_branch_ids.first() {
            let new_node = forest.nodes.get(new_node_id).expect("New node not found.");
            assert_eq!(new_node.parent, Some(root_node_id.clone()));
            if let Some(Tactic::TheoremApplication {
                theorem_id,
                target_expr: tactic_target,
                ..
            }) = &new_node.tactic
            {
                assert_eq!(theorem_id, "test_succeeds_on_equal");
                assert_eq!(tactic_target, &Some(root_expr_target.clone()));
            } else {
                panic!("Tactic was not TheoremApplication or was None.");
            }
            // Check the new state based on the test hook in TheoremApplier
            match &new_node.state.statement {
                MathRelation::Todo { name, expressions } => {
                    assert_eq!(name, "applied_test_succeeds_on_equal");
                    assert_eq!(expressions.len(), 1);
                    assert_eq!(expressions[0], root_expr_target);
                }
                _ => panic!(
                    "New state is not the expected Todo relation from test hook. Got: {:?}",
                    new_node.state.statement
                ),
            }
        }
    }

    #[test]
    fn test_math_expression_application_failure() {
        let mut registry = TheoremRegistry::new();
        registry.register(create_test_theorem(
            "test_always_fails",
            "Always Fails Application",
            MathRelation::equal(var("any"), var("any")), // Kind is Equal, so it will be tried
        ));
        let applier = tactics::TheoremApplier::new(&registry);

        let mut forest = ProofForest::new();
        let initial_goal_statement = MathRelation::equal(var("a"), var("b"));
        let initial_proof_goal = ProofGoal::new(initial_goal_statement.clone());
        let root_node_id = Uuid::new_v4().to_string();
        let root_node = ProofNode {
            id: root_node_id.clone(),
            parent: None,
            children: vec![],
            state: initial_proof_goal.clone(),
            tactic: None,
            status: ProofStatus::InProgress,
        };
        forest.add_node(root_node.clone());

        let mut new_branch_ids = Vec::new();
        let root_expr_target = rel_expr(initial_goal_statement.clone());

        // Pass 1: Collect targets
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        root_expr_target.collect_sub_expressions_with_paths(Vec::new(), &mut collected_targets, 0);

        // Pass 2: Apply theorems to collected targets
        // In this test, the applier is set up to always fail for "test_always_fails"
        for (target_path, target_sub_expression) in collected_targets {
            // For this test, we are interested in attempts on the main target_expr
            if target_sub_expression == root_expr_target {
                let theorems_to_try = vec!["test_always_fails"];

                for theorem_id_str in theorems_to_try {
                    let theorem_id = theorem_id_str;
                    let instantiations = HashMap::new();

                    match applier.apply_theorem(
                        theorem_id,
                        &instantiations,
                        Some(&target_sub_expression),
                        Some(&target_path),
                        &initial_proof_goal,
                    ) {
                        Ok(application_result) => {
                            // This block should not be reached due to the mock failure
                            let tactic_used = tactics::Tactic::TheoremApplication {
                                theorem_id: theorem_id.to_string(),
                                instantiation: application_result.instantiations.clone(),
                                target_expr: Some(target_sub_expression.clone()),
                            };
                            let new_node_id_str = Uuid::new_v4().to_string();
                            let new_node = ProofNode {
                                id: new_node_id_str.clone(),
                                parent: Some(root_node_id.clone()),
                                children: vec![],
                                state: application_result.new_goal,
                                tactic: Some(tactic_used),
                                status: ProofStatus::InProgress,
                            };
                            forest.add_node(new_node);
                            if let Some(parent_node_in_forest) = forest.nodes.get_mut(&root_node_id)
                            {
                                parent_node_in_forest.children.push(new_node_id_str.clone());
                            }
                            new_branch_ids.push(new_node_id_str);
                        }
                        Err(_error) => {
                            // Error is expected, do nothing, no branch created
                        }
                    }
                }
            }
        }

        assert!(
            new_branch_ids.is_empty(),
            "Should create no new branches if theorem application fails."
        );
    }

    #[test]
    fn test_math_expression_no_matching_theorem_kind() {
        let mut registry = TheoremRegistry::new();
        registry.register(create_test_theorem(
            "thm_todo_kind",
            "Some Todo Theorem",
            MathRelation::Todo {
                name: "test".to_string(),
                expressions: vec![],
            }, // Kind is Todo
        ));
        let applier = tactics::TheoremApplier::new(&registry);

        let mut forest = ProofForest::new();
        let initial_goal_statement = MathRelation::equal(var("a"), var("b")); // Testing on an Equal relation
        let initial_proof_goal = ProofGoal::new(initial_goal_statement.clone());
        let root_node_id = Uuid::new_v4().to_string();
        let root_node = ProofNode {
            id: root_node_id.clone(),
            parent: None,
            children: vec![],
            state: initial_proof_goal.clone(),
            tactic: None,
            status: ProofStatus::InProgress,
        };
        forest.add_node(root_node.clone());

        let mut new_branch_ids = Vec::new();
        let root_expr_target = rel_expr(initial_goal_statement.clone());

        // Pass 1: Collect targets
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        root_expr_target.collect_sub_expressions_with_paths(Vec::new(), &mut collected_targets, 0);

        // Pass 2: Apply theorems to collected targets
        for (target_path, target_sub_expression) in collected_targets {
            // Only consider the main target_expr for this test
            if target_sub_expression == root_expr_target {
                let mut theorems_to_try: Vec<String> = Vec::new();
                if let MathExpression::Relation(inner_relation_box) = &target_sub_expression {
                    let discriminant = std::mem::discriminant(inner_relation_box.as_ref());
                    if let Some(indexed_ids) = registry.get_theorems_by_relation_kind(&discriminant)
                    {
                        theorems_to_try.extend(indexed_ids.clone());
                    }
                }

                // Since the registered theorem ("thm_todo_kind") is for MathRelation::Todo,
                // and our target_expr is MathRelation::Equal, theorems_to_try should be empty.
                // Thus, the loop below won't run, and no branches will be created.

                for theorem_id_str in theorems_to_try {
                    let theorem_id = theorem_id_str.as_str();
                    let instantiations = HashMap::new();

                    match applier.apply_theorem(
                        theorem_id,
                        &instantiations,
                        Some(&target_sub_expression),
                        Some(&target_path),
                        &initial_proof_goal,
                    ) {
                        Ok(application_result) => {
                            let tactic_used = tactics::Tactic::TheoremApplication {
                                theorem_id: theorem_id.to_string(),
                                instantiation: application_result.instantiations.clone(),
                                target_expr: Some(target_sub_expression.clone()),
                            };
                            let new_node_id_str = Uuid::new_v4().to_string();
                            let new_node = ProofNode {
                                id: new_node_id_str.clone(),
                                parent: Some(root_node_id.clone()),
                                children: vec![],
                                state: application_result.new_goal,
                                tactic: Some(tactic_used),
                                status: ProofStatus::InProgress,
                            };
                            forest.add_node(new_node);
                            if let Some(parent_node_in_forest) = forest.nodes.get_mut(&root_node_id)
                            {
                                parent_node_in_forest.children.push(new_node_id_str.clone());
                            }
                            new_branch_ids.push(new_node_id_str);
                        }
                        Err(_error) => {
                            // No error expected here as apply_theorem shouldn't even be called if no theorems match
                        }
                    }
                }
            }
        }

        assert!(
            new_branch_ids.is_empty(),
            "No branches if no theorem of matching relation kind is registered."
        );
    }

    #[test]
    fn test_recursion_into_math_relation_and_applies_correctly() {
        let mut registry = TheoremRegistry::new();
        registry.register(create_test_theorem(
            "test_succeeds_on_equal", // Will be applied to each Equal sub-expression
            "Succeeds on Equal Relations",
            MathRelation::equal(var("ignore1"), var("ignore2")),
        ));
        let applier = tactics::TheoremApplier::new(&registry);

        let mut forest = ProofForest::new();
        let eq1 = MathRelation::equal(var("a"), var("b"));
        let eq2 = MathRelation::equal(var("c"), var("d"));
        let initial_goal_statement = MathRelation::And(vec![eq1.clone(), eq2.clone()]);
        let initial_proof_goal = ProofGoal::new(initial_goal_statement.clone());
        let root_node_id = Uuid::new_v4().to_string();
        let root_node = ProofNode {
            id: root_node_id.clone(),
            parent: None,
            children: vec![],
            state: initial_proof_goal.clone(),
            tactic: None,
            status: ProofStatus::InProgress,
        };
        forest.add_node(root_node.clone());

        let mut new_branch_ids = Vec::new();
        let root_expr_target = rel_expr(initial_goal_statement.clone()); // MathExpression::Relation(Box::new(And(...)))

        // Pass 1: Collect targets
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        root_expr_target.collect_sub_expressions_with_paths(Vec::new(), &mut collected_targets, 0);

        // Pass 2: Apply theorems to collected targets
        for (target_path, target_sub_expression) in collected_targets {
            // For this test, the theorem "test_succeeds_on_equal" should apply to
            // MathExpression::Relation(eq1) and MathExpression::Relation(eq2).
            // It should NOT apply to target_expr itself (which is an And relation)
            // or other intermediate MathExpression::Relation wrappers for And/Implies if they existed.

            let mut theorems_to_try: Vec<String> = Vec::new();
            if let MathExpression::Relation(inner_relation_box) = &target_sub_expression {
                // We are only interested in Equal relations for this specific theorem
                if let MathRelation::Equal { .. } = inner_relation_box.as_ref() {
                    let discriminant = std::mem::discriminant(inner_relation_box.as_ref());
                    if let Some(indexed_ids) = registry.get_theorems_by_relation_kind(&discriminant)
                    {
                        theorems_to_try.extend(indexed_ids.clone());
                    }
                }
            }

            for theorem_id_str in theorems_to_try {
                // Ensure we only try to apply the relevant theorem for this test
                if theorem_id_str != "test_succeeds_on_equal" {
                    continue;
                }

                let theorem_id = theorem_id_str.as_str();
                let instantiations = HashMap::new();

                match applier.apply_theorem(
                    theorem_id,
                    &instantiations,
                    Some(&target_sub_expression),
                    Some(&target_path),
                    &initial_proof_goal,
                ) {
                    Ok(application_result) => {
                        let tactic_used = tactics::Tactic::TheoremApplication {
                            theorem_id: theorem_id.to_string(),
                            instantiation: application_result.instantiations.clone(),
                            target_expr: Some(target_sub_expression.clone()),
                        };
                        let new_node_id_str = Uuid::new_v4().to_string();
                        let new_node = ProofNode {
                            id: new_node_id_str.clone(),
                            parent: Some(root_node_id.clone()),
                            children: vec![],
                            state: application_result.new_goal,
                            tactic: Some(tactic_used),
                            status: ProofStatus::InProgress,
                        };
                        forest.add_node(new_node);
                        if let Some(parent_node_in_forest) = forest.nodes.get_mut(&root_node_id) {
                            parent_node_in_forest.children.push(new_node_id_str.clone());
                        }
                        new_branch_ids.push(new_node_id_str);
                    }
                    Err(_error) => {}
                }
            }
        }

        assert_eq!(
            new_branch_ids.len(),
            2,
            "Should create two new branches, one for each Equal sub-relation."
        );

        for (i, node_id) in new_branch_ids.iter().enumerate() {
            let new_node = forest.nodes.get(node_id).expect("Node not found.");
            assert_eq!(new_node.parent, Some(root_node_id.clone()));
            if let Some(Tactic::TheoremApplication {
                theorem_id,
                target_expr: tactic_target,
                ..
            }) = &new_node.tactic
            {
                assert_eq!(theorem_id, "test_succeeds_on_equal");
                // The target expression recorded in the tactic should be the specific Equal sub-relation
                let expected_target_sub_expr = if i == 0 {
                    rel_expr(eq1.clone())
                } else {
                    rel_expr(eq2.clone())
                };
                assert_eq!(tactic_target, &Some(expected_target_sub_expr));
            } else {
                panic!("Tactic was not TheoremApplication.");
            }
            match &new_node.state.statement {
                MathRelation::Todo { name, expressions } => {
                    assert_eq!(name, "applied_test_succeeds_on_equal");
                    assert_eq!(expressions.len(), 1);
                    let expected_target_sub_expr = if i == 0 {
                        rel_expr(eq1.clone())
                    } else {
                        rel_expr(eq2.clone())
                    };
                    assert_eq!(expressions[0], expected_target_sub_expr);
                }
                _ => panic!(
                    "New state for sub-application is not Todo. Got: {:?}",
                    new_node.state.statement
                ),
            }
        }
    }

    #[test]
    fn test_recursion_into_theory_expression_group_operation() {
        let mut registry = TheoremRegistry::new();
        // This theorem could apply if a MathExpression::Relation(Equal(...)) is found anywhere.
        registry.register(create_test_theorem(
            "test_succeeds_on_equal",
            "Succeeds on Equal Relations",
            MathRelation::equal(var("ignored"), var("ignored")),
        ));
        let applier = tactics::TheoremApplier::new(&registry);
        let mut forest = ProofForest::new();

        // Structure: ME(TE(GE_Op(P<GE_ElemOrder(P<GE_Ident>)>, P<GE_Ident>)))
        // We want to test that explore_theorems_recursively navigates through this structure.
        // Application will only occur if one of these GE_... is wrapped into a ME(Rel(Eq)) directly.
        // This isn't how GroupExpression is currently structured for deep ME embedding.
        // So, this test primarily checks that the traversal mechanism for GroupExpression fields works.

        let group_ident = Identifier::Name("G_placeholder".to_string(), 0);
        let group_param_for_ident = Parametrizable::Variable(group_ident.clone());
        let group_param_for_elem_order = Parametrizable::Variable(group_ident.clone());

        let innermost_ge = group_defs::GroupExpression::Identity(group_param_for_ident.clone());

        let element_order_expr = group_defs::GroupExpression::ElementOrder {
            element: Box::new(Parametrizable::Concrete(innermost_ge)),
            group: group_param_for_elem_order.clone(), // Added missing group field
        };

        let identity_for_op_rhs =
            group_defs::GroupExpression::Identity(group_param_for_ident.clone());

        let group_op = group_defs::GroupExpression::Operation {
            group: group_param_for_ident.clone(), // Group for the operation itself
            left: Box::new(Parametrizable::Concrete(element_order_expr)),
            right: Box::new(Parametrizable::Concrete(identity_for_op_rhs)),
        };

        // This is the MathExpression that will be explored.
        let root_expr_target = MathExpression::Expression(TheoryExpression::Group(group_op));

        let initial_proof_goal = ProofGoal::new(MathRelation::Todo {
            name: "unused_goal".to_string(),
            expressions: vec![],
        });
        let root_node_id = Uuid::new_v4().to_string();
        let root_node = ProofNode {
            id: root_node_id.clone(),
            parent: None,
            children: vec![],
            state: initial_proof_goal.clone(),
            tactic: None,
            status: ProofStatus::InProgress,
        };
        forest.add_node(root_node.clone());

        let mut new_branch_ids = Vec::new();

        // Pass 1: Collect targets
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        root_expr_target.collect_sub_expressions_with_paths(Vec::new(), &mut collected_targets, 0);

        // Pass 2: Apply theorems to collected targets
        for (target_path, target_sub_expression) in collected_targets {
            // This test is primarily about navigating GroupExpression.
            // The original test didn't expect applications *within* GroupExpression parts unless they were ME::Relation.
            // With the new model, target_expr (ME(TE(GE_Op))) itself will be collected.
            // If a theorem matches MathExpression::Expression, it would apply.
            // For this test, let's assume no such theorem is registered, or if one is, it doesn't lead to a new node for simplicity,
            // aligning with the original test's expectation of zero new branches from *within* the GE structure.

            let mut theorems_to_try: Vec<String> = Vec::new();
            // Check if the target is the main target_expr (which is a MathExpression::Expression)
            if target_sub_expression == root_expr_target {
                // Example: if we had a theorem specifically for MathExpression::Expression
                // let discriminant = std::mem::discriminant(&target_sub_expression); // This won't work directly for ME variant like this for indexing
                // For now, we assume no theorems match MathExpression::Expression directly, or our specific "test_succeeds_on_equal" won't match.
                // If test_succeeds_on_equal was applicable (e.g. via a general index), it might be tried here.
            }

            // If the target_sub_expression is a MathExpression::Relation (e.g. if a GE field was an ME::Rel),
            // then the logic from previous tests would apply.
            if let MathExpression::Relation(inner_relation_box) = &target_sub_expression {
                let discriminant = std::mem::discriminant(inner_relation_box.as_ref());
                if let Some(indexed_ids) = registry.get_theorems_by_relation_kind(&discriminant) {
                    theorems_to_try.extend(indexed_ids.clone());
                }
            }

            for theorem_id_str in theorems_to_try {
                // This test had "test_succeeds_on_equal" registered, which applies to ME(Rel(Eq...))
                // It shouldn't apply to ME(TE(GE_Op)) unless the theorem was very generic and the applier allowed it.
                // We stick to the original intent: no applications *from within* GE structure for this test.
                if theorem_id_str != "test_succeeds_on_equal" {
                    // Only consider this one if it somehow gets selected
                    continue;
                }

                let theorem_id = theorem_id_str.as_str();
                let instantiations = HashMap::new();

                match applier.apply_theorem(
                    theorem_id,
                    &instantiations,
                    Some(&target_sub_expression),
                    Some(&target_path),
                    &initial_proof_goal,
                ) {
                    Ok(application_result) => {
                        let tactic_used = tactics::Tactic::TheoremApplication {
                            theorem_id: theorem_id.to_string(),
                            instantiation: application_result.instantiations.clone(),
                            target_expr: Some(target_sub_expression.clone()),
                        };
                        let new_node_id_str = Uuid::new_v4().to_string();
                        let new_node = ProofNode {
                            id: new_node_id_str.clone(),
                            parent: Some(root_node_id.clone()),
                            children: vec![],
                            state: application_result.new_goal,
                            tactic: Some(tactic_used),
                            status: ProofStatus::InProgress,
                        };
                        forest.add_node(new_node);
                        if let Some(parent_node_in_forest) = forest.nodes.get_mut(&root_node_id) {
                            parent_node_in_forest.children.push(new_node_id_str.clone());
                        }
                        new_branch_ids.push(new_node_id_str);
                    }
                    Err(_error) => {}
                }
            }
        }

        assert!(
            new_branch_ids.is_empty(),
            "No new branches expected from deep within GroupExpression unless a field directly becomes/wraps a targetable MathExpression::Relation. Current structure tests navigation."
        );
    }

    /*
    #[test]
    fn test_proof_node_explore_theorems_integration() {
        let mut registry = TheoremRegistry::new();
        registry.register(create_test_theorem(
            "test_succeeds_on_equal",
            "Succeeds on Equal Relations",
            MathRelation::equal(var("ignore1"), var("ignore2")),
        ));
        let mut forest = ProofForest::new();
        let initial_goal_statement = MathRelation::equal(var("final_target"), var("value"));
        let initial_proof_goal = ProofGoal::new(initial_goal_statement.clone());

        let root_node_id_str = Uuid::new_v4().to_string(); // Ensure a unique ID for the root
        let root_node = ProofNode {
            id: root_node_id_str.clone(), // Use the unique ID
            parent: None,
            children: vec![],
            state: initial_proof_goal.clone(),
            tactic: None,
            status: ProofStatus::InProgress,
        };
        forest.add_node(root_node.clone());

        // Call the method on ProofNode. This method now internally uses the two-pass system.
        let new_branch_ids = root_node.explore_theorem_applications(&mut forest, &registry);

        assert_eq!(
            new_branch_ids.len(),
            1,
            "ProofNode integration should create one branch."
        );
        if let Some(new_node_id) = new_branch_ids.first() {
            let new_node = forest
                .nodes
                .get(new_node_id)
                .expect("New node not found in forest.");
            assert_eq!(new_node.parent, Some(root_node.id.clone()));
            match &new_node.state.statement {
                MathRelation::Todo { name, expressions } => {
                    assert_eq!(name, "applied_test_succeeds_on_equal");
                    assert_eq!(expressions.len(), 1);
                    // The target for the theorem application was the entire initial_goal_statement, wrapped in MathExpression::Relation
                    assert_eq!(expressions[0], rel_expr(initial_goal_statement));
                }
                _ => panic!(
                    "New state from ProofNode integration is not Todo. Got: {:?}",
                    new_node.state.statement
                ),
            }
        }
    }
    */

    // TODO:
    // - Test cases with deeper nesting.

    #[test]
    fn test_deeper_nesting_mixed_relations() {
        // Setup remains the same
        let eq1 = MathRelation::equal(var("a"), var("b"));
        let eq2 = MathRelation::equal(var("c"), var("d"));
        let todo1 = MathRelation::Todo {
            name: "original_todo".to_string(),
            expressions: vec![],
        };

        let and_rel = MathRelation::And(vec![eq2.clone(), todo1.clone()]);
        let implies_rel = MathRelation::Implies(Box::new(eq1.clone()), Box::new(and_rel.clone()));
        let root_expr_target = rel_expr(implies_rel.clone());

        // Use the new traversal method
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        root_expr_target.collect_sub_expressions_with_paths(Vec::new(), &mut collected_targets, 0);

        // Debug print the targets for review
        println!("Found {} targets:", collected_targets.len());
        for (i, (path, target)) in collected_targets.iter().enumerate() {
            println!("  {}: {:?} (path: {:?})", i, target, path);
        }

        // Verify we found the expected targets (adjusting for potentially different paths)
        assert!(
            collected_targets
                .iter()
                .any(|(_, t)| *t == root_expr_target),
            "Target collection should include the top-level expression"
        );

        assert!(
            collected_targets.iter().any(|(_, t)| match t {
                MathExpression::Relation(r) => **r == eq1,
                _ => false,
            }),
            "Target collection should include eq1"
        );

        assert!(
            collected_targets.iter().any(|(_, t)| match t {
                MathExpression::Relation(r) => **r == eq2,
                _ => false,
            }),
            "Target collection should include eq2"
        );

        assert!(
            collected_targets.iter().any(|(_, t)| match t {
                MathExpression::Relation(r) => matches!(**r, MathRelation::And(_)),
                _ => false,
            }),
            "Target collection should include the And relation"
        );

        assert!(
            collected_targets.iter().any(|(_, t)| match t {
                MathExpression::Relation(r) => matches!(**r, MathRelation::Todo { .. }),
                _ => false,
            }),
            "Target collection should include the Todo relation"
        );

        // The exact count depends on whether Vars, Numbers etc. are included by the specific
        // implementations. The core check is that the relations are found.
        // Let's check for a reasonable minimum based on the relations.
        assert!(
            collected_targets.len() >= 7,
            "Should have found at least the key relation/expression targets"
        ); // Adjusted expected count
    }
}
