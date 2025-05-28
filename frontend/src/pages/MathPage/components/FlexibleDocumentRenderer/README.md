# FlexibleDocumentRenderer

A comprehensive React component system for rendering mathematical documents with ultra-flexible schema support and turn-math integration.

## Overview

The FlexibleDocumentRenderer is designed to support 14 different mathematical document types with dynamic rendering capabilities, rich text formatting, and sophisticated mathematical content rendering using the existing turn-math.tsx component.

## Architecture

### Core Components

1. **FlexibleDocumentRenderer** (`index.tsx`)
   - Main orchestrator component
   - Handles document structure (header, abstract, TOC, body, bibliography)
   - Manages document-level metadata and styling

2. **SectionRenderer** (`SectionRenderer.tsx`)
   - Renders individual document sections
   - Handles nested sections and hierarchical numbering
   - Manages section titles, content, and metadata

3. **SectionContentRenderer** (`SectionContentRenderer.tsx`)
   - Renders all types of section content
   - Supports paragraphs, math blocks, lists, tables, images, etc.
   - Integrates with layout renderers for complex layouts

4. **RichTextRenderer** (`RichTextRenderer.tsx`)
   - Handles rich text formatting with multiple segment types
   - Supports styled text, links, footnotes, inline code
   - Integrates with MathRenderer for inline mathematical content

5. **MathRenderer** (`MathRenderer.tsx`)
   - **Key Feature**: Integrates with turn-math.tsx for mathematical rendering
   - Automatically detects mathematical content patterns
   - Falls back to text rendering when turn-math fails
   - Supports both inline and block mathematical expressions

### Layout System

6. **LayoutRenderers** (`LayoutRenderers/index.tsx`)
   - **SideBySideRenderer**: Comparative layouts with configurable panels
   - **PanelLayoutRenderer**: Tabs, accordion, and grid layouts
   - **AnnotationOverlayRenderer**: Interactive annotations and tooltips
   - **InteractiveControlsRenderer**: Dynamic controls (sliders, dropdowns, etc.)

### Document Type Specialization

7. **DocumentTypeRenderers** (`DocumentTypeRenderers/index.tsx`)
   - Specialized wrappers for each of the 14 document types
   - Type-specific styling and behavior
   - Responsive design adaptations

## Supported Document Types

1. **ScientificPaper** - Academic papers with LaTeX-style formatting
2. **BlogPost** - Modern blog layout with author info and social features
3. **TooltipSummary** - Compact summaries for tooltips and overlays
4. **AnimatedPresentation** - Full-screen presentations with controls
5. **ResourcePanel** - Searchable resource databases with sidebar navigation
6. **WikiPage** - Wikipedia-style encyclopedic content
7. **Textbook** - Educational content with structured learning materials
8. **PersonalNotes** - Individual note-taking with informal styling
9. **MathematicianNotes** - Professional mathematical notation and proofs
10. **StudyNotes** - Student-friendly learning materials
11. **ComparisonPage** - Side-by-side content comparison
12. **TypeMappingDisplay** - Type theory mappings and transformations
13. **TransformationMapping** - Mathematical transformation visualizations
14. **InteractivePlayground** - Interactive mathematical exploration tools

## Turn-Math Integration

The MathRenderer component provides seamless integration with the existing turn-math.tsx system:

### Features
- **Automatic Detection**: Recognizes mathematical symbols, Greek letters, and notation patterns
- **Content Classification**: Distinguishes between mathematical and textual content
- **Fallback Rendering**: Gracefully handles parsing failures
- **Symbol Support**: Unicode mathematical symbols, Greek letters, operators

### Usage Example
```typescript
// Mathematical content is automatically detected and rendered with turn-math
{
  type: "Math",
  content: { 
    content: "∫ f(x) dx = F(x) + C", 
    display_style: false 
  }
}

// Results in turn-math rendering with proper mathematical typography
```

## Type System

The renderer uses a comprehensive TypeScript type system that mirrors Rust exports via ts_rs:

### Core Types
- `FlexibleDocument`: Main document structure
- `DocumentType`: Union of 14 document type variants
- `Section`: Document sections with content and metadata
- `SectionContentNode`: All possible content types
- `RichTextSegment`: Rich text formatting options
- `MathNode`: Mathematical content representation

### Layout Types
- `SideBySideLayout`: Comparative panel configurations
- `PanelLayout`: Multi-panel arrangements
- `AnnotationOverlay`: Interactive annotation systems
- `InteractiveControls`: Dynamic user controls

## Demo System

The included demo component (`demo.tsx`) provides:
- Interactive document type selector
- Live preview of all 14 document types
- Sample mathematical content showcasing turn-math integration
- Configuration display showing layout and audience settings

### Running the Demo
```bash
# Navigate to the component directory
cd frontend/src/pages/MathPage/components/FlexibleDocumentRenderer

# Import and use in your React application
import FlexibleDocumentDemo from './demo';
```

## CSS Architecture

### Modular Styling
- `styles.module.css`: Core document and content styling
- `LayoutRenderers/styles.module.css`: Layout-specific styles
- `DocumentTypeRenderers/styles.module.css`: Document type specializations
- `demo.module.css`: Demo interface styling

### Responsive Design
- Mobile-first approach
- Flexible grid systems
- Adaptive typography
- Touch-friendly interactive elements

## Best Practices

### Mathematical Content
1. Use Unicode mathematical symbols when possible
2. Provide fallback text for complex expressions
3. Test with both inline and block mathematical content
4. Leverage turn-math's existing rendering capabilities

### Document Structure
1. Design sections hierarchically
2. Use metadata for enhanced functionality
3. Implement proper accessibility attributes
4. Consider target audience in presentation configuration

### Performance
1. Lazy load complex mathematical content
2. Implement content virtualization for long documents
3. Cache parsed mathematical expressions
4. Optimize images and media content

## Extension Points

### Adding New Document Types
1. Add type to `DocumentType` union in `types.ts`
2. Create specialized wrapper in `DocumentTypeRenderers`
3. Add CSS styling for the new type
4. Update demo with sample content

### Custom Content Types
1. Extend `SectionContentNode` union
2. Add renderer in `SectionContentRenderer.tsx`
3. Implement type-specific styling
4. Add TypeScript type definitions

### Mathematical Extensions
1. Enhance `MathRenderer` with additional LaTeX patterns
2. Add custom mathematical notation support
3. Integrate with external math libraries
4. Implement interactive mathematical elements

## Integration Notes

This component system is designed to work seamlessly with:
- Existing turn-math.tsx mathematical rendering
- Rust backend via ts_rs type exports
- Modern React applications with TypeScript
- CSS modules and responsive design patterns

The architecture provides maximum flexibility while maintaining type safety and performance optimization for mathematical document rendering scenarios. 