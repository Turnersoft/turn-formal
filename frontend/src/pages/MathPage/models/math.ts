// Types for the math theory JSON data

// Definition JSON structures
export interface Member {
  name: string;
  type: string;
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

export interface Theorem {
  id?: string;
  name: string;
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

// Overall math content structure
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
