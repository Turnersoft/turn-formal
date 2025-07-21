use std::{collections::HashSet, sync::Arc};

use crate::turn_render::Identifier;

use super::{
    expressions::{MathExpression, TheoryExpression},
    extract::Parametrizable,
    interpretation::TypeViewOperator,
    location::Located,
    objects::MathObject,
    relations::MathRelation,
};

// Import Group types for child node implementations
use crate::subjects::math::theories::groups::definitions::{
    Group, GroupElement, GroupExpression, GroupHomomorphism,
};

/// A trait for collecting all identifiers (variable dependencies) that occur within a type.
///
/// This trait is used to analyze the dependency structure of mathematical expressions.
/// It recursively traverses through all sub-components and extracts any `Identifier`
/// instances that represent variables or named entities.
///
/// # Purpose
///
/// The main use case is to understand variable dependencies within mathematical expressions:
/// - Finding all free variables in an expression
/// - Analyzing parameter dependencies in expressions
/// - Understanding variable scope in mathematical contexts
///
/// # Examples
///
/// ```rust
/// use crate::subjects::math::formalism::collect_identifier::CollectIdentifier;
/// use crate::subjects::math::formalism::expressions::MathExpression;
/// use crate::turn_render::Identifier;
///
/// // Create an expression with variables
/// let x_id = Identifier::new_simple("x".to_string());
///
/// // A math expression with a variable
/// let math_expr = MathExpression::ViewAs {
///     expression: Located::new_variable(x_id.clone()),
///     view: Located::new_concrete(TypeViewOperator::AsCyclicGroup),
/// };
///
/// // Collect all identifiers
/// let identifiers = math_expr.collect_identifier();
/// // identifiers now contains: {x}
/// ```
pub trait CollectIdentifier {
    /// Collect all identifiers that occur within this type.
    ///
    /// Returns a `HashSet<Identifier>` containing all unique identifiers
    /// found by recursively traversing through the structure.
    fn collect_identifier(&self) -> HashSet<Identifier>;
}

impl<T: CollectIdentifier> CollectIdentifier for Arc<T> {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        self.as_ref().collect_identifier()
    }
}

impl<T: CollectIdentifier> CollectIdentifier for Located<T> {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        self.data.collect_identifier()
    }
}

impl<T: CollectIdentifier> CollectIdentifier for Parametrizable<T> {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        match self {
            Parametrizable::Concrete(value) => value.collect_identifier(),
            Parametrizable::Variable(id) => {
                let mut identifiers = HashSet::new();
                identifiers.insert(id.clone());
                identifiers
            }
        }
    }
}

impl<T: CollectIdentifier> CollectIdentifier for Vec<T> {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        let mut identifiers = HashSet::new();
        for item in self {
            identifiers.extend(item.collect_identifier());
        }
        identifiers
    }
}

impl<T: CollectIdentifier> CollectIdentifier for Option<T> {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        match self {
            Some(value) => value.collect_identifier(),
            None => HashSet::new(),
        }
    }
}

impl CollectIdentifier for MathExpression {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        match self {
            MathExpression::Object(obj) => obj.collect_identifier(),
            MathExpression::Expression(theory_expr) => theory_expr.collect_identifier(),
            MathExpression::Relation(rel) => rel.collect_identifier(),
            MathExpression::Number(_) => HashSet::new(),
            MathExpression::ViewAs { expression, view } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(expression.collect_identifier());
                identifiers.extend(view.collect_identifier());
                identifiers
            }
        }
    }
}

impl CollectIdentifier for TheoryExpression {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        match self {
            TheoryExpression::Group(group_expr) => group_expr.collect_identifier(),
            TheoryExpression::Ring(_ring_expr) => {
                // TODO: Implement for RingExpression when available
                HashSet::new()
            }
            TheoryExpression::Field(_field_expr) => {
                // TODO: Implement for FieldExpression when available
                HashSet::new()
            }
        }
    }
}

impl CollectIdentifier for MathRelation {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        match self {
            MathRelation::And(relations) => relations.collect_identifier(),
            MathRelation::Or(relations) => relations.collect_identifier(),
            MathRelation::Not(relation) => relation.collect_identifier(),
            MathRelation::Implies(left, right) => {
                let mut identifiers = HashSet::new();
                identifiers.extend(left.collect_identifier());
                identifiers.extend(right.collect_identifier());
                identifiers
            }
            MathRelation::Equivalent(left, right) => {
                let mut identifiers = HashSet::new();
                identifiers.extend(left.collect_identifier());
                identifiers.extend(right.collect_identifier());
                identifiers
            }
            MathRelation::Equal { left, right } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(left.collect_identifier());
                identifiers.extend(right.collect_identifier());
                identifiers
            }
            MathRelation::True | MathRelation::False => HashSet::new(),
            // For theory-specific relations, we don't traverse into them
            // since they're not part of the core MathExpression tree
            MathRelation::GroupTheory(g) => g.collect_identifier(),
            MathRelation::NumberTheory(_) => todo!(),
            MathRelation::SetTheory(_) => todo!(),
            MathRelation::RingTheory(_) => todo!(),
            MathRelation::TopologyTheory(_) => todo!(),
            MathRelation::CategoryTheory(_) => todo!(),
            MathRelation::ProbabilityTheory(_) => todo!(),
        }
    }
}

