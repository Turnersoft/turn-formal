// Example theorem implementation from Turn-Formal
// Prove that in a group, inverses are unique

use std::collections::HashMap;
use turn_formal::math::formalism::{
    core::{MathObjectType, Theorem},
    expressions::{Identifier, MathExpression},
    proof::{ProofBranch, Tactic, TheoremBuilder},
    relations::MathRelation,
};

/// Prove the theorem that in a group, inverses are unique
pub fn prove_inverse_uniqueness() -> Theorem {
    // Create a group structure for our proof
    let group = create_abstract_group();

    // Create element variables
    let g_var = create_element_variable(&group, "g", 1);
    let h1_var = create_element_variable(&group, "h1", 2);
    let h2_var = create_element_variable(&group, "h2", 3);
    let e_var = GroupExpression::identity(group.clone());

    // Create relations for our proof
    let relation1 = group_operation_equals(&group, &g_var, &h1_var, &e_var);
    let relation2 = group_operation_equals(&group, &g_var, &h2_var, &e_var);

    // Create the theorem statement: if g*h1 = e and g*h2 = e, then h1 = h2
    let theorem_statement = MathRelation::Implies(
        Box::new(MathRelation::And(vec![
            relation1.clone(),
            relation2.clone(),
        ])),
        Box::new(MathRelation::equal(
            h1_var.to_math_expression(),
            h2_var.to_math_expression(),
        )),
    );

    // Build the proof
    let builder = TheoremBuilder::new("Group Inverse Uniqueness", theorem_statement, vec![]);

    // Initial branch
    let p0 = builder.initial_branch();

    // Add proof steps using syntax tree-based tactics

    // 1. Introduce the assumptions
    let p1 = p0.tactics_intro_expr("assumptions", MathExpression::Var(Identifier::E(51)), 1);

    // 2. Multiply the first equation h1*(g*h1 = e) to get h1*g*h1 = h1*e
    let h1_times_g_h1 = GroupExpression::operation(
        group.clone(),
        h1_var.clone(),
        GroupExpression::operation(group.clone(), g_var.clone(), h1_var.clone()),
    );
    let h1_times_e = GroupExpression::operation(group.clone(), h1_var.clone(), e_var.clone());

    // Extract left and right sides of the equation directly
    let g_h1_expr = GroupExpression::operation(group.clone(), g_var.clone(), h1_var.clone())
        .to_math_expression();
    let e_expr = e_var.to_math_expression();

    let p2 = p1.tactics_subs_expr(
        g_h1_expr,
        e_expr.clone(), // Clone to avoid move
        None,
        2,
    );

    // 3. Apply associativity: (h1*g)*h1 = h1*e
    let mut associativity_instantiation = HashMap::new();
    associativity_instantiation.insert("x".to_string(), h1_var.to_math_expression());
    associativity_instantiation.insert("y".to_string(), g_var.to_math_expression());
    associativity_instantiation.insert("z".to_string(), h1_var.to_math_expression());

    let p3 = p2.tactics_theorem_app_expr(
        "group_axiom_associativity",
        associativity_instantiation,
        None,
    );

    // 4. Now use the second assumption to substitute g*h2 = e in h1*g*h2
    let h1_times_g_h2 = GroupExpression::operation(
        group.clone(),
        h1_var.clone(),
        GroupExpression::operation(group.clone(), g_var.clone(), h2_var.clone()),
    );

    // Extract left and right sides of the second equation directly
    let g_h2_expr = GroupExpression::operation(group.clone(), g_var.clone(), h2_var.clone())
        .to_math_expression();

    let p4 = p3.tactics_subs_expr(
        g_h2_expr,
        e_expr.clone(), // Clone to avoid move
        None,
        3,
    );

    // 5. Apply identity property: h1*e = h1
    let mut identity_instantiation = HashMap::new();
    identity_instantiation.insert("x".to_string(), h1_var.to_math_expression());

    let p5 = p4.tactics_theorem_app_expr("group_axiom_identity", identity_instantiation, None);

    // 6. By similar steps with the second equation, we get h2 = h1
    let p6 = p5
        .tactics_subs_expr(
            h1_var.to_math_expression(),
            h2_var.to_math_expression(),
            None,
            4,
        )
        .should_complete();

    // Build the theorem
    builder.build()
}
