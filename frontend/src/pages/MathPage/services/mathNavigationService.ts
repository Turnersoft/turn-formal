import { useNavigate } from 'react-router-dom';
import type { Location } from 'react-router-dom';

// Import the mathDataService to validate content existence
import { mathDataService } from './mathDataService';

// Types for mathematical content navigation
export interface DefinitionNavigation {
  term_id: string;
  theory_context: string;
}

export interface TheoremNavigation {
  theorem_id: string;
  theory_context: string;
}

export interface MathNavigationTarget {
  type: 'definition' | 'theorem' | 'theory';
  theory: string;
  id?: string;
}

/**
 * Helper function to extract document ID from section ID
 */
function extractDocumentIdFromSectionId(sectionId: string): string | null {
  // Handle new clear ID pattern: group_theory.def.{group_type}.{section_type}
  if (sectionId.includes('.def.')) {
    const parts = sectionId.split('.');
    if (parts.length >= 3) {
      // Return: group_theory.def.{group_type}
      return parts.slice(0, 3).join('.');
    }
  }

  // Handle legacy patterns like group_theory.topological-main-groupbasic-section
  if (sectionId.includes('-main-groupbasic-section') || 
      sectionId.includes('-main-topologicalgroup-section') ||
      sectionId.includes('-main-liegroup-section') ||
      sectionId.includes('-main-cyclicgroup-section') ||
      sectionId.includes('-main-symmetricgroup-section') ||
      sectionId.includes('-main-dihedralgroup-section') ||
      sectionId.includes('-main-alternatinggroup-section') ||
      sectionId.includes('-main-productgroup-section')) {
    
    // For legacy patterns, try to map to the correct document
    if (sectionId.includes('topological-')) {
      return 'group_theory.def.topological_group';
    } else if (sectionId.includes('lie-')) {
      return 'group_theory.def.lie_group';
    } else if (sectionId.includes('cyclic-')) {
      return 'group_theory.def.cyclic_group';
    } else if (sectionId.includes('symmetric-')) {
      return 'group_theory.def.symmetric_group';
    } else if (sectionId.includes('dihedral-')) {
      return 'group_theory.def.dihedral_group';
    } else if (sectionId.includes('alternating-')) {
      return 'group_theory.def.alternating_group';
    } else if (sectionId.includes('product-')) {
      return 'group_theory.def.product_group';
    } else {
      // Default to generic group for basic group references
      return 'group_theory.def.generic_group';
    }
  }

  // For other patterns, try to extract a reasonable document ID
  const parts = sectionId.split('.');
  if (parts.length >= 2) {
    // Try to find the document ID by removing the last segment
    return parts.slice(0, -1).join('.');
  }

  return null;
}

/**
 * Helper function to map section ID to target section within document
 */
function mapSectionIdToTargetSection(sectionId: string): string {
  // Handle new clear ID pattern - return as is
  if (sectionId.includes('.def.') && sectionId.split('.').length === 4) {
    return sectionId;
  }

  // Handle legacy patterns - map to new pattern or main section
  if (sectionId.includes('-main-groupbasic-section')) {
    // Map all basic group references to the generic group's main section
    return 'group_theory.def.generic_group.main_section';
  }

  if (sectionId.includes('-main-topologicalgroup-section')) {
    return 'group_theory.def.topological_group.main_section';
  }

  if (sectionId.includes('-main-liegroup-section')) {
    return 'group_theory.def.lie_group.main_section';
  }

  if (sectionId.includes('-main-cyclicgroup-section')) {
    return 'group_theory.def.cyclic_group.main_section';
  }

  if (sectionId.includes('-main-symmetricgroup-section')) {
    return 'group_theory.def.symmetric_group.main_section';
  }

  if (sectionId.includes('-main-dihedralgroup-section')) {
    return 'group_theory.def.dihedral_group.main_section';
  }

  if (sectionId.includes('-main-alternatinggroup-section')) {
    return 'group_theory.def.alternating_group.main_section';
  }

  if (sectionId.includes('-main-productgroup-section')) {
    return 'group_theory.def.product_group.main_section';
  }

  // For other sections, return as is and hope for the best
  return sectionId;
}

/**
 * Helper function to validate that content exists before navigation
 */
