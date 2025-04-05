use super::*;

#[cfg(test)]
pub mod advanced_cic_features_tests {
    use std::rc::Rc;

    use crate::foundational_theories::type_theory_v2::calculi::cic::{
        context::{ConstraintKind, UniverseConstraint},
        tests::setup_context,
        Level, Term, Type, TypeChecker, Universe,
    };

    #[test]
    fn test_subtyping_with_universes() {
        let mut ctx = setup_context();

        // Test that Type₀ is a subtype of Type₁
        let type0 = Term::Sort(Universe::Type(Level::new(0)));
        let type1 = Term::Sort(Universe::Type(Level::new(1)));

        // Add constraint Type₀ < Type₁
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        // A term of type Type₀ should be accepted where Type₁ is expected
        let term = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(1))),
            Rc::new(type0),
        );

        assert!((&term).type_check(&ctx).is_ok());
    }

    #[test]
    fn test_conversion_checking() {
        let ctx = setup_context();

        // Test conversion between beta-equivalent terms
        let term1 = Term::App(
            Rc::new(Term::Lambda(
                "x".to_string(),
                Rc::new(Type::Type(Level::new(0))),
                Rc::new(Term::Var("x".to_string())),
            )),
            Rc::new(Term::Sort(Universe::Type(Level::new(0)))),
        );

        let term2 = Term::Sort(Universe::Type(Level::new(0)));

        // Both terms should have the same type after reduction
        let ty1 = (&term1)
            .type_check(&ctx)
            .expect("Failed to type check term1");
        let ty2 = (&term2)
            .type_check(&ctx)
            .expect("Failed to type check term2");

        assert!(matches!(&*ty1, Type::Type(l) if l.0 == 1));
        assert!(matches!(&*ty2, Type::Type(l) if l.0 == 1));
    }

    #[test]
    fn test_cumulative_universes() {
        let mut ctx = setup_context();

        // Test cumulativity: if A : Type₀ then A : Type₁
        let term = Term::Sort(Universe::Type(Level::new(0)));

        // Add cumulativity constraint
        ctx.constraints.push(UniverseConstraint {
            left: Level::new(0),
            right: Level::new(1),
            kind: ConstraintKind::LessThan,
        });

        // Should be able to type check against both Type₀ and Type₁
        let ty1 = (&term)
            .type_check(&ctx)
            .expect("Failed to type check against Type₁");
        assert!(matches!(&*ty1, Type::Type(l) if l.0 == 1));

        // The term itself should also type check in Type₁
        let lambda = Term::Lambda(
            "x".to_string(),
            Rc::new(Type::Type(Level::new(1))),
            Rc::new(term.clone()),
        );
        assert!((&lambda).type_check(&ctx).is_ok());
    }

    #[test]
    #[ignore]
    fn test_universe_polymorphism() {
        // TODO: Implement test for universe polymorphism
        unimplemented!("Test universe polymorphism");
    }

    #[test]
    #[ignore]
    fn test_higher_order_unification() {
        // TODO: Implement test for higher-order unification
        unimplemented!("Test higher-order unification");
    }

    #[test]
    #[ignore]
    fn test_implicit_arguments() {
        // TODO: Implement test for implicit arguments
        unimplemented!("Test implicit arguments");
    }

    #[test]
    #[ignore]
    fn test_coercive_subtyping() {
        // TODO: Implement test for coercive subtyping
        unimplemented!("Test coercive subtyping");
    }

    #[test]
    #[ignore]
    fn test_proof_irrelevance() {
        // TODO: Implement test for proof irrelevance
        unimplemented!("Test proof irrelevance");
    }

    #[test]
    #[ignore]
    fn test_strict_positivity() {
        // TODO: Implement test for strict positivity checking
        unimplemented!("Test strict positivity");
    }

    #[test]
    #[ignore]
    fn test_sized_types() {
        // TODO: Implement test for sized types
        unimplemented!("Test sized types");
    }

    #[test]
    #[ignore]
    fn test_predicative_polymorphism() {
        // TODO: Implement test for predicative polymorphism
        unimplemented!("Test predicative polymorphism");
    }

    #[test]
    #[ignore]
    fn test_impredicative_prop() {
        // TODO: Implement test for impredicative Prop
        unimplemented!("Test impredicative Prop");
    }

    #[test]
    #[ignore]
    fn test_singleton_elimination() {
        // TODO: Implement test for singleton elimination
        unimplemented!("Test singleton elimination");
    }
}
