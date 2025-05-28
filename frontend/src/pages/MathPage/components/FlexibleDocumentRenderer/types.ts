// TypeScript types that correspond to the Rust types exported by ts_rs
// These should match the exports from section_node.rs

export interface FlexibleDocument {
  id: string;
  title: string;
  document_type: DocumentType;
  presentation_config?: PresentationConfig;
  language?: string;
  version?: string;
  authors?: string[];
  date_published?: string;
  date_modified?: string;
  abstract_content?: Section;
  table_of_contents?: TocNode;
  body: Section[];
  footnotes?: Section[];
  glossary?: Section[];
  bibliography: BibEntry[];
}

export type DocumentType = 
  | "ScientificPaper"
  | "Textbook" 
  | "WikiPage"
  | "PersonalNotes"
  | "BlogPost"
  | "TooltipSummary"
  | "AnimatedPresentation"
  | "TypeMappingDisplay"
  | "ComparisonPage"
  | "TransformationMapping"
  | "InteractivePlayground"
  | "MathematicianNotes"
  | "StudyNotes"
  | "ResourcePanel";

export interface PresentationConfig {
  layout_style: LayoutStyle;
  interaction_features: InteractionFeature[];
  target_audience: AudienceLevel;
  formality_level: FormalityLevel;
  animation_config?: AnimationConfig;
}

export type LayoutStyle = 
  | "SingleColumn"
  | "TwoColumn"
  | "MultiPanel"
  | "Sidebar"
  | "Dashboard"
  | "Presentation"
  | "Compact";

export type InteractionFeature =
  | "ClickableLinks"
  | "HoverTooltips"
  | "ExpandableProofs"
  | "InteractiveControls"
  | "Animations"
  | "TypeAnnotations"
  | "HighlightCorrespondence"
  | "ParameterAdjustment";

export type AudienceLevel =
  | "Expert"
  | "Graduate"
  | "Undergraduate"
  | "HighSchool"
  | "GeneralPublic"
  | "Mathematician"
  | "Student";

export type FormalityLevel =
  | "FullyFormal"
  | "SemiFormal"
  | "Intuitive"
  | "Conversational"
  | "Sketchy";

export interface AnimationConfig {
  enabled_animations: string[];
  animation_speed: number;
  auto_play?: boolean;
  show_controls?: boolean;
}

export interface Section {
  id: string;
  title?: ParagraphNode;
  content: SectionContentNode[];
  metadata?: [string, string][];
  display_options?: SectionDisplayOptions;
}

export interface SectionDisplayOptions {
  show_title_numbering?: boolean;
}

export interface ParagraphNode {
  segments: RichTextSegment[];
  alignment?: TextAlignment;
}

export type TextAlignment = "Left" | "Center" | "Right" | "Justify";

export type RichTextSegment = 
  | { type: "Text"; content: string }
  | { type: "StyledText"; text: string; styles: TextStyle[] }
  | { type: "Math"; content: MathNode }
  | { type: "Link"; content: RichTextSegment[]; target: LinkTarget; tooltip?: string }
  | { type: "FootnoteReference"; content: string }
  | { type: "CodeInline"; content: string };

export type TextStyle =
  | "Bold"
  | "Italic"
  | "Underline"
  | "Strikethrough"
  | "Superscript"
  | "Subscript"
  | { type: "Color"; content: string }
  | { type: "BackgroundColor"; content: string }
  | { type: "FontSize"; content: string }
  | { type: "FontFamily"; content: string };

export type LinkTarget = 
  | { type: "Url"; content: string }
  | { type: "InternalPageId"; content: string }
  | { type: "DefinitionId"; term_id: string; theory_context?: string }
  | { type: "DefinitionAspect"; term_id: string; aspect_id: string; theory_context?: string }
  | { type: "TheoremId"; content: string }
  | { type: "ObjectConstructorTemplate"; template_id: string; parameters?: [string, MathNode][]; target_abstraction_level?: number }
  | { type: "GlossaryTerm"; content: string }
  | { type: "BibliographyKey"; content: string }
  | { type: "InteractiveElementId"; content: string }
  | { type: "TooltipDocument"; content: FlexibleDocument }
  | { type: "AnimationTrigger"; animation_id: string; trigger_type: AnimationTriggerType };

