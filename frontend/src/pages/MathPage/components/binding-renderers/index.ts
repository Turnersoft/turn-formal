// Core binding renderers - pure Rust type rendering
export { DocumentRenderer } from './core/DocumentRenderer';
export { SectionContentRenderer } from './core/SectionContentRenderer';
export { DataNavigationSidebar } from './core/DataNavigationSidebar';
export { RichTextRenderer } from './core/RichTextRenderer';
export { LinkRenderer } from './core/LinkRenderer';

// Usage examples demonstrating the binding renderer system  
export { UsageExample } from './examples/UsageExample';
export { LinkTest } from './examples/LinkTest';
export { GroupTheoryLinkTest } from './examples/GroupTheoryLinkTest';
export { RoutingTest } from './examples/RoutingTest';

// Services
export { MathNavigationService, useMathNavigation, useMathNavigationState } from '../../services/mathNavigationService';
export type { DefinitionNavigation, TheoremNavigation, MathNavigationTarget } from '../../services/mathNavigationService';

// Re-export binding types for convenience
export type { MathNode } from '../turn-render/bindings/MathNode';
export type { MathNodeContent } from '../turn-render/bindings/MathNodeContent';
export type { MathematicalContent } from '../turn-render/bindings/MathematicalContent';
export type { BracketStyle } from '../turn-render/bindings/BracketStyle';
export type { BracketSize } from '../turn-render/bindings/BracketSize';
export type { RelationOperatorNode } from '../turn-render/bindings/RelationOperatorNode';

// Link-related binding types
export type { RichTextSegment } from '../turn-render/bindings/RichTextSegment';
export type { LinkTarget } from '../turn-render/bindings/LinkTarget';
export type { TextStyle } from '../turn-render/bindings/TextStyle';

/**
 * Binding-Based Component Architecture with Routing
 * 
 * This module provides React components that directly use auto-generated
 * TypeScript bindings from Rust, along with a comprehensive routing system
 * for mathematical content navigation.
 * 
 * Key Features:
 * - Direct type mapping from Rust to React
 * - URL-based navigation to specific definitions and theorems
 * - Automatic content loading based on route parameters
 * - Cross-theory linking and navigation
 * - Full end-to-end type safety
 * 
 * Routing System:
 * - /math/definition/{theory}/{termId} - Navigate to specific definitions
 * - /math/theorem/{theory}/{theoremId} - Navigate to specific theorems
 * - /math/theory/{theoryName} - Navigate to theory overviews
 * 
 * Usage:
 * ```typescript
 * import { 
 *   MathNodeRenderer, 
 *   DocumentRenderer, 
 *   useMathNavigation 
 * } from '@/components/binding-renderers';
 * 
 * const navigation = useMathNavigation();
 * 
 * // Navigate to a specific definition
 * navigation.navigateToDefinition({
 *   term_id: 'group_definition',
 *   theory_context: 'GroupTheory'
 * });
 * 
 * // Navigate to a theorem
 * navigation.navigateToTheorem({
 *   theorem_id: 'inverse_uniqueness',
 *   theory_context: 'GroupTheory'
 * });
 * ```
 */ 