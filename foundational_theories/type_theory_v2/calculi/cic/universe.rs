use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Level(pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Universe {
    Prop,
    Type(Level),
}

impl Level {
    pub fn new(n: u32) -> Self {
        Level(n)
    }

    pub fn max(self, other: Level) -> Level {
        Level(self.0.max(other.0))
    }

    pub fn imax(self, other: Level) -> Level {
        // For impredicative universes:
        // imax(u,v) = if v = 0 then 0 else max(u,v)
        if other.0 == 0 {
            Level(0)
        } else {
            self.max(other)
        }
    }

    pub fn succ(self) -> Level {
        Level(self.0 + 1)
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn is_succ(self) -> bool {
        self.0 > 0
    }

    pub fn is_param(self) -> bool {
        false // We'll add universe parameters later
    }

    pub fn is_max(self) -> bool {
        false // We'll add max levels later
    }
}

impl Universe {
    pub fn is_prop(self) -> bool {
        matches!(self, Universe::Prop)
    }

    pub fn is_type(self) -> bool {
        matches!(self, Universe::Type(_))
    }

    pub fn level(self) -> Option<Level> {
        match self {
            Universe::Type(l) => Some(l),
            _ => None,
        }
    }

    pub fn max(self, other: Universe) -> Universe {
        match (self, other) {
            (Universe::Prop, Universe::Prop) => Universe::Prop,
            (Universe::Prop, Universe::Type(_)) => other,
            (Universe::Type(_), Universe::Prop) => self,
            (Universe::Type(l1), Universe::Type(l2)) => Universe::Type(l1.max(l2)),
        }
    }

    pub fn imax(self, other: Universe) -> Universe {
        match (self, other) {
            (Universe::Prop, _) => Universe::Prop,
            (_, Universe::Prop) => Universe::Prop,
            (Universe::Type(l1), Universe::Type(l2)) => {
                // For impredicative universes:
                // imax(u,v) = if v = 0 then u else max(u,v)
                if l2.is_zero() {
                    Universe::Type(l1)
                } else {
                    Universe::Type(l1.max(l2))
                }
            }
        }
    }

    pub fn succ(self) -> Universe {
        match self {
            Universe::Prop => Universe::Type(Level::new(1)),
            Universe::Type(l) => {
                // For Type₀, successor is Type₁
                // For other levels, increment by 1
                if l.is_zero() {
                    Universe::Type(Level::new(1))
                } else {
                    Universe::Type(l.succ())
                }
            }
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Universe::Prop => write!(f, "Prop"),
            Universe::Type(level) => write!(f, "Type_{}", level),
        }
    }
}
