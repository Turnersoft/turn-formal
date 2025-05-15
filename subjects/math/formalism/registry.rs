use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::turn_render::{MathNode, ToTurnMath};

use super::theorem::Theorem;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TheoremMeta {
    pub theorem: Theorem,
    pub categories: Vec<String>,
    pub module_path: String,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SerializableTheoremMeta {
    theorem: MathNode,
    categories: Vec<String>,
    module_path: String,
    file_path: String,
}

#[linkme::distributed_slice]
pub static THEOREM_REGISTRATIONS: [OnceLock<TheoremMeta>] = [..];

pub fn get_theorem_metas() -> Vec<TheoremMeta> {
    THEOREM_REGISTRATIONS
        .iter()
        .filter_map(|cell| cell.get().cloned())
        .collect()
}

pub fn get_serializable_theorem_metas() -> Vec<SerializableTheoremMeta> {
    THEOREM_REGISTRATIONS
        .iter()
        .filter_map(|cell| {
            cell.get().map(|meta| {
                let math_node = meta.theorem.to_turn_math(meta.theorem.id.clone());
                SerializableTheoremMeta {
                    theorem: math_node,
                    categories: meta.categories.clone(),
                    module_path: meta.module_path.clone(),
                    file_path: meta.file_path.clone(),
                }
            })
        })
        .collect()
}
