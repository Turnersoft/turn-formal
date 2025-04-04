//! Tactic combinators for building complex proof strategies

use crate::formalize_v2::subjects::logic::propositional::tactics::{
    ProofStateTactic, TacticResult,
};

use super::{proof_state::ProofState, TacticError};

/// A tactic that applies two tactics in sequence
#[derive(Debug, Clone)]
pub struct Then<F, S> {
    first: F,
    second: S,
}

impl<F, S> Then<F, S> {
    pub fn new(first: F, second: S) -> Self {
        Self { first, second }
    }
}

impl<F, S> ProofStateTactic for Then<F, S>
where
    F: ProofStateTactic<Error = TacticError>,
    S: ProofStateTactic<Error = TacticError>,
{
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let mut intermediate = self.first.apply(state)?;
        self.second.apply(&mut intermediate)
    }
}

/// A tactic that tries the first tactic and if it fails, tries the second
#[derive(Debug, Clone)]
pub struct OrElse<F, S> {
    first: F,
    second: S,
}

impl<F, S> OrElse<F, S> {
    pub fn new(first: F, second: S) -> Self {
        Self { first, second }
    }
}

impl<F, S> ProofStateTactic for OrElse<F, S>
where
    F: ProofStateTactic<Error = TacticError>,
    S: ProofStateTactic<Error = TacticError>,
{
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        match self.first.apply(state) {
            Ok(result) => Ok(result),
            Err(_) => self.second.apply(state),
        }
    }
}

/// A tactic that repeats another tactic until it fails
#[derive(Debug, Clone)]
pub struct Repeat<T> {
    tactic: T,
}

impl<T> Repeat<T> {
    pub fn new(tactic: T) -> Self {
        Self { tactic }
    }
}

impl<T> ProofStateTactic for Repeat<T>
where
    T: ProofStateTactic<Error = TacticError>,
{
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let mut current = state.clone();
        let mut last_result = None;

        while let Ok(next_state) = self.tactic.apply(&mut current) {
            current = next_state;
            last_result = Some(current.clone());
        }

        match last_result {
            Some(result) => Ok(result),
            None => self.tactic.apply(state), // Try at least once
        }
    }
}

/// A tactic that applies two tactics in parallel and combines their results
#[derive(Debug, Clone)]
pub struct AndThen<F, S> {
    first: F,
    second: S,
}

impl<F, S> AndThen<F, S> {
    pub fn new(first: F, second: S) -> Self {
        Self { first, second }
    }
}

impl<F, S> ProofStateTactic for AndThen<F, S>
where
    F: ProofStateTactic<Error = TacticError>,
    S: ProofStateTactic<Error = TacticError>,
{
    type Error = TacticError;

    fn apply(&self, state: &mut ProofState) -> Result<ProofState, Self::Error> {
        let mut first_state = self.first.apply(state)?;
        self.second.apply(&mut first_state)
    }
}

// Extension trait for composing tactics
pub trait TacticExt: Sized {
    fn or_else<U>(self, other: U) -> OrElse<Self, U>
    where
        Self: ProofStateTactic<Error = TacticError>,
        U: ProofStateTactic<Error = TacticError>,
    {
        OrElse::new(self, other)
    }
}

// Implement TacticExt for all types
impl<T> TacticExt for T {}
