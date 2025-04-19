use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;

use super::super::super::super::math::theories::groups::definitions::{
    AbelianPropertyVariant, FinitePropertyVariant, Group, GroupIdentity, GroupInverse,
    GroupInverseApplication, GroupNotation, GroupOperationProperty, GroupOperationVariant,
    GroupProperty, GroupSymbol, LieGroup, SimplePropertyVariant, TopologicalGroup,
};

use super::super::super::super::math::theories::groups::theorems::{
    prove_abelian_squared_criterion, prove_identity_uniqueness_with_syntax_trees,
    prove_inverse_product_rule, prove_inverse_uniqueness, prove_lagrange_theorem,
};

use super::super::super::super::math::theorem::core::Theorem;
use super::super::super::super::math::theorem::proof::{ProofBranch, Tactic};

#[derive(Serialize, Deserialize)]
pub struct DefinitionExport {
    pub id: String,
    pub name: String,
    pub description: String,
    pub type_name: String,
    pub variants: Option<Vec<VariantExport>>,
    pub fields: Option<Vec<FieldExport>>,
    pub implementations: Option<Vec<ImplementationExport>>,
}

#[derive(Serialize, Deserialize)]
pub struct VariantExport {
    pub id: String,
    pub name: String,
    pub description: String,
    pub fields: Option<Vec<FieldExport>>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldExport {
    pub name: String,
    pub type_name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImplementationExport {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterExport>,
    pub return_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParameterExport {
    pub name: String,
    pub type_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TheoremExport {
    pub id: String,
    pub name: String,
    pub statement: String,
    pub description: String,
    pub proof_steps: Vec<ProofStepExport>,
    pub references: Option<Vec<ReferenceExport>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ProofStepExport {
    pub id: String,
    pub description: String,
    pub formula: Option<String>,
    pub tactic_name: Option<String>,
    pub tactic_args: Option<HashMap<String, String>>,
    pub justification: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceExport {
    pub id: String,
    pub reference_type: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MathContentExport {
    pub definitions: Vec<DefinitionExport>,
    pub theorems: Vec<TheoremExport>,
}

// Function to export all group theory definitions
pub fn export_group_theory_definitions() -> Vec<DefinitionExport> {
    let mut definitions = Vec::new();

    // Export GroupOperationVariant enum
    definitions.push(DefinitionExport {
        id: "group_operation_variant".to_string(),
        name: "Group Operation Variant".to_string(),
        description: "Types of operations that can be used in group structures".to_string(),
        type_name: "enum".to_string(),
        variants: Some(vec![
            VariantExport {
                id: "multiplication".to_string(),
                name: "Multiplication".to_string(),
                description: "Standard multiplication (used in most abstract groups)".to_string(),
                fields: None,
            },
            VariantExport {
                id: "addition".to_string(),
                name: "Addition".to_string(),
                description: "Addition (used in additive groups)".to_string(),
                fields: None,
            },
            VariantExport {
                id: "composition".to_string(),
                name: "Composition".to_string(),
                description: "Composition (used in transformation groups)".to_string(),
                fields: None,
            },
            VariantExport {
                id: "matrix_multiplication".to_string(),
                name: "Matrix Multiplication".to_string(),
                description: "Matrix multiplication (for matrix groups)".to_string(),
                fields: None,
            },
            VariantExport {
                id: "direct_product".to_string(),
                name: "Direct Product".to_string(),
                description: "Direct product of groups".to_string(),
                fields: None,
            },
            VariantExport {
                id: "semidirect_product".to_string(),
                name: "Semidirect Product".to_string(),
                description: "Semidirect product of groups".to_string(),
                fields: None,
            },
            VariantExport {
                id: "free_product".to_string(),
                name: "Free Product".to_string(),
                description: "Free product (used in combinatorial group theory)".to_string(),
                fields: None,
            },
        ]),
        fields: None,
        implementations: None,
    });

    // Export GroupSymbol enum
    definitions.push(DefinitionExport {
        id: "group_symbol".to_string(),
        name: "Group Symbol".to_string(),
        description: "Symbols used to denote group operations".to_string(),
        type_name: "enum".to_string(),
        variants: Some(vec![
            VariantExport {
                id: "times".to_string(),
                name: "Times".to_string(),
                description: "Multiplication: ×".to_string(),
                fields: None,
            },
            VariantExport {
                id: "dot".to_string(),
                name: "Dot".to_string(),
                description: "Multiplication: ·".to_string(),
                fields: None,
            },
            VariantExport {
                id: "asterisk".to_string(),
                name: "Asterisk".to_string(),
                description: "Multiplication: *".to_string(),
                fields: None,
            },
            VariantExport {
                id: "plus".to_string(),
                name: "Plus".to_string(),
                description: "Addition: +".to_string(),
                fields: None,
            },
            VariantExport {
                id: "circle".to_string(),
                name: "Circle".to_string(),
                description: "Circle: ∘".to_string(),
                fields: None,
            },
            VariantExport {
                id: "semi_direct_left".to_string(),
                name: "SemiDirectLeft".to_string(),
                description: "Semidirect product: ⋊".to_string(),
                fields: None,
            },
            VariantExport {
                id: "semi_direct_right".to_string(),
                name: "SemiDirectRight".to_string(),
                description: "Semidirect product: ⋉".to_string(),
                fields: None,
            },
            VariantExport {
                id: "direct_product".to_string(),
                name: "DirectProduct".to_string(),
                description: "Direct product: ×".to_string(),
                fields: None,
            },
        ]),
        fields: None,
        implementations: None,
    });

    // Export Group struct
    definitions.push(DefinitionExport {
        id: "group".to_string(),
        name: "Group".to_string(),
        description:
            "Algebraic structure consisting of a set and binary operation satisfying group axioms"
                .to_string(),
        type_name: "struct".to_string(),
        variants: None,
        fields: Some(vec![
            FieldExport {
                name: "base_set".to_string(),
                type_name: "Set".to_string(),
                description: "The underlying set of the group".to_string(),
            },
            FieldExport {
                name: "operation".to_string(),
                type_name: "GroupOperation".to_string(),
                description: "The binary operation with its properties".to_string(),
            },
            FieldExport {
                name: "properties".to_string(),
                type_name: "Vec<GroupProperty>".to_string(),
                description: "Properties specific to the group structure".to_string(),
            },
        ]),
        implementations: None,
    });

    // Add more definitions as needed...

    definitions
}

// Function to extract proof steps from a theorem's proof branches
fn extract_proof_steps(branch: &ProofBranch) -> Vec<ProofStepExport> {
    let mut steps = Vec::new();

    // Process the tactics in this branch
    for (index, tactic) in branch.tactics.iter().enumerate() {
        let step_id = format!("step_{}", index + 1);

        // Extract information from the tactic
        let description = match tactic {
            Tactic::Intro(info) => info.description.clone(),
            Tactic::Subs(info) => format!("Substitution: {}", info.description),
            Tactic::TheoremApp(info) => format!("Apply theorem: {}", info.theorem_name),
            // Add other tactic types as needed
            _ => "Proof step".to_string(),
        };

        // Create the proof step export
        steps.push(ProofStepExport {
            id: step_id,
            description,
            formula: None, // Would extract from the tactic if available
            tactic_name: Some(format!("{:?}", tactic)),
            tactic_args: None,   // Would extract from the tactic if available
            justification: None, // Would extract from the tactic if available
        });
    }

    // Process child branches recursively
    for child in &branch.children {
        let mut child_steps = extract_proof_steps(child);
        steps.extend(child_steps);
    }

    steps
}

// Function to export a theorem
fn export_theorem(theorem: &Theorem) -> TheoremExport {
    let proof_steps = extract_proof_steps(&theorem.proof);

    TheoremExport {
        id: format!("theorem_{}", theorem.name.to_lowercase().replace(" ", "_")),
        name: theorem.name.clone(),
        statement: format!("{:?}", theorem.statement),
        description: theorem.description.clone().unwrap_or_default(),
        proof_steps,
        references: None,
        tags: None,
    }
}

// Function to export all group theory theorems
pub fn export_group_theory_theorems() -> Vec<TheoremExport> {
    let mut theorems = Vec::new();

    // Export the inverse uniqueness theorem
    let inverse_uniqueness = prove_inverse_uniqueness();
    theorems.push(export_theorem(&inverse_uniqueness));

    // Export the identity uniqueness theorem
    let identity_uniqueness = prove_identity_uniqueness_with_syntax_trees();
    theorems.push(export_theorem(&identity_uniqueness));

    // Export the inverse product rule theorem
    let inverse_product_rule = prove_inverse_product_rule();
    theorems.push(export_theorem(&inverse_product_rule));

    // Export the abelian squared criterion theorem
    let abelian_squared = prove_abelian_squared_criterion();
    theorems.push(export_theorem(&abelian_squared));

    // Export Lagrange's theorem
    let lagrange = prove_lagrange_theorem();
    theorems.push(export_theorem(&lagrange));

    theorems
}

// Main function to export all mathematics content
pub fn export_math_content() -> MathContentExport {
    MathContentExport {
        definitions: export_group_theory_definitions(),
        theorems: export_group_theory_theorems(),
    }
}

// Function to get all math content as JSON
pub fn get_math_content_json() -> String {
    let content = export_math_content();
    serde_json::to_string_pretty(&content).unwrap_or_else(|_| "{}".to_string())
}

// Function to create placeholder sample data for frontend development
pub fn get_sample_math_content_json() -> String {
    // Create sample definitions
    let definitions = vec![
        DefinitionExport {
            id: "group".to_string(),
            name: "Group".to_string(),
            description: "A group is a set G equipped with a binary operation that combines any two elements to form a third element. The operation must satisfy four conditions: closure, associativity, identity, and invertibility.".to_string(),
            type_name: "struct".to_string(),
            variants: None,
            fields: Some(vec![
                FieldExport {
                    name: "base_set".to_string(),
                    type_name: "Set".to_string(),
                    description: "The underlying set".to_string(),
                },
                FieldExport {
                    name: "operation".to_string(),
                    type_name: "GroupOperation".to_string(),
                    description: "The binary operation".to_string(),
                },
            ]),
            implementations: None,
        },
        DefinitionExport {
            id: "abelian_group".to_string(),
            name: "Abelian Group".to_string(),
            description: "An abelian group is a group in which the binary operation is commutative".to_string(),
            type_name: "enum".to_string(),
            variants: Some(vec![
                VariantExport {
                    id: "abelian".to_string(),
                    name: "Abelian".to_string(),
                    description: "Commutative".to_string(),
                    fields: None,
                },
                VariantExport {
                    id: "non_abelian".to_string(),
                    name: "NonAbelian".to_string(),
                    description: "Non-commutative".to_string(),
                    fields: None,
                },
            ]),
            fields: None,
            implementations: None,
        },
    ];

    // Create sample theorems
    let theorems = vec![
        TheoremExport {
            id: "inverse_uniqueness".to_string(),
            name: "Group Inverse Uniqueness".to_string(),
            statement: "For all elements g in a group G, if g*h₁ = e and g*h₂ = e, then h₁ = h₂"
                .to_string(),
            description: "This theorem proves that inverses in a group are unique".to_string(),
            proof_steps: vec![
                ProofStepExport {
                    id: "step_1".to_string(),
                    description: "Assume g*h₁ = e and g*h₂ = e".to_string(),
                    formula: Some("g*h₁ = e, g*h₂ = e".to_string()),
                    tactic_name: Some("Intro".to_string()),
                    tactic_args: None,
                    justification: Some("Given assumptions".to_string()),
                },
                ProofStepExport {
                    id: "step_2".to_string(),
                    description: "Multiply the first equation by h₂ on the left".to_string(),
                    formula: Some("h₂*(g*h₁) = h₂*e".to_string()),
                    tactic_name: Some("Multiply".to_string()),
                    tactic_args: None,
                    justification: Some("Multiplication is well-defined".to_string()),
                },
                ProofStepExport {
                    id: "step_3".to_string(),
                    description: "Use associativity: (h₂*g)*h₁ = h₂*e".to_string(),
                    formula: Some("(h₂*g)*h₁ = h₂".to_string()),
                    tactic_name: Some("Associativity".to_string()),
                    tactic_args: None,
                    justification: Some("Group axiom of associativity".to_string()),
                },
                ProofStepExport {
                    id: "step_4".to_string(),
                    description: "Use second assumption: h₂*g = e".to_string(),
                    formula: Some("e*h₁ = h₂".to_string()),
                    tactic_name: Some("Substitute".to_string()),
                    tactic_args: None,
                    justification: Some("Substitution from second assumption".to_string()),
                },
                ProofStepExport {
                    id: "step_5".to_string(),
                    description: "Use identity property: e*h₁ = h₁".to_string(),
                    formula: Some("h₁ = h₂".to_string()),
                    tactic_name: Some("Identity".to_string()),
                    tactic_args: None,
                    justification: Some("Group axiom of identity".to_string()),
                },
            ],
            references: Some(vec![ReferenceExport {
                id: "ref_1".to_string(),
                reference_type: "definition".to_string(),
                name: "Group".to_string(),
            }]),
            tags: Some(vec![
                "group theory".to_string(),
                "basic properties".to_string(),
            ]),
        },
        TheoremExport {
            id: "lagrange".to_string(),
            name: "Lagrange's Theorem".to_string(),
            statement:
                "If H is a subgroup of a finite group G, then the order of H divides the order of G"
                    .to_string(),
            description:
                "This foundational theorem relates the size of a subgroup to the size of the group"
                    .to_string(),
            proof_steps: vec![
                ProofStepExport {
                    id: "step_1".to_string(),
                    description: "Define left cosets of H in G".to_string(),
                    formula: Some("gH = {gh : h ∈ H} for g ∈ G".to_string()),
                    tactic_name: Some("Define".to_string()),
                    tactic_args: None,
                    justification: Some("Standard definition of left cosets".to_string()),
                },
                ProofStepExport {
                    id: "step_2".to_string(),
                    description: "Show that cosets partition G".to_string(),
                    formula: Some("G = ⋃_{g ∈ G} gH and gH ∩ g'H = ∅ or gH = g'H".to_string()),
                    tactic_name: Some("Prove".to_string()),
                    tactic_args: None,
                    justification: Some("Equivalence relation properties".to_string()),
                },
                ProofStepExport {
                    id: "step_3".to_string(),
                    description: "Each coset has |H| elements".to_string(),
                    formula: Some("|gH| = |H| for all g ∈ G".to_string()),
                    tactic_name: Some("Prove".to_string()),
                    tactic_args: None,
                    justification: Some("Bijection between H and gH".to_string()),
                },
                ProofStepExport {
                    id: "step_4".to_string(),
                    description: "Let [G:H] be the number of distinct cosets".to_string(),
                    formula: Some("[G:H] = number of distinct cosets".to_string()),
                    tactic_name: Some("Define".to_string()),
                    tactic_args: None,
                    justification: Some("Definition of index".to_string()),
                },
                ProofStepExport {
                    id: "step_5".to_string(),
                    description: "Then |G| = [G:H] * |H|".to_string(),
                    formula: Some("|G| = [G:H] * |H|".to_string()),
                    tactic_name: Some("Conclude".to_string()),
                    tactic_args: None,
                    justification: Some("Counting elements in the partition".to_string()),
                },
            ],
            references: Some(vec![
                ReferenceExport {
                    id: "ref_1".to_string(),
                    reference_type: "definition".to_string(),
                    name: "Subgroup".to_string(),
                },
                ReferenceExport {
                    id: "ref_2".to_string(),
                    reference_type: "definition".to_string(),
                    name: "Coset".to_string(),
                },
            ]),
            tags: Some(vec![
                "group theory".to_string(),
                "finite groups".to_string(),
                "subgroups".to_string(),
            ]),
        },
    ];

    // Create the full content export
    let content = MathContentExport {
        definitions,
        theorems,
    };

    serde_json::to_string_pretty(&content).unwrap_or_else(|_| "{}".to_string())
}
