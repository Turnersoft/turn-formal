import { mockAbstractionData } from '../mocks/abstractionData';
import { 
  ContentManifest, 
  ContentBundle, 
  MathContent, 
  convertMathematicalContentToDefinition,
  convertMathematicalContentToTheorem
} from '../pages/MathPage/models/math';
import type { MathematicalContent } from '../pages/MathPage/components/turn-render/bindings/MathematicalContent';

/**
 * Service for fetching mathematical abstraction data from the backend
 */
export async function fetchGroupTheoryAbstractionData(): Promise<Record<string, any>> {
  try {
    // In a production environment, this would be a real API call
    // For this demo, we'll use mock data
    
    // Simulate API call delay
    await new Promise(resolve => setTimeout(resolve, 500));
    
    // In a production environment, this would be:
    // const response = await fetch('/api/math/theories/groups/abstraction_data');
    // const data = await response.json();
    // return data;
    
    // For now, return mock data
    return mockAbstractionData;
  } catch (error) {
    console.error('Error fetching group theory abstraction data:', error);
    // Fallback to mock data
    return mockAbstractionData;
  }
}

/**
 * Load the content manifest from the new JSON format
 */
export async function fetchContentManifest(): Promise<ContentManifest> {
  try {
    const response = await fetch('/manifest.json');
    if (!response.ok) {
      throw new Error(`Failed to fetch manifest: ${response.statusText}`);
    }
    const manifest: ContentManifest = await response.json();
    return manifest;
  } catch (error) {
    console.error('Error fetching content manifest:', error);
    throw error;
  }
}

/**
 * Load a specific content bundle by file path
 */
export async function fetchContentBundle(filePath: string): Promise<ContentBundle> {
  try {
    const response = await fetch(`/${filePath}`);
    if (!response.ok) {
      throw new Error(`Failed to fetch ${filePath}: ${response.statusText}`);
    }
    const bundle: ContentBundle = await response.json();
    return bundle;
  } catch (error) {
    console.error(`Error fetching content bundle ${filePath}:`, error);
    throw error;
  }
}

/**
 * Load math content for a specific theory using the new ContentBundle format
 */
export async function fetchMathContentFromBundles(theoryId: string): Promise<MathContent> {
  try {
    // First load the manifest to find the theory files
    const manifest = await fetchContentManifest();
    const theory = manifest.theories.find(t => t.theory_id === theoryId);
    
    if (!theory) {
      throw new Error(`Theory not found: ${theoryId}`);
    }

    // Load all content bundles for this theory
    const contentBundles: { definitions?: ContentBundle; theorems?: ContentBundle } = {};
    const allMathematicalContent: any[] = [];
    
    for (const file of theory.files) {
      const bundle = await fetchContentBundle(file.file_path);
      
      if (file.content_type === 'definitions') {
        contentBundles.definitions = bundle;
      } else if (file.content_type === 'theorems') {
        contentBundles.theorems = bundle;
      }
      
      allMathematicalContent.push(...bundle.content);
    }

    // Convert to legacy format for backward compatibility
    const definitions = contentBundles.definitions?.content
      .map(convertMathematicalContentToDefinition)
      .filter(d => d !== null) || [];

    const theorems = contentBundles.theorems?.content
      .map(convertMathematicalContentToTheorem)
      .filter(t => t !== null) || [];

    const mathContent: MathContent = {
      theory: theory.theory_name,
      folder: theoryId,
      definitions: definitions as any[],
      theorems: theorems as any[],
      contentBundles,
      mathematicalContent: allMathematicalContent,
      metadata: {
        version: contentBundles.definitions?.version || contentBundles.theorems?.version || '1.0.0',
        summary: `${theory.theory_name} with ${theory.item_count} items`
      }
    };

    return mathContent;
  } catch (error) {
    console.error(`Error fetching math content for theory ${theoryId}:`, error);
    throw error;
  }
}

/**
 * Get list of available theories from the manifest
 */
export async function fetchAvailableTheories(): Promise<{ theories: Array<{ name: string; path: string; hasDefinitions: boolean; hasTheorems: boolean; description?: string }> }> {
  try {
    const manifest = await fetchContentManifest();
    
    const theories = manifest.theories.map(theory => ({
      name: theory.theory_name,
      path: theory.theory_id,
      hasDefinitions: theory.files.some(f => f.content_type === 'definitions'),
      hasTheorems: theory.files.some(f => f.content_type === 'theorems'),
      description: `${theory.item_count} mathematical objects and theorems`
    }));

    return { theories };
  } catch (error) {
    console.error('Error fetching available theories:', error);
    throw error;
  }
} 