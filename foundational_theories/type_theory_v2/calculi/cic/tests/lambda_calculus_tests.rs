use super::*;

#[cfg(test)]
pub mod test_lambda_calculus {
    use super::*;

    #[test]
    fn test_basic_lambda() {
        let ctx = setup_context();

        // λx:Prop. x
        let term = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Prop),
            Rc::new(Term::Var("x".to_string())),
        );

        let ty = (&term)
            .type_check(&ctx)
            .expect("Failed to type check lambda term");
        match &*ty {
            Type::Pi(_, _, _) => (),
            _ => panic!("Expected Pi type"),
        }
    }

    #[test]
    fn test_application() {
        let ctx = setup_context();

        // (λx:Prop. x) Prop
        let term = Term::App(
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Prop),
                Rc::new(Term::Var("x".to_string())),
            )),
            Rc::new(Term::Sort(Universe::Prop)),
        );

        let reduced = term.reduce().expect("Failed to reduce term");
        if !matches!(*reduced, Term::Sort(Universe::Prop)) {
            panic!("Expected Prop universe");
        }
    }

    #[test]
    #[ignore]
    fn test_alpha_conversion() {
        // TODO: Implement test for alpha conversion
        unimplemented!("Test alpha conversion");
    }

    #[test]
    #[ignore]
    fn test_capture_avoiding_substitution() {
        // TODO: Implement test for capture-avoiding substitution
        unimplemented!("Test capture-avoiding substitution");
    }

    #[test]
    #[ignore]
    fn test_beta_reduction_multiple_args() {
        // TODO: Implement test for beta reduction with multiple arguments
        unimplemented!("Test beta reduction with multiple arguments");
    }

    #[test]
    #[ignore]
    fn test_eta_conversion() {
        // TODO: Implement test for eta conversion
        unimplemented!("Test eta conversion");
    }

    #[test]
    #[ignore]
    fn test_normal_forms() {
        // TODO: Implement test for normal forms
        unimplemented!("Test normal forms");
    }

    #[test]
    #[ignore]
    fn test_weak_head_normal_forms() {
        // TODO: Implement test for weak head normal forms
        unimplemented!("Test weak head normal forms");
    }

    #[test]
    #[ignore]
    fn test_confluence_of_reduction() {
        // TODO: Implement test for confluence of reduction
        unimplemented!("Test confluence of reduction");
    }

    #[test]
    #[ignore]
    fn test_church_numerals() {
        // TODO: Implement test for Church numerals and basic arithmetic
        unimplemented!("Test Church numerals and basic arithmetic");
    }

    #[test]
    #[ignore]
    fn test_y_combinator() {
        // TODO: Implement test for Y combinator and fixed points
        unimplemented!("Test Y combinator and fixed points");
    }
}
