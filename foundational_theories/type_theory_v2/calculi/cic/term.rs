use super::context::Context;
use super::type_::Type;
use super::typing::TypeError;
use super::universe::Universe;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: String,
    pub params: Vec<(String, Rc<Type>)>,
    pub return_type: Rc<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub constructor: String,
    pub bound_vars: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchBranch {
    pub pattern: Pattern,
    pub body: Rc<Term>,
}

#[derive(Debug, Clone)]
pub enum Term {
    // Variables and basic terms
    Var(String),
    App(Rc<Term>, Rc<Term>),
    Lambda(String, Rc<Type>, Rc<Term>),
    Pi(String, Rc<Type>, Rc<Term>),
    Sort(Universe),
    Constructor(String, Rc<Term>),
    Match(Rc<Term>, Vec<MatchBranch>),
    // Primitive type constructors
    Number(i64),
    Bool(bool),
    Unit,
    // Product and sum type constructors
    Pair(Rc<Term>, Rc<Term>),
    Left(Rc<Term>, Rc<Type>), // Second argument is the right type of the sum
    Right(Rc<Term>, Rc<Type>), // Second argument is the left type of the sum
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Term::Var(x1), Term::Var(x2)) => x1 == x2,
            (Term::App(f1, a1), Term::App(f2, a2)) => f1 == f2 && a1 == a2,
            (Term::Lambda(x1, t1, b1), Term::Lambda(x2, t2, b2)) => {
                x1 == x2 && t1 == t2 && b1 == b2
            }
            (Term::Pi(x1, t1, b1), Term::Pi(x2, t2, b2)) => x1 == x2 && t1 == t2 && b1 == b2,
            (Term::Sort(u1), Term::Sort(u2)) => u1 == u2,
            (Term::Constructor(n1, a1), Term::Constructor(n2, a2)) => n1 == n2 && a1 == a2,
            (Term::Match(s1, b1), Term::Match(s2, b2)) => s1 == s2 && b1 == b2,
            (Term::Number(n1), Term::Number(n2)) => n1 == n2,
            (Term::Bool(b1), Term::Bool(b2)) => b1 == b2,
            (Term::Unit, Term::Unit) => true,
            (Term::Pair(t1, t2), Term::Pair(u1, u2)) => t1 == u1 && t2 == u2,
            (Term::Left(t1, ty1), Term::Left(t2, ty2)) => t1 == t2 && ty1 == ty2,
            (Term::Right(t1, ty1), Term::Right(t2, ty2)) => t1 == t2 && ty1 == ty2,
            _ => false,
        }
    }
}

impl Term {
    pub fn substitute(&self, var: &str, arg: &Rc<Term>) -> Result<Rc<Term>, TypeError> {
        match self {
            Term::Var(x) if x == var => Ok(arg.clone()),
            Term::App(f, a) => {
                let new_f = (&**f).substitute(var, arg)?;
                let new_a = (&**a).substitute(var, arg)?;
                Ok(Rc::new(Term::App(new_f, new_a)))
            }
            Term::Lambda(x, ty, body) if x != var => {
                let new_ty = ty.substitute(var, arg)?;
                let new_body = (&**body).substitute(var, arg)?;
                Ok(Rc::new(Term::Lambda(x.clone(), new_ty, new_body)))
            }
            Term::Pi(x, ty1, ty2) if x != var => {
                let new_ty1 = ty1.substitute(var, arg)?;
                let new_ty2 = ty2.substitute(var, arg)?;
                Ok(Rc::new(Term::Pi(x.clone(), new_ty1, new_ty2)))
            }
            _ => Ok(Rc::new(self.clone())),
        }
    }

    pub fn beta_reduce(&self) -> Rc<Term> {
        match self {
            Term::App(f, arg) => {
                let f_reduced = (&**f).beta_reduce();
                let arg_reduced = (&**arg).beta_reduce();
                match &*f_reduced {
                    Term::Lambda(x, _, body) => match (&**body).substitute(x, &arg_reduced) {
                        Ok(reduced) => reduced,
                        Err(_) => Rc::new(Term::App(f_reduced, arg_reduced)),
                    },
                    _ => Rc::new(Term::App(f_reduced, arg_reduced)),
                }
            }
            _ => Rc::new(self.clone()),
        }
    }

    pub fn contains_var(&self, var: &str) -> bool {
        match self {
            Term::Var(name) => name == var,
            Term::App(f, arg) => f.contains_var(var) || arg.contains_var(var),
            Term::Lambda(x, _, body) => x != var && body.contains_var(var),
            Term::Pi(x, param_type, return_type) => {
                x != var && (param_type.contains_var(var) || return_type.contains_var(var))
            }
            Term::Sort(_) => false,
            Term::Constructor(_, term) => term.contains_var(var),
            Term::Match(scrutinee, branches) => {
                scrutinee.contains_var(var)
                    || branches.iter().any(|branch| branch.body.contains_var(var))
            }
            Term::Number(_) | Term::Bool(_) | Term::Unit => false,
            Term::Pair(t1, t2) => t1.contains_var(var) || t2.contains_var(var),
            Term::Left(t, _) | Term::Right(t, _) => t.contains_var(var),
        }
    }

