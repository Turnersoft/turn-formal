use super::proof::tactics::{ContextOrStatement, RelationSource, RewriteDirection, Tactic, Target};
use super::{
    expressions::{MathExpression, TheoryExpression},
    extract::Parametrizable,
    interpretation::TypeViewOperator,
    location::Located,
    objects::MathObject,
    proof::{
        ContextEntry, DefinitionState, NodeRole, ProofForest, ProofGoal, ProofNode,
        QuantifiedMathObject, Quantifier, SubgoalCombination, TacticOutcome, ValueBindedVariable,
    },
    relations::{MathRelation, Quantification},
};
use crate::subjects::math::theories::groups::definitions::GroupAction;
use crate::subjects::math::theories::{
    groups::definitions::{
        AlternatingGroup, CyclicGroup, DihedralGroup, FreeGroup, GenericGroup, Group, GroupElement,
        GroupExpression, GroupHomomorphism, SymmetricGroup,
    },
    number_theory::definitions::Number,
    zfc::definitions::{GenericSet, Set, SetElement},
};
use crate::turn_render::{Identifier, RichText, RichTextSegment};
use std::fmt;
use std::sync::Arc;

/// A trait for structured debug output showing type information like {:#?} but readable
pub trait ShortDebug {
    fn short_debug(&self) -> String;
}

