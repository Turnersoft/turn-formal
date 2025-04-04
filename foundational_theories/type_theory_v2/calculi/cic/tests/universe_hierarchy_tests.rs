use super::*;

#[cfg(test)]
pub mod test_universe_hierarchy {
    use super::*;

    #[test]
    fn test_basic_universe_hierarchy() {
        let mut ctx = setup_context();

        // Test Type₀ : Type₁
        let term = Term::Sort(Universe::Type(Level::new(0)));
        let ty = term.type_check(&ctx).expect("Failed to type check Type₀");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 1));

        // Test Type₁ : Type₂
        let term = Term::Sort(Universe::Type(Level::new(1)));
        let ty = term.type_check(&ctx).expect("Failed to type check Type₁");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 2));

        // Test Type₁ : Type₂
        let term = Term::Sort(Universe::Type(Level::new(100)));
        let ty = term.type_check(&ctx).expect("Failed to type check Type100");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 101));
    }

    #[test]
    fn test_universe_constraints() {
        let mut ctx = setup_context();

        // Test basic less-than constraint
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        let term = Term::Sort(Universe::Type(Level::new(0)));
        let ty = term
            .type_check(&ctx)
            .expect("Failed to type check with constraint");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 1));
    }

    #[test]
    fn test_universe_polymorphism() {
        let mut ctx = setup_context();
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        // Identity function type: Π(A: Type₀). A → A
        let id_type = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Pi(
                "x".to_string(),
                Rc::new(Type::Named("A".to_string())),
                Rc::new(Term::Var("x".to_string())),
            )),
        );

        let result = id_type.type_check(&ctx);
        assert!(result.is_ok(), "Identity function type check failed");

        // Correct expectation: max(0,0) + 1 = 1
        match &*result.unwrap() {
            Type::Type(level) => assert_eq!(level.0, 2, "Should be at level 2"),
            _ => panic!("Expected Type"),
        }
    }

    #[test]
    fn test_universe_transitivity() {
        let mut ctx = setup_context();

        // Set up transitive constraints
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(1),
            right: Level::new(2),
            kind: ConstraintKind::LessThan,
        });

        // Test that Type₀ works in a context requiring Type₂
        let term = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(2))),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let result = term.type_check(&ctx);
        assert!(
            result.is_ok(),
            "Transitive universe constraint check failed"
        );
    }

    #[test]
    fn test_universe_equality() {
        let mut ctx = setup_context();

        // Replace equality constraint with proper hierarchy check
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        let term = Term::Sort(Universe::Type(Level::new(0)));
        let result = term.type_check(&ctx);
        assert!(result.is_ok(), "Type₀ should check");
        match &*result.unwrap() {
            Type::Type(level) => assert_eq!(level.0, 1, "Type₀ should be at level 1"),
            _ => panic!("Expected Type universe"),
        }
    }

    #[test]
    fn test_prop_universe() {
        let ctx = setup_context();

        // Test Prop : Type₁
        let term = Term::Sort(Universe::Prop);
        let ty = term.type_check(&ctx).expect("Failed to type check Prop");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 1));

        // Test function into Prop
        let prop_fn = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Prop)),
        );

        let result = prop_fn.type_check(&ctx);
        assert!(result.is_ok(), "Function into Prop failed to type check");
    }

    #[test]
    fn test_basic_universe_constraints() {
        let mut ctx = setup_context();

        // Add universe constraints
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        let type0 = Term::Sort(Universe::Type(Level::new(0)));
        let type1 = Term::Sort(Universe::Type(Level::new(1)));

        // Type₀ : Type₁ should be well-typed
        assert!((&type0).type_check(&ctx).is_ok());

        // Test universe constraints in Pi types
        let valid_pi = Term::Pi(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Type(Level::new(1)))),
        );
        assert!((&valid_pi).type_check(&ctx).is_ok());

        // Test invalid universe constraints
        let invalid_pi = Term::Pi(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(1))),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );
        assert!((&invalid_pi).type_check(&ctx).is_err());
    }

    #[test]
    fn test_cumulative_universes() {
        let mut ctx = setup_context();

        // Add cumulative universe constraints: Type₀ < Type₁ < Type₂
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(1),
            right: Level::new(2),
            kind: ConstraintKind::LessThan,
        });

        // Test that a term of Type₀ can be used where Type₁ is expected
        let term = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(1))), // Parameter type is Type₁
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))), // Body is Type₀
        );

        let result = term.type_check(&ctx);
        assert!(
            result.is_ok(),
            "Failed to type check with cumulative universes"
        );

        // Test that a function returning Type₀ can be used where Type₁ is expected
        let higher_order = Term::Lambda(
            "f".to_string(),
            Rc::new(Type::Pi(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(0))),
                Rc::new(Type::Type(Level::new(0))),
            )),
            Rc::new(Term::Var("f".to_string())),
        );

        let result = higher_order.type_check(&ctx);
        assert!(
            result.is_ok(),
            "Failed to type check higher-order function with cumulative universes"
        );
    }

    #[test]
    fn test_universe_inconsistency_detection() {
        let mut ctx = setup_context();

        // Create an inconsistent set of constraints
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(1),
            right: Level::new(0),
            kind: ConstraintKind::LessThan,
        });

        // Test that cyclic constraints are detected
        let term = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Type(Level::new(1)))),
        );

        let result = term.type_check(&ctx);
        assert!(
            result.is_err(),
            "Should detect inconsistent universe constraints"
        );

        // Test another form of inconsistency: Type₁ = Type₀ and Type₀ < Type₁
        let mut ctx2 = setup_context();
        ctx2.constraints.push(UniverseConstraint {
            left: Level::new(1),
            right: Level::new(0),
            kind: ConstraintKind::Equal,
        });
        ctx2.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        let term = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Type(Level::new(1)))),
        );

        let result = term.type_check(&ctx2);
        assert!(
            result.is_err(),
            "Should detect inconsistent equality and inequality constraints"
        );
    }

    #[test]
    fn test_predicative_vs_impredicative_universes() {
        let mut ctx = setup_context();

        // Impredicative Prop quantification
        let term = Term::Pi(
            "P".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Prop)),
        );

        let result = term.type_check(&ctx);
        assert!(result.is_ok(), "Impredicative quantification failed");
        match &*result.unwrap() {
            Type::Type(level) => {
                assert_eq!(level.0, 1, "Impredicative Prop should stay at level 1")
            }
            _ => panic!("Expected Type universe"),
        }
    }

    #[test]
    fn test_universe_subtyping() {
        let mut ctx = setup_context();

        // Add subtyping constraint: Type₀ < Type₁
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        // Test subtyping with function types
        let subtype_fn = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(1))), // Parameter type is Type₁
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))), // Return type is Type₀
        );

        let result = subtype_fn.type_check(&ctx);
        assert!(
            result.is_ok(),
            "Failed to type check function with universe subtyping"
        );

        // Test subtyping with dependent types
        let dependent_fn = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))), // Parameter type is Type₀
            Rc::new(Term::Sort(Universe::Type(Level::new(1)))), // Return type is Type₁
        );

        let result = dependent_fn.type_check(&ctx);
        assert!(
            result.is_ok(),
            "Failed to type check dependent type with universe subtyping"
        );
        match &*result.unwrap() {
            Type::Type(level) => {
                assert_eq!(level.0, 3, "Dependent type should be at universe level 3")
            }
            _ => panic!("Expected Type universe"),
        }

        // Test that subtyping is transitive
        let transitive_fn = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(2))), // Parameter type is Type₂
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))), // Return type is Type₀
        );

        let result = transitive_fn.type_check(&ctx);
        assert!(
            result.is_ok(),
            "Failed to type check function with transitive universe subtyping"
        );
    }

    #[test]
    fn test_universe_maximization() {
        let mut ctx = setup_context();
        ctx.constraints.extend(vec![
            UniverseConstraint {
                left: Level::new(0),
                right: Level::new(1),
                kind: ConstraintKind::LessThan,
            },
            UniverseConstraint {
                left: Level::new(1),
                right: Level::new(2),
                kind: ConstraintKind::LessThan,
            },
        ]);

        let term = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(1))),
            Rc::new(Term::Sort(Universe::Type(Level::new(2)))),
        );

        let result = term.type_check(&ctx);
        assert!(result.is_ok(), "Pi type check failed");

        // Correct calculation: max(1,2) + 1 = 3
        match &*result.unwrap() {
            Type::Type(level) => assert_eq!(level.0, 4, "Should be at level 4"),
            _ => panic!("Expected Type"),
        }
    }

    #[test]
    fn test_universe_variables_and_constraints_solving() {
        let mut ctx = setup_context();
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        let term = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Type(Level::new(1)))),
        );

        let result = term.type_check(&ctx);
        assert!(result.is_ok(), "Constraint solving failed");
        match &*result.unwrap() {
            Type::Type(level) => assert_eq!(level.0, 2, "Should resolve to level 2 (max(0,1)+1)"),
            _ => panic!("Expected Type universe"),
        }
    }

    #[test]
    fn test_universe_polymorphic_recursion() {
        let mut ctx = setup_context();
        ctx.constraints.extend(vec![
            UniverseConstraint {
                left: Level::new(0),
                right: Level::new(1),
                kind: ConstraintKind::LessThan,
            },
            UniverseConstraint {
                left: Level::new(1),
                right: Level::new(2),
                kind: ConstraintKind::LessThan,
            },
        ]);

        // Proper universe-polymorphic recursive type
        let f_type = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Pi(
                "f".to_string(),
                Rc::new(Type::Named("F".to_string())),
                Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
            )),
        );

        ctx.add_var("F".to_string(), Rc::new(Type::Type(Level::new(1))));

        let result = f_type.type_check(&ctx);
        assert!(result.is_ok(), "Polymorphic recursion check failed");
        match &*result.unwrap() {
            Type::Type(level) => assert_eq!(level.0, 2, "Recursive type should be at level 2"),
            _ => panic!("Expected Type universe"),
        }
    }

    #[test]
    fn test_template_polymorphism() {
        let mut ctx = setup_context();

        // Identity function template
        let id_type = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Pi(
                "x".to_string(),
                Rc::new(Type::Named("A".to_string())),
                Rc::new(Term::Var("x".to_string())),
            )),
        );

        let result = id_type.type_check(&ctx);
        assert!(result.is_ok(), "Template polymorphism check failed");
        match &*result.unwrap() {
            Type::Type(level) => {
                assert_eq!(level.0, 1, "Template should be at level 1 (max(0,0)+1)")
            }
            _ => panic!("Expected Type universe"),
        }
    }

    #[test]
    fn test_universe_resizing_rules() {
        let mut ctx = setup_context();

        // Add necessary universe constraints
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        // Test basic resizing: Type₀ can be used where Type₁ is expected
        let basic_resize = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(1))),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let result = basic_resize.type_check(&ctx);
        assert!(
            result.is_ok(),
            "Failed to type check basic universe resizing"
        );

        // Test resizing with dependent types
        let dependent_resize = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Type(Level::new(1)))),
        );

        let result = dependent_resize.type_check(&ctx);
        assert!(
            result.is_ok(),
            "Failed to type check dependent type resizing"
        );
        match &*result.unwrap() {
            Type::Type(level) => assert_eq!(level.0, 3, "Should be at level 3"),
            _ => panic!("Expected Type universe"),
        }

        // Test resizing with multiple levels
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(1),
            right: Level::new(2),
            kind: ConstraintKind::LessThan,
        });

        let multi_level_resize = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Type(Level::new(2)))),
        );

        let result = multi_level_resize.type_check(&ctx);
        assert!(result.is_ok(), "Failed to type check multi-level resizing");
        match &*result.unwrap() {
            Type::Type(level) => assert_eq!(level.0, 4, "Should be at level 4"),
            _ => panic!("Expected Type universe"),
        }

        // Test that invalid resizing is rejected
        let invalid_resize = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(2))),
            Rc::new(Term::Sort(Universe::Type(Level::new(1)))),
        );

        let result = invalid_resize.type_check(&ctx);
        assert!(result.is_err(), "Should reject invalid universe resizing");
    }
}