impl CollectIdentifier for MathObject {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        match self {
            MathObject::Group(group) => group.collect_identifier(),
            // For other math objects, we don't traverse into them
            // since they're not part of the core expression tree
            MathObject::Ring(_) => HashSet::new(),
            MathObject::Field(_) => HashSet::new(),
            MathObject::Module(_) => HashSet::new(),
            MathObject::Algebra(_) => HashSet::new(),
            MathObject::TopologicalSpace(_) => HashSet::new(),
            MathObject::VectorSpace(_) => HashSet::new(),
            MathObject::Set(_) => HashSet::new(),
            MathObject::Function(_) => HashSet::new(),
        }
    }
}

impl CollectIdentifier for TypeViewOperator {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        match self {
            TypeViewOperator::AsGroupElement { group } => group.collect_identifier(),
            TypeViewOperator::AsGroup { operation } => operation
                .as_ref()
                .map_or(HashSet::new(), |op| op.collect_identifier()),
            TypeViewOperator::AsHomomorphism { source, target } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(source.collect_identifier());
                identifiers.extend(target.collect_identifier());
                identifiers
            }
            TypeViewOperator::AsFunction { domain } => domain
                .as_ref()
                .map_or(HashSet::new(), |dom| dom.collect_identifier()),
            TypeViewOperator::Custom { parameters, .. } => parameters.collect_identifier(),
            // For other view operators, we don't need to traverse deeper
            TypeViewOperator::AsRingElement { .. }
            | TypeViewOperator::AsFieldElement { .. }
            | TypeViewOperator::AsRing { .. }
            | TypeViewOperator::AsTopologicalSpace { .. }
            | TypeViewOperator::AsCyclicGroup
            | TypeViewOperator::AsPoint
            | TypeViewOperator::AsLinearTransformation => HashSet::new(),
        }
    }
}

// Implementations for primitive types used in Located<T>
impl CollectIdentifier for i32 {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        HashSet::new()
    }
}

impl CollectIdentifier for u32 {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        HashSet::new()
    }
}

impl CollectIdentifier for usize {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        HashSet::new()
    }
}

impl CollectIdentifier for String {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        HashSet::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::theories::groups::definitions::GroupExpression;
    use crate::turn_render::Identifier;

    #[test]
    fn test_collect_identifier_from_group_expression() {
        // Create some identifiers
        let x_id = Identifier::new_simple("x".to_string());
        let y_id = Identifier::new_simple("y".to_string());
        let g_id = Identifier::new_simple("G".to_string());

        // Create a group expression with variables
        let group_expr = GroupExpression::Operation {
            group: Located::new_variable(g_id.clone()),
            left: Located::new_variable(x_id.clone()),
            right: Located::new_variable(y_id.clone()),
        };

        // Collect identifiers
        let identifiers = group_expr.collect_identifier();

        // Check that all identifiers are collected
        assert!(identifiers.contains(&x_id));
        assert!(identifiers.contains(&y_id));
        assert!(identifiers.contains(&g_id));
        assert_eq!(identifiers.len(), 3);
    }

    #[test]
    fn test_collect_identifier_from_math_expression() {
        // Create identifiers
        let x_id = Identifier::new_simple("x".to_string());

        // Create a math expression with a variable
        let math_expr = MathExpression::ViewAs {
            expression: Located::new_variable(x_id.clone()),
            view: Located::new_concrete(TypeViewOperator::AsCyclicGroup),
        };

        // Collect identifiers
        let identifiers = math_expr.collect_identifier();

        // Check that the identifier is collected
        assert!(identifiers.contains(&x_id));
        assert_eq!(identifiers.len(), 1);
    }

    #[test]
    fn test_collect_identifier_from_nested_expression() {
        // Create identifiers
        let x_id = Identifier::new_simple("x".to_string());
        let y_id = Identifier::new_simple("y".to_string());
        let g_id = Identifier::new_simple("G".to_string());

        // Create a nested math expression: x = y in group G
        let group_expr = TheoryExpression::Group(GroupExpression::Operation {
            group: Located::new_variable(g_id.clone()),
            left: Located::new_variable(x_id.clone()),
            right: Located::new_variable(y_id.clone()),
        });

        let math_expr = MathExpression::Expression(group_expr);

        // Collect identifiers
        let identifiers = math_expr.collect_identifier();

        // Check that all identifiers are collected
        assert!(identifiers.contains(&x_id));
        assert!(identifiers.contains(&y_id));
        assert!(identifiers.contains(&g_id));
        assert_eq!(identifiers.len(), 3);
    }
}
