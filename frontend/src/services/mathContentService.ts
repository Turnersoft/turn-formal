/**
 * Mathematical Content Service
 * 
 * Single Entry Point for all mathematical content operations in React
 * This service handles loading, parsing, and providing mathematical content to components
 */

import { mathContentParser, MathematicalContentSource, ParsedMathContent } from './contentParser';
import { MathematicalContent } from '../pages/MathPage/components/turn-render/bindings/MathematicalContent';
import { MathematicalContentType } from '../pages/MathPage/components/turn-render/bindings/MathematicalContentType';

export interface MathContentServiceConfig {
  contentUrl?: string;
  enableCaching?: boolean;
  retryAttempts?: number;
}

export interface MathContentServiceState {
  isLoading: boolean;
  isLoaded: boolean;
  error: string | null;
  content: ParsedMathContent | null;
  lastUpdated: Date | null;
}

/**
 * Content Manifest structure for file-splitting
 */
interface ContentManifest {
  theories: TheoryManifest[];
  total_items: number;
  generated_at: string;
  version: string;
}

interface TheoryManifest {
  theory_id: string;
  theory_name: string;
  files: ContentFile[];
  item_count: number;
}

interface ContentFile {
  file_path: string;
  content_type: string; // "l1_definitions", "l3_constructors", "theorems"
  item_count: number;
  items: string[]; // List of content IDs in this file
}

interface ContentBundle {
  theory_name: string;
  content_type: string;
  version: string;
  exported_at: string;
  content: Record<string, MathematicalContent>;
}

/**
 * Modern Math Content Service with file-splitting support
 * Loads content on-demand for better performance
 */
class MathContentService {
  private manifest: ContentManifest | null = null;
  private loadedBundles: Map<string, ContentBundle> = new Map();
  private contentCache: Map<string, MathematicalContent> = new Map();
  private baseUrl: string;

  constructor(baseUrl: string = '/') {
    this.baseUrl = baseUrl.endsWith('/') ? baseUrl : baseUrl + '/';
  }

