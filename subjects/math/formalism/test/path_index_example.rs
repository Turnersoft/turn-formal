#[cfg(test)]
mod tests {
    use crate::subjects::math::formalism::expressions::{Identifier, MathExpression};
    use crate::subjects::math::formalism::proof::collect::CollectSubExpressions;
    use crate::subjects::math::formalism::relations::MathRelation;

    // Helper function to create a relation expression
    fn rel_expr(relation: MathRelation) -> MathExpression {
        MathExpression::Relation(Box::new(relation))
    }

    // Helper function to create a variable expression
    fn var(name: &str) -> MathExpression {
        MathExpression::var(name)
    }

    // Helper to print expression details
    fn print_expression_details(expr: &MathExpression) {
        match expr {
            MathExpression::Relation(rel_box) => match &**rel_box {
                MathRelation::Equal { left, right, .. } => {
                    println!("  Expr: Equal: {:?} = {:?}", left, right);
                }
                MathRelation::And(items) => {
                    println!("  Expr: And relation with {} items", items.len());
                }
                MathRelation::Implies(_, _) => {
                    println!("  Expr: Implies relation");
                }
                MathRelation::Todo { name, expressions } => {
                    println!("  Expr: Todo Name: {}, Exprs: {:?}", name, expressions);
                }
                _ => println!("  Expr: Other relation type: {:?}", rel_box),
            },
            MathExpression::Var(id) => {
                if let Identifier::Name(name, _) = id {
                    println!("  Expr: Variable: {}", name);
                } else {
                    println!("  Expr: Other identifier: {:?}", id);
                }
            }
            _ => println!("  Expr: Other expression type: {:?}", expr),
        }
    }

    #[test]
    fn test_path_collection() {
        // Create a nested expression: (a = b) => (c = d ∧ e = f)
        let a_eq_b = MathRelation::equal(var("a"), var("b"));
        let c_eq_d = MathRelation::equal(var("c"), var("d"));
        let e_eq_f = MathRelation::equal(var("e"), var("f"));

        let and_rel = MathRelation::And(vec![c_eq_d, e_eq_f]);
        let implies_rel = MathRelation::Implies(Box::new(a_eq_b), Box::new(and_rel));

        let root_expr = rel_expr(implies_rel);

        // Collect all paths using the new method
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        root_expr.collect_sub_expressions_with_paths(Vec::new(), &mut collected_targets, 0);

        // Print all paths for debugging
        println!("Collected {} paths:", collected_targets.len());
        for (path, expr) in &collected_targets {
            println!("Path: {:?}", path);
            print_expression_details(expr);
        }

        // We should have multiple paths in our collection
        assert!(collected_targets.len() > 1);

        // The first path should be empty (referring to the root expression)
        assert_eq!(collected_targets[0].0, Vec::<usize>::new());
        assert_eq!(collected_targets[0].1, root_expr);
    }

