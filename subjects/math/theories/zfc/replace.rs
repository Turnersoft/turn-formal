use std::collections::HashMap;

use crate::{
    subjects::math::formalism::{
        expressions::MathExpression, proof::ContextEntry, replace::Substitutable,
    },
    turn_render::Identifier,
};

use super::Set;

impl Substitutable for Set {
    fn substitute(
        &self,
        _instantiations: &HashMap<Identifier, MathExpression>,
        _context: &Vec<ContextEntry>,
    ) -> Self {
        // For now, we don't substitute within sets. This might be needed later.
        self.clone()
    }
}
