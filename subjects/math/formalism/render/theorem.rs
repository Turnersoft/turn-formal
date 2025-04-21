use crate::{
    subjects::math::formalism::{
        core::{ProofState, Theorem},
        proof::ProofForest,
    },
    turn_render::{MathNode, MathNodeContent, ToTurnMath},
};

impl ToTurnMath for Theorem {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Create the initial proof state node
        let initial_state_node = self
            .initial_proof_state
            .to_turn_math(format!("{}:initial_state", master_id));

        // Use Theorem variant
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Theorem {
                name: self.name.clone(),
                description: self.description.clone(),
                initial_proof_state: Box::new(initial_state_node),
            }),
        }
    }
}

impl ToTurnMath for ProofState {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Create a more human-readable statement representation
        let readable_statement = self.create_readable_statement();

        // Convert statement to MathNode with improved readability
        let statement_node = MathNode {
            id: format!("{}:statement", master_id),
            content: Box::new(MathNodeContent::Text(readable_statement)),
        };

        // Convert quantifiers to MathNodes
        let quantifier_nodes = self
            .quantifier
            .iter()
            .enumerate()
            .map(|(i, q)| MathNode {
                id: format!("{}:quantifier_{}", master_id, i),
                content: Box::new(MathNodeContent::Text(format!("{:?}", q))),
            })
            .collect::<Vec<_>>();

        // Convert variable bindings to MathNodes
        let variable_nodes = self
            .value_variables
            .iter()
            .enumerate()
            .map(|(i, v)| MathNode {
                id: format!("{}:variable_{}", master_id, i),
                content: Box::new(MathNodeContent::Text(format!("{:?}", v))),
            })
            .collect::<Vec<_>>();

        // Use ProofState variant
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::ProofState {
                statement: Box::new(statement_node),
                path: self.path.clone(),
                justification: self.justification.clone(),
                quantifiers: quantifier_nodes,
                variables: variable_nodes,
            }),
        }
    }
}

impl ProofState {
    /// Create a more readable statement for display in the frontend
    fn create_readable_statement(&self) -> String {
        // First try simplified standard theorem descriptions based on common patterns
        let raw_statement = format!("{:?}", self.statement);

        // Check for well-known group theorems by looking for key phrases in the statement
        if raw_statement.contains("Inverse")
            && raw_statement.contains("GroupOperation")
            && (raw_statement.contains("Equal ") || raw_statement.contains("Equals "))
        {
            if raw_statement.contains("Uniqueness")
                || (raw_statement.contains("h1") && raw_statement.contains("h2"))
            {
                return "For any element g in a group, its inverse is unique.".to_string();
            } else if raw_statement.contains("Product")
                || (raw_statement.contains("(a·b)")
                    || raw_statement.contains("left: Inverse")
                        && raw_statement.contains("element: Operation"))
            {
                return "For elements a and b in a group, (a·b)⁻¹ = b⁻¹·a⁻¹.".to_string();
            }
        }

        // Improved detection for identity element uniqueness
        if (raw_statement.contains("Identity") && raw_statement.contains("Uniqueness"))
            || (raw_statement.contains("e1_identity_axiom")
                && raw_statement.contains("e2_identity_axiom"))
        {
            return "In a group, the identity element is unique.".to_string();
        }

        // Improved detection for the abelian group squared criterion theorem
        if ((raw_statement.contains("Abelian") || raw_statement.contains("abelian"))
            && (raw_statement.contains("Equivalent") || raw_statement.contains("Equivalent")))
            || (raw_statement.contains("IsIsomorphicTo")
                && raw_statement.contains("a·b")
                && raw_statement.contains("a·a")
                && raw_statement.contains("b·b"))
        {
            return "A group is abelian if and only if (a·b)·(a·b) = (a·a)·(b·b) for all a,b in the group.".to_string();
        }

        if raw_statement.contains("Lagrange")
            || (raw_statement.contains("Subgroup") && raw_statement.contains("OrderDivides"))
        {
            return "If H is a subgroup of a finite group G, then the order of H divides the order of G.".to_string();
        }

        // If no specific pattern matches, try to clean up the raw Debug output
        let mut cleaned = raw_statement
            .replace("Expression(Group(", "")
            .replace(")))", ")")
            .replace("))", ")")
            .replace("{", "")
            .replace("}", "")
            .replace("meta: RelationDetail", "")
            .replace("expressions: [", "")
            .replace("], metadata: , description: None", "")
            .replace("Some(", "")
            .replace("None", "")
            .replace("\\\"", "\"")
            .replace("\\\\", "\\");

        // Simplify nested parentheses
        while cleaned.contains("((") {
            cleaned = cleaned.replace("((", "(");
        }
        while cleaned.contains("))") {
            cleaned = cleaned.replace("))", ")");
        }

        // Add line breaks for readability of longer statements
        if cleaned.len() > 100 {
            let mut result = String::new();
            let parts: Vec<&str> = cleaned.split(", ").collect();
            for (i, part) in parts.iter().enumerate() {
                if i > 0 && i % 3 == 0 {
                    result.push_str(",\n");
                } else if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(part);
            }
            cleaned = result;
        }

        // Add a prefix to distinguish this as a raw statement
        format!("Formal statement: {}", cleaned)
    }
}

impl ToTurnMath for ProofForest {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Create a summary of the forest
        let summary = format!(
            "Contains {} nodes, {} root(s), and {} bookmark(s)",
            self.nodes.len(),
            self.roots.len(),
            self.bookmarks.len()
        );

        // Create MathNodes for each root
        let root_nodes = self
            .roots
            .iter()
            .enumerate()
            .map(|(i, &root_id)| {
                if let Some(root_node) = self.nodes.get(&root_id) {
                    let state_id = format!("{}:root_{}", master_id, i);
                    root_node.state.to_turn_math(state_id)
                } else {
                    // Fallback for roots that don't have nodes (shouldn't happen)
                    MathNode {
                        id: format!("{}:root_{}", master_id, i),
                        content: Box::new(MathNodeContent::Text(format!(
                            "Missing root {}",
                            root_id
                        ))),
                    }
                }
            })
            .collect::<Vec<_>>();

        // Extract bookmarks as (name, node_id) pairs
        let bookmarks = self
            .bookmarks
            .iter()
            .map(|(name, &node_id)| (name.clone(), node_id.to_string()))
            .collect::<Vec<_>>();

        // Use ProofForest variant
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::ProofForest {
                summary,
                roots: root_nodes,
                bookmarks,
            }),
        }
    }
}
