use super::*;

#[cfg(test)]
pub mod test_edge_cases {
    use super::*;

    #[test]
    fn test_unbound_variable_errors() {
        let ctx = setup_context();

        // Test unbound variable in term
        let term = Term::Var("undefined".to_string());
        match term.type_check(&ctx) {
            Err(TypeError::UnboundVariable(_)) => (),
            _ => panic!("Expected UnboundVariable error"),
        }

        // Test unbound variable in type
        let term = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Named("undefined".to_string())),
            Rc::new(Term::Var("x".to_string())),
        );
        match term.type_check(&ctx) {
            Err(TypeError::UnboundVariable(_)) => (),
            _ => panic!("Expected UnboundVariable error"),
        }
    }

    #[test]
    fn test_type_mismatch_errors() {
        let mut ctx = setup_context();

        // Test type mismatch in application
        let term = Term::App(
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(0))),
                Rc::new(Term::Var("x".to_string())),
            )),
            Rc::new(Term::Sort(Universe::Prop)), // Prop when Type₀ expected
        );
        match term.type_check(&ctx) {
            Err(TypeError::TypeMismatch { .. }) => (),
            _ => panic!("Expected TypeMismatch error"),
        }

        // Test type mismatch in pattern matching
        let bool_type = InductiveType {
            name: "Bool".to_string(),
            params: vec![],
            universe_level: Level::new(0),
            constructors: vec![
                Constructor {
                    name: "true".to_string(),
                    params: vec![],
                    return_type: Rc::new(Type::Named("Bool".to_string())),
                },
                Constructor {
                    name: "false".to_string(),
                    params: vec![],
                    return_type: Rc::new(Type::Named("Bool".to_string())),
                },
            ],
        };
        ctx.add_definition(
            "Bool".to_string(),
            Rc::new(Type::Named("Bool".to_string())),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let term = Term::Match(
            Rc::new(Term::Sort(Universe::Prop)), // Trying to match on Prop instead of Bool
            vec![MatchBranch {
                pattern: Pattern {
                    constructor: "true".to_string(),
                    bound_vars: vec![],
                },
                body: Rc::new(Term::Constructor("false".to_string(), Rc::new(Term::Unit))),
            }],
        );
        match term.type_check(&ctx) {
            Err(TypeError::TypeMismatch { .. }) => (),
            _ => panic!("Expected TypeMismatch error"),
        }
    }

    #[test]
    fn test_universe_inconsistency_errors() {
        let mut ctx = setup_context();

        // Test universe inconsistency with circular constraints
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

        let term = Term::Sort(Universe::Type(Level::new(0)));
        match term.type_check(&ctx) {
            Err(TypeError::UniverseError(_)) => (),
            _ => panic!("Expected Universe error"),
        }
    }

    #[test]
    #[ignore]
    fn test_non_positive_recursive_types() {
        // TODO: Implement test for non-positive recursive types
        unimplemented!("Test non-positive recursive types");
    }

    #[test]
    #[ignore]
    fn test_invalid_pattern_matching() {
        // TODO: Implement test for invalid pattern matching
        unimplemented!("Test invalid pattern matching");
    }

    #[test]
    #[ignore]
    fn test_universe_level_overflow() {
        // TODO: Implement test for universe level overflow
        unimplemented!("Test universe level overflow");
    }

    #[test]
    #[ignore]
    fn test_circular_definitions() {
        // TODO: Implement test for circular definitions
        unimplemented!("Test circular definitions");
    }

    #[test]
    #[ignore]
    fn test_invalid_recursion() {
        // TODO: Implement test for invalid recursion
        unimplemented!("Test invalid recursion");
    }

    #[test]
    #[ignore]
    fn test_ambiguous_unification() {
        // TODO: Implement test for ambiguous unification
        unimplemented!("Test ambiguous unification");
    }

    #[test]
    #[ignore]
    fn test_invalid_universe_constraints() {
        // TODO: Implement test for invalid universe constraints
        unimplemented!("Test invalid universe constraints");
    }

    #[test]
    #[ignore]
    fn test_type_in_type_paradox() {
        // TODO: Implement test for Type-in-Type paradox
        unimplemented!("Test Type-in-Type paradox");
    }

    #[test]
    #[ignore]
    fn test_impredicativity_violations() {
        // TODO: Implement test for impredicativity violations
        unimplemented!("Test impredicativity violations");
    }
}
