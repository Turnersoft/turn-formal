use crate::subjects::math::formalism::expressions::{MathExpression, TheoryExpression};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::proof::collect::CollectSubExpressions;
use crate::subjects::math::formalism::proof::path_index::{PathError, ReplaceableAtPath};

use super::definitions::SetRelation;

impl SetRelation {
    pub fn collect_contained_expressions(
        &self,
        base_path: Vec<usize>,
        collected_targets: &mut Vec<(Vec<usize>, MathExpression)>,
        depth: usize,
    ) {
        if depth > 100 {
            return;
        }
        match self {
            SetRelation::ElementOf { element, set } => {
                let mut path_e = base_path.clone();
                path_e.push(1);
                match element {
                    Parametrizable::Concrete(e) => {
                        // Add code to extract MathExpression from element if needed
                    }
                    Parametrizable::Variable(_) => {
                        // Handle variable case
                    }
                }

                let mut path_s = base_path.clone();
                path_s.push(2);
                match set {
                    Parametrizable::Concrete(s) => {
                        // Add code to extract MathExpression from set if needed
                    }
                    Parametrizable::Variable(_) => {
                        // Handle variable case
                    }
                }
            }
            SetRelation::SubsetOf { subset, superset } => {
                let mut path_sub = base_path.clone();
                path_sub.push(1);
                match subset {
                    Parametrizable::Concrete(s) => {
                        // Add code to extract MathExpression from subset if needed
                    }
                    Parametrizable::Variable(_) => {
                        // Handle variable case
                    }
                }

                let mut path_super = base_path.clone();
                path_super.push(2);
                match superset {
                    Parametrizable::Concrete(s) => {
                        // Add code to extract MathExpression from superset if needed
                    }
                    Parametrizable::Variable(_) => {
                        // Handle variable case
                    }
                }
            }
            // Handle other SetTheoryRelation variants as needed
            _ => {}
        }
    }
}

impl ReplaceableAtPath for SetRelation {
    fn replace_at_path_recursive(
        self,
        path: &[usize],
        replacement: MathExpression,
    ) -> Result<Self, PathError> {
        if path.is_empty() {
            return Err(PathError::TypeMismatch);
        }

        let current_idx = path[0];
        let remaining_path = &path[1..];

        match self {
            SetRelation::ElementOf { element, set } => match current_idx {
                1 => {
                    match element {
                        Parametrizable::Concrete(e) => {
                            // We should wrap the result in Parametrizable::Concrete
                            Ok(SetRelation::ElementOf {
                                element: Parametrizable::Concrete(e),
                                set,
                            })
                        }
                        Parametrizable::Variable(_) => {
                            // Handle variable case
                            Ok(SetRelation::ElementOf { element, set })
                        }
                    }
                }
                2 => {
                    match set {
                        Parametrizable::Concrete(s) => {
                            // We should wrap the result in Parametrizable::Concrete
                            Ok(SetRelation::ElementOf {
                                element,
                                set: Parametrizable::Concrete(s),
                            })
                        }
                        Parametrizable::Variable(_) => {
                            // Handle variable case
                            Ok(SetRelation::ElementOf { element, set })
                        }
                    }
                }
                _ => Err(PathError::InvalidPath),
            },
            SetRelation::SubsetOf { subset, superset } => match current_idx {
                1 => {
                    match subset {
                        Parametrizable::Concrete(s) => {
                            // We should wrap the result in Parametrizable::Concrete
                            Ok(SetRelation::SubsetOf {
                                subset: Parametrizable::Concrete(s),
                                superset,
                            })
                        }
                        Parametrizable::Variable(_) => {
                            // Handle variable case
                            Ok(SetRelation::SubsetOf { subset, superset })
                        }
                    }
                }
                2 => {
                    match superset {
                        Parametrizable::Concrete(s) => {
                            // We should wrap the result in Parametrizable::Concrete
                            Ok(SetRelation::SubsetOf {
                                subset,
                                superset: Parametrizable::Concrete(s),
                            })
                        }
                        Parametrizable::Variable(_) => {
                            // Handle variable case
                            Ok(SetRelation::SubsetOf { subset, superset })
                        }
                    }
                }
                _ => Err(PathError::InvalidPath),
            },
            // Handle other SetTheoryRelation variants as needed
            _ => Err(PathError::NotImplemented),
        }
    }
}
