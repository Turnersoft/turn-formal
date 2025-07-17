use std::sync::Arc;

use super::expressions::{MathExpression, TheoryExpression};
use super::interpretation::TypeViewOperator;
use super::objects::MathObject;
use super::relations::MathRelation;
use crate::subjects::math::theories::fields::Field;
use crate::subjects::math::theories::groups::definitions::Group;
use crate::subjects::math::theories::number_theory::definitions::Number;
use crate::subjects::math::theories::rings::Ring;
use crate::turn_render::Identifier;

impl MathObject {
    /// Returns a reference to the inner `Group`,
    /// or an error if the variant is not `MathObject::Group`.
    pub fn get_group(&self) -> Result<&Group, String> {
        if let MathObject::Group(g) = self {
            Ok(g)
        } else {
            Err("Type mismatch: Expected MathObject::Group".to_string())
        }
    }

    /// Returns a mutable reference to the inner `Group`,
    /// or an error if the variant is not `MathObject::Group`.
    pub fn get_mut_group(&mut self) -> Result<&mut Group, String> {
        if let MathObject::Group(g) = self {
            Ok(g)
        } else {
            Err("Type mismatch: Expected MathObject::Group".to_string())
        }
    }
}

impl MathExpression {
    // pub fn get_var(&self) -> Result<&Identifier, String> {
    //     if let MathExpression::Var(id) = self {
    //         Ok(id)
    //     } else {
    //         Err(format!(
    //             "Expected MathExpression::Var, but found {}",
    //             self.get_variant_name()
    //         ))
    //     }
    // }

    pub fn get_object(&self) -> Result<&MathObject, String> {
        if let MathExpression::Object(obj) = self {
            Ok(obj)
        } else {
            Err(format!(
                "Expected MathExpression::Object, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_expression(&self) -> Result<&TheoryExpression, String> {
        if let MathExpression::Expression(expr) = self {
            Ok(expr)
        } else {
            Err(format!(
                "Expected MathExpression::Expression, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_relation(&self) -> Result<&Arc<MathRelation>, String> {
        if let MathExpression::Relation(rel) = self {
            Ok(rel)
        } else {
            Err(format!(
                "Expected MathExpression::Relation, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_number(&self) -> Result<&Number, String> {
        if let MathExpression::Number(num) = self {
            Ok(num)
        } else {
            Err(format!(
                "Expected MathExpression::Number, but found {}",
                self.get_variant_name()
            ))
        }
    }

    // pub fn get_view_as(&self) -> Result<(&MathExpression, &TypeViewOperator), String> {
    //     if let MathExpression::ViewAs { expression, view } = self {
    //         Ok((&expression.data.unwrap(), &view.data))
    //     } else {
    //         Err(format!(
    //             "Expected MathExpression::ViewAs, but found {}",
    //             self.get_variant_name()
    //         ))
    //     }
    // }
}
