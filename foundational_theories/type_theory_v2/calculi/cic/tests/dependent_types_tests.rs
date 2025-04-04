use super::*;

#[cfg(test)]
pub mod test_dependent_types {
    use super::*;

    #[test]
    fn test_dependent_function_type() {
        let ctx = setup_context();

        // Test a dependent function type: Π(A:Type₀). A → A
        let term = Term::Pi(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Pi(
                "x".to_string(),
                Rc::new(Type::Named("A".to_string())),
                Rc::new(Term::Var("A".to_string())),
            )),
        );

        let ty = (&term)
            .type_check(&ctx)
            .expect("Failed to type check dependent function type");
        match &*ty {
            Type::Type(l) if l.0 == 1 => (),
            _ => panic!("Expected Type₁"),
        }
    }

    #[test]
    fn test_dependent_application() {
        let mut ctx = setup_context();

        // Create a dependent function: λ(A:Type₀). λ(x:A). x
        let id_fn = Term::Lambda(
            "A".to_string(),
            Rc::new(Type::Type(Level::new(0))),
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Named("A".to_string())),
                Rc::new(Term::Var("x".to_string())),
            )),
        );

        // Apply it to Type₀
        let applied = Term::App(
            Rc::new(id_fn),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let ty = (&applied)
            .type_check(&ctx)
            .expect("Failed to type check dependent application");

        // Should have type Type₀ → Type₀
        match &*ty {
            Type::Pi(_, param_ty, body_ty) => {
                assert!(matches!(&**param_ty, Type::Type(l) if l.0 == 0));
                assert!(matches!(&**body_ty, Type::Type(l) if l.0 == 0));
            }
            _ => panic!("Expected Pi type"),
        }
    }

    #[test]
    #[ignore]
    fn test_dependent_pair_type() {
        // TODO: Implement test for dependent pair types (Σ-types)
        unimplemented!("Test dependent pair types");
    }

    #[test]
    #[ignore]
    fn test_dependent_pattern_matching() {
        // TODO: Implement test for dependent pattern matching
        unimplemented!("Test dependent pattern matching");
    }

    #[test]
    #[ignore]
    fn test_equality_types() {
        // TODO: Implement test for equality types (identity types)
        unimplemented!("Test equality types");
    }

    #[test]
    #[ignore]
    fn test_path_types() {
        // TODO: Implement test for path types and their properties
        unimplemented!("Test path types");
    }

    #[test]
    #[ignore]
    fn test_transport() {
        // TODO: Implement test for transport (coercion along equality)
        unimplemented!("Test transport");
    }

    #[test]
    #[ignore]
    fn test_j_eliminator() {
        // TODO: Implement test for J eliminator (identity type elimination)
        unimplemented!("Test J eliminator");
    }

    #[test]
    #[ignore]
    fn test_singleton_types() {
        // TODO: Implement test for singleton types
        unimplemented!("Test singleton types");
    }

    #[test]
    #[ignore]
    fn test_large_elimination() {
        // TODO: Implement test for large elimination (type-level computation)
        unimplemented!("Test large elimination");
    }

    #[test]
    #[ignore]
    fn test_dependent_type_families() {
        // TODO: Implement test for dependent type families
        unimplemented!("Test dependent type families");
    }

    #[test]
    #[ignore]
    fn test_dependent_type_inference() {
        // TODO: Implement test for dependent type inference
        unimplemented!("Test dependent type inference");
    }
}
