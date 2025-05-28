/**
 * Utility to clean up mathematical notation from Rust debug output
 * Converts messy debug strings to proper mathematical notation
 */

export function cleanMathNotation(text: string): string {
  if (!text || typeof text !== 'string') {
    return text;
  }

  // Cache cleaned results to avoid reprocessing
  const cleaned = text
    // Handle GL(n, Field) patterns - most comprehensive first
    .replace(/GL\(\s*(\d+)\s*,\s*Basic\(FieldBasic\s*\{[^}]*base_set:\s*Empty[^}]*\}\s*\)\s*\)/g, 'GL($1, 𝔽)')
    .replace(/GL\(\s*(\d+)\s*,\s*Basic\(FieldBasic\s*\{[^}]+\}\s*\)\s*\)/g, 'GL($1, 𝔽)')
    
    // Handle Symbol("x") -> x
    .replace(/Symbol\("([^"]+)"\)/g, '$1')
    
    // Handle generator notation ⟨Symbol("g")⟩ -> ⟨g⟩  
    .replace(/⟨Symbol\("([^"]+)"\)⟩/g, '⟨$1⟩')
    
    // Handle Var(Name("identifier", number)) -> identifier
    .replace(/Var\(Name\("([^"]+)",\s*\d+\)\)/g, '$1')
    
    // Handle FieldOperation patterns
    .replace(/FieldOperation\s*\{[^}]+\}/g, '⊕')
    
    // Handle GroupOperation patterns  
    .replace(/GroupOperation\s*\{[^}]+\}/g, '∘')
    
    // Handle DihedralGroup patterns
    .replace(/Dihedral\(DihedralGroup\s*\{[^}]+order:\s*(\d+)[^}]*\}\)/g, 'D_$1')
    
    // Handle CyclicGroup patterns
    .replace(/Cyclic\(CyclicGroup\s*\{[^}]+order:\s*(\d+)[^}]*\}\)/g, 'C_$1')
    
    // Handle SymmetricGroup patterns
    .replace(/Symmetric\(SymmetricGroup\s*\{[^}]+degree:\s*(\d+)[^}]*\}\)/g, 'S_$1')
    
    // Handle AlternatingGroup patterns
    .replace(/Alternating\(AlternatingGroup\s*\{[^}]+degree:\s*(\d+)[^}]*\}\)/g, 'A_$1')
    
    // Handle complex nested patterns first
    .replace(/FieldOperation\s*\{\s*operation_type:\s*[^,]+,\s*symbol:\s*"([^"]+)"[^}]*\}/g, '$1')
    .replace(/GroupOperation\s*\{\s*operation_type:\s*[^,]+,\s*notation:\s*[^,]+[^}]*\}/g, '∘')
    .replace(/VariantSet\s*\{\s*inner:\s*\{\s*\}\s*\}/g, '')
    
    // Clean up remaining Rust struct patterns
    .replace(/\w+\s*\{[^}]*\}/g, '')
    
    // Clean up extra whitespace and commas
    .replace(/,\s*,/g, ',')
    .replace(/,\s*\)/g, ')')
    .replace(/\(\s*,/g, '(')
    .replace(/\s+/g, ' ')
    .trim()
    
    // Handle remaining parentheses issues
    .replace(/\(\s*\)/g, '')
    .replace(/,\s*$/, '');

  return cleaned || text; // Fallback to original if cleaning resulted in empty string
}

/**
 * Clean mathematical expressions specifically for group theory
 */
export function cleanGroupNotation(text: string): string {
  const cleaned = cleanMathNotation(text);
  
  // Additional group theory specific cleaning
  return cleaned
    // Handle specific group patterns
    .replace(/GeneralLinear\([^)]+\)/g, 'GL')
    .replace(/Basic\([^)]+\)/g, '𝔽')
    .replace(/\bBasic\b/g, '𝔽')  // Handle standalone "Basic"
    .replace(/Empty/g, '∅')
    .replace(/Multiplication\s*\([^)]*\)/g, '×')
    .replace(/Addition\s*\([^)]*\)/g, '+')
    .replace(/Infix\(Times\)/g, '×')
    .replace(/One/g, '1')
    .replace(/MultiplicativeInverse/g, '⁻¹')
    
    // Final cleanup for common patterns
    .replace(/GL\(\s*(\d+)\s*,\s*𝔽\s*\)/g, 'GL($1, 𝔽)')  // Ensure proper spacing
    .replace(/\s*,\s*\)/g, ')') // Remove trailing commas
    .replace(/\(\s*,/g, '(')    // Remove leading commas
    .replace(/\s+/g, ' ')       // Normalize whitespace
    .trim();
}

/**
 * Extract a clean title from potentially messy mathematical content
 */
export function extractCleanTitle(content: any): string {
  // Handle different content structures
  if (typeof content === 'string') {
    return cleanGroupNotation(content);
  }
  
  if (content?.title) {
    return cleanGroupNotation(content.title);
  }
  
  if (content?.ScientificPaper?.title) {
    return cleanGroupNotation(content.ScientificPaper.title);
  }
  
  if (content?.name) {
    return cleanGroupNotation(content.name);
  }
  
  // Fallback
  return 'Mathematical Content';
}

/**
 * Clean text content in paragraph segments
 */
export function cleanSegmentText(text: string): string {
  if (!text) return text;
  
  return cleanGroupNotation(text)
    // Additional text-specific cleaning
    .replace(/\(display as Level\d+\) not fully implemented\.?/g, '')
    .replace(/Details for Group variant /g, '')
    .trim();
} 