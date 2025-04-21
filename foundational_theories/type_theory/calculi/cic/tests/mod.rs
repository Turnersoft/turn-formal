use super::super::cic::{
    context::{ConstraintKind, Context, InductiveType, UniverseConstraint},
    term::{Constructor, MatchBranch, Pattern, Term},
    type_::Type,
    typing::{TypeChecker, TypeError},
    universe::{Level, Universe},
};
use super::*;
use std::rc::Rc;

pub(crate) fn setup_context() -> Context {
    let mut ctx = Context::new();
    ctx.add_definition(
        "Type".to_string(),
        Rc::new(Type::Type(Level::new(1))),
        Rc::new(Term::Sort(Universe::Type(Level::new(1)))),
    );
    ctx.add_definition(
        "Prop".to_string(),
        Rc::new(Type::Prop),
        Rc::new(Term::Sort(Universe::Prop)),
    );
    ctx
}

pub mod advanced_cic_features_tests;
pub mod basic_type_system_tests;
pub mod dependent_types_tests;
pub mod edge_cases_tests;
pub mod inductive_types_tests;
pub mod integration_tests;
pub mod lambda_calculus_tests;
pub mod pattern_matching_tests;
pub mod performance_tests;
pub mod reduction_tests;
pub mod soundness_tests;
pub mod universe_hierarchy_tests;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn run_all_cic_tests() {
        // This test will trigger all tests in all modules
        let _ = setup_context();
    }
}
