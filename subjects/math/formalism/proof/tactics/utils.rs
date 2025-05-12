use crate::subjects::math::formalism::expressions::{Identifier, MathExpression};

/// Convert an Identifier to a human-readable string
pub fn name_to_string(id: &Identifier) -> String {
    match id {
        Identifier::Name(name, _) => name.clone(),
        // No Generated or Indexed variants, use a match-all:
        _ => format!("{:?}", id),
    }
}

/// Generate a short summary of an expression for human readability
pub fn expression_summary(expr: &MathExpression) -> String {
    // This is a simplified version for display purposes
    match expr {
        MathExpression::Number(n) => format!("{:?}", n),
        MathExpression::Var(id) => name_to_string(id),
        MathExpression::Object(name) => format!("{:?}", name),
        MathExpression::Relation(_) => "[Relation]".to_string(),
        MathExpression::Expression(_) => "[Theory Expression]".to_string(),
        MathExpression::ViewAs { expression, view } => {
            format!("[{} as {:?}]", expression_summary(expression), view)
        }
    }
}

/// Create a simple expression from a string (for testing/prototyping)
pub fn create_expr(s: &str) -> MathExpression {
    // Since we can't rely on Number being FromStr, just create a variable
    MathExpression::Var(Identifier::Name(s.to_string(), 0))
}
