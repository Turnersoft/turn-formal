//! Type theory foundation for propositional logic

use crate::parse::entities::Identifier;
use crate::{
    formalize_v2::{
        foundational_theories::type_theory_v2::calculi::simply_typed::{
            goals::Context,
            terms::{SumSide, Term},
            types::Type,
        },
        subjects::logic::propositional::{tactics::TacticError, Proposition},
    },
    parse::Parse,
};
use std::collections::HashMap;
use uuid::Uuid;

/// Type theory foundation for propositional logic
#[derive(Debug, Default, Clone)]
pub struct TypeTheoryFoundation {
    base_counters: HashMap<String, u32>,
    stlc_context: Context,
}

impl TypeTheoryFoundation {
    /// Create a new type theory foundation
    pub fn new() -> Self {
        TypeTheoryFoundation {
            base_counters: HashMap::new(),
            stlc_context: Context::new(),
        }
    }

    /// Create a new identifier with the given base name
    fn create_identifier(&mut self, base: &str) -> Identifier {
        let counter = self.base_counters.entry(base.to_string()).or_insert(0);
        *counter += 1;
        Identifier::parse(&format!("{}_{}", base, counter))
    }

    /// Add an assumption to the context
    pub fn add_assumption(&mut self, prop: &Proposition) -> Term {
        // Just create a new variable name for the assumption
        let var = self.create_identifier(&format!("x_\"{}\"", prop.to_string()));
        Term::Variable(var)
    }

    /// Make an assumption, looking it up in the context
    pub fn make_assumption(&mut self, prop: &Proposition) -> Result<Term, TacticError> {
        // Create the term
        let var = self.create_identifier(&format!("x_\"{}\"", prop.to_string()));

        // Add it to our STLC context with proper type
        self.stlc_context
            .add_variable(var.clone(), self.proposition_to_type(prop));

        Ok(Term::Variable(var))
    }

    /// Convert a proposition to its type theory representation
    pub fn proposition_to_type(&self, prop: &Proposition) -> Type {
        match prop {
            Proposition::True => Type::Unit,
            Proposition::False => Type::Bottom,
            Proposition::Atomic(_) => Type::Unit,
            Proposition::And(p, q) => Type::Product {
                left: Box::new(self.proposition_to_type(p)),
                right: Box::new(self.proposition_to_type(q)),
            },
            Proposition::Or(p, q) => Type::Sum {
                left: Box::new(self.proposition_to_type(p)),
                right: Box::new(self.proposition_to_type(q)),
            },
            Proposition::Implies(p, q) => Type::Function {
                param_type: Box::new(self.proposition_to_type(p)),
                return_type: Box::new(self.proposition_to_type(q)),
            },
            Proposition::Not(p) => Type::Function {
                param_type: Box::new(self.proposition_to_type(p)),
                return_type: Box::new(Type::Bottom),
            },
        }
    }

