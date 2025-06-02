import type { MathematicalContent } from '../components/binding-renderers';

export interface MathDataExport {
  theory_name: string;
  version: string;
  exported_at: string;
  content: Record<string, MathematicalContent>;
}

// Raw data format from JSON files (can have content as array or object)
interface RawMathDataExport {
  theory_name: string;
  version: string;
  exported_at: string;
  content: MathematicalContent[] | Record<string, MathematicalContent>;
}

export interface MathDataIndex {
  id: string;
  title: string;
  type: string;
  level?: string;
  section_count: number;
  content_preview: string;
}

export class MathDataService {
  private static instance: MathDataService;
  private dataCache: Map<string, MathDataExport> = new Map();
  private indexCache: Map<string, MathDataIndex[]> = new Map();

  public static getInstance(): MathDataService {
    if (!MathDataService.instance) {
      MathDataService.instance = new MathDataService();
    }
    return MathDataService.instance;
  }

  /**
   * Convert array content format to object format
   */
  private normalizeContentFormat(rawData: RawMathDataExport): MathDataExport {
    let content: Record<string, MathematicalContent>;
    
    if (Array.isArray(rawData.content)) {
      // Convert array to object using the id field
      content = {};
      for (const item of rawData.content) {
        if (item && typeof item === 'object' && 'id' in item) {
          content[item.id] = item;
        }
      }
    } else {
      // Already in object format
      content = rawData.content;
    }

    return {
      theory_name: rawData.theory_name,
      version: rawData.version,
      exported_at: rawData.exported_at,
      content
    };
  }

  /**
   * Load and parse JSON data from public folder
   */
  async loadMathData(filename: string): Promise<MathDataExport> {
    if (this.dataCache.has(filename)) {
      return this.dataCache.get(filename)!;
    }

    try {
      const response = await fetch(`/${filename}`);
      if (!response.ok) {
        throw new Error(`Failed to load ${filename}: ${response.statusText}`);
      }

      const rawData: RawMathDataExport = await response.json();
      const normalizedData = this.normalizeContentFormat(rawData);
      
      this.dataCache.set(filename, normalizedData);
      
      // Generate index for navigation
      this.generateDataIndex(filename, normalizedData);
      
      return normalizedData;
    } catch (error) {
      console.error(`Error loading math data from ${filename}:`, error);
      throw error;
    }
  }

  /**
   * Get all available data files
   */
  getAvailableDataFiles(): string[] {
    return [
      'math_content.json',
      'group_theory_l1_definitions.json', 
      'group_theory_theorems.json',
      'group_theory_l3_constructors.json',
      // New manifest-based files
      'group_theory.definitions.json',
      'group_theory.theorems.json'
    ];
  }

  /**
   * Load manifest-based data from the new JSON format
   */
  async loadFromManifest(): Promise<string[]> {
    try {
      const response = await fetch('/manifest.json');
      if (!response.ok) {
        console.warn('Manifest file not found, using legacy file list');
        return this.getAvailableDataFiles();
      }

      const manifest = await response.json();
      const availableFiles: string[] = [];
      
      for (const theory of manifest.theories) {
        for (const file of theory.files) {
          availableFiles.push(file.file_path);
        }
      }
      
      return [...this.getAvailableDataFiles(), ...availableFiles];
    } catch (error) {
      console.warn('Failed to load manifest, using legacy file list:', error);
      return this.getAvailableDataFiles();
    }
  }

  /**
   * Get all available data files including manifest-based files
   */
  async getAvailableDataFilesAsync(): Promise<string[]> {
    return await this.loadFromManifest();
  }

  /**
   * Get content index for navigation
   */
  async getContentIndex(filename: string): Promise<MathDataIndex[]> {
    if (this.indexCache.has(filename)) {
      return this.indexCache.get(filename)!;
    }

    // Load data to generate index
    await this.loadMathData(filename);
    return this.indexCache.get(filename) || [];
  }

  /**
   * Get specific content by ID
   */
  async getContentById(filename: string, contentId: string): Promise<MathematicalContent | null> {
    const data = await this.loadMathData(filename);
    return data.content[contentId] || null;
  }

