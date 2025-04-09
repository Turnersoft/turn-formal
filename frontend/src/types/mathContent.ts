// TypeScript interfaces for math content derived from Rust

// Definition types
export interface MathDefinition {
  id: string;
  name: string;
  description: string;
  type: "enum" | "struct" | "trait";
  variants?: EnumVariant[];
  fields?: StructField[];
  implementations?: MethodImplementation[];
}

export interface EnumVariant {
  id: string;
  name: string;
  description: string;
  fields?: StructField[];
}

export interface StructField {
  name: string;
  type: string;
  description: string;
}

export interface MethodImplementation {
  name: string;
  description: string;
  parameters: MethodParameter[];
  returnType: string;
}

export interface MethodParameter {
  name: string;
  type: string;
}

// Theorem types
export interface MathTheorem {
  id: string;
  name: string;
  statement: string;
  description: string;
  proofSteps: ProofStep[];
  references?: MathReference[];
  tags?: string[];
}

export interface ProofStep {
  id: string;
  description: string;
  formula?: string;
  tacticName?: string;
  tacticArgs?: Record<string, string>;
  justification?: string;
}

export interface MathReference {
  id: string;
  type: "theorem" | "definition" | "paper";
  name: string;
}

// Combined content types
export interface MathContent {
  definitions: MathDefinition[];
  theorems: MathTheorem[];
  relations?: MathRelation[];
}

export interface MathRelation {
  id: string;
  type: string;
  entities: string[];
  description: string;
}