  /**
   * Initialize the service by loading the manifest
   */
  async initialize(): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}manifest.json`);
      if (!response.ok) {
        throw new Error(`Failed to load manifest: ${response.statusText}`);
      }
      this.manifest = await response.json();
      
      if (!this.manifest) {
        throw new Error('Received invalid manifest data');
      }
      
      console.log('✅ Math Content Service initialized:', {
        theories: this.manifest.theories.length,
        totalItems: this.manifest.total_items,
        version: this.manifest.version
      });
    } catch (error) {
      console.error('❌ Failed to initialize Math Content Service:', error);
      throw error;
    }
  }

  /**
   * Get the manifest (must call initialize() first)
   */
  getManifest(): ContentManifest | null {
    return this.manifest;
  }

  /**
   * Load a specific theory and content type
   */
  async loadTheoryContent(theoryId: string, contentType: string): Promise<ContentBundle> {
    const bundleKey = `${theoryId}_${contentType}`;
    
    // Return cached bundle if already loaded
    if (this.loadedBundles.has(bundleKey)) {
      return this.loadedBundles.get(bundleKey)!;
    }

    if (!this.manifest) {
      throw new Error('Service not initialized. Call initialize() first.');
    }

    // Find the theory in manifest
    const theory = this.manifest.theories.find(t => t.theory_id === theoryId);
    if (!theory) {
      throw new Error(`Theory not found: ${theoryId}`);
    }

    // Find the content file
    const contentFile = theory.files.find(f => f.content_type === contentType);
    if (!contentFile) {
      throw new Error(`Content type not found: ${contentType} for theory ${theoryId}`);
    }

    try {
      const response = await fetch(`${this.baseUrl}${contentFile.file_path}`);
      if (!response.ok) {
        throw new Error(`Failed to load ${contentFile.file_path}: ${response.statusText}`);
      }
      
      const bundle: ContentBundle = await response.json();
      
      // Cache the bundle
      this.loadedBundles.set(bundleKey, bundle);
      
      // Cache individual content items
      Object.entries(bundle.content).forEach(([id, content]) => {
        this.contentCache.set(id, content);
      });

      console.log(`✅ Loaded ${bundleKey}:`, {
        items: Object.keys(bundle.content).length,
        theory: bundle.theory_name,
        type: bundle.content_type
      });

      return bundle;
    } catch (error) {
      console.error(`❌ Failed to load ${bundleKey}:`, error);
      throw error;
    }
  }

  /**
   * Load all available content for a theory
   */
  async loadAllTheoryContent(theoryId: string): Promise<Map<string, ContentBundle>> {
    if (!this.manifest) {
      throw new Error('Service not initialized. Call initialize() first.');
    }

    const theory = this.manifest.theories.find(t => t.theory_id === theoryId);
    if (!theory) {
      throw new Error(`Theory not found: ${theoryId}`);
    }

    const bundles = new Map<string, ContentBundle>();
    
    for (const file of theory.files) {
      try {
        const bundle = await this.loadTheoryContent(theoryId, file.content_type);
        bundles.set(file.content_type, bundle);
      } catch (error) {
        console.warn(`⚠️  Failed to load ${file.content_type} for ${theoryId}:`, error);
      }
    }

    return bundles;
  }

  /**
   * Get a specific content item by ID (loads on-demand if needed)
   */
  async getContentById(id: string): Promise<MathematicalContent | null> {
    // Check cache first
    if (this.contentCache.has(id)) {
      return this.contentCache.get(id)!;
    }

    if (!this.manifest) {
      throw new Error('Service not initialized. Call initialize() first.');
    }

    // Find which bundle contains this content ID
    for (const theory of this.manifest.theories) {
      for (const file of theory.files) {
        if (file.items.includes(id)) {
          try {
            await this.loadTheoryContent(theory.theory_id, file.content_type);
            return this.contentCache.get(id) || null;
          } catch (error) {
            console.error(`❌ Failed to load content for ${id}:`, error);
            return null;
          }
        }
      }
    }

    console.warn(`⚠️  Content not found: ${id}`);
    return null;
  }

  /**
   * Get all currently loaded content
   */
  getAllLoadedContent(): Map<string, MathematicalContent> {
    return new Map(this.contentCache);
  }

  /**
   * Get content IDs by theory and type
   */
  getContentIds(theoryId?: string, contentType?: string): string[] {
    if (!this.manifest) {
      return [];
    }

    let allIds: string[] = [];

    for (const theory of this.manifest.theories) {
      if (theoryId && theory.theory_id !== theoryId) continue;
      
      for (const file of theory.files) {
        if (contentType && file.content_type !== contentType) continue;
        allIds.push(...file.items);
      }
    }

    return allIds;
  }

  /**
   * Get available theories
   */
  getAvailableTheories(): TheoryManifest[] {
    return this.manifest?.theories || [];
  }

  /**
   * Get available content types for a theory
   */
  getAvailableContentTypes(theoryId: string): string[] {
    if (!this.manifest) return [];
    
    const theory = this.manifest.theories.find(t => t.theory_id === theoryId);
    return theory?.files.map(f => f.content_type) || [];
  }

  /**
   * Preload all content (for when you need everything)
   */
  async preloadAllContent(): Promise<void> {
    if (!this.manifest) {
      throw new Error('Service not initialized. Call initialize() first.');
    }

    console.log('🚀 Preloading all mathematical content...');
    const loadPromises: Promise<any>[] = [];

    for (const theory of this.manifest.theories) {
      for (const file of theory.files) {
        loadPromises.push(
          this.loadTheoryContent(theory.theory_id, file.content_type)
            .catch(error => {
              console.warn(`⚠️  Failed to preload ${theory.theory_id}/${file.content_type}:`, error);
            })
        );
      }
    }

    await Promise.all(loadPromises);
    console.log('✅ All content preloaded:', {
      loadedBundles: this.loadedBundles.size,
      cachedItems: this.contentCache.size
    });
  }

  /**
   * Legacy compatibility: Load from single file
   */
  async loadLegacyFormat(fileName: string = 'math_content.json'): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}${fileName}`);
      if (!response.ok) {
        throw new Error(`Failed to load ${fileName}: ${response.statusText}`);
      }
      
      const data = await response.json();
      
      // Handle legacy format
      if (data.content) {
        Object.entries(data.content).forEach(([id, content]) => {
          this.contentCache.set(id, content as MathematicalContent);
        });
        console.log('✅ Legacy format loaded:', Object.keys(data.content).length, 'items');
      }
    } catch (error) {
      console.error('❌ Failed to load legacy format:', error);
      throw error;
    }
  }

  /**
   * Clear all caches
   */
  clearCache(): void {
    this.loadedBundles.clear();
    this.contentCache.clear();
    console.log('🧹 Cache cleared');
  }
}

// Export singleton instance
export const mathContentService = new MathContentService();
export { MathContentService, type ContentManifest, type TheoryManifest, type ContentFile, type ContentBundle };

// Types are already exported above

// Export the parser types for convenience
export type { MathematicalContentSource, ParsedMathContent } from './contentParser'; 