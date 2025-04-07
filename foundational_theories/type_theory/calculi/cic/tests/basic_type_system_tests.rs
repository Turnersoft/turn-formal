use super::*;

#[cfg(test)]
pub mod test_basic_type_system {
    use super::*;

    #[test]
    fn test_type_inference() {
        let mut ctx = setup_context();

        // Test type inference for variables
        let var_term = Term::Var("Type".to_string());
        let ty = (&var_term)
            .type_check(&ctx)
            .expect("Failed to type check Type variable");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 1));

        // Test type inference for applications
        let app_term = Term::App(
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(1))),
                Rc::new(Term::Var("x".to_string())),
            )),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );
        let ty = (&app_term)
            .type_check(&ctx)
            .expect("Failed to type check application");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 1));
    }

    #[test]
    fn test_explicit_annotations() {
        let ctx = setup_context();

        // Test explicit type annotations
        let term = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(1))),
            Rc::new(Term::Var("x".to_string())),
        );
        let ty = term.type_check(&ctx).expect("Failed to type check lambda");
        assert!(matches!(&*ty, Type::Pi(_, _, _)));
    }

    #[test]
    fn test_type_errors() {
        let ctx = setup_context();

        // Test unbound variable error
        let term = Term::Var("undefined".to_string());
        assert!(matches!(
            term.type_check(&ctx),
            Err(TypeError::UnboundVariable(_))
        ));

        // Test type mismatch error
        let term = Term::App(
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(1))),
                Rc::new(Term::Var("x".to_string())),
            )),
            Rc::new(Term::Number(42)), // Number when Type₁ expected
        );
        assert!(matches!(
            term.type_check(&ctx),
            Err(TypeError::TypeMismatch { .. })
        ));
    }

    #[test]
    fn test_subtyping() {
        let mut ctx = setup_context();

        // Test basic universe subtyping
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(1),
            right: Level::new(2),
            kind: ConstraintKind::LessThan,
        });

        let term = Term::Sort(Universe::Type(Level::new(1)));
        let ty = term
            .type_check(&ctx)
            .expect("Failed to type check universe");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 2));
    }

    #[test]
    fn test_conversion() {
        let mut ctx = setup_context();

        // Test beta conversion
        let id_term = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(1))),
            Rc::new(Term::Var("x".to_string())),
        );
        let app_term = Term::App(
            Rc::new(id_term),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );
        let reduced = app_term.reduce().expect("Failed to reduce application");
        assert!(matches!(&*reduced, Term::Sort(Universe::Type(level)) if level.0 == 0));

        // Test delta conversion (definitions)
        ctx.add_var("MyType".to_string(), Rc::new(Type::Type(Level::new(1))));
        let var_term = Term::Var("MyType".to_string());
        let ty = (&var_term)
            .type_check(&ctx)
            .expect("Failed to type check defined variable");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 1));
    }

    #[test]
    fn test_universe_constraints() {
        let mut ctx = setup_context();

        // Test basic universe constraints
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(1),
            right: Level::new(2),
            kind: ConstraintKind::LessThan,
        });

        let term = Term::Sort(Universe::Type(Level::new(1)));
        let ty = term
            .type_check(&ctx)
            .expect("Failed to type check with constraints");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 2));
    }

    #[test]
    fn test_type_inference_complex() {
        let mut ctx = setup_context();

        // Test multiple applications
        let app_term = Term::App(
            Rc::new(Term::App(
                Rc::new(Term::Lambda(
                    "x".to_string(),
                    Rc::new(Type::Type(Level::new(1))),
                    Rc::new(Term::Lambda(
                        "y".to_string(),
                        Rc::new(Type::Type(Level::new(1))),
                        Rc::new(Term::Var("x".to_string())),
                    )),
                )),
                Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
            )),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );
        let ty = app_term
            .type_check(&ctx)
            .expect("Failed to type check multiple applications");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 1));

        // Test type inference with universe constraints
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        let constrained_term = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );
        let ty = constrained_term
            .type_check(&ctx)
            .expect("Failed to type check with universe constraints");
        assert!(matches!(&*ty, Type::Pi(_, _, _)));

        // Test type inference with dependent types
        let dependent_term = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Var("x".to_string())),
        );
        let ty = dependent_term
            .type_check(&ctx)
            .expect("Failed to type check dependent types");
        assert!(matches!(&*ty, Type::Pi(_, _, _)));
    }

    #[test]
    fn test_subtyping_advanced() {
        let mut ctx = setup_context();

        // Test transitive universe constraints
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

        // Test subtyping with Pi types
        let pi_term = Term::Pi(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Sort(Universe::Type(Level::new(1)))),
        );
        let ty = pi_term
            .type_check(&ctx)
            .expect("Failed to type check Pi type with universe constraints");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 2));

        // Test subtyping with nested Pi types
        let nested_pi_term = Term::Pi(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Pi(
                "y".to_string(),
                Rc::new(Type::Type(Level::new(1))),
                Rc::new(Term::Sort(Universe::Type(Level::new(2)))),
            )),
        );
        let ty = nested_pi_term
            .type_check(&ctx)
            .expect("Failed to type check nested Pi type");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 3));

        // Test subtyping with applications
        let app_term = Term::App(
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(1))),
                Rc::new(Term::Sort(Universe::Type(Level::new(2)))),
            )),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))), // Type₀ can be used where Type₁ is expected
        );
        let ty = app_term
            .type_check(&ctx)
            .expect("Failed to type check application with subtyping");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 3));
    }

    #[test]
    fn test_conversion_rules() {
        let mut ctx = setup_context();

        // Test beta conversion with multiple arguments
        let multi_arg_term = Term::App(
            Rc::new(Term::App(
                Rc::new(Term::Lambda(
                    "x".to_string(),
                    Rc::new(Type::Type(Level::new(1))),
                    Rc::new(Term::Lambda(
                        "y".to_string(),
                        Rc::new(Type::Type(Level::new(1))),
                        Rc::new(Term::Var("x".to_string())),
                    )),
                )),
                Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
            )),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let reduced = multi_arg_term.reduce().expect("Failed to reduce multi-argument term");
        assert!(matches!(&*reduced, Term::Sort(Universe::Type(level)) if level.0 == 0));

        // Test delta conversion with definitions
        ctx.add_definition(
            "MyType".to_string(),
            Rc::new(Type::Type(Level::new(1))),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let def_term = Term::Var("MyType".to_string());
        let ty = def_term.type_check(&ctx).expect("Failed to type check definition");
        assert!(matches!(&*ty, Type::Type(level) if level.0 == 1));

        // Test conversion with dependent types
        let dep_term = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::App(
                Rc::new(Term::Lambda(
                    "x".to_string(),
                    Rc::new(Type::Type(Level::new(0))),
                    Rc::new(Term::Var("x".to_string())),
                )),
                Rc::new(Term::Var("A".to_string())),
            )),
        );

        let dep_ty = dep_term.type_check(&ctx).expect("Failed to type check dependent term");
        assert!(matches!(&*dep_ty, Type::Type(level) if level.0 == 1));

        // Test conversion with let bindings
        let let_term = Term::App(
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(1))),
                Rc::new(Term::Var("x".to_string())),
            )),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let let_ty = let_term.type_check(&ctx).expect("Failed to type check let binding");
        assert!(matches!(&*let_ty, Type::Type(level) if level.0 == 1));

        // Test conversion with nested applications
        let nested_app = Term::App(
            Rc::new(Term::App(
                Rc::new(Term::Lambda(
                    "f".to_string(),
                    Rc::new(Type::Pi(
                        "x".to_string(),
                        Rc::new(Type::Type(Level::new(0))),
                        Rc::new(Type::Type(Level::new(0))),
                    )),
                    Rc::new(Term::Lambda(
                        "x".to_string(),
                        Rc::new(Type::Type(Level::new(0))),
                        Rc::new(Term::App(
                            Rc::new(Term::Var("f".to_string())),
                            Rc::new(Term::Var("x".to_string())),
                        )),
                    )),
                )),
                Rc::new(Term::Lambda(
                    "x".to_string(),
                    Rc::new(Type::Type(Level::new(0))),
                    Rc::new(Term::Var("x".to_string())),
                )),
            )),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let nested_ty = nested_app.type_check(&ctx).expect("Failed to type check nested application");
        assert!(matches!(&*nested_ty, Type::Type(level) if level.0 == 0));
    }

    #[test]
    fn test_universe_polymorphism() {
        let mut ctx = setup_context();

        // Add universe constraints for cumulativity
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        // Test 1: Basic universe polymorphic identity function
        // id : ∀(T: Type₁). T → T
        let id_term = Term::Lambda(
            "T".to_string(),
            Rc::new(Type::Type(Level::new(1))),
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Named("T".to_string())),
                Rc::new(Term::Var("x".to_string())),
            )),
        );

        // Verify id has the correct type
        let id_type = id_term.type_check(&ctx).unwrap();
        assert!(matches!(&*id_type, Type::Pi(t, t_type, body_type) 
            if t == "T" 
            && matches!(&**t_type, Type::Type(level) if level.0 == 1)
            && matches!(&**body_type, Type::Pi(x, param_type, return_type)
                if x == "x"
                && matches!(&**param_type, Type::Named(name) if name == "T")
                && matches!(&**return_type, Type::Named(name) if name == "T"))));

        // Test 2: Universe polymorphism with Type₀
        // id Type₀ : Type₀ → Type₀
        let type0_term = Term::Sort(Universe::Type(Level::new(0)));
        let id_type0 = Term::App(Rc::new(id_term.clone()), Rc::new(type0_term.clone()));
        let result_type0 = id_type0.type_check(&ctx).unwrap();

        // The result should be Type₀ → Type₀
        assert!(
            matches!(&*result_type0, Type::Pi(_, param_type, return_type)
            if matches!(&**param_type, Type::Named(name) if name == "T")
            && matches!(&**return_type, Type::Named(name) if name == "T"))
        );

        // Test 3: Universe polymorphism with Type₁
        // id Type₁ : Type₁ → Type₁
        let type1_term = Term::Sort(Universe::Type(Level::new(1)));
        let id_type1 = Term::App(Rc::new(id_term.clone()), Rc::new(type1_term.clone()));
        let result_type1 = id_type1.type_check(&ctx).unwrap();

        // The result should be Type₁ → Type₁
        assert!(
            matches!(&*result_type1, Type::Pi(_, param_type, return_type)
            if matches!(&**param_type, Type::Named(name) if name == "T")
            && matches!(&**return_type, Type::Named(name) if name == "T"))
        );

        // Test 4: Universe cumulativity
        // Since Type₀ : Type₁, we can use Type₀ where Type₁ is expected
        let cumul_term = Term::App(
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(1))),
                Rc::new(Term::Var("x".to_string())),
            )),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        // This should type check because of cumulativity
        assert!(cumul_term.type_check(&ctx).is_ok());

        // Test 5: Application preserves universe levels
        let applied0 = Term::App(Rc::new(id_type0), Rc::new(type0_term));
        let applied0_type = applied0.type_check(&ctx).unwrap();
        assert!(matches!(&*applied0_type, Type::Type(level) if level.0 == 0));

        let applied1 = Term::App(Rc::new(id_type1), Rc::new(type1_term));
        let applied1_type = applied1.type_check(&ctx).unwrap();
        assert!(matches!(&*applied1_type, Type::Type(level) if level.0 == 1));
    }
}