async function validateContentExists(contentId: string, dataFile: string): Promise<{
  exists: boolean;
  actualId?: string;
  suggestions?: string[];
}> {
  try {
    console.log('🔍 Validating content exists:', contentId, 'in file:', dataFile);
    
    const data = await mathDataService.loadMathData(dataFile);
    const availableIds = Object.keys(data.content);
    
    // Check for exact match
    if (data.content[contentId]) {
      console.log('✅ Exact match found for:', contentId);
      return { exists: true, actualId: contentId };
    }
    
    // Try to find best match
    let bestMatch: string | undefined;
    
    // Try partial matching strategies
    bestMatch = availableIds.find(id => id === contentId) ||
               availableIds.find(id => id.endsWith(`.${contentId.split('.').pop()}`)) ||
               availableIds.find(id => id.startsWith(contentId) || contentId.startsWith(id)) ||
               availableIds.find(id => id.includes(contentId) || contentId.includes(id));
    
    if (bestMatch) {
      console.log('📍 Found alternative match:', bestMatch, 'for requested:', contentId);
      return { 
        exists: true, 
        actualId: bestMatch,
        suggestions: availableIds.filter(id => 
          id.includes(contentId.split('.')[0]) || 
          contentId.includes(id.split('.')[0])
        ).slice(0, 5)
      };
    }
    
    // No match found
    console.log('❌ No match found for:', contentId);
    console.log('📋 Available content IDs:', availableIds.slice(0, 10)); // Show first 10 for debugging
    
    return { 
      exists: false, 
      suggestions: availableIds.slice(0, 5) // Return first 5 as suggestions
    };
    
  } catch (error) {
    console.error('❌ Error validating content:', error);
    return { exists: false };
  }
}

/**
 * Helper function to scroll to a section within the current page
 */
function scrollToSection(sectionId: string) {
  console.log('📍 Attempting to scroll to section:', sectionId);
  
  // Try multiple possible element IDs
  const possibleIds = [
    sectionId,
    sectionId.replace(/\./g, '-'), // Replace dots with dashes
    sectionId.split('.').pop() || sectionId, // Just the last part, fallback to full ID
    `section-${sectionId}`,
    `${sectionId}-section`
  ].filter(Boolean); // Remove any undefined values

  for (const id of possibleIds) {
    const element = document.getElementById(id);
    if (element) {
      console.log('✅ Found element with ID:', id);
      element.scrollIntoView({ 
        behavior: 'smooth', 
        block: 'start' 
      });
      
      // Add highlight effect
      element.classList.add('highlighted-section');
      setTimeout(() => {
        element.classList.remove('highlighted-section');
      }, 2000);
      
      return;
    }
  }

  console.log('❌ Could not find element for section:', sectionId);
  console.log('🔍 Tried IDs:', possibleIds);
}

/**
 * Service for handling navigation to mathematical content
 */
export class MathNavigationService {
  private navigate: (path: string) => void;

  constructor(navigate: (path: string) => void) {
    this.navigate = navigate;
  }

  /**
   * Navigate to a specific definition with enhanced section support and content validation
   */
  async navigateToDefinition(definition: DefinitionNavigation) {
    console.log('🔍 Navigating to definition:', definition.term_id);
    
    // Extract document ID from section ID
    const documentId = extractDocumentIdFromSectionId(definition.term_id);
    const targetSectionId = mapSectionIdToTargetSection(definition.term_id);
    
    console.log('📄 Extracted document ID:', documentId);
    console.log('🎯 Target section ID:', targetSectionId);

    if (!documentId) {
      console.error('❌ Could not extract document ID from:', definition.term_id);
      // Fallback: navigate to theory overview
      this.navigateToTheory(definition.theory_context);
      return;
    }

    // Use the term_id directly for navigation if it looks like a simple content ID
    if (!definition.term_id.includes('-main-') && !definition.term_id.includes('.def.')) {
      console.log('🚀 Simple content navigation to:', definition.term_id);
      const path = `/math/definition/${definition.theory_context}/${definition.term_id}`;
      this.navigate(path);
      return;
    }

    // For complex IDs, validate content exists before navigating
    const dataFile = MathNavigationService.getDataFileForTheory(definition.theory_context);
    const validation = await validateContentExists(documentId, dataFile);
    
    if (!validation.exists) {
      console.error('❌ Content does not exist:', documentId);
      if (validation.suggestions && validation.suggestions.length > 0) {
        console.log('💡 Suggested alternatives:', validation.suggestions);
        // Navigate to the first suggestion
        const suggestion = validation.suggestions[0];
        console.log('🔄 Redirecting to suggested content:', suggestion);
        this.navigate(`/math/definition/${definition.theory_context}/${suggestion}`);
      } else {
        // Fallback: navigate to theory overview
        console.log('🔄 No suggestions available, navigating to theory overview');
        this.navigateToTheory(definition.theory_context);
      }
      return;
    }

    // Use the validated actual ID for navigation
    const actualDocumentId = validation.actualId || documentId;
    const path = `/math/definition/${definition.theory_context}/${actualDocumentId}`;
    console.log('🚀 Navigating to validated URL:', path);
    
    this.navigate(path);

    // Wait a bit for the page to load, then scroll to section
    setTimeout(() => {
      scrollToSection(targetSectionId);
    }, 100);
  }