  /**
   * Get paginated content for large datasets
   */
  async getPaginatedContent(filename: string, page: number = 1, pageSize: number = 10): Promise<{
    content: Array<{ id: string; data: MathematicalContent }>;
    total: number;
    pages: number;
    currentPage: number;
  }> {
    const data = await this.loadMathData(filename);
    const contentEntries = Object.entries(data.content);
    
    const total = contentEntries.length;
    const pages = Math.ceil(total / pageSize);
    const startIndex = (page - 1) * pageSize;
    const endIndex = startIndex + pageSize;
    
    const paginatedEntries = contentEntries.slice(startIndex, endIndex);
    const content = paginatedEntries.map(([id, data]) => ({ id, data }));

    return {
      content,
      total,
      pages,
      currentPage: page
    };
  }

  /**
   * Search content by title or content text
   */
  async searchContent(filename: string, query: string): Promise<Array<{ id: string; data: MathematicalContent; relevance: number }>> {
    const data = await this.loadMathData(filename);
    const results: Array<{ id: string; data: MathematicalContent; relevance: number }> = [];

    for (const [id, content] of Object.entries(data.content)) {
      let relevance = 0;
      const searchText = query.toLowerCase();

      // Search in content type
      if (typeof content.content_type === 'object' && 'ScientificPaper' in content.content_type) {
        const paper = content.content_type.ScientificPaper;
        
        // Title match (higher relevance)
        if (paper.title.toLowerCase().includes(searchText)) {
          relevance += 10;
        }

        // Content search in sections
        if (paper.structure?.body) {
          for (const section of paper.structure.body) {
            if (section.title?.segments) {
              for (const segment of section.title.segments) {
                if (typeof segment === 'object' && 'Text' in segment && 
                    segment.Text.toLowerCase().includes(searchText)) {
                  relevance += 5;
                }
              }
            }
          }
        }
      }

      if (relevance > 0) {
        results.push({ id, data: content, relevance });
      }
    }

    return results.sort((a, b) => b.relevance - a.relevance);
  }

  /**
   * Generate navigation index from loaded data
   */
  private generateDataIndex(filename: string, data: MathDataExport): void {
    const index: MathDataIndex[] = [];

    for (const [id, content] of Object.entries(data.content)) {
      if (typeof content.content_type === 'object' && 'ScientificPaper' in content.content_type) {
        const paper = content.content_type.ScientificPaper;
        
        // Extract preview text from first paragraph
        let preview = 'No preview available';
        if (paper.structure?.abstract_content?.content?.[0] && 
            typeof paper.structure.abstract_content.content[0] === 'object' && 
            'Paragraph' in paper.structure.abstract_content.content[0]) {
          const paragraph = paper.structure.abstract_content.content[0].Paragraph;
          if (paragraph.segments?.[0] && typeof paragraph.segments[0] === 'object' && 'Text' in paragraph.segments[0]) {
            preview = paragraph.segments[0].Text.substring(0, 150) + '...';
          }
        }

        // Determine level from metadata
        let level = '1';
        if (paper.structure?.body?.[0]?.metadata) {
          for (const [key, value] of paper.structure.body[0].metadata) {
            if (key === 'abstraction_level') {
              level = value;
              break;
            }
          }
        }

        index.push({
          id,
          title: paper.title,
          type: paper.paper_type,
          level,
          section_count: paper.structure?.body?.length || 0,
          content_preview: preview
        });
      }
    }

    this.indexCache.set(filename, index);
  }

  /**
   * Get content statistics
   */
  async getContentStats(filename: string): Promise<{
    total_documents: number;
    total_sections: number;
    by_level: Record<string, number>;
    by_type: Record<string, number>;
  }> {
    const data = await this.loadMathData(filename);
    const stats = {
      total_documents: 0,
      total_sections: 0,
      by_level: {} as Record<string, number>,
      by_type: {} as Record<string, number>
    };

    for (const content of Object.values(data.content)) {
      stats.total_documents++;

      if (typeof content.content_type === 'object' && 'ScientificPaper' in content.content_type) {
        const paper = content.content_type.ScientificPaper;
        
        // Count sections
        stats.total_sections += paper.structure?.body?.length || 0;
        
        // Group by type
        const type = paper.paper_type;
        stats.by_type[type] = (stats.by_type[type] || 0) + 1;

        // Group by level
        let level = '1';
        if (paper.structure?.body?.[0]?.metadata) {
          for (const [key, value] of paper.structure.body[0].metadata) {
            if (key === 'abstraction_level') {
              level = value;
              break;
            }
          }
        }
        stats.by_level[level] = (stats.by_level[level] || 0) + 1;
      }
    }

    return stats;
  }
}

export const mathDataService = MathDataService.getInstance(); 