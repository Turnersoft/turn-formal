use super::context::Context;
use super::term::Term;
use super::typing::{TypeChecker, TypeError, TypeResult};
use super::universe::{Level, Universe};
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Universe types
    Prop,
    Type(Level),
    // Primitive types
    Number,
    Bool,
    Bottom,
    Top,
    Unit,
    // Type constructors
    Sum(Rc<Type>, Rc<Type>),
    Product(Rc<Type>, Rc<Type>),
    Function(Rc<Type>, Rc<Type>), // Simple function type without dependent types
    // Dependent types
    Pi(String, Rc<Type>, Rc<Type>),
    // Named types (for definitions)
    Named(String),
    // Applied types (e.g., Vector[A, n])
    App(Rc<Type>, Rc<Term>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Prop => write!(f, "Prop"),
            Type::Type(level) => write!(f, "Type_{}", level),
            Type::Number => write!(f, "Number"),
            Type::Bool => write!(f, "Bool"),
            Type::Bottom => write!(f, "⊥"),
            Type::Top => write!(f, "⊤"),
            Type::Unit => write!(f, "()"),
            Type::Sum(ty1, ty2) => write!(f, "{} + {}", ty1, ty2),
            Type::Product(ty1, ty2) => write!(f, "{} × {}", ty1, ty2),
            Type::Function(ty1, ty2) => write!(f, "{} → {}", ty1, ty2),
            Type::Pi(x, ty1, ty2) => write!(f, "Π{}: {}. {}", x, ty1, ty2),
            Type::Named(name) => write!(f, "{}", name),
            Type::App(ty, term) => write!(f, "{}", ty),
        }
    }
}

impl TypeChecker for Type {
    fn type_check(&self, ctx: &Context) -> TypeResult {
        match self {
            Type::Named(name) => {
                // Look up the type and check it
                if let Some(ty) = ctx.lookup_type(name) {
                    Ok(ty)
                } else {
                    Err(TypeError::UnboundVariable(name.clone()))
                }
            }
            Type::Prop => Ok(Rc::new(Type::Type(Level::new(1)))),
            Type::Type(level) => Ok(Rc::new(Type::Type(Level::new(level.0 + 1)))),
            // Primitive types are in Type₀
            Type::Number | Type::Bool | Type::Bottom | Type::Top | Type::Unit => {
                Ok(Rc::new(Type::Type(Level::new(0))))
            }
            Type::Sum(ty1, ty2) | Type::Product(ty1, ty2) | Type::Function(ty1, ty2) => {
                let ty1_type = ty1.type_check(ctx)?;
                let ty2_type = ty2.type_check(ctx)?;
                match (self) {
                    Type::Product(_, _) => {
                        // For products, if both types are propositions, the result is also a proposition
                        if let (Type::Named(n1), Type::Named(n2)) = (&**ty1, &**ty2) {
                            if let (Some(t1), Some(t2)) = (ctx.lookup_type(n1), ctx.lookup_type(n2))
                            {
                                if matches!(&*t1, Type::Prop) && matches!(&*t2, Type::Prop) {
                                    return Ok(Rc::new(Type::Prop));
                                }
                            }
                        }
                    }
                    _ => (),
                }
                match (&*ty1_type, &*ty2_type) {
                    (Type::Type(l1), Type::Type(l2)) => Ok(Rc::new(Type::Type((*l1).max(*l2)))),
                    _ => Ok(Rc::new(Type::Type(Level::new(1)))), // Default to Type₁ for non-Type cases
                }
            }
            Type::Pi(param_name, param_ty, return_ty) => {
                let param_type = param_ty.type_check(ctx)?;
                let mut inner_ctx = ctx.clone();
                inner_ctx.add_var(param_name.clone(), param_type.clone());
                let return_type = return_ty.type_check(&inner_ctx)?;
                match (&*param_type, &*return_type) {
                    (Type::Type(l1), Type::Type(l2)) => Ok(Rc::new(Type::Type((*l1).max(*l2)))),
                    _ => Ok(Rc::new(Type::Type(Level::new(1)))), // Default to Type₁ for non-Type cases
                }
            }
            Type::App(ty, term) => {
                let ty_type = ty.type_check(ctx)?;
                let term_type = term.type_check(ctx)?;
                match (&*ty_type, &*term_type) {
                    (Type::Type(l1), Type::Type(l2)) => Ok(Rc::new(Type::Type((*l1).max(*l2)))),
                    _ => Ok(Rc::new(Type::Type(Level::new(1)))), // Default to Type₁ for non-Type cases
                }
            }
        }
    }
}

