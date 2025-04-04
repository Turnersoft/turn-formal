use super::term::{Constructor, MatchBranch, Pattern, Term};
use super::type_::Type;
use super::typing::{TypeChecker, TypeError};
use super::universe::{Level, Universe};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct InductiveType {
    pub name: String,
    pub params: Vec<(String, Rc<Type>)>,
    pub constructors: Vec<Constructor>,
    pub universe_level: Level,
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub ty: Rc<Type>,
    pub term: Rc<Term>,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub vars: HashMap<String, Rc<Type>>,
    pub definitions: HashMap<String, Definition>,
    pub inductives: HashMap<String, InductiveType>,
    pub constraints: Vec<UniverseConstraint>,
}

#[derive(Debug, Clone)]
pub struct UniverseConstraint {
    pub left: Level,
    pub right: Level,
    pub kind: ConstraintKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintKind {
    LessThan,
    Equal,
}

impl Context {
    pub fn new() -> Self {
        Context {
            vars: HashMap::new(),
            definitions: HashMap::new(),
            inductives: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_var(&mut self, name: String, ty: Rc<Type>) {
        self.vars.insert(name, ty);
    }

    pub fn add_definition(&mut self, name: String, ty: Rc<Type>, term: Rc<Term>) {
        self.definitions.insert(
            name.clone(),
            Definition {
                name: name.clone(),
                ty,
                term,
            },
        );
    }

    pub fn lookup_type(&self, name: &str) -> Option<Rc<Type>> {
        if let Some(def) = self.definitions.get(name) {
            return Some(def.ty.clone());
        }
        self.vars.get(name).cloned()
    }

    pub fn lookup_definition(&self, name: &str) -> Option<&Definition> {
        self.definitions.get(name)
    }

    pub fn add_universe_constraint(&mut self, constraint: UniverseConstraint) {
        self.constraints.push(constraint);
    }

    pub fn add_inductive_type(&mut self, name: String, ind_type: InductiveType) {
        // First add the type constructor
        if !ind_type.params.is_empty() {
            let mut ty = Rc::new(Type::Type(ind_type.universe_level.clone()));
            for (param_name, param_type) in ind_type.params.iter().rev() {
                ty = Rc::new(Type::Pi(param_name.clone(), param_type.clone(), ty));
            }
            self.add_var(name.clone(), ty);
        } else {
            self.add_var(
                name.clone(),
                Rc::new(Type::Type(ind_type.universe_level.clone())),
            );
        }

        // Add the inductive type itself
        self.inductives.insert(name.clone(), ind_type.clone());

        // Add each constructor as a definition
        for constructor in ind_type.constructors {
            let mut constructor_type = constructor.return_type.clone();

            // Add constructor parameters
            for (param_name, param_type) in constructor.params.iter().rev() {
                constructor_type = Rc::new(Type::Pi(
                    param_name.clone(),
                    param_type.clone(),
                    constructor_type,
                ));
            }

            // Add type parameters
            for (param_name, param_type) in ind_type.params.iter().rev() {
                constructor_type = Rc::new(Type::Pi(
                    param_name.clone(),
                    param_type.clone(),
                    constructor_type,
                ));
            }

            // Create constructor term with correct number of parameters
            let mut constructor_term = Rc::new(Term::Constructor(
                constructor.name.clone(),
                Rc::new(Term::Unit),
            ));

            // Add type parameters as arguments
            for (param_name, _) in ind_type.params.iter() {
                constructor_term = Rc::new(Term::App(
                    constructor_term,
                    Rc::new(Term::Var(param_name.clone())),
                ));
            }

            // Add constructor parameters as arguments
            for (param_name, _) in constructor.params.iter() {
                constructor_term = Rc::new(Term::App(
                    constructor_term,
                    Rc::new(Term::Var(param_name.clone())),
                ));
            }

            self.add_definition(constructor.name.clone(), constructor_type, constructor_term);
        }
    }

    pub fn check_type(&self, ty: &Rc<Type>) -> Result<(), TypeError> {
        match &**ty {
            Type::Named(name) => {
                // For named types, check if they exist in the context
                if self.lookup_type(name).is_some() {
                    Ok(())
                } else {
                    Err(TypeError::UnboundVariable(name.clone()))
                }
            }
            Type::Pi(_, param_ty, return_ty) => {
                self.check_type(param_ty)?;
                self.check_type(return_ty)
            }
            Type::Prop => Ok(()),
            Type::Type(level) => {
                // Check universe constraints
                for constraint in &self.constraints {
                    match constraint.kind {
                        ConstraintKind::LessThan => {
                            if !(constraint.left < constraint.right) {
                                return Err(TypeError::UniverseError(format!(
                                    "Universe level constraint {} < {} violated",
                                    constraint.left, constraint.right
                                )));
                            }
                        }
                        ConstraintKind::Equal => {
                            if !(constraint.left == constraint.right) {
                                return Err(TypeError::UniverseError(format!(
                                    "Universe level equality constraint {} = {} violated",
                                    constraint.left, constraint.right
                                )));
                            }
                        }
                    }
                }
                Ok(())
            }
            // Primitive types are always well-formed
            Type::Number | Type::Bool | Type::Bottom | Type::Top | Type::Unit => Ok(()),
            // Type constructors - check their components
            Type::Sum(left, right) | Type::Product(left, right) | Type::Function(left, right) => {
                self.check_type(left)?;
                self.check_type(right)
            }
            Type::App(ty, term) => {
                self.check_type(ty)?;
                term.type_check(self).map(|_| ())
            }
        }
    }

    pub fn types_convertible(&self, ty1: &Rc<Type>, ty2: &Rc<Type>) -> bool {
        self.types_equal(&**ty1, &**ty2)
    }

    pub fn types_equal(&self, ty1: &Type, ty2: &Type) -> bool {
        match (ty1, ty2) {
            // Named types - check if they refer to the same type
            (Type::Named(n1), Type::Named(n2)) => {
                if n1 == n2 {
                    return true;
                }
                // If names are different, check their definitions
                match (self.lookup_type(n1), self.lookup_type(n2)) {
                    (Some(def1), Some(def2)) => self.types_equal(&def1, &def2),
                    _ => false,
                }
            }
            // Simply typed lambda calculus (λ→)
            (Type::Function(d1, c1), Type::Function(d2, c2)) => {
                self.types_equal(&**d1, &**d2) && self.types_equal(&**c1, &**c2)
            }
            (Type::Number, Type::Number) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Unit, Type::Unit) => true,
            (Type::Bottom, Type::Bottom) => true,
            (Type::Top, Type::Top) => true,
            (Type::Product(l1, r1), Type::Product(l2, r2)) => {
                self.types_equal(&**l1, &**l2) && self.types_equal(&**r1, &**r2)
            }
            (Type::Sum(l1, r1), Type::Sum(l2, r2)) => {
                self.types_equal(&**l1, &**l2) && self.types_equal(&**r1, &**r2)
            }

            // Lambda P (λP) and CoC
            (Type::Pi(x1, t1, u1), Type::Pi(x2, t2, u2)) => {
                self.types_equal(&**t1, &**t2) && self.types_equal(&**u1, &**u2)
            }
            (Type::Prop, Type::Prop) => true, // Propositions
            (Type::Type(l1), Type::Type(l2)) => l1 == l2,

            _ => false,
        }
    }

    pub fn terms_equal(&self, t1: &Term, t2: &Term) -> bool {
        match (t1, t2) {
            // Simply typed terms
            (Term::Number(n1), Term::Number(n2)) => n1 == n2,
            (Term::Bool(b1), Term::Bool(b2)) => b1 == b2,
            (Term::Unit, Term::Unit) => true,
            (Term::Pair(l1, r1), Term::Pair(l2, r2)) => {
                self.terms_equal(&**l1, &**l2) && self.terms_equal(&**r1, &**r2)
            }
            (Term::Left(t1, ty1), Term::Left(t2, ty2)) => {
                self.terms_equal(&**t1, &**t2) && self.types_equal(&**ty1, &**ty2)
            }
            (Term::Right(t1, ty1), Term::Right(t2, ty2)) => {
                self.terms_equal(&**t1, &**t2) && self.types_equal(&**ty1, &**ty2)
            }

            // Lambda terms (common to all calculi)
            (Term::Var(x1), Term::Var(x2)) => x1 == x2,
            (Term::App(f1, a1), Term::App(f2, a2)) => {
                self.terms_equal(&**f1, &**f2) && self.terms_equal(&**a1, &**a2)
            }
            (Term::Lambda(x1, t1, b1), Term::Lambda(x2, t2, b2)) => {
                self.types_equal(&**t1, &**t2) && self.terms_equal(&**b1, &**b2)
            }

            // CIC specific
            (Term::Constructor(n1, a1), Term::Constructor(n2, a2)) => {
                n1 == n2 && self.terms_equal(&**a1, &**a2)
            }
            (Term::Sort(u1), Term::Sort(u2)) => u1 == u2,

            _ => false,
        }
    }

    pub fn max_level(&self, ty1: &Type, ty2: &Type) -> Result<Level, TypeError> {
        match (ty1, ty2) {
            (Type::Type(l1), Type::Type(l2)) => Ok((*l1).max(*l2)),
            _ => Err(TypeError::TypeMismatch {
                expected: Rc::new(Type::Type(Level::new(0))),
                got: Rc::new(ty1.clone()),
            }),
        }
    }

    pub fn check_constructor(&self, ind_name: &str, ctor: &Constructor) -> Result<(), TypeError> {
        // Check that constructor types are well-formed and return to parent type
        for (_, ty) in &ctor.params {
            self.check_type(ty)?;
        }
        self.check_type(&ctor.return_type)?;
        Ok(())
    }

    pub fn type_check_constructor(
        &self,
        name: &str,
        args: &Rc<Term>,
    ) -> Result<Rc<Type>, TypeError> {
        for ind_type in self.inductives.values() {
            if let Some(ctor) = ind_type.constructors.iter().find(|c| c.name == name) {
                let args_vec = match &**args {
                    Term::App(_, _) => self.collect_app_args(args),
                    Term::Var(_) => vec![args.clone()],
                    _ => vec![args.clone()],
                };

                // First check if we have enough arguments for both type parameters and constructor parameters
                let total_params = ind_type.params.len() + ctor.params.len();
                if args_vec.len() != total_params {
                    return Err(TypeError::WrongNumberOfArguments {
                        expected: total_params,
                        got: args_vec.len(),
                    });
                }

                // Split arguments into type parameters and constructor parameters
                let (type_args, ctor_args) = args_vec.split_at(ind_type.params.len());

                // Check type parameters
                for (arg, (_, param_ty)) in type_args.iter().zip(&ind_type.params) {
                    let arg_ty = (&**arg).type_check(self)?;
                    if !self.types_convertible(&arg_ty, param_ty) {
                        return Err(TypeError::TypeMismatch {
                            expected: param_ty.clone(),
                            got: arg_ty,
                        });
                    }
                }

                // Check constructor parameters
                for (arg, (_, param_ty)) in ctor_args.iter().zip(&ctor.params) {
                    let arg_ty = (&**arg).type_check(self)?;
                    // Substitute type parameters in the parameter type
                    let mut param_ty = param_ty.clone();
                    for (i, (name, _)) in ind_type.params.iter().enumerate() {
                        param_ty = param_ty.substitute(name, &type_args[i])?;
                    }
                    if !self.types_convertible(&arg_ty, &param_ty) {
                        return Err(TypeError::TypeMismatch {
                            expected: param_ty,
                            got: arg_ty,
                        });
                    }
                }

                // Build the return type with the correct type parameters
                let mut return_type = ctor.return_type.clone();
                for (i, (name, _)) in ind_type.params.iter().enumerate() {
                    return_type = return_type.substitute(name, &type_args[i])?;
                }
                return Ok(return_type);
            }
        }
        Err(TypeError::UnboundVariable(name.to_string()))
    }

    pub fn collect_app_args(&self, term: &Rc<Term>) -> Vec<Rc<Term>> {
        let mut args = Vec::new();
        let mut curr = term;
        while let Term::App(f, arg) = &**curr {
            args.push(arg.clone());
            curr = f;
        }
        args.reverse();
        args
    }

    pub fn type_check_match(
        &self,
        scrutinee: &Rc<Term>,
        branches: &[MatchBranch],
    ) -> Result<Rc<Type>, TypeError> {
        let scrut_ty = (&**scrutinee).type_check(self)?;
        let ind_name = self.extract_inductive_name(&scrut_ty)?;

        let ind_type = self.inductives.get(&ind_name).ok_or_else(|| {
            TypeError::ConstructorError(format!("Type {} is not an inductive type", ind_name))
        })?;

        let mut branch_ty = None;
        for branch in branches {
            let ctor = ind_type
                .constructors
                .iter()
                .find(|c| c.name == branch.pattern.constructor)
                .ok_or_else(|| TypeError::UnboundVariable(branch.pattern.constructor.clone()))?;

            let mut branch_ctx = self.clone();
            for (var, (_, ty)) in branch.pattern.bound_vars.iter().zip(&ctor.params) {
                branch_ctx.add_var(var.clone(), ty.clone());
            }

            let body_ty = (&*branch.body).type_check(&branch_ctx)?;

            if let Some(prev_ty) = &branch_ty {
                if !self.types_convertible(&body_ty, prev_ty) {
                    return Err(TypeError::TypeMismatch {
                        expected: prev_ty.clone(),
                        got: body_ty,
                    });
                }
            } else {
                branch_ty = Some(body_ty);
            }
        }

        branch_ty.ok_or_else(|| {
            TypeError::ConstructorError("Match expression has no branches".to_string())
        })
    }

    fn extract_inductive_name(&self, ty: &Rc<Type>) -> Result<String, TypeError> {
        match &**ty {
            Type::Named(name) => Ok(name.clone()),
            Type::App(base_ty, _) => self.extract_inductive_name(base_ty),
            _ => Err(TypeError::TypeMismatch {
                expected: Rc::new(Type::Named("inductive type".to_string())),
                got: ty.clone(),
            }),
        }
    }

    pub fn get_constructor(&self, name: &str) -> Option<Constructor> {
        self.inductives.values().find_map(|inductive| {
            inductive
                .constructors
                .iter()
                .find(|c| c.name == name)
                .cloned()
        })
    }

    pub fn get_inductive_type(&self, name: &str) -> Option<InductiveType> {
        self.inductives.get(name).cloned()
    }

    pub fn apply_type(&self, base_ty: &Rc<Type>, args: &[Rc<Term>]) -> Result<Rc<Type>, TypeError> {
        match &**base_ty {
            Type::Named(name) => {
                if let Some(ind_type) = self.inductives.get(name) {
                    if args.len() != ind_type.params.len() {
                        return Err(TypeError::WrongNumberOfArguments {
                            expected: ind_type.params.len(),
                            got: args.len(),
                        });
                    }

                    // Check that each argument has the correct type
                    for (arg, (_, param_ty)) in args.iter().zip(&ind_type.params) {
                        let arg_ty = (&**arg).type_check(self)?;
                        if !self.types_convertible(&arg_ty, param_ty) {
                            return Err(TypeError::TypeMismatch {
                                expected: param_ty.clone(),
                                got: arg_ty,
                            });
                        }
                    }

                    // Build the applied type
                    let mut result = base_ty.clone();
                    for arg in args {
                        result = Rc::new(Type::App(result, arg.clone()));
                    }
                    Ok(result)
                } else {
                    Err(TypeError::UnboundVariable(name.clone()))
                }
            }
            _ => Err(TypeError::TypeMismatch {
                expected: Rc::new(Type::Named("type constructor".to_string())),
                got: base_ty.clone(),
            }),
        }
    }
}
