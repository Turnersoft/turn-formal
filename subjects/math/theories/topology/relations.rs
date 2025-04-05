// Module: src/formalize_v2/subjects/math/theories/topology/relations.rs
// Defines relations specific to topology theory

use crate::subjects::math::theorem::expressions::MathExpression;
use crate::subjects::math::theorem::relations::RelationDetail;
use serde::{Deserialize, Serialize};

/// Entity information for topology relation operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopologyRelationEntity {
    /// Optional ID for referencing this relation
    pub id: Option<String>,

    /// Optional description explaining this relation instance
    pub description: Option<String>,

    /// Optional key-value pairs for additional context
    pub tags: Vec<(String, String)>,
}

/// Relations specific to topology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologyRelation {
    /// A set is open in a topological space
    IsOpen {
        entity: TopologyRelationEntity,
        set: MathExpression,
        space: MathExpression,
    },

    /// A set is closed in a topological space
    IsClosed {
        entity: TopologyRelationEntity,
        set: MathExpression,
        space: MathExpression,
    },

    /// A set is a neighborhood of a point
    IsNeighborhood {
        entity: TopologyRelationEntity,
        set: MathExpression,
        point: MathExpression,
        space: MathExpression,
    },

    /// A set is a basis for a topology
    IsBasis {
        entity: TopologyRelationEntity,
        collection: MathExpression,
        space: MathExpression,
    },

    /// A set is the closure of another
    IsClosure {
        entity: TopologyRelationEntity,
        closure: MathExpression,
        set: MathExpression,
        space: MathExpression,
    },

    /// A set is the interior of another
    IsInterior {
        entity: TopologyRelationEntity,
        interior: MathExpression,
        set: MathExpression,
        space: MathExpression,
    },

    /// A set is the boundary of another
    IsBoundary {
        entity: TopologyRelationEntity,
        boundary: MathExpression,
        set: MathExpression,
        space: MathExpression,
    },

    /// A topological space is connected
    IsConnected {
        entity: TopologyRelationEntity,
        space: MathExpression,
    },

    /// A topological space is path-connected
    IsPathConnected {
        entity: TopologyRelationEntity,
        space: MathExpression,
    },

    /// A topological space is compact
    IsCompact {
        entity: TopologyRelationEntity,
        space: MathExpression,
    },

    /// A topological space is Hausdorff
    IsHausdorff {
        entity: TopologyRelationEntity,
        space: MathExpression,
    },

    /// A function is continuous
    IsContinuous {
        entity: TopologyRelationEntity,
        function: MathExpression,
        domain: MathExpression,
        codomain: MathExpression,
    },

    /// Two topological spaces are homeomorphic
    AreHomeomorphic {
        entity: TopologyRelationEntity,
        first: MathExpression,
        second: MathExpression,
    },

    /// A sequence converges to a point
    Converges {
        entity: TopologyRelationEntity,
        sequence: MathExpression,
        limit: MathExpression,
        space: MathExpression,
    },

    /// A function is a homeomorphism
    IsHomeomorphism {
        entity: TopologyRelationEntity,
        function: MathExpression,
        domain: MathExpression,
        codomain: MathExpression,
    },

    /// A space is a subspace of another
    IsSubspace {
        entity: TopologyRelationEntity,
        subspace: MathExpression,
        space: MathExpression,
    },

    /// A collection is an open cover of a space
    IsOpenCover {
        entity: TopologyRelationEntity,
        cover: MathExpression,
        space: MathExpression,
    },

    /// A cover has a finite subcover
    HasFiniteSubcover {
        entity: TopologyRelationEntity,
        cover: MathExpression,
        space: MathExpression,
    },

    /// A space is locally compact
    IsLocallyCompact {
        entity: TopologyRelationEntity,
        space: MathExpression,
    },

    /// A space is paracompact
    IsParacompact {
        entity: TopologyRelationEntity,
        space: MathExpression,
    },

    /// A space is metrizable
    IsMetrizable {
        entity: TopologyRelationEntity,
        space: MathExpression,
    },

    /// Custom topology relation
    Custom {
        entity: TopologyRelationEntity,
        name: String,
        parameters: Vec<MathExpression>,
    },
}

// Helper methods for constructor functions
impl TopologyRelation {
    /// Create a new IsOpen relation
    pub fn is_open(set: &MathExpression, space: &MathExpression) -> Self {
        let entity = TopologyRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        TopologyRelation::IsOpen {
            entity,
            set: set.clone(),
            space: space.clone(),
        }
    }

    /// Create a new IsClosed relation
    pub fn is_closed(set: &MathExpression, space: &MathExpression) -> Self {
        let entity = TopologyRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        TopologyRelation::IsClosed {
            entity,
            set: set.clone(),
            space: space.clone(),
        }
    }

    /// Create a new IsConnected relation
    pub fn is_connected(space: &MathExpression) -> Self {
        let entity = TopologyRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        TopologyRelation::IsConnected {
            entity,
            space: space.clone(),
        }
    }

    /// Create a new IsCompact relation
    pub fn is_compact(space: &MathExpression) -> Self {
        let entity = TopologyRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        TopologyRelation::IsCompact {
            entity,
            space: space.clone(),
        }
    }

    /// Create a new IsContinuous relation
    pub fn is_continuous(
        function: &MathExpression,
        domain: &MathExpression,
        codomain: &MathExpression,
    ) -> Self {
        let entity = TopologyRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        TopologyRelation::IsContinuous {
            entity,
            function: function.clone(),
            domain: domain.clone(),
            codomain: codomain.clone(),
        }
    }

    /// Create a new AreHomeomorphic relation
    pub fn are_homeomorphic(first: &MathExpression, second: &MathExpression) -> Self {
        let entity = TopologyRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        TopologyRelation::AreHomeomorphic {
            entity,
            first: first.clone(),
            second: second.clone(),
        }
    }

    /// Create a custom relation
    pub fn custom(name: &str, parameters: Vec<MathExpression>) -> Self {
        let entity = TopologyRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        TopologyRelation::Custom {
            entity,
            name: name.to_string(),
            parameters,
        }
    }
}
