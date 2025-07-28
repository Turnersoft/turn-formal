use std::collections::HashMap;

use crate::{
    subjects::math::formalism::{
        expressions::MathExpression, location::Located, proof::ContextEntry, traits::Substitutable,
    },
    turn_render::Identifier,
};

use super::Set;

impl Substitutable for Set {
    fn substitute(
        &self,
        _instantiations: &HashMap<Identifier, String>,
        _target: &Located<MathExpression>,
        _context: &Vec<ContextEntry>,
    ) -> Self {
        // For now, we don't substitute within sets. This might be needed later.
        self.clone()
    }
}