impl Type {
    pub fn substitute_with_ctx(
        &self,
        var: &str,
        term: &Rc<Term>,
        ctx: &Context,
    ) -> Result<Rc<Type>, TypeError> {
        match self {
            Type::Named(name) => {
                if name == var {
                    // If we're substituting for a type variable, we need to get the type of the term
                    match &**term {
                        Term::Sort(Universe::Type(level)) => Ok(Rc::new(Type::Type(level.clone()))),
                        _ => term.type_check(ctx),
                    }
                } else {
                    // For named types, we substitute in their definition if it exists
                    if let Some(def) = ctx.lookup_type(name) {
                        def.substitute_with_ctx(var, term, ctx)
                    } else {
                        Ok(Rc::new(self.clone()))
                    }
                }
            }
            Type::Prop => Ok(Rc::new(Type::Prop)),
            Type::Type(l) => Ok(Rc::new(Type::Type(l.clone()))),
            // Primitive types remain unchanged under substitution
            Type::Number => Ok(Rc::new(Type::Number)),
            Type::Bool => Ok(Rc::new(Type::Bool)),
            Type::Bottom => Ok(Rc::new(Type::Bottom)),
            Type::Top => Ok(Rc::new(Type::Top)),
            Type::Unit => Ok(Rc::new(Type::Unit)),
            // Type constructors substitute recursively
            Type::Sum(ty1, ty2) => {
                let new_ty1 = ty1.substitute_with_ctx(var, term, ctx)?;
                let new_ty2 = ty2.substitute_with_ctx(var, term, ctx)?;
                Ok(Rc::new(Type::Sum(new_ty1, new_ty2)))
            }
            Type::Product(ty1, ty2) => {
                let new_ty1 = ty1.substitute_with_ctx(var, term, ctx)?;
                let new_ty2 = ty2.substitute_with_ctx(var, term, ctx)?;
                Ok(Rc::new(Type::Product(new_ty1, new_ty2)))
            }
            Type::Function(ty1, ty2) => {
                let new_ty1 = ty1.substitute_with_ctx(var, term, ctx)?;
                let new_ty2 = ty2.substitute_with_ctx(var, term, ctx)?;
                Ok(Rc::new(Type::Function(new_ty1, new_ty2)))
            }
            Type::Pi(x, ty1, ty2) => {
                let new_ty1 = ty1.substitute_with_ctx(var, term, ctx)?;
                let new_ty2 = if x == var {
                    ty2.clone()
                } else {
                    ty2.substitute_with_ctx(var, term, ctx)?
                };
                Ok(Rc::new(Type::Pi(x.clone(), new_ty1, new_ty2)))
            }
            Type::App(ty, t) => {
                let new_ty = ty.substitute_with_ctx(var, term, ctx)?;
                let new_term = if (**t).contains_var(var) {
                    Rc::new(t.substitute_with_ctx(var, term, ctx)?)
                } else {
                    t.clone()
                };
                Ok(Rc::new(Type::App(new_ty, new_term)))
            }
        }
    }

    pub fn substitute(&self, var: &str, term: &Rc<Term>) -> Result<Rc<Type>, TypeError> {
        // Create a context with just the necessary bindings for substitution
        let mut ctx = Context::new();
        ctx.add_var("Type0".to_string(), Rc::new(Type::Type(Level::new(1))));
        ctx.add_var("Prop".to_string(), Rc::new(Type::Type(Level::new(1))));
        ctx.add_var("A".to_string(), Rc::new(Type::Type(Level::new(0))));
        self.substitute_with_ctx(var, term, &ctx)
    }

