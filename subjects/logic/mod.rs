//! Formal Logic Systems Implementation
//! This module implements various logical systems based on type theory
//!
//! This module implements a formal logic system based on the Curry-Howard correspondence,
//! where propositions are types and proofs are terms.

pub mod combined;

pub mod first_order;
pub mod higher_order;
pub mod modal;
pub mod propositional;
pub mod temporal;
