# Protocol for Rendering High-Level Formal Mathematics

## 1. Objective

The primary objective of this protocol is to define a standard way to render formal mathematical statements and their proofs that is rich, interactive, and transparent. Unlike traditional informal mathematics published in text, this rendering should expose the underlying formal structure, including:

*   The precise statement of theorems.
*   The full context (axioms, definitions, hypotheses).
*   The sequence of tactics or proof steps applied.
*   The evolution of the proof state after each step.
*   Clear connections between these components, facilitated by interactivity.

This aims to make formal proofs more auditable, understandable, and educational.

## 2. Core Components to Render

A complete rendering of a formal proof under this protocol should include the following components:

### 2.1. Theorem Statement
*   **Content:** The mathematical proposition being asserted.
*   **Presentation:** Rendered using a standard math notation (e.g., LaTeX, MathML) for readability.
*   **Identifiers:** Key entities (variables, constants, functions, predicates) should be uniquely identifiable for linking and highlighting.

### 2.2. Proof Context
The context in which the theorem is proven. This can be divided into:

*   **Global Context (Optional, can be linked):**
    *   Relevant imported theories, axioms, and foundational definitions assumed.
    *   These might be collapsed by default but expandable or linkable.
*   **Local Context (Initial Proof State):**
    *   **Variables & Types:** All free variables in the theorem statement and their formally defined types.
    *   **Hypotheses/Assumptions:** Any conditions or premises under which the theorem holds.
    *   **Goal(s):** The initial statement(s) to be proven, derived from the main theorem statement.

### 2.3. Proof Steps
A sequential display of the steps taken to prove the theorem. Each step should consist of:

*   **Step Identifier:** A unique way to refer to this step (e.g., step number).
*   **Tactic Applied:**
    *   The name or a clear description of the formal tactic or inference rule used (e.g., `apply_lemma "lemma_X"`, `rewrite_using "eq_H1"`, `induction_on "n"`).
    *   Parameters or specific instantiations used with the tactic should be visible.
*   **Proof State *Before* Tactic (Optional but Recommended):**
    *   A snapshot of the active goals and relevant hypotheses immediately before this tactic was applied. This helps in understanding the tactic's effect.
*   **Proof State *After* Tactic:**
    *   The new set of goals (if any).
    *   Newly introduced hypotheses or local definitions.
    *   Modifications to existing hypotheses (e.g., a hypothesis being consumed or altered).
    *   This state should clearly show the transformation from the previous state.

### 2.4. Final Proof State
*   Indication of proof completion (e.g., "QED", "No goals remaining").

## 3. Interactivity

To enhance understanding and navigation, the rendered output should be interactive.

### 3.1. Highlighting and Linking
*   **Mouse Hover Effects:** When the user hovers over certain elements, related elements should be highlighted. This creates visual connections between:
    *   A **term** in a proof state (goal or hypothesis) and its **definition** or declaration in the context.
    *   A **hypothesis** and the **tactic step(s)** that used or introduced it.
    *   A **tactic** and the specific **hypotheses/goals** it operated on.
    *   A **term/hypothesis/goal** in one proof state and its corresponding (potentially transformed) version in the subsequent proof state.
    *   Parts of an expression targeted by a tactic (e.g., the sub-expression being rewritten).
*   **Click to Navigate (Optional):** Clicking on a definition could take the user to its full specification.

### 3.2. Collapsibility
*   For long proofs or extensive contexts, sections (e.g., global context, individual proof steps, detailed proof states) should be collapsible/expandable to manage information density.

## 4. Data Model Considerations (Conceptual Backend)

To support this rendering protocol, the underlying formal system should be able to provide proof data in a structured format. This might involve:

*   A **Proof Trace:** A list of (tactic_info, state_before, state_after) tuples.
*   **Unique Identifiers:** Stable IDs or referenceable paths for all mathematical objects, expressions, sub-expressions, hypotheses, and goals to enable robust linking for highlighting.
*   **Typed Entities:** All rendered terms should have accessible type information.
*   **Source Mapping (Optional):** Links back to the formal source code that generated the proof.

## 5. Rendering Technology

*   **Primary Format:** HTML for structure.
*   **Mathematics:** MathJax, KaTeX (for LaTeX rendering), or native MathML.
*   **Styling:** CSS for visual presentation and clear distinction between components.
*   **Interactivity:** JavaScript for highlighting, collapsibility, and other dynamic behaviors.

## 6. Styling Guidelines (Brief)

*   **Clarity:** Use distinct visual styles (e.g., background colors, borders, typography) for the theorem statement, context sections, tactic applications, and proof states.
*   **Consistency:** Apply highlighting effects (e.g., background change, bolding, underlining) consistently.
*   **Readability:** Prioritize clear fonts, sufficient spacing, and logical layout.
*   **Responsiveness (Optional):** Ensure the rendering is usable on various screen sizes.

This protocol provides a framework. Specific implementations may adapt or extend these guidelines based on the capabilities of the formal system and the desired user experience. 