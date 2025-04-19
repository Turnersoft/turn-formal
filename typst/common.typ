#let project-title = "Intro to Turn-Lang's formal library - Turn-Formal"
#let project-subtitle = "A path to formalize critical subjects (in Rust)"
#let project-author = "Turner, Creator of turn-lang.com"

// Common colors
#let turn-blue = rgb("#1a73e8")
#let turn-red = rgb("#ea4335")
#let turn-green = rgb("#34a853")
#let turn-yellow = rgb("#fbbc05")

// Code examples
#let proof-branch-example = ```rust
let state = ProofState::new();
let branch1 = state
    .tactics_intro_expr("a", MathExpression::Var(Identifier::E(1)), 0)
    .tactics_intro_expr("b", MathExpression::Var(Identifier::E(2)), 1);

// Add some proof steps
let p1 = branch1.tactics_intro_expr("a", create_var("a"), 1);
let p2 = p1.tactics_intro_expr("b", create_var("b"), 2);

// Mark as complete
let p3 = p2.should_complete();
```

#let theorem-example = ```rust
// Prove that in a group, inverses are unique
pub fn prove_inverse_uniqueness() -> Theorem {
    // Create a group structure for our proof
    let group = create_abstract_group();

    // Create element variables
    let g_var = create_element_variable(&group, "g", 1);
    let h1_var = create_element_variable(&group, "h1", 2);
    let h2_var = create_element_variable(&group, "h2", 3);
    let e_var = GroupExpression::identity(group.clone());

    // Create relations for our proof
    let relation1 = group_operation_equals(&group, &g_var, &h1_var, &e_var);
    let relation2 = group_operation_equals(&group, &g_var, &h2_var, &e_var);

    // Create the theorem statement: if g*h1 = e and g*h2 = e, then h1 = h2
    let theorem_statement = MathRelation::Implies(
        Box::new(MathRelation::And(vec![
            relation1.clone(),
            relation2.clone(),
        ])),
        Box::new(MathRelation::equal(
            h1_var.to_math_expression(),
            h2_var.to_math_expression(),
        )),
    );

    // Build the proof
    let builder = TheoremBuilder::new("Group Inverse Uniqueness", theorem_statement, vec![]);
    
    // Initial branch
    let p0 = builder.initial_branch();
    
    // ... proof steps ...
    
    // Build the theorem
    builder.build()
}
```

// Section content
#let section1-content = [
  Formal systems provide a foundation for rigorous mathematical reasoning with machine-checkable proofs.
  
  *Key characteristics of formal systems:*
  - Precise, unambiguous language and syntax
  - Well-defined rules of inference
  - Mechanically verifiable proofs
  - Foundation for automated theorem proving

  *Why formalization matters:*
  - Eliminates ambiguity in mathematical proofs
  - Enables machine verification of correctness
  - Facilitates the development of verified software
  - Provides a basis for advanced AI reasoning systems
  - Bridges the gap between mathematical theory and practical applications

  *Applications of formalization span numerous domains:*
  - Verifying correctness of cryptographic protocols
  - Ensuring safety-critical systems meet specifications
  - Developing certified compilers and software
  - Advancing mathematical knowledge through verified proofs
  - Creating foundations for AI reasoning and decision-making
]

#let section2-content = [
  Traditional formal systems often operate at a low level of abstraction, making them
  challenging to use for everyday mathematical practice.

  *High-level formal mathematics aims to:*
  - Match the intuition and workflow of human mathematicians
  - Abstract away mechanical details while maintaining rigor
  - Provide a natural language-like experience for formal proof development
  - Enable mathematicians to work at their level of conceptual understanding
  - Bridge the gap between informal mathematical practice and formal verification

  *The abstraction gap:*
  - Traditional formal systems: Detailed, granular steps that are machine-friendly but human-hostile
  - Informal mathematics: Intuitive leaps that are human-friendly but machine-hostile
  - High-level formal systems: The ideal middle ground that serves both humans and machines
]

#let section3-content = [
  Turn-Formal, implemented in Rust, offers significant advantages over existing systems like Lean4:

  *Key advantages:*
  - *Performance*: Rust's speed and memory safety combine rigor with efficiency
  - *Accessibility*: Domain-specific language makes formal mathematics more approachable
  - *Integration*: Seamless interoperability with the broader Rust ecosystem
  - *Modularity*: Flexible architecture for various mathematical domains
  - *Expressiveness*: Rich syntax for intuitive theorem statements and proofs

  *Comparison with Lean4:*
  - More intuitive syntax for common mathematical constructs
  - Stronger performance characteristics for large-scale formalization
  - Better integration with production programming environments
  - Focus on developer experience and practical applications
  - Built-in support for domain-specific mathematical theories
]

#let section4-content = [
  Turn-Formal is designed with developers in mind, making formal verification accessible to software engineers.

  *Developer-friendly features:*
  - Familiar Rust syntax and semantics
  - Strong type system that catches errors early
  - Flexible tactics system that can be extended for specific domains
  - Clear, chainable API for proof construction
  - Comprehensive documentation and examples

  *Core components:*
  - *ProofState*: Represents the current state in a formal proof
  - *Tactics*: Operations that transform proof states (like introduction, substitution)
  - *TheoremBuilder*: Constructs formal theorems with structured proofs
  - *ProofBranch*: Manages different paths in a proof exploration
  - *MathRelation*: Represents mathematical relationships
]

#let section5-content = [
  The roadmap for Turn-Formal focuses on expanding capabilities and accessibility:

  *Short-term goals:*
  - Complete foundational mathematics library
  - Enhance tactic system with machine learning suggestions
  - Improve proof visualization and exploration tools
  - Develop integrations with common IDEs

  *Medium-term goals:*
  - Build domain-specific libraries for cryptography, distributed systems
  - Create automated proof search capabilities
  - Develop translation layers for interoperability with other proof assistants
  - Establish a package ecosystem for community contributions

  *Long-term vision:*
  - Formal verification as a standard part of software development
  - Bridge between theorem provers and mainstream programming
  - Accessible formal methods for non-specialists
  - Unified platform for mathematical formalization across disciplines
] 