use super::*;

#[cfg(test)]
pub mod test_integration {
    use super::*;
    use crate::formalize_v2::foundational_theories::type_theory_v2::calculi::cic::{
        context::Context,
        term::Term,
        typing::TypeChecker,
        universe::{Level, Universe},
        Type,
    };
    use std::rc::Rc;

    #[test]
    #[ignore]
    fn test_type_inference_with_inductive_types() {
        // TODO: Implement test for type inference with inductive types
        unimplemented!("Test type inference with inductive types");
    }

    #[test]
    #[ignore]
    fn test_pattern_matching_with_dependent_types() {
        // TODO: Implement test for pattern matching with dependent types
        unimplemented!("Test pattern matching with dependent types");
    }

    #[test]
    #[ignore]
    fn test_universe_polymorphism_with_inductive_types() {
        // TODO: Implement test for universe polymorphism with inductive types
        unimplemented!("Test universe polymorphism with inductive types");
    }

    #[test]
    #[ignore]
    fn test_reduction_with_pattern_matching() {
        // TODO: Implement test for reduction with pattern matching
        unimplemented!("Test reduction with pattern matching");
    }

    #[test]
    fn test_type_checking_with_universe_constraints() {
        let mut ctx = Context::new();

        // Test 1: Type₀ : Type₁
        let type0 = Term::Sort(Universe::Type(Level::new(0)));
        let type_of_type0 = type0.type_check(&ctx).unwrap();
        assert_eq!(type_of_type0, Rc::new(Type::Type(Level::new(1))));

        // Test 2: Type₁ : Type₂
        let type1 = Term::Sort(Universe::Type(Level::new(1)));
        let type_of_type1 = type1.type_check(&ctx).unwrap();
        assert_eq!(type_of_type1, Rc::new(Type::Type(Level::new(2))));

        // Test 3: Identity function preserves universe level
        let x = "x".to_string();
        let type0_var = Term::Sort(Universe::Type(Level::new(0)));
        let var = Term::Var(x.clone());
        let id_fn = Term::Lambda(x.clone(), Rc::new(Type::Type(Level::new(0))), Rc::new(var));

        let id_fn_type = id_fn.type_check(&ctx).unwrap();
        assert!(matches!(&*id_fn_type, Type::Pi(_, param_type, return_type)
            if matches!(&**param_type, Type::Type(l1) if l1.0 == 0)
            && matches!(&**return_type, Type::Type(l2) if l2.0 == 0)));

        // Test 4: Application of identity function to Type₀
        let app = Term::App(Rc::new(id_fn), Rc::new(type0));
        let app_type = app.type_check(&ctx).unwrap();
        assert_eq!(app_type, Rc::new(Type::Type(Level::new(0))));
    }
}