  /**
   * Navigate to a specific theorem with enhanced section support and content validation
   */
  async navigateToTheorem(theorem: TheoremNavigation) {
    console.log('🔍 Navigating to theorem:', theorem.theorem_id);
    
    // For theorems, we might need similar logic if they reference sections
    const documentId = extractDocumentIdFromSectionId(theorem.theorem_id) || theorem.theorem_id;
    
    // Use the theorem_id directly for navigation if it looks like a simple content ID
    if (!theorem.theorem_id.includes('-main-') && !theorem.theorem_id.includes('.def.')) {
      console.log('🚀 Simple theorem navigation to:', theorem.theorem_id);
      const path = `/math/theorem/${theorem.theory_context}/${theorem.theorem_id}`;
      this.navigate(path);
      return;
    }

    // For complex IDs, validate content exists before navigating
    const dataFile = MathNavigationService.getTheoremFileForTheory(theorem.theory_context);
    const validation = await validateContentExists(documentId, dataFile);
    
    if (!validation.exists) {
      console.error('❌ Theorem content does not exist:', documentId);
      if (validation.suggestions && validation.suggestions.length > 0) {
        console.log('💡 Suggested theorem alternatives:', validation.suggestions);
        const suggestion = validation.suggestions[0];
        console.log('🔄 Redirecting to suggested theorem:', suggestion);
        this.navigate(`/math/theorem/${theorem.theory_context}/${suggestion}`);
      } else {
        console.log('🔄 No theorem suggestions available, navigating to theory overview');
        this.navigateToTheory(theorem.theory_context);
      }
      return;
    }

    const actualDocumentId = validation.actualId || documentId;
    const path = `/math/theorem/${theorem.theory_context}/${actualDocumentId}`;
    console.log('🚀 Navigating to validated theorem URL:', path);
    
    this.navigate(path);

    // Scroll to theorem section if it's a section reference
    setTimeout(() => {
      scrollToSection(theorem.theorem_id);
    }, 100);
  }

  /**
   * Navigate to a theory overview
   */
  navigateToTheory(theoryName: string) {
    const path = `/math/theory/${theoryName}`;
    this.navigate(path);
  }

  /**
   * Parse a math content URL and extract navigation target
   */
  static parseContentUrl(pathname: string): MathNavigationTarget | null {
    // Handle theory overview URLs like /math/theory/GroupTheory
    const theoryRegex = /^\/math\/theory\/([^\/]+)$/;
    const theoryMatch = pathname.match(theoryRegex);
    
    if (theoryMatch) {
      const [, theory] = theoryMatch;
      return {
        type: 'theory',
        theory
      };
    }

    // Handle definition and theorem URLs like /math/definition/GroupTheory/content_id
    const mathContentRegex = /^\/math\/(definition|theorem)\/([^\/]+)\/(.+)$/;
    const match = pathname.match(mathContentRegex);
    
    if (match) {
      const [, type, theory, id] = match;
      return {
        type: type as 'definition' | 'theorem',
        theory,
        id
      };
    }
    
    return null;
  }

  /**
   * Get the appropriate data file for a theory context
   */
  static getDataFileForTheory(theoryContext: string): string {
    // Map theory contexts to their data files
    const theoryFileMap: { [key: string]: string } = {
      'GroupTheory': 'group_theory.definitions.json',
      'FieldTheory': 'field_theory.definitions.json',
      'RingTheory': 'ring_theory.definitions.json',
      'TopologyTheory': 'topology.definitions.json',
      // Add more mappings as needed
    };

    return theoryFileMap[theoryContext] || `${theoryContext.toLowerCase()}.definitions.json`;
  }

  /**
   * Get the theorem data file for a theory context
   */
  static getTheoremFileForTheory(theoryContext: string): string {
    const theoryFileMap: { [key: string]: string } = {
      'GroupTheory': 'group_theory.theorems.json',
      'FieldTheory': 'field_theory.theorems.json',
      'RingTheory': 'ring_theory.theorems.json',
      'TopologyTheory': 'topology.theorems.json',
      // Add more mappings as needed
    };

    return theoryFileMap[theoryContext] || `${theoryContext.toLowerCase()}.theorems.json`;
  }
}

/**
 * Hook to use the math navigation service
 */
export const useMathNavigation = () => {
  const navigate = useNavigate();
  return new MathNavigationService(navigate);
};

/**
 * Hook to get current navigation state from URL
 */
export const useMathNavigationState = (location: Location): MathNavigationTarget | null => {
  return MathNavigationService.parseContentUrl(location.pathname);
}; 