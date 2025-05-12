use crate::subjects::math::formalism::proof::{ProofForest, ProofNode};

/// Builder for creating case analysis branches in a proof
pub struct CaseAnalysisBuilder<'a> {
    parent_branch: ProofNode,
    target: Option<String>,
    case_node_id: Option<String>, // Single node ID for all cases
    cases: Vec<(String, ProofNode)>,
    forest: &'a mut ProofForest,
}

impl<'a> CaseAnalysisBuilder<'a> {
    /// Create a new case analysis builder
    pub fn new(parent: ProofNode, forest: &'a mut ProofForest) -> Self {
        Self {
            parent_branch: parent,
            target: None,
            case_node_id: None,
            cases: Vec::new(),
            forest,
        }
    }

    /// Set the target expression for the case analysis
    pub fn on_expression(&mut self, target: impl Into<String>) -> &mut Self {
        self.target = Some(target.into());
        self
    }

    /// Ensure there's a parent node for all cases
    fn ensure_case_parent(&mut self) -> String {
        match &self.case_node_id {
            Some(id) => id.clone(),
            None => {
                let target_str = self.target.as_ref().map_or("", |s| s.as_str());
                // Create a parent node for all cases
                // We would need to add branch functions to ProofForest, for now just create a placeholder
                let case_parent_id = format!("{}_cases", self.parent_branch.id);
                self.case_node_id = Some(case_parent_id.clone());
                case_parent_id
            }
        }
    }

    /// Add a case to the analysis
    pub fn case<F>(&mut self, description: &str, case_fn: F) -> &mut Self
    where
        F: FnOnce(ProofNode, &mut ProofForest) -> ProofNode,
    {
        let parent_id = self.ensure_case_parent();
        // For now, we're just using the parent branch since we don't have access to the actual node
        let parent_node = self.parent_branch.clone();

        // Let the caller fill in this branch
        let final_node = case_fn(parent_node.clone(), self.forest);

        // Store this case in our list
        self.cases.push((description.to_string(), final_node));

        self
    }

    /// Build the case analysis result
    pub fn build(self) -> CaseResult {
        CaseResult {
            parent: self.parent_branch,
            target: self.target,
            case_nodes: self.cases.into_iter().map(|(_, node)| node).collect(),
        }
    }
}

/// Result from a completed case analysis
pub struct CaseResult {
    /// The parent branch
    pub parent: ProofNode,
    /// The target expression
    pub target: Option<String>,
    /// The top level nodes for each case
    pub case_nodes: Vec<ProofNode>,
}
