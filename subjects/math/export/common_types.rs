use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Common metadata for all mathematical objects
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct MathObjectMetadata {
    /// Unique identifier for the object
    pub id: String,
    /// Display name for the object
    pub name: String,
    /// Domain the object belongs to (e.g., "zfc", "groups")
    pub domain: String,
    /// Type of the object within its domain (e.g., "set", "axiom", "theorem")
    pub object_type: String,
    /// Short description of the object
    pub description: String,
    /// When the object was first created/discovered (if applicable)
    pub discovery_date: Option<String>,
    /// Who discovered/created the object (if applicable)
    pub discoverer: Option<String>,
    /// Importance level (1-5, with 5 being most important)
    pub importance: u8,
    /// Difficulty level (1-5, with 5 being most difficult)
    pub difficulty: u8,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// LaTeX representation if applicable
    pub latex: Option<String>,
}

/// A mathematical group structure
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct GroupDefinition {
    /// Name of the group
    pub name: String,
    /// Order of the group (number of elements)
    pub order: Option<u32>,
    /// Whether the group is abelian (commutative)
    pub is_abelian: bool,
    /// Whether the group is cyclic
    pub is_cyclic: bool,
    /// Whether the group is finite
    pub is_finite: bool,
    /// Symbol for the identity element
    pub identity_symbol: String,
    /// Symbol for the group operation
    pub operation_symbol: String,
}

/// A group element
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct GroupElement {
    /// Symbol or name for this element
    pub symbol: String,
    /// Order of this element in the group
    pub order: Option<u32>,
    /// Whether this element is the identity
    pub is_identity: bool,
    /// Whether this element is an involution (self-inverse)
    pub is_involution: bool,
    /// Symbol for the inverse of this element
    pub inverse_symbol: Option<String>,
}

/// A group homomorphism
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct GroupHomomorphism {
    /// Name of the homomorphism
    pub name: String,
    /// Domain (source) group
    pub domain: String,
    /// Codomain (target) group
    pub codomain: String,
    /// Whether this is an isomorphism
    pub is_isomorphism: bool,
    /// Whether this is a monomorphism (injective)
    pub is_monomorphism: bool,
    /// Whether this is an epimorphism (surjective)
    pub is_epimorphism: bool,
    /// Whether this is an endomorphism (domain = codomain)
    pub is_endomorphism: bool,
    /// Whether this is an automorphism (isomorphism with domain = codomain)
    pub is_automorphism: bool,
}

/// Reference to another mathematical object, potentially in a different domain
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct DomainReference {
    /// Domain the referenced object belongs to
    pub domain: String,
    /// Type of the referenced object
    pub object_type: String,
    /// ID of the referenced object
    pub object_id: String,
    /// Display name (for UI purposes)
    pub display_name: String,
    /// Brief description for hover text
    pub hover_description: Option<String>,
    /// LaTeX representation for display
    pub latex: Option<String>,
}

/// Type of reference between mathematical objects
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub enum ReferenceType {
    /// References a definition
    Definition,
    /// References a theorem
    Theorem,
    /// References an axiom
    Axiom,
    /// References an example
    Example,
    /// References a special case
    SpecialCase,
    /// References a generalization
    Generalization,
    /// References an application
    Application,
}

/// A mathematical proof
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct Proof {
    /// Method of proof (direct, contradiction, induction, etc.)
    pub method: String,
    /// Sequence of steps in the proof
    pub steps: Vec<ProofStep>,
    /// Key insights that make the proof work
    pub key_insights: Vec<String>,
    /// Alternative approaches that could be used
    pub alternative_approaches: Option<Vec<String>>,
    /// References to other mathematical objects used in the proof
    pub references: Vec<DomainReference>,
}

/// A single step in a mathematical proof
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct ProofStep {
    /// Step number in the proof
    pub step_number: u32,
    /// Description of the step
    pub description: String,
    /// LaTeX representation of the step
    pub latex: String,
    /// Justification for this step (e.g., "By Axiom 3", "From steps 2 and 4")
    pub justification: String,
    /// Optional animation data for visualizing this step
    pub animation: Option<AnimationData>,
    /// References to other mathematical objects used in this step
    pub references: Option<Vec<DomainReference>>,
}

/// Data for animations to visualize mathematical concepts
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct AnimationData {
    /// Type of animation (e.g., "2d-graph", "3d-model", "state-transition")
    pub animation_type: String,
    /// Initial state for the animation (JSON string)
    #[ts(type = "string")]
    pub initial_state: String,
    /// Sequence of animation states
    pub states: Vec<AnimationState>,
    /// Additional settings for the animation renderer (JSON string)
    #[ts(type = "string | null")]
    pub settings: Option<String>,
    /// Description of what the animation shows
    pub description: Option<String>,
}

/// A single state in an animation sequence
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct AnimationState {
    /// Time position in the animation (e.g., 0.0 to 1.0)
    pub time_position: f32,
    /// Description of what's happening at this state
    pub description: String,
    /// State data (JSON string, format depends on animation_type)
    #[ts(type = "string")]
    pub state_data: String,
    /// Whether this is a key state that should pause for explanation
    pub is_key_state: bool,
}

/// A module to organize mathematical content
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct MathModule {
    /// Metadata for the module
    pub metadata: MathObjectMetadata,
    /// Submodules contained within this module
    pub submodules: Option<Vec<String>>,
    /// Definitions included in this module
    pub definitions: Option<Vec<String>>,
    /// Theorems included in this module
    pub theorems: Option<Vec<String>>,
    /// Examples included in this module
    pub examples: Option<Vec<String>>,
    /// Learning objectives for this module
    pub learning_objectives: Option<Vec<String>>,
}

/// Helper struct to hold extracted documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeWithDocs {
    /// Name of the type
    pub name: String,
    /// Documentation comments
    pub docs: String,
    /// Type of the definition (struct, enum)
    pub kind: String,
    /// Fields or variants with their documentation
    pub members: Vec<MemberWithDocs>,
}

/// Helper struct to hold extracted field/variant documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberWithDocs {
    /// Name of the field or variant
    pub name: String,
    /// Documentation comments
    pub docs: String,
    /// Type information (for struct fields)
    pub type_info: Option<String>,
}
