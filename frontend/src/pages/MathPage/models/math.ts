// Types for the math theory JSON data
import type { MathematicalContent } from '../components/turn-render/bindings/MathematicalContent';

// Definition JSON structures
export interface Member {
  name: string;
  type?: string | null;
  type_info?: string | null;
  docs: string;
  default_value?: string;
  attributes?: string[];
  visibility?: string;
  [key: string]: any; // Allow additional properties
}

export interface Definition {
  name: string;
  docs: string;
  kind: string;
  members: Member[];
  extends?: string[];
  implements?: string[];
  generics?: string[];
  visibility?: string;
  annotations?: string[];
  examples?: string[];
  source?: string;
  [key: string]: any; // Allow additional properties
}

// NEW JSON FORMAT TYPES - From ContentBundle structure

export interface ContentMetadata {
  language?: string;
  version?: string;
  created_at?: string;
  last_modified?: string;
  content_hash?: string;
}

export interface AcademicMetadata {
  authors: string[];
  date_published?: string;
  date_modified?: string;
  venue?: string;
  doi?: string;
  keywords: string[];
}

export interface RichTextSegment {
  Text?: string;
  Link?: { url: string; text: string };
  Math?: MathNode;
  [key: string]: any;
}

export interface ParagraphNode {
  segments: RichTextSegment[];
  alignment?: string;
}

export interface MathBlock {
  math: MathNode;
  label?: string;
  caption?: string;
}

export interface SectionContentNode {
  Paragraph?: ParagraphNode;
  MathBlock?: MathBlock;
  StructuredMath?: any; // Will contain theorem structures
  [key: string]: any;
}

export interface Section {
  id: string;
  title?: ParagraphNode;
  content: SectionContentNode[];
  metadata: any[];
  display_options?: any;
}

export interface DocumentStructure {
  abstract_content?: Section;
  table_of_contents?: any;
  body: Section[];
  footnotes: any[];
  glossary: any[];
  bibliography: any[];
}

export interface DocumentRelationships {
  parent_documents: string[];
  child_documents: string[];
  related_concepts: string[];
  cross_references: string[];
  dependency_graph?: any;
}

export interface ScientificPaperContent {
  title: string;
  paper_type: "Research" | "Review" | "Survey";
  venue?: string;
  peer_reviewed: boolean;
  content_metadata: ContentMetadata;
  academic_metadata: AcademicMetadata;
  structure: DocumentStructure;
  relationships: DocumentRelationships;
}

export interface ContentBundle {
  theory_name: string;
  content_type: "definitions" | "theorems";
  version: string;
  exported_at: string;
  content: MathematicalContent[];
}

export interface ContentFile {
  file_path: string;
  content_type: string;
  item_count: number;
  items: string[];
}

export interface TheoryManifest {
  theory_id: string;
  theory_name: string;
  files: ContentFile[];
  item_count: number;
}

export interface ContentManifest {
  theories: TheoryManifest[];
  total_items: number;
  generated_at: string;
  version: string;
}

// Nested structures from theorems.json
export interface MathNodeContent {
  Text?: string;
  Theorem?: TheoremContent;
  ProofState?: ProofStateContent;
  [key: string]: any;
}

export interface MathNode {
  id: string;
  content: MathNodeContent;
}

export interface ProofStateContent {
  statement?: MathNode;
  path?: string;
  justification?: string;
  quantifiers?: MathNode[];
  variables?: MathNode[];
  [key: string]: any;
}

export interface TheoremContent {
  name: string;
  description: string;
  initial_proof_state: MathNode;
  proof_steps?: MathNode[];
  theory?: string;
  [key: string]: any;
}

export interface TheoremJsonContent {
  Theorem: TheoremContent;
  [key: string]: any;
}

export interface Theorem {
  id?: string;
  name?: string;
  statement?: string;
  description?: string;
  proof_steps?: ProofStep[];
  tags?: string[];
  references?: Reference[];
  related_theorems?: string[];
  axioms_used?: string[];
  lemmas?: Theorem[];
  corollaries?: Theorem[];
  is_proven?: boolean;
  proof_method?: string;
  examples?: string[];
  counterexamples?: string[];
  formulas?: string[];
  // Nested content structure from theorems.json
  content?: TheoremJsonContent;
  [key: string]: any; // Allow additional properties
}

export interface ProofStep {
  id: string;
  description: string;
  formula?: string;
  justification?: string;
  references?: Reference[];
  assumptions?: string[];
  conclusion?: string;
  [key: string]: any; // Allow additional properties
}

export interface Reference {
  title: string;
  authors?: string[];
  year?: number;
  url?: string;
  doi?: string;
  citation?: string;
  [key: string]: any; // Allow additional properties
}

// Overall math content structure - UPDATED to support both formats
export interface MathContent {
  definitions: Definition[];
  theorems: Theorem[];
  folder: string; // Path to the theory folder
  theory: string; // Name of the theory
  description?: string;
  prerequisites?: string[];
  rawJson?: any; // Raw JSON content
  metadata?: {
    authors?: string[];
    version?: string;
    date?: string;
    summary?: string;
    [key: string]: any; // Allow additional properties
  };
  // NEW: Support for ContentBundle format
  contentBundles?: {
    definitions?: ContentBundle;
    theorems?: ContentBundle;
  };
  mathematicalContent?: MathematicalContent[]; // Direct access to new format
  [key: string]: any; // Allow additional properties
}

// Service interfaces
export interface TheoryFolder {
  name: string;
  path: string;
  hasDefinitions: boolean;
  hasTheorems: boolean;
  description?: string;
  [key: string]: any; // Allow additional properties
}

// API response types
export interface AvailableTheoriesResponse {
  theories: TheoryFolder[];
}

export interface TheoryContentResponse {
  definitions: Definition[];
  theorems: Theorem[];
  theory: string;
  [key: string]: any; // Allow additional properties
}

// Helper function to extract text content from RichTextSegment
export function extractTextFromSegments(segments: any[]): string {
  return segments
    .map(segment => {
      if (segment.Text) return segment.Text;
      if (segment.Link) return segment.Link.text;
      return '';
    })
    .join('');
}

// Helper function to convert MathematicalContent to legacy Definition format
export function convertMathematicalContentToDefinition(content: MathematicalContent): Definition | null {
  if (!content.content_type || typeof content.content_type !== 'object') return null;
  
  // Handle the ScientificPaper variant
  if ('ScientificPaper' in content.content_type) {
    const paper = content.content_type.ScientificPaper;
    
    return {
      name: content.id,
      docs: paper.structure.abstract_content ? 
        extractTextFromSegments(paper.structure.abstract_content.content
          .filter((c: any) => c.Paragraph)
          .flatMap((c: any) => c.Paragraph.segments)) : 
        paper.title,
      kind: "Document",
      members: [],
      source: "ContentBundle"
    };
  }

  return null;
}

// Helper function to convert MathematicalContent to legacy Theorem format
export function convertMathematicalContentToTheorem(content: MathematicalContent): Theorem | null {
  if (!content.content_type || typeof content.content_type !== 'object') return null;
  
  // Handle the ScientificPaper variant
  if ('ScientificPaper' in content.content_type) {
    const paper = content.content_type.ScientificPaper;
    
    return {
      id: content.id,
      name: paper.title,
      description: paper.structure.abstract_content ? 
        extractTextFromSegments(paper.structure.abstract_content.content
          .filter((c: any) => c.Paragraph)
          .flatMap((c: any) => c.Paragraph.segments)) : 
        '',
      tags: paper.academic_metadata.keywords,
      is_proven: true
    };
  }

  return null;
}