    pub fn reduce_with_ctx(&self, ctx: &Context) -> Result<Rc<Type>, TypeError> {
        match self {
            Type::Named(name) => {
                // Delta-reduction: expand definitions
                if let Some(def) = ctx.lookup_type(name) {
                    Ok(def)
                } else {
                    Err(TypeError::UnboundVariable(name.clone()))
                }
            }
            // Other cases remain unchanged
            Type::Pi(x, ty1, ty2) => {
                let ty1 = ty1.reduce_with_ctx(ctx)?;
                let ty2 = ty2.reduce_with_ctx(ctx)?;
                Ok(Rc::new(Type::Pi(x.clone(), ty1, ty2)))
            }
            Type::Sum(ty1, ty2) => {
                let ty1 = ty1.reduce_with_ctx(ctx)?;
                let ty2 = ty2.reduce_with_ctx(ctx)?;
                Ok(Rc::new(Type::Sum(ty1, ty2)))
            }
            Type::Product(ty1, ty2) => {
                let ty1 = ty1.reduce_with_ctx(ctx)?;
                let ty2 = ty2.reduce_with_ctx(ctx)?;
                Ok(Rc::new(Type::Product(ty1, ty2)))
            }
            Type::Function(ty1, ty2) => {
                let ty1 = ty1.reduce_with_ctx(ctx)?;
                let ty2 = ty2.reduce_with_ctx(ctx)?;
                Ok(Rc::new(Type::Function(ty1, ty2)))
            }
            // Primitive types remain unchanged
            _ => Ok(Rc::new(self.clone())),
        }
    }

    pub fn is_convertible_with_ctx(&self, other: &Type, ctx: &Context) -> Result<bool, TypeError> {
        // First reduce both types
        let self_reduced = self.reduce_with_ctx(ctx)?;
        let other_reduced = other.reduce_with_ctx(ctx)?;

        match (&*self_reduced, &*other_reduced) {
            (Type::Named(n1), Type::Named(n2)) if n1 == n2 => Ok(true),
            (Type::Prop, Type::Prop) => Ok(true),
            (Type::Type(l1), Type::Type(l2)) => Ok(l1 == l2),
            // Primitive types are equal to themselves
            (Type::Number, Type::Number)
            | (Type::Bool, Type::Bool)
            | (Type::Bottom, Type::Bottom)
            | (Type::Top, Type::Top)
            | (Type::Unit, Type::Unit) => Ok(true),
            // Type constructors are equal if their components are convertible
            (Type::Sum(ty1, ty2), Type::Sum(other_ty1, other_ty2))
            | (Type::Product(ty1, ty2), Type::Product(other_ty1, other_ty2))
            | (Type::Function(ty1, ty2), Type::Function(other_ty1, other_ty2)) => Ok(ty1
                .is_convertible_with_ctx(other_ty1, ctx)?
                && ty2.is_convertible_with_ctx(other_ty2, ctx)?),
            (Type::Pi(x1, ty11, ty12), Type::Pi(x2, ty21, ty22)) => {
                if !ty11.is_convertible_with_ctx(ty21, ctx)? {
                    return Ok(false);
                }
                // Rename x2 to x1 in ty22 to compare bodies
                let ty22_renamed =
                    ty22.substitute_with_ctx(x2, &Rc::new(Term::Var(x1.clone())), ctx)?;
                ty12.is_convertible_with_ctx(&ty22_renamed, ctx)
            }
            _ => Ok(false),
        }
    }

    pub fn is_convertible(&self, other: &Type) -> Result<bool, TypeError> {
        // Create a context with just the necessary bindings for conversion
        let mut ctx = Context::new();
        ctx.add_var("Type0".to_string(), Rc::new(Type::Type(Level::new(1))));
        ctx.add_var("Prop".to_string(), Rc::new(Type::Type(Level::new(1))));
        ctx.add_var("A".to_string(), Rc::new(Type::Type(Level::new(0))));
        self.is_convertible_with_ctx(other, &ctx)
    }

    pub fn contains_var(&self, var: &str) -> bool {
        match self {
            Type::Named(name) => name == var,
            Type::Prop
            | Type::Type(_)
            | Type::Number
            | Type::Bool
            | Type::Bottom
            | Type::Top
            | Type::Unit => false,
            Type::Sum(ty1, ty2) | Type::Product(ty1, ty2) | Type::Function(ty1, ty2) => {
                ty1.contains_var(var) || ty2.contains_var(var)
            }
            Type::Pi(x, ty1, ty2) => {
                if x == var {
                    ty1.contains_var(var)
                } else {
                    ty1.contains_var(var) || ty2.contains_var(var)
                }
            }
            Type::App(ty, term) => ty.contains_var(var),
        }
    }
}
