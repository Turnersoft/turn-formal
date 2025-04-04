use super::term::{Constructor, MatchBranch, Term};
use super::type_::Type;
use super::typing::TypeError;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Reduction {
    Beta,  // (λx.t) u → t[x:=u]
    Delta, // Constants and definitions
    Iota,  // Pattern matching
    Zeta,  // Let bindings
}

impl Term {
    pub fn reduce(&self) -> Result<Rc<Term>, TypeError> {
        match self {
            Term::App(f, arg) => {
                let f = (&**f).reduce()?;
                match &*f {
                    Term::Lambda(var, _, body) => {
                        // Beta reduction
                        (&**body).substitute(var, arg)
                    }
                    _ => Ok(Rc::new(Term::App(f, arg.clone()))),
                }
            }

            Term::Match(scrutinee, branches) => {
                let scrutinee = (&**scrutinee).reduce()?;
                match &*scrutinee {
                    Term::Constructor(name, args) => {
                        // Iota reduction
                        self.reduce_match(name, args, branches)
                    }
                    _ => Ok(Rc::new(Term::Match(scrutinee, branches.clone()))),
                }
            }

            Term::Lambda(x, ty, body) => {
                let body = (&**body).reduce()?;
                Ok(Rc::new(Term::Lambda(x.clone(), ty.clone(), body)))
            }

            Term::Pi(x, ty1, ty2) => {
                let ty1 = ty1.clone();
                let ty2 = ty2.clone();
                Ok(Rc::new(Term::Pi(x.clone(), ty1, ty2)))
            }

            _ => Ok(Rc::new(self.clone())),
        }
    }

    fn reduce_match(
        &self,
        ctor: &str,
        args: &Rc<Term>,
        branches: &[MatchBranch],
    ) -> Result<Rc<Term>, TypeError> {
        // Find matching branch
        for branch in branches {
            if branch.pattern.constructor == ctor {
                // Substitute bound variables with arguments
                let mut result = branch.body.clone();
                let args_vec = match &**args {
                    Term::App(_, _) => collect_app_args(args),
                    _ => vec![args.clone()],
                };
                for (var, arg) in branch.pattern.bound_vars.iter().zip(args_vec.iter()) {
                    result = (&*result).substitute(var, arg)?;
                }
                return Ok(result);
            }
        }
        // No matching branch found, return unchanged
        Ok(Rc::new(self.clone()))
    }
}

impl Type {
    pub fn reduce_type(&self) -> Type {
        match self {
            Type::Named(name) => Type::Named(name.clone()),
            Type::Pi(x, ty1, ty2) => {
                let ty1 = Rc::new((&**ty1).reduce_type());
                let ty2 = Rc::new((&**ty2).reduce_type());
                Type::Pi(x.clone(), ty1, ty2)
            }
            // Primitive types and type constructors remain unchanged
            Type::Prop => Type::Prop,
            Type::Type(l) => Type::Type(l.clone()),
            Type::Number => Type::Number,
            Type::Bool => Type::Bool,
            Type::Bottom => Type::Bottom,
            Type::Top => Type::Top,
            Type::Unit => Type::Unit,
            Type::Sum(ty1, ty2) => {
                let ty1 = Rc::new((&**ty1).reduce_type());
                let ty2 = Rc::new((&**ty2).reduce_type());
                Type::Sum(ty1, ty2)
            }
            Type::Product(ty1, ty2) => {
                let ty1 = Rc::new((&**ty1).reduce_type());
                let ty2 = Rc::new((&**ty2).reduce_type());
                Type::Product(ty1, ty2)
            }
            Type::Function(ty1, ty2) => {
                let ty1 = Rc::new((&**ty1).reduce_type());
                let ty2 = Rc::new((&**ty2).reduce_type());
                Type::Function(ty1, ty2)
            }
            Type::App(ty, term) => {
                let ty = Rc::new((&**ty).reduce_type());
                let term = term.reduce().unwrap_or_else(|_| term.clone());
                Type::App(ty, term)
            }
        }
    }
}

fn collect_app_args(term: &Rc<Term>) -> Vec<Rc<Term>> {
    let mut args = Vec::new();
    let mut curr = term;
    while let Term::App(f, arg) = &**curr {
        args.push(arg.clone());
        curr = f;
    }
    args.reverse();
    args
}