    /// Create a term for conjunction introduction
    pub fn make_and_intro(&self, left: Term, right: Term) -> Term {
        Term::Product {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a term for conjunction elimination (left)
    pub fn make_and_elim_left(&self, term: Term) -> Term {
        Term::ProjectLeft(Box::new(term))
    }

    /// Create a term for conjunction elimination (right)
    pub fn make_and_elim_right(&self, term: Term) -> Term {
        Term::ProjectRight(Box::new(term))
    }

    /// Create a term for implication introduction
    pub fn make_implies_intro(&mut self, body: Term) -> Term {
        Term::Abstraction {
            param_name: self.create_identifier("x"),
            param_type: Box::new(Type::Unit),
            body: Box::new(body),
        }
    }

    /// Create a term for implication elimination
    pub fn make_implies_elim(&self, func: Term, arg: Term) -> Term {
        Term::Application {
            function: Box::new(func),
            argument: Box::new(arg),
        }
    }

    /// Create a term for disjunction introduction (left)
    pub fn make_or_intro_left(&self, term: Term) -> Term {
        Term::InjectSum {
            term: Box::new(term),
            target_type: Box::new(Type::Sum {
                left: Box::new(Type::Unit),
                right: Box::new(Type::Unit),
            }),
            side: SumSide::Left,
        }
    }

    /// Create a term for disjunction introduction (right)
    pub fn make_or_intro_right(&self, term: Term) -> Term {
        Term::InjectSum {
            term: Box::new(term),
            target_type: Box::new(Type::Sum {
                left: Box::new(Type::Unit),
                right: Box::new(Type::Unit),
            }),
            side: SumSide::Right,
        }
    }

    /// Create a term for disjunction elimination
    pub fn make_or_elim(&self, or_term: Term, left_case: Term, right_case: Term) -> Term {
        Term::CaseSum {
            term: Box::new(or_term),
            left_case: Box::new(left_case),
            right_case: Box::new(right_case),
            result_type: Box::new(Type::Unit), // Default to Unit, should be inferred from context
        }
    }

    /// Create a term for true introduction
    pub fn make_true_intro(&self) -> Term {
        Term::Unit
    }

    /// Create a term for false elimination
    pub fn make_false_elim(&mut self, false_term: Term, goal_type: Type) -> Term {
        // From false (⊥), we can derive anything
        // This is implemented as a function that takes a ⊥ and returns any type
        let var_name = self.create_identifier("x");
        Term::Application {
            function: Box::new(Term::Abstraction {
                param_name: var_name.clone(),
                param_type: Box::new(Type::Bottom),
                body: Box::new(Term::CaseSum {
                    term: Box::new(Term::Variable(var_name)),
                    left_case: Box::new(Term::Unit),
                    right_case: Box::new(Term::Unit),
                    result_type: Box::new(goal_type),
                }),
            }),
            argument: Box::new(false_term),
        }
    }

    /// Create a term for double negation elimination
    pub fn make_double_negation_elim(&mut self, _term: Term) -> Term {
        // Create identifiers for our lambda abstractions
        let not_not_a = self.create_identifier("not_not_a");
        let a_id = self.create_identifier("a");

        // Construct the term that proves A from ¬¬A
        // The basic idea is to create a function that takes ¬¬A and returns A
        // by showing that assuming ¬A leads to a contradiction
        Term::Abstraction {
            param_name: not_not_a.clone(),
            param_type: Box::new(Type::Function {
                param_type: Box::new(Type::Function {
                    param_type: Box::new(Type::Unit),
                    return_type: Box::new(Type::Bottom),
                }),
                return_type: Box::new(Type::Bottom),
            }),
            body: Box::new(Term::Application {
                function: Box::new(Term::Variable(not_not_a)),
                argument: Box::new(Term::Abstraction {
                    param_name: a_id.clone(),
                    param_type: Box::new(Type::Unit),
                    body: Box::new(Term::Variable(a_id)),
                }),
            }),
        }
    }

    /// Create a term for negation introduction
    pub fn make_not_intro(&mut self, contradiction: Term) -> Term {
        Term::Abstraction {
            param_name: self.create_identifier("x"),
            param_type: Box::new(Type::Unit),
            body: Box::new(contradiction),
        }
    }

    /// Create a term for negation elimination
    pub fn make_not_elim(&self, term: Term, not_term: Term) -> Term {
        Term::Application {
            function: Box::new(not_term),
            argument: Box::new(term),
        }
    }

    /// Create a term for contraposition
    pub fn make_contraposition(&mut self, impl_term: Term) -> Term {
        let nb_id = self.create_identifier("nb");
        let a_id = self.create_identifier("a");

        Term::Abstraction {
            param_name: nb_id.clone(),
            param_type: Box::new(Type::Unit),
            body: Box::new(Term::Abstraction {
                param_name: a_id.clone(),
                param_type: Box::new(Type::Unit),
                body: Box::new(Term::Application {
                    function: Box::new(Term::Variable(nb_id)),
                    argument: Box::new(Term::Application {
                        function: Box::new(impl_term),
                        argument: Box::new(Term::Variable(a_id)),
                    }),
                }),
            }),
        }
    }

    /// Create a term for hypothetical syllogism
    pub fn make_hypothetical_syllogism(&mut self, impl1: Term, impl2: Term) -> Term {
        // Get the types from both implications
        let (param_type, _mid_type, _return_type) = match (
            impl1.infer_type(&self.stlc_context),
            impl2.infer_type(&self.stlc_context),
        ) {
            (
                Ok(Type::Function {
                    param_type: p1,
                    return_type: r1,
                }),
                Ok(Type::Function {
                    param_type: p2,
                    return_type: r2,
                }),
            ) => (*p1, *r1, *r2),
            _ => panic!("Expected function types"),
        };

        // Create the composed function
        Term::Abstraction {
            param_name: self.create_identifier("x"),
            param_type: Box::new(param_type),
            body: Box::new(Term::Application {
                function: Box::new(impl2),
                argument: Box::new(Term::Application {
                    function: Box::new(impl1),
                    argument: Box::new(Term::Variable(self.create_identifier("x"))),
                }),
            }),
        }
    }
}
