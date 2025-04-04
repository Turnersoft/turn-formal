use super::*;

#[cfg(test)]
pub mod test_reduction {
    use super::*;

    #[test]
    fn test_beta_reduction() {
        let ctx = setup_context();

        // (λx:Type₀. x) Type₀
        let term = Term::App(
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(0))),
                Rc::new(Term::Var("x".to_string())),
            )),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let reduced = term.reduce().expect("Failed to reduce term");
        assert!(matches!(&*reduced, Term::Sort(Universe::Type(l)) if l.0 == 0));
    }

    #[test]
    fn test_delta_reduction() {
        let mut ctx = setup_context();

        // Add a definition to the context
        let id_fn = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Var("x".to_string())),
        );
        ctx.add_definition(
            "id".to_string(),
            Rc::new(Type::Pi(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(0))),
                Rc::new(Type::Type(Level::new(0))),
            )),
            Rc::new(id_fn),
        );

        // Test reduction of a variable bound to a definition
        let term = Term::App(
            Rc::new(Term::Var("id".to_string())),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let reduced = term.reduce().expect("Failed to reduce term");
        assert!(matches!(&*reduced, Term::Sort(Universe::Type(l)) if l.0 == 0));
    }

    #[test]
    fn test_pattern_match_reduction() {
        let mut ctx = setup_context();

        // Define Bool type
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

        // match true with
        // | true => false
        // | false => true
        let term = Term::Match(
            Rc::new(Term::Constructor("true".to_string(), Rc::new(Term::Unit))),
            vec![
                MatchBranch {
                    pattern: Pattern {
                        constructor: "true".to_string(),
                        bound_vars: vec![],
                    },
                    body: Rc::new(Term::Constructor("false".to_string(), Rc::new(Term::Unit))),
                },
                MatchBranch {
                    pattern: Pattern {
                        constructor: "false".to_string(),
                        bound_vars: vec![],
                    },
                    body: Rc::new(Term::Constructor("true".to_string(), Rc::new(Term::Unit))),
                },
            ],
        );

        let reduced = term.reduce().expect("Failed to reduce term");
        assert!(matches!(&*reduced, Term::Constructor(name, _) if name == "false"));
    }

    #[test]
    #[ignore]
    fn test_eta_reduction() {
        // TODO: Implement test for eta reduction
        unimplemented!("Test eta reduction");
    }

    #[test]
    #[ignore]
    fn test_zeta_reduction() {
        // TODO: Implement test for zeta reduction (let bindings)
        unimplemented!("Test zeta reduction");
    }

    #[test]
    #[ignore]
    fn test_iota_reduction() {
        // TODO: Implement test for iota reduction (fix-point reduction)
        unimplemented!("Test iota reduction");
    }

    #[test]
    #[ignore]
    fn test_strong_normalization() {
        // TODO: Implement test for strong normalization
        unimplemented!("Test strong normalization");
    }

    #[test]
    #[ignore]
    fn test_weak_head_normal_form() {
        // TODO: Implement test for weak head normal form reduction
        unimplemented!("Test weak head normal form");
    }

    #[test]
    #[ignore]
    fn test_head_normal_form() {
        // TODO: Implement test for head normal form reduction
        unimplemented!("Test head normal form");
    }

    #[test]
    #[ignore]
    fn test_normal_form() {
        // TODO: Implement test for normal form reduction
        unimplemented!("Test normal form");
    }

    #[test]
    #[ignore]
    fn test_reduction_strategies() {
        // TODO: Implement test for different reduction strategies
        unimplemented!("Test reduction strategies");
    }

    #[test]
    #[ignore]
    fn test_reduction_confluence() {
        // TODO: Implement test for reduction confluence (Church-Rosser)
        unimplemented!("Test reduction confluence");
    }
}
