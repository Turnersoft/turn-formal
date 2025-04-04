//! Theme type for the formalize_v2 visualization system
//!
//! Defines the available themes and their properties

use std::fmt;
use std::str::FromStr;

/// Available themes for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(feature = "theorem_visualizer")]
pub enum Theme {
    /// Light theme with dark text on light background
    Light,
    /// Dark theme with light text on dark background
    Dark,
}

#[cfg(feature = "theorem_visualizer")]
impl Theme {
    /// Get the CSS class name for this theme
    pub fn to_class(self) -> String {
        match self {
            Theme::Light => "light-theme".to_string(),
            Theme::Dark => "dark-theme".to_string(),
        }
    }
}

#[cfg(feature = "theorem_visualizer")]
impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}

#[cfg(feature = "theorem_visualizer")]
impl FromStr for Theme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err(()),
        }
    }
}
