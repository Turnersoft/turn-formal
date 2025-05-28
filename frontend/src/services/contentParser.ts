// Mathematical Content Parser Service
// This service handles parsing and consuming mathematical content from various sources
// Validates against TypeScript bindings and organizes for frontend navigation

import type { MathematicalContent } from '../pages/MathPage/components/turn-render/bindings/MathematicalContent';

// Organized content structure for frontend navigation
export interface MathematicalContentSource {
  id: string;
  theory: string;
  abstraction_level: 1 | 2 | 3 | 4;
  content_type: 'definition' | 'theorem' | 'example' | 'proof' | 'schema';
  display_mode: 'full' | 'tooltip' | 'reference';
  category: string; // For sidebar organization: "Core Concepts", "Examples", "Advanced", etc.
  title: string; // Extracted title for navigation
  content: MathematicalContent; // Validated against TypeScript bindings
}

export interface TheoryExport {
  theory_name: string;
  version: string;
  exported_at: string;
  content_sources: MathematicalContentSource[];
}

export interface ParsedMathContent {
  theories: Map<string, TheoryExport>;
  index: Map<string, MathematicalContentSource>; // For quick lookups by ID
  categories: Map<string, MathematicalContentSource[]>; // For sidebar organization
  navigation: NavigationStructure; // For frontend navigation
}

export interface NavigationStructure {
  theories: TheoryNavigation[];
}

export interface TheoryNavigation {
  id: string;
  name: string;
  description: string;
  categories: CategoryNavigation[];
}

export interface CategoryNavigation {
  id: string;
  name: string;
  abstraction_level?: number;
  content_type?: string;
  items: ContentNavItem[];
}

export interface ContentNavItem {
  id: string;
  title: string;
  content_type: string;
  abstraction_level: number;
}

/**
 * Enhanced Mathematical Content Parser with Type Validation
 * 
 * Features:
 * - Validates against TypeScript bindings
 * - Organizes content for frontend navigation
 * - Supports sidebar categorization
 * - Maintains backward compatibility
 */
export class MathContentParser {
  private parsedContent: ParsedMathContent | null = null;

  /**
   * Parse mathematical content from source with type validation
   * 
   * @param source - Currently JSON string, future: directory path or file content
   * @returns Parsed and validated mathematical content
   */
  async parse(source: string | ArrayBuffer): Promise<ParsedMathContent> {
    // TODO: Future WASM implementation
    // if (this.isWasmAvailable()) {
    //   return this.parseWithWasm(source);
    // }
    
    return this.parseFromJson(source as string);
  }

