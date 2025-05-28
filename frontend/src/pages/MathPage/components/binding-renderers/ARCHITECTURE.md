# Binding-Based Component Architecture

## Overview

This directory implements a new React component architecture that directly uses auto-generated TypeScript bindings from Rust. This eliminates custom type definitions and ensures perfect type alignment between the backend and frontend.

## Key Components

### Core Renderers

#### `MathNodeRenderer`
- **Purpose**: Renders mathematical expressions using the `MathNode` binding type
- **Input**: `MathNode` from Rust bindings  
- **Features**: Handles all `MathNodeContent` variants (Identifier, Power, Bracketed, Matrix, etc.)
- **Styling**: CSS modules with mathematical typography

#### `DocumentRenderer`  
- **Purpose**: Renders full mathematical documents using `MathematicalContent` binding type
- **Input**: `MathematicalContent` from Rust bindings
- **Features**: Handles ScientificPaper, WikiPage, Textbook content types
- **Integration**: Uses `MathNodeRenderer` for embedded mathematics

#### `RichTextRenderer`
- **Purpose**: Renders rich text segments (placeholder implementation)
- **Future**: Will handle full RichTextSegment binding types

### Example Components

#### `UsageExample`
- **Purpose**: Demonstrates the complete binding-based workflow
- **Features**: Shows both individual MathNode and full document rendering
- **Benefits**: Illustrates type safety and binding integration

## Architecture Benefits

### 1. Type Safety
```typescript
// Guaranteed type alignment - compiler catches mismatches
const node: MathNode = await service.getMathNode(id);
<MathNodeRenderer node={node} />  // ✅ Type safe
```

### 2. Auto-Updates
- Rust type changes automatically propagate to TypeScript
- Compiler errors highlight required component updates
- No manual type synchronization needed

### 3. Zero Runtime Transformation
```typescript
// Service returns exact binding type - no conversion needed
const content = await mathService.getDocument(id);
return <DocumentRenderer content={content} />;
```

### 4. Single Source of Truth
- All types defined in Rust
- TypeScript bindings auto-generated via ts-rs
- Components use bindings directly

## Implementation Patterns

### Union Type Handling
```typescript
// Pattern for handling Rust enum variants in TypeScript
if (typeof content === 'object' && content !== null && 'Power' in content) {
  const { base, exponent } = content.Power;
  return <PowerRenderer base={base} exponent={exponent} />;
}
```

### Binding Type Structure
```typescript
// MathNode binding structure
type MathNode = {
  id: string;
  content: MathNodeContent;  // Union of all content variants
};

// Component directly accepts this type
interface MathNodeRendererProps {
  node: MathNode;  // Exact binding type
  inline?: boolean;
}
```

### CSS Module Organization
```css
/* MathNodeRenderer.module.css */
.mathNode { /* Base math styling */ }
.identifier { /* Variable styling */ }
.operator { /* Operator styling */ }
.relationship { /* Relation styling */ }
```

## Integration with Services

### Service Layer
```typescript
// Services provide exact binding types
interface MathContentService {
  getMathNode(id: string): Promise<MathNode>;
  getDocument(id: string): Promise<MathematicalContent>;
  parseExport(json: string): Promise<ParsedContent>;
}
```

### Component Usage
```typescript
// Clean service-to-component flow
function MathPage({ documentId }: { documentId: string }) {
  const [content, setContent] = useState<MathematicalContent | null>(null);
  
  useEffect(() => {
    mathService.getDocument(documentId).then(setContent);
  }, [documentId]);
  
  return content ? <DocumentRenderer content={content} /> : <Loading />;
}
```

## File Structure

```
binding-renderers/
├── core/                          # Core rendering components
│   ├── MathNodeRenderer.tsx       # Math expression renderer
│   ├── MathNodeRenderer.module.css
│   ├── DocumentRenderer.tsx       # Document renderer
│   ├── DocumentRenderer.module.css  
│   └── RichTextRenderer.tsx       # Rich text renderer
├── examples/                      # Usage examples
│   ├── UsageExample.tsx          # Complete workflow demo
│   └── UsageExample.module.css
├── README.md                      # Architecture overview
├── ARCHITECTURE.md               # This file
└── index.ts                       # Main exports
```

## Migration Strategy

### Phase 1: Core Components ✅
- ✅ MathNodeRenderer with binding types
- ✅ Basic DocumentRenderer structure  
- ✅ Example usage patterns

### Phase 2: Full Coverage
- [ ] Complete all MathNodeContent variants
- [ ] Full SectionContentNode support
- [ ] Rich text segment rendering
- [ ] Interactive features (selection, editing)

### Phase 3: Service Integration  
- [ ] Update services to return binding types
- [ ] Migrate existing components
- [ ] Performance optimization

## Development Guidelines

### Adding New Variants
1. Check if binding type exists in `turn-render/bindings/`
2. Add type check pattern: `'VariantName' in content`
3. Extract data: `const data = (content as any).VariantName`
4. Implement rendering logic
5. Add CSS styling

### Type Assertions
```typescript
// Use type assertions for union type access
const { base, exponent } = (content as any).Power;

// Avoid if binding types are well-defined
const powerContent = content as { Power: { base: MathNode; exponent: MathNode } };
```

### Performance Considerations
- Components are lightweight (direct binding usage)
- CSS modules provide scoped styling
- Recursive rendering handled efficiently
- No runtime type transformations

## Future Enhancements

1. **Interactive Math**: Selection, editing, transformation
2. **Accessibility**: ARIA labels, screen reader support  
3. **Export Features**: PDF, LaTeX generation
4. **Performance**: Virtual scrolling for large documents
5. **Theming**: Math styling customization

## Conclusion

This binding-based architecture provides a robust foundation for mathematical content rendering with guaranteed type safety, automatic updates, and clean separation of concerns. The direct use of Rust binding types eliminates many common frontend/backend integration issues and provides a maintainable, scalable solution. 