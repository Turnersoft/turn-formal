use serde::{Deserialize, Serialize};

pub trait GetAbstractionLevel {
    fn level(&self) -> AbstractionLevel;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbstractionLevel {
    Level1,
    Level2,
    Level3,
    Level4,
}

// Implement PartialOrd for AbstractionLevel
impl PartialOrd for AbstractionLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Implement Ord for AbstractionLevel
impl Ord for AbstractionLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Define the ordering: Level1 < Level2 < Level3 < Level4
        match (self, other) {
            (AbstractionLevel::Level1, AbstractionLevel::Level1) => std::cmp::Ordering::Equal,
            (AbstractionLevel::Level1, _) => std::cmp::Ordering::Less,

            (AbstractionLevel::Level2, AbstractionLevel::Level1) => std::cmp::Ordering::Greater,
            (AbstractionLevel::Level2, AbstractionLevel::Level2) => std::cmp::Ordering::Equal,
            (AbstractionLevel::Level2, _) => std::cmp::Ordering::Less,

            (AbstractionLevel::Level3, AbstractionLevel::Level3) => std::cmp::Ordering::Equal,
            (AbstractionLevel::Level3, AbstractionLevel::Level4) => std::cmp::Ordering::Less,
            (AbstractionLevel::Level3, _) => std::cmp::Ordering::Greater,

            (AbstractionLevel::Level4, AbstractionLevel::Level4) => std::cmp::Ordering::Equal,
            (AbstractionLevel::Level4, _) => std::cmp::Ordering::Greater,
        }
    }
}