  /**
   * Enhanced JSON-based parsing with validation and organization
   */
  private async parseFromJson(jsonSource: string): Promise<ParsedMathContent> {
    try {
      const rawData = JSON.parse(jsonSource);
      
      // Validate the raw structure first
      if (!this.validateRawStructure(rawData)) {
        throw new Error('Invalid JSON structure - does not match expected format');
      }

      // Transform and organize the content
      const theories = new Map<string, TheoryExport>();
      const index = new Map<string, MathematicalContentSource>();
      const categories = new Map<string, MathematicalContentSource[]>();

      // Handle different export formats from Rust
      if (this.isUnifiedMathExport(rawData)) {
        const theoryExport = this.transformUnifiedMathExport(rawData);
        theories.set('unified_theories', theoryExport);
        
        // Build index and categories
        theoryExport.content_sources.forEach(source => {
          // Validate each content item against TypeScript bindings
          if (this.validateMathematicalContent(source.content)) {
            index.set(source.id, source);
            
            // Organize by category for sidebar
            const categoryKey = this.getCategoryKey(source);
            if (!categories.has(categoryKey)) {
              categories.set(categoryKey, []);
            }
            categories.get(categoryKey)!.push(source);
          } else {
            console.warn(`⚠️ Content validation failed for ${source.id}`);
          }
        });
      }

      // Build navigation structure for frontend
      const navigation = this.buildNavigationStructure(theories);

      this.parsedContent = { theories, index, categories, navigation };
      return this.parsedContent;
    } catch (error) {
      console.error('Failed to parse mathematical content:', error);
      throw new Error(`Content parsing failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Validate raw JSON structure before processing
   */
  private validateRawStructure(data: any): boolean {
    if (!data || typeof data !== 'object') return false;
    
    // Must have basic export structure
    if (!data.theory_name || !data.version || !data.content) return false;
    
    // Content must be an object
    if (typeof data.content !== 'object') return false;
    
    return true;
  }

  /**
   * Validate MathematicalContent against TypeScript bindings
   */
  private validateMathematicalContent(content: any): content is MathematicalContent {
    // Basic structure validation
    if (!content || typeof content !== 'object') return false;
    if (typeof content.id !== 'string') return false;
    if (!content.content_type || typeof content.content_type !== 'object') return false;
    
    // Validate content_type has exactly one key (union type)
    const contentTypeKeys = Object.keys(content.content_type);
    if (contentTypeKeys.length !== 1) return false;
    
    const contentTypeName = contentTypeKeys[0];
    const contentTypeValue = content.content_type[contentTypeName];
    
    // Validate specific content types
    switch (contentTypeName) {
      case 'ScientificPaper':
        return this.validateScientificPaperContent(contentTypeValue);
      case 'WikiPage':
      case 'Textbook':
      case 'PersonalNotes':
        // TODO: Add validation for other content types
        return typeof contentTypeValue === 'object';
      default:
        console.warn(`Unknown content type: ${contentTypeName}`);
        return false;
    }
  }

  /**
   * Validate ScientificPaperContent against TypeScript bindings
   */
  private validateScientificPaperContent(content: any): boolean {
    if (!content || typeof content !== 'object') return false;
    
    // Required fields
    if (typeof content.title !== 'string') return false;
    if (typeof content.paper_type !== 'string') return false;
    if (typeof content.peer_reviewed !== 'boolean') return false;
    
    // Required objects
    if (!content.content_metadata || typeof content.content_metadata !== 'object') return false;
    if (!content.academic_metadata || typeof content.academic_metadata !== 'object') return false;
    if (!content.structure || typeof content.structure !== 'object') return false;
    if (!content.relationships || typeof content.relationships !== 'object') return false;
    
    // venue can be null or string
    if (content.venue !== null && typeof content.venue !== 'string') return false;
    
    return true;
  }

  /**
   * Get category key for sidebar organization
   */
  private getCategoryKey(source: MathematicalContentSource): string {
    const { theory, abstraction_level, content_type } = source;
    
    // Create hierarchical categories for better organization
    if (content_type === 'schema') {
      return `${theory}:schemas`;
    } else if (abstraction_level === 1) {
      return `${theory}:foundations`;
    } else if (abstraction_level === 2) {
      return `${theory}:types`;
    } else if (abstraction_level === 3) {
      return `${theory}:constructors`;
    } else if (abstraction_level === 4) {
      return `${theory}:examples`;
    } else {
      return `${theory}:other`;
    }
  }

  /**
   * Build navigation structure for frontend sidebar
   */
  private buildNavigationStructure(theories: Map<string, TheoryExport>): NavigationStructure {
    const theoryNavs: TheoryNavigation[] = [];

    theories.forEach((theoryExport, theoryKey) => {
      const categoryMap = new Map<string, ContentNavItem[]>();
      
      // Group content by categories
      theoryExport.content_sources.forEach(source => {
        const categoryKey = this.getCategoryKey(source);
        const categoryName = this.getCategoryDisplayName(categoryKey);
        
        if (!categoryMap.has(categoryName)) {
          categoryMap.set(categoryName, []);
        }
        
        categoryMap.get(categoryName)!.push({
          id: source.id,
          title: source.title,
          content_type: source.content_type,
          abstraction_level: source.abstraction_level
        });
      });
      
      // Convert to CategoryNavigation array
      const categories: CategoryNavigation[] = Array.from(categoryMap.entries()).map(([name, items]) => ({
        id: name.toLowerCase().replace(/\s+/g, '_'),
        name,
        items: items.sort((a, b) => a.title.localeCompare(b.title))
      }));
      
      theoryNavs.push({
        id: theoryKey,
        name: this.getTheoryDisplayName(theoryExport.theory_name),
        description: `${theoryExport.content_sources.length} items`,
        categories: categories.sort((a, b) => this.getCategorySortOrder(a.id) - this.getCategorySortOrder(b.id))
      });
    });

    return { theories: theoryNavs };
  }

  /**
   * Get display name for categories in sidebar
   */
  private getCategoryDisplayName(categoryKey: string): string {
    if (categoryKey.includes(':schemas')) return 'Schemas & Foundations';
    if (categoryKey.includes(':foundations')) return 'Core Foundations';
    if (categoryKey.includes(':types')) return 'Type Definitions';
    if (categoryKey.includes(':constructors')) return 'Constructors';
    if (categoryKey.includes(':examples')) return 'Concrete Examples';
    return 'Other';
  }

  /**
   * Get theory display name
   */
  private getTheoryDisplayName(theoryName: string): string {
    if (theoryName.includes('Group')) return 'Group Theory';
    if (theoryName.includes('Topology')) return 'Topology';
    if (theoryName.includes('ZFC')) return 'Set Theory (ZFC)';
    return theoryName;
  }

  /**
   * Category sort order for consistent sidebar display
   */
  private getCategorySortOrder(categoryId: string): number {
    if (categoryId.includes('schemas')) return 1;
    if (categoryId.includes('foundations')) return 2;
    if (categoryId.includes('types')) return 3;
    if (categoryId.includes('constructors')) return 4;
    if (categoryId.includes('examples')) return 5;
    return 6;
  }

  /**
   * Type guard for unified math export format
   */
  private isUnifiedMathExport(data: any): boolean {
    return data && 
           data.theory_name && 
           data.version && 
           data.exported_at && 
           data.content && 
           typeof data.content === 'object';
  }

  /**
   * Transform unified math export with proper categorization
   */
  private transformUnifiedMathExport(rawData: any): TheoryExport {
    const contentSources: MathematicalContentSource[] = [];

    Object.entries(rawData.content).forEach(([contentId, mathDocument]) => {
      const source = this.parseUnifiedContentEntry(contentId, mathDocument);
      if (source) {
        contentSources.push(source);
      }
    });

    return {
      theory_name: rawData.theory_name || 'Unknown Theory',
      version: rawData.version || '1.0.0',
      exported_at: rawData.exported_at || new Date().toISOString(),
      content_sources: contentSources.sort((a, b) => {
        // Sort by abstraction level, then by title
        if (a.abstraction_level !== b.abstraction_level) {
          return a.abstraction_level - b.abstraction_level;
        }
        return a.title.localeCompare(b.title);
      })
    };
  }

  /**
   * Parse individual unified export content entries with enhanced categorization
   */
  private parseUnifiedContentEntry(contentId: string, mathDocument: any): MathematicalContentSource | null {
    // Parse content IDs like "group_theory.z6z_concrete.l2"
    const parts = contentId.split('.');
    if (parts.length < 3) return null;

    const [theory, type, levelPart] = parts;
    const level = parseInt(levelPart.replace('l', '')) as 1 | 2 | 3 | 4;

    // Extract title from content
    const title = this.extractTitleFromContent(mathDocument);
    
    // Determine category based on type and level
    const category = this.determineCategoryFromType(type, level);

    return {
      id: contentId,
      theory: theory.replace('_', ' '),
      abstraction_level: level,
      content_type: this.inferContentTypeFromId(type),
      display_mode: 'full',
      category,
      title,
      content: mathDocument as MathematicalContent
    };
  }

  /**
   * Extract title from MathematicalContent for navigation
   */
  private extractTitleFromContent(mathDocument: any): string {
    try {
      if (mathDocument?.content_type?.ScientificPaper?.title) {
        return mathDocument.content_type.ScientificPaper.title;
      }
      
      // Fallback: try to extract from first section
      const body = mathDocument?.content_type?.ScientificPaper?.structure?.body;
      if (body && body.length > 0 && body[0].title) {
        const segments = body[0].title.segments || [];
        return segments.map((s: any) => s.Text || s.text || '').join('').trim() || 'Untitled';
      }
      
      return 'Untitled Content';
    } catch (error) {
      return 'Untitled Content';
    }
  }

  /**
   * Determine category from type and level for better organization
   */
  private determineCategoryFromType(typeStr: string, level: number): string {
    if (typeStr.includes('schema')) return 'Schemas';
    if (level === 1) return 'Foundations';
    if (level === 2 && typeStr.includes('concrete')) return 'Examples';
    if (level === 2) return 'Types';
    if (level === 3) return 'Constructors';
    if (level === 4) return 'Examples';
    return 'Other';
  }

  /**
   * Infer content type from unified export content ID
   */
  private inferContentTypeFromId(typeStr: string): 'definition' | 'theorem' | 'example' | 'proof' | 'schema' {
    if (typeStr.includes('schema')) return 'schema';
    if (typeStr.includes('type')) return 'definition';
    if (typeStr.includes('constructor')) return 'definition';
    if (typeStr.includes('concrete')) return 'example';
    return 'definition';
  }

  /**
   * Get parsed content (cached)
   */
  getParsedContent(): ParsedMathContent | null {
    return this.parsedContent;
  }

  /**
   * Get content by ID
   */
  getContentById(id: string): MathematicalContentSource | null {
    return this.parsedContent?.index.get(id) || null;
  }

  /**
   * Get all content for a theory
   */
  getTheoryContent(theoryName: string): TheoryExport | null {
    return this.parsedContent?.theories.get(theoryName) || null;
  }

  /**
   * Get content by category for sidebar organization
   */
  getContentByCategory(categoryKey: string): MathematicalContentSource[] {
    return this.parsedContent?.categories.get(categoryKey) || [];
  }

  /**
   * Get navigation structure for frontend
   */
  getNavigationStructure(): NavigationStructure | null {
    return this.parsedContent?.navigation || null;
  }

  /**
   * Filter content by criteria
   */
  filterContent(criteria: {
    theory?: string;
    abstraction_level?: number;
    content_type?: string;
    display_mode?: string;
    category?: string;
  }): MathematicalContentSource[] {
    if (!this.parsedContent) return [];

    const allContent = Array.from(this.parsedContent.index.values());
    
    return allContent.filter(source => {
      if (criteria.theory && !source.theory.toLowerCase().includes(criteria.theory.toLowerCase())) return false;
      if (criteria.abstraction_level && source.abstraction_level !== criteria.abstraction_level) return false;
      if (criteria.content_type && source.content_type !== criteria.content_type) return false;
      if (criteria.display_mode && source.display_mode !== criteria.display_mode) return false;
      if (criteria.category && source.category !== criteria.category) return false;
      return true;
    });
  }
}

// Singleton instance for the application
export const mathContentParser = new MathContentParser(); 