# Binding-Based Component Architecture

This directory contains React components that directly use the auto-generated TypeScript bindings from Rust.

## Design Principles

1. **Direct Type Mapping**: Each component directly accepts the exact binding type as props
2. **No Custom Types**: Components use only the auto-generated binding types  
3. **Service Integration**: Components receive data processed by services using binding types
4. **Type Safety**: Full end-to-end type safety from Rust → Bindings → Components

## Component Structure

```
binding-renderers/
├── core/                          # Core rendering components
│   ├── MathNodeRenderer.tsx       # Renders MathNode binding type
│   ├── SectionRenderer.tsx        # Renders Section binding type  
│   ├── DocumentRenderer.tsx       # Renders MathDocument binding type
│   └── RichTextRenderer.tsx       # Renders RichTextSegment binding type
├── content/                       # Content-specific renderers
│   ├── MathContentRenderers.tsx   # Math-specific content variants
│   ├── SectionContentRenderers.tsx # Section content variants
│   └── StructuredMathRenderers.tsx # Structured math content
├── layouts/                       # Layout components
│   ├── PanelLayoutRenderer.tsx    # Panel layouts
│   ├── GridLayoutRenderer.tsx     # Grid layouts
│   └── SideBySideRenderer.tsx     # Side-by-side layouts
└── specialized/                   # Theory-specific renderers
    ├── GroupTheoryRenderer.tsx    # Group theory specific
    ├── TopologyRenderer.tsx       # Topology specific
    └── SetTheoryRenderer.tsx      # Set theory specific
```

## Usage Pattern

```typescript
import { MathNodeRenderer } from './binding-renderers/core/MathNodeRenderer';
import type { MathNode } from '../turn-render/bindings/MathNode';

// Service provides exact binding type
const mathNode: MathNode = await mathContentService.getMathNode(id);

// Component directly accepts binding type
<MathNodeRenderer node={mathNode} />
```

## Benefits

- **Type Safety**: Compile-time guarantee that components match Rust types
- **Auto-Updates**: When Rust types change, TypeScript compiler catches mismatches
- **No Duplication**: Single source of truth for type definitions
- **Service Integration**: Seamless flow from services to components 