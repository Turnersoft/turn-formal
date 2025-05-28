// Core binding renderers - pure Rust type rendering
export { DocumentRenderer } from './core/DocumentRenderer';
export { SectionContentRenderer } from './core/SectionContentRenderer';
export { DataNavigationSidebar } from './core/DataNavigationSidebar';

// Usage examples demonstrating the binding renderer system  
export { UsageExample } from './examples/UsageExample';

// Re-export binding types for convenience
export type { MathNode } from '../turn-render/bindings/MathNode';
export type { MathNodeContent } from '../turn-render/bindings/MathNodeContent';
export type { MathematicalContent } from '../turn-render/bindings/MathematicalContent';
export type { BracketStyle } from '../turn-render/bindings/BracketStyle';
export type { BracketSize } from '../turn-render/bindings/BracketSize';
export type { RelationOperatorNode } from '../turn-render/bindings/RelationOperatorNode';

/**
 * Binding-Based Component Architecture
 * 
 * This module provides React components that directly use auto-generated
 * TypeScript bindings from Rust. This ensures type safety and eliminates
 * the need for custom type definitions.
 * 
 * Key Benefits:
 * - Direct type mapping from Rust to React
 * - Automatic updates when Rust types change  
 * - No runtime type transformations
 * - Full end-to-end type safety
 * 
 * Usage:
 * ```typescript
 * import { MathNodeRenderer, DocumentRenderer } from '@/components/binding-renderers';
 * import type { MathNode, MathematicalContent } from '@/components/binding-renderers';
 * 
 * // Service provides exact binding type
 * const mathNode: MathNode = await mathService.getMathNode(id);
 * const document: MathematicalContent = await mathService.getDocument(id);
 * 
 * // Components directly accept binding types
 * <MathNodeRenderer node={mathNode} />
 * <DocumentRenderer content={document} />
 * ```
 */ 