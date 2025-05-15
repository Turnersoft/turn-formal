use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use super::theorem::Theorem;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TheoremMeta {
    pub theorem: Theorem,
    pub categories: Vec<String>,
    pub module_path: String,
    pub file_path: String,
}

#[linkme::distributed_slice]
pub static THEOREM_REGISTRATIONS: [OnceLock<TheoremMeta>] = [..];

pub fn get_theorems() -> Vec<&'static Theorem> {
    THEOREM_REGISTRATIONS
        .iter()
        .filter_map(|cell| cell.get().map(|meta| &meta.theorem))
        .collect()
}