export type AnimationTriggerType = "Click" | "Hover" | "Toggle" | "Sequence";

export type SectionContentNode =
  | { type: "Paragraph"; content: ParagraphNode }
  | { type: "MathBlock"; math: MathNode; label?: string; caption?: ParagraphNode }
  | { type: "StructuredMath"; content: StructuredMathContentNode }
  | { type: "List"; content: ListNode }
  | { type: "Table"; content: TableNode }
  | { type: "CodeBlock"; content: CodeBlockNode }
  | { type: "Image"; content: ImageNode }
  | { type: "InteractiveDiagram"; content: InteractiveDiagramNode }
  | { type: "CollapsibleBlock"; content: CollapsibleBlockNode }
  | { type: "Grid"; content: GridNode }
  | { type: "Columns"; content: ColumnsNode }
  | { type: "ThematicBreak"; content: ThematicBreakNode }
  | { type: "QuoteBlock"; content: { content: ParagraphNode[]; attribution?: ParagraphNode } }
  | { type: "AlertBox"; style: AlertBoxStyle; content: SectionContentNode[] }
  | { type: "CustomComponent"; component_name: string; props?: string; fallback_content?: SectionContentNode[] }
  | { type: "EmbeddedSectionRef"; content: string }
  | { type: "SubSection"; content: Section }
  | { type: "SideBySideLayout"; content: SideBySideLayout }
  | { type: "PanelLayout"; content: PanelLayout }
  | { type: "AnnotationOverlay"; content: AnnotationOverlay }
  | { type: "InteractiveControls"; content: InteractiveControls }
  | { type: "EmbeddedDocument"; content: FlexibleDocument };

// Additional types referenced above
export interface MathNode {
  // This should match your math_node.rs exports
  content: string;
  display_style?: boolean;
}

export interface TocNode {
  title: string;
  target_id: string;
  children: TocNode[];
}

export interface BibEntry {
  entry_type: string;
  fields: [string, string][];
}

export interface StructuredMathContentNode {
  // Define based on your Rust enum
}

export interface ListNode {
  items: ListItemNode[];
  style: ListStyle;
  start_index?: number;
}

export interface ListItemNode {
  content: SectionContentNode[];
}

export type ListStyle = 
  | { type: "Unordered"; content: UnorderedListStyle }
  | { type: "Ordered"; content: OrderedListStyle };

export type UnorderedListStyle = "Disc" | "Circle" | "Square" | "None";
export type OrderedListStyle = "Decimal" | "AlphaLower" | "AlphaUpper" | "RomanLower" | "RomanUpper";

export interface TableNode {
  caption?: ParagraphNode;
  header_rows?: TableRowNode[];
  body_rows: TableRowNode[];
  footer_rows?: TableRowNode[];
  column_styles?: ColumnStyle[];
  table_style_options?: TableStyleOptions;
}

export interface TableRowNode {
  cells: TableCellNode[];
}

export interface TableCellNode {
  content: SectionContentNode[];
  col_span?: number;
  row_span?: number;
  cell_type: TableCellType;
  alignment?: TextAlignment;
}

export type TableCellType = "Header" | "Data";

export interface ColumnStyle {
  width?: string;
  alignment?: TextAlignment;
}

export interface TableStyleOptions {
  borders?: boolean;
  striped_rows?: boolean;
}

export interface CodeBlockNode {
  code: string;
  language?: string;
  caption?: ParagraphNode;
  show_line_numbers?: boolean;
  highlight_lines?: number[];
  is_executable?: boolean;
}

export interface ImageNode {
  src: string;
  alt_text?: string;
  caption?: ParagraphNode;
  width?: string;
  height?: string;
  alignment?: HorizontalAlignment;
}