    #[test]
    fn test_path_finding() {
        // Create a nested expression: (a = b) => (c = d ∧ e = f)
        let a_eq_b = MathRelation::equal(var("a"), var("b"));
        let c_eq_d = MathRelation::equal(var("c"), var("d"));
        let e_eq_f = MathRelation::equal(var("e"), var("f"));

        let and_rel = MathRelation::And(vec![c_eq_d, e_eq_f]);
        let implies_rel = MathRelation::Implies(Box::new(a_eq_b), Box::new(and_rel));

        let root_expr = rel_expr(implies_rel);

        // Collect paths
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        root_expr.collect_sub_expressions_with_paths(Vec::new(), &mut collected_targets, 0);

        // Find the specific path for variable 'c'.
        // Based on traversal: root(Rel) -> Implies(1) -> Conclusion(2) -> And(100) -> C=D(100) -> Left(1) -> Var(c)
        // Expected path: [1, 2, 100, 1] - Check traversal.rs for exact convention used.
        // Let's assume path conventions used in traversal.rs lead to this:
        // root = []
        // root.Relation = [1]
        // root.Relation.Implies.conclusion = [1, 2]
        // root.Relation.Implies.conclusion.And[0] = [1, 2, 100] (element 0 of Vec starts at 100)
        // root.Relation.Implies.conclusion.And[0].Relation(C=D) = [1, 2, 100, 1]
        // root.Relation.Implies.conclusion.And[0].Relation(C=D).left = [1, 2, 100, 1, 1]
        let c_var_path = vec![1, 2, 100, 1, 1];

        // Find the pair with the target path
        let found_pair = collected_targets
            .iter()
            .find(|(path, _)| *path == c_var_path);

        assert!(
            found_pair.is_some(),
            "Failed to find pair with path {:?}",
            c_var_path
        );

        // Verify it's the variable c
        if let Some((_, found_expr)) = found_pair {
            match found_expr {
                MathExpression::Var(Identifier::Name(var_name, _)) => {
                    assert_eq!(var_name, "c");
                }
                _ => panic!(
                    "Expected MathExpression::Var(c) at path {:?}, found {:?}",
                    c_var_path, found_expr
                ),
            }
        }

        // Similarly, try to find the variable d
        // Path: root -> Implies(1) -> Conclusion(2) -> And(100) -> C=D(100) -> Right(2) -> Var(d)
        let d_var_path = vec![1, 2, 100, 1, 2];
        let found_pair_d = collected_targets
            .iter()
            .find(|(path, _)| *path == d_var_path);
        assert!(
            found_pair_d.is_some(),
            "Failed to find pair with path {:?}",
            d_var_path
        );

        // Verify it's the variable d
        if let Some((_, found_expr_d)) = found_pair_d {
            match found_expr_d {
                MathExpression::Var(Identifier::Name(var_name, _)) => {
                    assert_eq!(var_name, "d");
                }
                _ => panic!(
                    "Expected MathExpression::Var(d) at path {:?}, found {:?}",
                    d_var_path, found_expr_d
                ),
            }
        }
    }

    #[test]
    fn test_path_index_with_vectors() {
        // Create an expression with a collection: a = b ∧ c = d ∧ e = f
        let a_eq_b = MathRelation::equal(var("a"), var("b"));
        let c_eq_d = MathRelation::equal(var("c"), var("d"));
        let e_eq_f = MathRelation::equal(var("e"), var("f"));

        let and_rel = MathRelation::And(vec![a_eq_b, c_eq_d, e_eq_f]);
        let root_expr = rel_expr(and_rel);

        // Collect all paths
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        root_expr.collect_sub_expressions_with_paths(Vec::new(), &mut collected_targets, 0);

        // Print all paths for debugging
        println!("Collected {} paths with vectors:", collected_targets.len());
        for (path, expr) in &collected_targets {
            println!("Path: {:?}", path);
            print_expression_details(expr);
        }

        // Path for variable 'c': root -> And(1) -> Vec[1](101) -> C=D(1) -> Left(1) -> Var(c)
        let c_var_path = vec![1, 101, 1, 1];

        // Find the pair with the target path
        let found_pair = collected_targets
            .iter()
            .find(|(path, _)| *path == c_var_path);
        assert!(
            found_pair.is_some(),
            "Failed to find pair with path {:?}",
            c_var_path
        );

        // Verify it's the variable c
        if let Some((_, found_expr)) = found_pair {
            match found_expr {
                MathExpression::Var(Identifier::Name(var_name, _)) => {
                    assert_eq!(var_name, "c");
                }
                _ => panic!(
                    "Expected MathExpression::Var(c) at path {:?}, found {:?}",
                    c_var_path, found_expr
                ),
            }
        }

        // Verify the index for c=d relation itself is [1, 101, 1]
        let c_eq_d_path = vec![1, 101, 1];
        let found_rel_pair = collected_targets
            .iter()
            .find(|(path, _)| *path == c_eq_d_path);
        assert!(
            found_rel_pair.is_some(),
            "Failed to find relation pair with path {:?}",
            c_eq_d_path
        );
        if let Some((_, found_expr)) = found_rel_pair {
            assert!(
                matches!(found_expr, MathExpression::Relation(b) if matches!(**b, MathRelation::Equal{..})),
                "Expression at {:?} should be C=D relation",
                c_eq_d_path
            );
        }
    }
}
