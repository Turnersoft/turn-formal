/// Helper functions for extracting subgoals from tactics that create multiple sub-goals
/// These provide ergonomic ways to bind sub-goals to named variables
use super::{ProofGoal, ProofNode};

/// Consume `Vec<ProofNode>` and return exactly 2 nodes
/// Panics if the vector doesn't have exactly 2 elements
pub fn take2(mut v: Vec<ProofNode>) -> (ProofNode, ProofNode) {
    if v.len() != 2 {
        panic!("Expected exactly 2 nodes, got {}", v.len());
    }
    let b = v.pop().unwrap();
    let a = v.pop().unwrap();
    (a, b)
}

/// Consume `Vec<ProofNode>` and return exactly 3 nodes
/// Panics if the vector doesn't have exactly 3 elements
pub fn take3(mut v: Vec<ProofNode>) -> (ProofNode, ProofNode, ProofNode) {
    if v.len() != 3 {
        panic!("Expected exactly 3 nodes, got {}", v.len());
    }
    let c = v.pop().unwrap();
    let b = v.pop().unwrap();
    let a = v.pop().unwrap();
    (a, b, c)
}

/// Split into the first N and the rest
/// Returns (first_n, rest_as_vector)
pub fn takeN(mut v: Vec<ProofNode>, n: usize) -> (Vec<ProofNode>, Vec<ProofNode>) {
    if v.len() < n {
        panic!("Expected at least {} nodes, got {}", n, v.len());
    }
    let rest = v.split_off(n);
    (v, rest)
}

/// Extract first node and leave the rest
/// Returns (first, rest_as_vector)
pub fn take1_rest(mut v: Vec<ProofNode>) -> (ProofNode, Vec<ProofNode>) {
    if v.is_empty() {
        panic!("Expected at least 1 node, got 0");
    }
    let first = v.remove(0);
    (first, v)
}

/// Extract first two nodes and leave the rest
/// Returns (first, second, rest_as_vector)
pub fn take2_rest(mut v: Vec<ProofNode>) -> (ProofNode, ProofNode, Vec<ProofNode>) {
    if v.len() < 2 {
        panic!("Expected at least 2 nodes, got {}", v.len());
    }
    let second = v.remove(1);
    let first = v.remove(0);
    (first, second, v)
}

/// Convenient macro for pattern matching on sub-goals
#[macro_export]
macro_rules! extract_subgoals {
    ($outcome:expr, 2) => {{
        let sub_nodes = $outcome.sub_nodes;
        crate::subjects::math::formalism::proof::helpers::take2(sub_nodes)
    }};
    ($outcome:expr, 3) => {{
        let sub_nodes = $outcome.sub_nodes;
        crate::subjects::math::formalism::proof::helpers::take3(sub_nodes)
    }};
    ($outcome:expr, induction) => {{
        let sub_nodes = $outcome.sub_nodes;
        crate::subjects::math::formalism::proof::helpers::induction_goals(sub_nodes)
    }};
    ($outcome:expr, conjunction) => {{
        let sub_nodes = $outcome.sub_nodes;
        crate::subjects::math::formalism::proof::helpers::conjunction_goals(sub_nodes)
    }};
    ($outcome:expr, disjunction) => {{
        let sub_nodes = $outcome.sub_nodes;
        crate::subjects::math::formalism::proof::helpers::disjunction_goal(sub_nodes)
    }};
}
