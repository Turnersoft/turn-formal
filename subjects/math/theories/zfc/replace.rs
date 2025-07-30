use std::collections::HashMap;
use std::fmt::Debug;

use crate::{
    subjects::math::formalism::{
        expressions::MathExpression,
        location::Located,
        proof::ContextEntry,
        traits::{Search, Substitutable, instantiable::InstantiationType},
    },
    turn_render::Identifier,
};

use super::Set;

impl<U: 'static + Clone + Debug + Search> Substitutable<U> for Set {
    fn substitute(
        &self,
        _instantiations: &HashMap<Identifier, InstantiationType>,
        _target: &Located<U>,
        _context: &Vec<ContextEntry>,
    ) -> Self {
        // For now, we don't substitute within sets. This might be needed later.
        self.clone()
    }
}