    pub fn substitute_with_ctx(
        &self,
        var: &str,
        term: &Rc<Term>,
        ctx: &Context,
    ) -> Result<Term, TypeError> {
        match self {
            Term::Var(name) => {
                if name == var {
                    Ok((**term).clone())
                } else {
                    Ok(self.clone())
                }
            }
            Term::App(f, arg) => {
                let new_f = f.substitute_with_ctx(var, term, ctx)?;
                let new_arg = arg.substitute_with_ctx(var, term, ctx)?;
                Ok(Term::App(Rc::new(new_f), Rc::new(new_arg)))
            }
            Term::Lambda(x, ty, body) => {
                let new_ty = (**ty).substitute_with_ctx(var, term, ctx)?;
                let new_body = if x == var {
                    (**body).clone()
                } else {
                    body.substitute_with_ctx(var, term, ctx)?
                };
                Ok(Term::Lambda(x.clone(), new_ty, Rc::new(new_body)))
            }
            Term::Pi(x, param_type, return_type) => {
                let new_param_type = (**param_type).substitute_with_ctx(var, term, ctx)?;
                let new_return_type = if x == var {
                    (**return_type).clone()
                } else {
                    return_type.substitute_with_ctx(var, term, ctx)?
                };
                Ok(Term::Pi(
                    x.clone(),
                    new_param_type,
                    Rc::new(new_return_type),
                ))
            }
            Term::Sort(u) => Ok(Term::Sort(u.clone())),
            Term::Constructor(name, t) => {
                let new_t = t.substitute_with_ctx(var, term, ctx)?;
                Ok(Term::Constructor(name.clone(), Rc::new(new_t)))
            }
            Term::Match(scrutinee, branches) => {
                let new_scrutinee = scrutinee.substitute_with_ctx(var, term, ctx)?;
                let new_branches = branches
                    .iter()
                    .map(|branch| {
                        let new_body = branch.body.substitute_with_ctx(var, term, ctx)?;
                        Ok(MatchBranch {
                            pattern: branch.pattern.clone(),
                            body: Rc::new(new_body),
                        })
                    })
                    .collect::<Result<Vec<_>, TypeError>>()?;
                Ok(Term::Match(Rc::new(new_scrutinee), new_branches))
            }
            Term::Number(n) => Ok(Term::Number(*n)),
            Term::Bool(b) => Ok(Term::Bool(*b)),
            Term::Unit => Ok(Term::Unit),
            Term::Pair(t1, t2) => {
                let new_t1 = t1.substitute_with_ctx(var, term, ctx)?;
                let new_t2 = t2.substitute_with_ctx(var, term, ctx)?;
                Ok(Term::Pair(Rc::new(new_t1), Rc::new(new_t2)))
            }
            Term::Left(t, ty) => {
                let new_t = t.substitute_with_ctx(var, term, ctx)?;
                let new_ty = (**ty).substitute_with_ctx(var, term, ctx)?;
                Ok(Term::Left(Rc::new(new_t), new_ty))
            }
            Term::Right(t, ty) => {
                let new_t = t.substitute_with_ctx(var, term, ctx)?;
                let new_ty = (**ty).substitute_with_ctx(var, term, ctx)?;
                Ok(Term::Right(Rc::new(new_t), new_ty))
            }
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(name) => write!(f, "{}", name),
            Term::App(func, arg) => write!(f, "({} {})", func, arg),
            Term::Lambda(x, ty, body) => write!(f, "λ{}: {}. {}", x, ty, body),
            Term::Pi(x, ty, body) => write!(f, "Π{}: {}. {}", x, ty, body),
            Term::Sort(universe) => write!(f, "{}", universe),
            Term::Constructor(name, args) => write!(f, "{}({})", name, args),
            Term::Match(scrutinee, branches) => {
                write!(f, "match {} with {{ ", scrutinee)?;
                for branch in branches {
                    write!(f, "{} => {}; ", branch.pattern, branch.body)?;
                }
                write!(f, "}}")
            }
            Term::Number(n) => write!(f, "{}", n),
            Term::Bool(b) => write!(f, "{}", b),
            Term::Unit => write!(f, "()"),
            Term::Pair(t1, t2) => write!(f, "({}, {})", t1, t2),
            Term::Left(t, ty) => write!(f, "inl({}: {})", t, ty),
            Term::Right(t, ty) => write!(f, "inr({}: {})", t, ty),
        }
    }
}

impl fmt::Display for MatchBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.pattern, self.body)
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.constructor)?;
        if !self.bound_vars.is_empty() {
            write!(f, " {}", self.bound_vars.join(" "))?;
        }
        Ok(())
    }
}