// Helper function to add indentation
fn indent(text: &str, level: usize) -> String {
    let indent_str = "  ".repeat(level);
    text.lines()
        .map(|line| {
            if line.trim().is_empty() {
                line.to_string()
            } else {
                format!("{}{}", indent_str, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// Helper to truncate long IDs but show enough for identification
fn short_id(id: &str) -> String {
    if id.len() > 8 {
        format!("{}...", &id[..8])
    } else {
        id.to_string()
    }
}

// Extract a concise mathematical expression from a Located expression using ShortDebug trait
fn extract_variable_name<T>(located: &Located<T>) -> String
where
    T: ShortDebug + 'static,
{
    match &located.data {
        Parametrizable::Variable(id) => {
            // Handle variable case directly
            id.body.clone()
        }
        Parametrizable::Concrete(concrete) => {
            // ✅ IMPROVED: Try to extract variable names intelligently for specific cases
            if let Some(concrete_any) = (concrete.as_ref() as &dyn std::any::Any).downcast_ref::<crate::subjects::math::theories::groups::definitions::GroupExpression>() {
                match concrete_any {
                    crate::subjects::math::theories::groups::definitions::GroupExpression::Element { element: Some(el), .. } => {
                        // This is a GroupExpression::Element - try to extract the variable name from the element
                        extract_variable_name(el)
                    }
                    _ => {
                        // For other GroupExpression types, use short_debug
            concrete.short_debug()
                    }
                }
            } else {
                // For non-GroupExpression types, use short_debug
                concrete.short_debug()
            }
        }
    }
}

// Implementations for primitive types
impl ShortDebug for i32 {
    fn short_debug(&self) -> String {
        self.to_string()
    }
}

impl ShortDebug for i64 {
    fn short_debug(&self) -> String {
        self.to_string()
    }
}

impl ShortDebug for String {
    fn short_debug(&self) -> String {
        if self.len() > 30 {
            format!("\"{}...\"", &self[..30])
        } else {
            format!("\"{}\"", self)
        }
    }
}

// Missing type implementations for compilation
impl ShortDebug for TypeViewOperator {
    fn short_debug(&self) -> String {
        format!("TypeView::{:?}", self)
            .split("::")
            .last()
            .unwrap_or("View")
            .to_string()
    }
}

impl ShortDebug for CyclicGroup {
    fn short_debug(&self) -> String {
        if let Some(order) = self.order {
            format!("CyclicGroup {{ order: {} }}", order)
        } else {
            "CyclicGroup { order: ∞ }".to_string()
        }
    }
}

impl ShortDebug for DihedralGroup {
    fn short_debug(&self) -> String {
        format!("DihedralGroup {{ order: {} }}", self.order)
    }
}

impl ShortDebug for FreeGroup {
    fn short_debug(&self) -> String {
        format!("FreeGroup {{ rank: {} }}", self.rank)
    }
}

impl ShortDebug for SymmetricGroup {
    fn short_debug(&self) -> String {
        format!("SymmetricGroup {{ degree: {} }}", self.degree)
    }
}

impl ShortDebug for AlternatingGroup {
    fn short_debug(&self) -> String {
        format!("AlternatingGroup {{ degree: {} }}", self.degree)
    }
}

impl ShortDebug for GenericSet {
    fn short_debug(&self) -> String {
        "GenericSet".to_string()
    }
}

impl ShortDebug for SetElement {
    fn short_debug(&self) -> String {
        "SetElement".to_string()
    }
}

impl ShortDebug for Target {
    fn short_debug(&self) -> String {
        let scope_str = match &self.scope {
            ContextOrStatement::Context(ids) => {
                if ids.is_empty() {
                    "Context[]".to_string()
                } else if ids.len() == 1 {
                    format!("Context[{}]", ids[0].body)
                } else {
                    format!("Context[{} items]", ids.len())
                }
            }
            ContextOrStatement::Statement => "Statement".to_string(),
            ContextOrStatement::Both => "Both".to_string(),
        };

        let indices_str = match &self.vec_indices {
            Some(indices) if !indices.is_empty() => format!(" @{:?}", indices),
            _ => "".to_string(),
        };

        format!("{}:{}{}", scope_str, short_id(&self.id), indices_str)
    }
}

impl ShortDebug for RelationSource {
    fn short_debug(&self) -> String {
        match self {
            RelationSource::LocalAssumption(id) => format!("Local({})", id.body),
            RelationSource::Theorem(name, Some(index)) => format!("{}[{}]", name, index),
            RelationSource::Theorem(name, None) => name.clone(),
        }
    }
}

impl ShortDebug for RewriteDirection {
    fn short_debug(&self) -> String {
        match self {
            RewriteDirection::Forward => "→".to_string(),
            RewriteDirection::Backward => "←".to_string(),
        }
    }
}

// Detailed Tactic implementations showing all relevant information
impl ShortDebug for Tactic {
    fn short_debug(&self) -> String {
        match self {
            Tactic::AssumeImplicationAntecedent { with_name } => {
                format!("AssumeImplication({})", with_name.body)
            }
            Tactic::SplitGoalConjunction => "SplitGoalConjunction".to_string(),
            Tactic::SplitGoalDisjunction { disjunct_index } => {
                format!("SplitGoalDisjunction[{}]", disjunct_index)
            }
            Tactic::CaseAnalysis { on_variable, cases } => {
                format!("CaseAnalysis({}, {} cases)", on_variable.body, cases.len())
            }
            Tactic::Induction {
                variable_name,
                hypothesis_name,
            } => {
                format!(
                    "Induction({} via {})",
                    variable_name.body, hypothesis_name.body
                )
            }
            Tactic::ProvideWitness {
                target_quantifier,
                witness,
            } => {
                format!(
                    "ProvideWitness({} := {})",
                    target_quantifier.body,
                    witness.short_debug()
                )
            }
            Tactic::SplitAssumptionConjunction {
                target_hypothesis,
                with_names,
            } => {
                if with_names.is_empty() {
                    format!("SplitAssumption({})", target_hypothesis.body)
                } else {
                    let names: Vec<String> = with_names.iter().map(|n| n.body.clone()).collect();
                    format!(
                        "SplitAssumption({} → [{}])",
                        target_hypothesis.body,
                        names.join(", ")
                    )
                }
            }
            Tactic::SplitAssumptionDisjunction {
                target_hypothesis,
                with_names,
            } => {
                format!(
                    "SplitDisjunction({}, {} cases)",
                    target_hypothesis.body,
                    with_names.len()
                )
            }
            Tactic::ByRelation(source) => {
                format!("ByRelation({})", source.short_debug())
            }
            Tactic::ByReflexivity => "ByReflexivity".to_string(),
            Tactic::ByContradiction {
                hypothesis1,
                hypothesis2,
            } => {
                format!(
                    "ByContradiction({} ⊥ {})",
                    hypothesis1.body, hypothesis2.body
                )
            }
            Tactic::ByGoalContradiction {
                conflicting_hypothesis,
            } => {
                format!("ByGoalContradiction({})", conflicting_hypothesis.body)
            }
            Tactic::Rewrite {
                using_rule,
                target,
                direction,
                instantiations,
            } => {
                let inst_info = if instantiations.is_empty() {
                    "".to_string()
                } else {
                    let bindings: Vec<String> = instantiations
                        .iter()
                        .map(|(k, v)| format!("{}:={}", k.body, v.body))
                        .collect();
                    format!(" with [{}]", bindings.join(", "))
                };
                format!(
                    "Rewrite({} {} @ {}{})",
                    using_rule.short_debug(),
                    direction.short_debug(),
                    target.short_debug(),
                    inst_info
                )
            }
            Tactic::UnfoldDefinition {
                definition_to_unfold,
                target,
            } => {
                format!(
                    "UnfoldDef({} @ {})",
                    definition_to_unfold.body,
                    target.short_debug()
                )
            }
            Tactic::IntroduceLetBinding {
                target_expression,
                with_name,
            } => {
                format!(
                    "Let({} := {})",
                    with_name.body,
                    target_expression.short_debug()
                )
            }
            Tactic::RenameBoundVariable {
                target,
                from_name,
                to_name,
            } => {
                format!(
                    "Rename({} → {} @ {})",
                    from_name.body,
                    to_name.body,
                    target.short_debug()
                )
            }
            Tactic::Revert {
                hypothesis_to_revert,
            } => {
                format!("Revert({})", hypothesis_to_revert.body)
            }
            Tactic::SearchAssumptions => "SearchAssumptions".to_string(),
            Tactic::SearchTheoremLibrary => "SearchTheoremLibrary".to_string(),
            Tactic::Search => "Search".to_string(),
            Tactic::Simplify { target } => {
                format!("Simplify({})", target.short_debug())
            }
            Tactic::Auto {
                depth,
                with_tactics,
            } => {
                let depth_str = depth.map_or("∞".to_string(), |d| d.to_string());
                format!("Auto(depth={}, {} tactics)", depth_str, with_tactics.len())
            }
            Tactic::DisproveByTheorem { theorem_id } => {
                format!("DisproveBy({})", theorem_id)
            }
        }
    }
}

impl ShortDebug for TacticOutcome {
    fn short_debug(&self) -> String {
        format!(
            "TacticOutcome {{\n  manager: {},\n  sub_nodes: [{}]\n}}",
            indent(&self.manager.short_debug(), 1),
            self.sub_nodes.len()
        )
    }
}

// ProofNode - THE MAIN TARGET with complete structure
impl ShortDebug for ProofNode {
    fn short_debug(&self) -> String {
        let mut result = String::new();
        result.push_str("ProofNode {\n");

        // ID
        result.push_str(&format!("  id: {},\n", short_id(&self.id)));

        // Parent
        if let Some(parent_id) = &self.parent {
            result.push_str(&format!("  parent: Some({}),\n", short_id(parent_id)));
        } else {
            result.push_str("  parent: None,\n");
        }

        // Children
        if self.children.is_empty() {
            result.push_str("  children: [],\n");
        } else {
            let child_ids: Vec<String> = self.children.iter().map(|id| short_id(id)).collect();
            result.push_str(&format!("  children: {:?},\n", child_ids));
        }

        // Tactic
        result.push_str(&format!(
            "  tactic: {},\n",
            indent(&self.tactic.short_debug(), 1)
        ));

        // Role
        result.push_str(&format!(
            "  role: {},\n",
            indent(&self.role.short_debug(), 1)
        ));

        // Description
        if let Some(desc) = &self.description {
            result.push_str(&format!(
                "  description: Some({})\n",
                indent(&desc.short_debug(), 1)
            ));
        } else {
            result.push_str("  description: None\n");
        }

        result.push_str("}");
        result
    }
}

impl ShortDebug for ProofForest {
    fn short_debug(&self) -> String {
        format!(
            "ProofForest {{\n  nodes: {} entries,\n  roots: {:?}\n}}",
            self.len(),
            self.roots.iter().map(|id| short_id(id)).collect::<Vec<_>>()
        )
    }
}

impl ShortDebug for ValueBindedVariable {
    fn short_debug(&self) -> String {
        format!(
            "ValueBindedVariable {{\n  name: {},\n  value: {}\n}}",
            indent(&self.name.short_debug(), 1),
            indent(&self.value.short_debug(), 1)
        )
    }
}

impl ShortDebug for QuantifiedMathObject {
    fn short_debug(&self) -> String {
        let q_type = match self.quantification {
            Quantification::Universal => "Universal",
            Quantification::Existential => "Existential",
            Quantification::UniqueExistential => "UniqueExistential",
        };
        format!(
            "QuantifiedMathObject {{\n  quantification: {},\n  variable: {},\n  object_type: {}\n}}",
            q_type,
            indent(&self.variable.short_debug(), 1),
            indent(&self.object_type.short_debug(), 1)
        )
    }
}

impl ShortDebug for RichText {
    fn short_debug(&self) -> String {
        let text_content = rich_text_to_string(self);
        if text_content.len() > 50 {
            format!("RichText(\"{}...\")", &text_content[..50])
        } else {
            format!("RichText(\"{}\")", text_content)
        }
    }
}

// Helper function to convert RichText to String
fn rich_text_to_string(rich_text: &RichText) -> String {
    rich_text
        .segments
        .iter()
        .map(|segment| match segment {
            RichTextSegment::Text(text) => text.clone(),
            RichTextSegment::StyledText { text, .. } => text.clone(),
            RichTextSegment::Math(_) => "[Math]".to_string(),
            RichTextSegment::Link { content, .. } => content
                .iter()
                .map(|seg| match seg {
                    RichTextSegment::Text(t) => t.clone(),
                    _ => "[Link]".to_string(),
                })
                .collect::<String>(),
            RichTextSegment::FootnoteReference(id) => format!("[{}]", id),
            RichTextSegment::CodeInline(code) => format!("`{}`", code),
        })
        .collect::<Vec<String>>()
        .join("")
}

impl ShortDebug for Identifier {
    fn short_debug(&self) -> String {
        format!("Identifier({})", self.body)
    }
}

// Add Arc support
impl<T: ShortDebug> ShortDebug for Arc<T> {
    fn short_debug(&self) -> String {
        self.as_ref().short_debug()
    }
}

// Add Number support
impl ShortDebug for Number {
    fn short_debug(&self) -> String {
        format!("Number({:?})", self)
    }
}

// Show type structure with content
impl<T: ShortDebug> ShortDebug for Located<T> {
    fn short_debug(&self) -> String {
        format!(
            "Located {{\n  id: {},\n  data: {}\n}}",
            short_id(&self.id),
            indent(&self.data.short_debug(), 1)
        )
    }
}

impl<T: ShortDebug> ShortDebug for Parametrizable<T> {
    fn short_debug(&self) -> String {
        match self {
            Parametrizable::Concrete(val) => {
                format!("Concrete(\n{}\n)", indent(&val.short_debug(), 1))
            }
            Parametrizable::Variable(id) => format!("Variable({})", id.short_debug()),
        }
    }
}

// ✅ CONCISE: Delegate to inner types for clean mathematical notation
impl ShortDebug for MathExpression {
    fn short_debug(&self) -> String {
        match self {
            MathExpression::Object(obj) => obj.short_debug(),
            MathExpression::Relation(rel) => rel.short_debug(),
            MathExpression::Expression(expr) => expr.short_debug(),
            MathExpression::Number(n) => n.short_debug(),
            MathExpression::ViewAs { expression, .. } => expression.short_debug(),
        }
    }
}

impl ShortDebug for MathObject {
    fn short_debug(&self) -> String {
        match self {
            MathObject::Group(g) => {
                format!("MathObject::Group(\n{}\n)", indent(&g.short_debug(), 1))
            }
            MathObject::Ring(_) => "MathObject::Ring".to_string(),
            MathObject::Field(_) => "MathObject::Field".to_string(),
            MathObject::Set(s) => format!("MathObject::Set(\n{}\n)", indent(&s.short_debug(), 1)),
            _ => "MathObject::Other".to_string(),
        }
    }
}

// Show relation structure clearly
impl ShortDebug for MathRelation {
    fn short_debug(&self) -> String {
        match self {
            MathRelation::Equal { left, right } => {
                format!(
                    "MathRelation::Equal {{\n  left: {},\n  right: {}\n}}",
                    indent(&left.short_debug(), 1),
                    indent(&right.short_debug(), 1)
                )
            }
            MathRelation::And(rels) => {
                let rel_strs: Vec<String> = rels
                    .iter()
                    .enumerate()
                    .map(|(i, r)| format!("[{}]: {}", i, indent(&r.short_debug(), 1)))
                    .collect();
                format!(
                    "MathRelation::And [\n{}\n]",
                    indent(&rel_strs.join(",\n"), 1)
                )
            }
            MathRelation::Or(rels) => {
                let rel_strs: Vec<String> = rels
                    .iter()
                    .enumerate()
                    .map(|(i, r)| format!("[{}]: {}", i, indent(&r.short_debug(), 1)))
                    .collect();
                format!(
                    "MathRelation::Or [\n{}\n]",
                    indent(&rel_strs.join(",\n"), 1)
                )
            }
            MathRelation::Not(rel) => {
                format!("MathRelation::Not(\n{}\n)", indent(&rel.short_debug(), 1))
            }
            MathRelation::Implies(left, right) => {
                format!(
                    "MathRelation::Implies {{\n  left: {},\n  right: {}\n}}",
                    indent(&left.short_debug(), 1),
                    indent(&right.short_debug(), 1)
                )
            }
            MathRelation::Equivalent(left, right) => {
                format!(
                    "MathRelation::Equivalent {{\n  left: {},\n  right: {}\n}}",
                    indent(&left.short_debug(), 1),
                    indent(&right.short_debug(), 1)
                )
            }
            MathRelation::True => "MathRelation::True".to_string(),
            MathRelation::False => "MathRelation::False".to_string(),
            _ => "MathRelation::Other".to_string(),
        }
    }
}

impl ShortDebug for TheoryExpression {
    fn short_debug(&self) -> String {
        match self {
            TheoryExpression::Group(g) => g.short_debug(),
            TheoryExpression::Ring(_) => "Ring".to_string(),
            TheoryExpression::Field(_) => "Field".to_string(),
        }
    }
}

// Show group type structure
impl ShortDebug for Group {
    fn short_debug(&self) -> String {
        match self {
            Group::Generic(g) => format!("Group::Generic(\n{}\n)", indent(&g.short_debug(), 1)),
            Group::Trivial(_) => "Group::Trivial".to_string(),
            Group::Cyclic(n) => format!("Group::Cyclic(\n{}\n)", indent(&n.short_debug(), 1)),
            Group::Dihedral(n) => format!("Group::Dihedral(\n{}\n)", indent(&n.short_debug(), 1)),
            Group::Free(gens) => format!("Group::Free(\n{}\n)", indent(&gens.short_debug(), 1)),
            Group::Symmetric(n) => format!("Group::Symmetric(\n{}\n)", indent(&n.short_debug(), 1)),
            Group::Alternating(n) => {
                format!("Group::Alternating(\n{}\n)", indent(&n.short_debug(), 1))
            }
            _ => "Group::Other".to_string(),
        }
    }
}

impl ShortDebug for GroupAction {
    fn short_debug(&self) -> String {
        todo!()
    }
}

impl ShortDebug for GenericGroup {
    fn short_debug(&self) -> String {
        format!(
            "GenericGroup {{\n  base_set: {}\n}}",
            indent(&self.base_set.short_debug(), 1)
        )
    }
}

impl ShortDebug for Set {
    fn short_debug(&self) -> String {
        match self {
            Set::Generic(g) => format!("Set::Generic(\n{}\n)", indent(&g.short_debug(), 1)),
            Set::Parametric { description, .. } => format!(
                "Set::Parametric {{ description: {} }}",
                description.short_debug()
            ),
            Set::Empty => "Set::Empty".to_string(),
            Set::Singleton { element, .. } => {
                format!("Set::Singleton {{ element: {} }}", element.short_debug())
            }
            _ => "Set::Other".to_string(),
        }
    }
}

// ✅ CONCISE: Use mathematical notation with short IDs for tracking
impl ShortDebug for GroupExpression {
    fn short_debug(&self) -> String {
        match self {
            GroupExpression::Element { element, .. } => match element {
                Some(e) => extract_variable_name(e),
                None => "elem".to_string(),
            },
            GroupExpression::Identity(_) => "e".to_string(),
            GroupExpression::Operation { left, right, .. } => {
                format!(
                    "({}*{})",
                    extract_variable_name(left),
                    extract_variable_name(right)
                )
            }
            GroupExpression::Inverse { element, .. } => {
                format!("{}⁻¹", extract_variable_name(element))
            }
            GroupExpression::Power { base, exponent, .. } => {
                format!(
                    "{}^{}",
                    extract_variable_name(base),
                    extract_variable_name(exponent)
                )
            }
            GroupExpression::Commutator { a, b, .. } => {
                format!(
                    "[{}, {}]",
                    extract_variable_name(a),
                    extract_variable_name(b)
                )
            }
            GroupExpression::GroupOrder { group } => {
                format!("|{}|", extract_variable_name(group))
            }
            GroupExpression::ElementOrder { element, .. } => {
                format!("ord({})", extract_variable_name(element))
            }
            _ => "GroupExpr".to_string(),
        }
    }
}

impl ShortDebug for GroupElement {
    fn short_debug(&self) -> String {
        match self {
            GroupElement::Integer(n) => format!("GroupElement::Integer({})", n),
            GroupElement::Symbol(s) => format!("GroupElement::Symbol({})", s.short_debug()),
            GroupElement::Permutation(p) => {
                if p.len() <= 4 {
                    format!("GroupElement::Permutation({:?})", p)
                } else {
                    format!("GroupElement::Permutation([{} elements])", p.len())
                }
            }
            GroupElement::Matrix(m) => format!(
                "GroupElement::Matrix({}×{})",
                m.len(),
                m.get(0).map_or(0, |row| row.len())
            ),
        }
    }
}

impl ShortDebug for GroupHomomorphism {
    fn short_debug(&self) -> String {
        format!(
            "GroupHomomorphism {{\n  domain: {},\n  codomain: {}\n}}",
            indent(&self.domain.short_debug(), 1),
            indent(&self.codomain.short_debug(), 1)
        )
    }
}

// PROOF DEBUGGING - Show structure clearly
impl ShortDebug for DefinitionState {
    fn short_debug(&self) -> String {
        match self {
            DefinitionState::Abstract => "DefinitionState::Abstract".to_string(),
            DefinitionState::Separate(expr) => format!(
                "DefinitionState::Separate(\n{}\n)",
                indent(&expr.short_debug(), 1)
            ),
            DefinitionState::Inlined => "DefinitionState::Inlined".to_string(),
            DefinitionState::ContainedInType => "DefinitionState::ContainedInType".to_string(),
        }
    }
}

impl ShortDebug for ContextEntry {
    fn short_debug(&self) -> String {
        // Show only essential mathematical content
        let ty_summary = match &self.ty.data {
            Parametrizable::Variable(id) => id.body.clone(),
            Parametrizable::Concrete(expr) => match expr.as_ref() {
                MathExpression::Object(obj) => match obj.as_ref() {
                    MathObject::Group(_) => "Group".to_string(),
                    MathObject::Set(_) => "Set".to_string(),
                    _ => "Object".to_string(),
                },
                MathExpression::Expression(theory_expr) => match theory_expr {
                    TheoryExpression::Group(group_expr) => match group_expr {
                        GroupExpression::Element { .. } => "GroupElement".to_string(),
                        GroupExpression::Identity(_) => "GroupIdentity".to_string(),
                        GroupExpression::Operation { .. } => "GroupOperation".to_string(),
                        _ => "GroupExpr".to_string(),
                    },
                    _ => "TheoryExpr".to_string(),
                },
                MathExpression::Relation(rel) => match rel.as_ref() {
                    MathRelation::Equal { left, right } => {
                        format!(
                            "{} = {}",
                            extract_variable_name(left),
                            extract_variable_name(right)
                        )
                    }
                    MathRelation::And(rels) => {
                        if rels.len() <= 2 {
                            let rel_summaries: Vec<String> = rels
                                .iter()
                                .map(|rel| match &rel.data {
                                    Parametrizable::Concrete(rel_arc) => match rel_arc.as_ref() {
                                        MathRelation::Equal { left, right } => {
                                            format!(
                                                "{} = {}",
                                                extract_variable_name(left),
                                                extract_variable_name(right)
                                            )
                                        }
                                        _ => "Rel".to_string(),
                                    },
                                    Parametrizable::Variable(id) => id.body.clone(),
                                })
                                .collect();
                            format!("({})", rel_summaries.join(") ∧ ("))
                        } else {
                            format!("And[{}]", rels.len())
                        }
                    }
                    _ => "Relation".to_string(),
                },
                _ => "MathExpr".to_string(),
            },
        };
        format!("{}: {}", self.name.body, ty_summary)
    }
}

impl ShortDebug for Quantifier {
    fn short_debug(&self) -> String {
        let q_type = match self.quantification {
            Quantification::Universal => "Universal",
            Quantification::Existential => "Existential",
            Quantification::UniqueExistential => "UniqueExistential",
        };
        format!(
            "Quantifier {{\n  quantification: {},\n  variable_name: {}\n}}",
            q_type,
            indent(&self.variable_name.short_debug(), 1)
        )
    }
}

// ProofGoal - show complete structure
impl ShortDebug for ProofGoal {
    fn short_debug(&self) -> String {
        let mut result = String::new();
        result.push_str("ProofGoal {\n");

        // Context
        if self.context.is_empty() {
            result.push_str("  context: [],\n");
        } else {
            result.push_str("  context: [\n");
            for (i, entry) in self.context.iter().enumerate() {
                result.push_str(&format!(
                    "    [{}]: {},\n",
                    i,
                    indent(&entry.short_debug(), 2)
                ));
            }
            result.push_str("  ],\n");
        }

        // Quantifiers
        if self.quantifiers.is_empty() {
            result.push_str("  quantifiers: [],\n");
        } else {
            result.push_str("  quantifiers: [\n");
            for (i, q) in self.quantifiers.iter().enumerate() {
                result.push_str(&format!("    [{}]: {},\n", i, indent(&q.short_debug(), 2)));
            }
            result.push_str("  ],\n");
        }

        // Statement - clean mathematical display with key IDs
        let statement_summary = match &self.statement.data {
            Parametrizable::Variable(id) => {
                format!("{} [{}]", id.body, short_id(&self.statement.id))
            }
            Parametrizable::Concrete(rel_arc) => match rel_arc.as_ref() {
                MathRelation::Equal { left, right } => {
                    let left_summary = extract_variable_name(left);
                    let right_summary = extract_variable_name(right);
                    format!(
                        "{} = {} [L:{}, R:{}]",
                        left_summary,
                        right_summary,
                        short_id(&left.id),
                        short_id(&right.id)
                    )
                }
                MathRelation::And(rels) => format!("And[{}]", rels.len()),
                MathRelation::Or(rels) => format!("Or[{}]", rels.len()),
                MathRelation::Implies(ant, cons) => {
                    let ant_summary = extract_variable_name(ant);
                    let cons_summary = extract_variable_name(cons);
                    format!("{} → {}", ant_summary, cons_summary)
                }
                _ => "Relation".to_string(),
            },
        };
        result.push_str(&format!("  statement: {}\n", statement_summary));

        result.push_str("}");
        result
    }
}

impl ShortDebug for SubgoalCombination {
    fn short_debug(&self) -> String {
        match self {
            SubgoalCombination::And => "SubgoalCombination::And".to_string(),
            SubgoalCombination::Or => "SubgoalCombination::Or".to_string(),
            SubgoalCombination::Custom(s) => {
                format!("SubgoalCombination::Custom({})", s.short_debug())
            }
        }
    }
}

// NodeRole - show complete structure
impl ShortDebug for NodeRole {
    fn short_debug(&self) -> String {
        match self {
            NodeRole::Goal(goal) => {
                format!("NodeRole::Goal(\n{}\n)", indent(&goal.short_debug(), 1))
            }
            NodeRole::SubgoalManager {
                subgoal_ids,
                combination_type,
            } => {
                format!(
                    "NodeRole::SubgoalManager {{\n  subgoal_ids: {:?},\n  combination_type: {}\n}}",
                    subgoal_ids
                        .iter()
                        .map(|id| short_id(id))
                        .collect::<Vec<_>>(),
                    indent(&combination_type.short_debug(), 1)
                )
            }
            NodeRole::AutomatedTacticStep {
                description,
                justification,
                best_node_id,
            } => {
                format!(
                    "NodeRole::AutomatedTacticStep {{\n  description: {},\n  best_node_id: {},\n  justification: {}\n}}",
                    indent(&description.short_debug(), 1),
                    short_id(best_node_id),
                    indent(&justification.short_debug(), 1)
                )
            }
            NodeRole::Disproved(theorem_id) => {
                format!("NodeRole::Disproved({})", short_id(theorem_id))
            }
            NodeRole::RewriteStep {
                goal,
                rewritten_from_id,
                rewritten_to_id,
            } => {
                format!(
                    "NodeRole::RewriteStep {{\n  rewritten_from_id: {},\n  rewritten_to_id: {},\n  goal: {}\n}}",
                    indent(&rewritten_from_id.short_debug(), 1),
                    indent(&rewritten_to_id.short_debug(), 1),
                    indent(&goal.short_debug(), 1)
                )
            }
            NodeRole::Completed => "NodeRole::Completed".to_string(),
        }
    }
}