export type HorizontalAlignment = "Left" | "Center" | "Right";

export interface InteractiveDiagramNode {
  diagram_type_id: string;
  data: string;
  caption?: ParagraphNode;
  config_options?: string;
}

export interface CollapsibleBlockNode {
  summary: RichTextSegment[];
  details: SectionContentNode[];
  initially_collapsed?: boolean;
}

export interface GridNode {
  items: GridItemNode[];
  column_template: string;
  row_gap?: string;
  column_gap?: string;
}

export interface GridItemNode {
  content: SectionContentNode;
  col_start?: number;
  col_end?: number;
  row_start?: number;
  row_end?: number;
}

export interface ColumnsNode {
  columns_content: SectionContentNode[][];
  column_widths?: string[];
  gap?: string;
}

export interface ThematicBreakNode {}

export type AlertBoxStyle = "Information" | "Success" | "Warning" | "Error" | "Note" | "Tip";

// Enhanced layout types
export interface SideBySideLayout {
  left_panel: Panel;
  right_panel: Panel;
  sync_scrolling?: boolean;
  highlight_correspondence?: boolean;
  layout_config?: SideBySideConfig;
}

export interface SideBySideConfig {
  left_width?: string;
  right_width?: string;
  gap?: string;
  responsive_breakpoint?: string;
}

export interface PanelLayout {
  panels: Panel[];
  layout_type: PanelLayoutType;
  panel_controls?: PanelControls;
}

export type PanelLayoutType =
  | "Tabs"
  | "Accordion"
  | { type: "Grid"; columns: number }
  | { type: "Sidebar"; main_panel_id: string }
  | "FloatingPanels";

export interface Panel {
  id: string;
  title?: ParagraphNode;
  content: SectionContentNode[];
  panel_role: PanelRole;
  initially_visible?: boolean;
  resizable?: boolean;
}

export type PanelRole =
  | "MainContent"
  | "ComparisonLeft"
  | "ComparisonRight"
  | "SourceTheory"
  | "TargetTheory"
  | "TypeAnnotations"
  | "ResourceBank"
  | "Navigation"
  | "ControlPanel"
  | "InfoBox";

export interface PanelControls {
  allow_minimize?: boolean;
  allow_close?: boolean;
  allow_reorder?: boolean;
}

export interface AnnotationOverlay {
  base_content: SectionContentNode[];
  annotations: Annotation[];
  overlay_style: OverlayStyle;
}

export interface Annotation {
  id: string;
  target_selector: string;
  annotation_content: RichTextSegment[];
  annotation_type: AnnotationType;
  position?: AnnotationPosition;
  styling?: AnnotationStyling;
}

export type AnnotationType = "TypeInfo" | "Definition" | "Explanation" | "Animation" | "Highlight" | "Warning" | "Step";

export type OverlayStyle = "Tooltip" | "Popover" | "Inline" | "Sidebar" | "Highlight";

export interface AnnotationPosition {
  x: number;
  y: number;
  anchor: PositionAnchor;
}

export type PositionAnchor = "TopLeft" | "TopRight" | "BottomLeft" | "BottomRight" | "Center";

export interface AnnotationStyling {
  color?: string;
  background_color?: string;
  border_color?: string;
  opacity?: number;
}

export interface InteractiveControls {
  controls: Control[];
  target_content_ids: string[];
  layout: ControlLayout;
}

export interface Control {
  id: string;
  label: string;
  control_type: ControlType;
  parameter_name: string;
  default_value: string;
  description?: string;
}

export type ControlType =
  | { type: "Slider"; min: number; max: number; step: number }
  | "Toggle"
  | { type: "Dropdown"; options: string[] }
  | { type: "NumberInput"; min?: number; max?: number }
  | "ColorPicker"
  | { type: "Button"; action: string }
  | { type: "RadioGroup"; options: string[] };

export type ControlLayout = "Horizontal" | "Vertical" | { type: "Grid"; columns: number } | "Floating"; 