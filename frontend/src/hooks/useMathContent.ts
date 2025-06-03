/**
 * React Hook for Mathematical Content
 * 
 * Provides easy access to mathematical content with automatic state management
 */

import { useState, useEffect, useCallback, useMemo } from 'react';
import { 
  mathContentService, 
  type ContentManifest, 
  type TheoryManifest, 
  type ContentBundle 
} from '../services/mathContentService';
import { MathDocument } from '../pages/MathPage/components/turn-render/bindings/MathDocument';

export interface UseMathContentState {
  isInitialized: boolean;
  isLoading: boolean;
  error: string | null;
  manifest: ContentManifest | null;
}

export interface UseMathContentResult {
  // State
  state: UseMathContentState;
  
  // Content Access
  getContentById: (id: string) => Promise<MathDocument | null>;
  getAllLoadedContent: () => Map<string, MathDocument>;
  
  // Theory Operations  
  loadTheory: (theoryId: string, contentType?: string) => Promise<ContentBundle | Map<string, ContentBundle>>;
  getAvailableTheories: () => TheoryManifest[];
  getAvailableContentTypes: (theoryId: string) => string[];
  getContentIds: (theoryId?: string, contentType?: string) => string[];
  
  // Bulk Operations
  preloadAllContent: () => Promise<void>;
  loadLegacyFormat: (fileName?: string) => Promise<void>;
  
  // Utilities
  clearCache: () => void;
  refresh: () => Promise<void>;
}

/**
 * React Hook for Mathematical Content with File-Splitting Support
 * 
 * This hook provides easy access to mathematical content with:
 * - Automatic service initialization
 * - On-demand content loading
 * - Theory and content type filtering
 * - Loading state management
 * - Error handling
 * 
 * @example
 * ```tsx
 * function MathComponent() {
 *   const { state, loadTheory, getContentById } = useMathContent();
 *   
 *   useEffect(() => {
 *     if (state.isInitialized) {
 *       loadTheory('group_theory', 'l1_definitions');
 *     }
 *   }, [state.isInitialized]);
 *   
 *   const handleContentClick = async (id: string) => {
 *     const content = await getContentById(id);
 *     // Use content...
 *   };
 *   
 *   if (!state.isInitialized) return <div>Initializing...</div>;
 *   return <div>Math content ready!</div>;
 * }
 * ```
 */
export function useMathContent(): UseMathContentResult {
  const [state, setState] = useState<UseMathContentState>({
    isInitialized: false,
    isLoading: false,
    error: null,
    manifest: null,
  });

  // Initialize the service on first mount
  useEffect(() => {
    let mounted = true;

    const initialize = async () => {
      setState(prev => ({ ...prev, isLoading: true, error: null }));

      try {
        await mathContentService.initialize();
        
        if (mounted) {
          setState({
            isInitialized: true,
            isLoading: false,
            error: null,
            manifest: mathContentService.getManifest(),
          });
        }
      } catch (error) {
        if (mounted) {
          setState(prev => ({
            ...prev,
            isLoading: false,
            error: error instanceof Error ? error.message : 'Initialization failed',
          }));
        }
      }
    };

    initialize();

    return () => {
      mounted = false;
    };
  }, []);

  // Content operations
  const getContentById = useCallback(async (id: string): Promise<MathDocument | null> => {
    if (!state.isInitialized) {
      console.warn('⚠️ Service not initialized yet');
      return null;
    }

    try {
      return await mathContentService.getContentById(id);
    } catch (error) {
      console.error(`❌ Failed to get content ${id}:`, error);
      return null;
    }
  }, [state.isInitialized]);

  const getAllLoadedContent = useCallback((): Map<string, MathDocument> => {
    return mathContentService.getAllLoadedContent();
  }, []);

  // Theory operations
  const loadTheory = useCallback(async (
    theoryId: string, 
    contentType?: string
  ): Promise<ContentBundle | Map<string, ContentBundle>> => {
    if (!state.isInitialized) {
      throw new Error('Service not initialized');
    }

    setState(prev => ({ ...prev, isLoading: true, error: null }));

    try {
      const result = contentType 
        ? await mathContentService.loadTheoryContent(theoryId, contentType)
        : await mathContentService.loadAllTheoryContent(theoryId);
      
      setState(prev => ({ ...prev, isLoading: false }));
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Failed to load theory';
      setState(prev => ({ ...prev, isLoading: false, error: errorMessage }));
      throw error;
    }
  }, [state.isInitialized]);

  const getAvailableTheories = useCallback((): TheoryManifest[] => {
    return mathContentService.getAvailableTheories();
  }, []);

  const getAvailableContentTypes = useCallback((theoryId: string): string[] => {
    return mathContentService.getAvailableContentTypes(theoryId);
  }, []);

  const getContentIds = useCallback((theoryId?: string, contentType?: string): string[] => {
    return mathContentService.getContentIds(theoryId, contentType);
  }, []);

  // Bulk operations
  const preloadAllContent = useCallback(async (): Promise<void> => {
    if (!state.isInitialized) {
      throw new Error('Service not initialized');
    }

    setState(prev => ({ ...prev, isLoading: true, error: null }));

    try {
      await mathContentService.preloadAllContent();
      setState(prev => ({ ...prev, isLoading: false }));
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Failed to preload content';
      setState(prev => ({ ...prev, isLoading: false, error: errorMessage }));
      throw error;
    }
  }, [state.isInitialized]);

  const loadLegacyFormat = useCallback(async (fileName?: string): Promise<void> => {
    setState(prev => ({ ...prev, isLoading: true, error: null }));

    try {
      await mathContentService.loadLegacyFormat(fileName);
      setState(prev => ({ ...prev, isLoading: false }));
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Failed to load legacy format';
      setState(prev => ({ ...prev, isLoading: false, error: errorMessage }));
      throw error;
    }
  }, []);

  // Utilities
  const clearCache = useCallback((): void => {
    mathContentService.clearCache();
  }, []);

  const refresh = useCallback(async (): Promise<void> => {
    setState({
      isInitialized: false,
      isLoading: false,
      error: null,
      manifest: null,
    });

    // Clear cache and re-initialize
    mathContentService.clearCache();
    
    setState(prev => ({ ...prev, isLoading: true }));

    try {
      await mathContentService.initialize();
      setState({
        isInitialized: true,
        isLoading: false,
        error: null,
        manifest: mathContentService.getManifest(),
      });
    } catch (error) {
      setState(prev => ({
        ...prev,
        isLoading: false,
        error: error instanceof Error ? error.message : 'Refresh failed',
      }));
    }
  }, []);

  // Memoized result to prevent unnecessary re-renders
  const result = useMemo((): UseMathContentResult => ({
    state,
    getContentById,
    getAllLoadedContent,
    loadTheory,
    getAvailableTheories,
    getAvailableContentTypes,
    getContentIds,
    preloadAllContent,
    loadLegacyFormat,
    clearCache,
    refresh,
  }), [
    state,
    getContentById,
    getAllLoadedContent,
    loadTheory,
    getAvailableTheories,
    getAvailableContentTypes,
    getContentIds,
    preloadAllContent,
    loadLegacyFormat,
    clearCache,
    refresh,
  ]);

  return result;
}

// Additional convenience hooks

/**
 * Hook to load a specific theory and content type
 */
export function useTheoryContent(theoryId: string, contentType?: string) {
  const { state, loadTheory } = useMathContent();
  const [theoryState, setTheoryState] = useState<{
    isLoading: boolean;
    error: string | null;
    content: ContentBundle | Map<string, ContentBundle> | null;
  }>({
    isLoading: false,
    error: null,
    content: null,
  });

  useEffect(() => {
    if (state.isInitialized && theoryId) {
      setTheoryState(prev => ({ ...prev, isLoading: true, error: null }));

      loadTheory(theoryId, contentType)
        .then(content => {
          setTheoryState({
            isLoading: false,
            error: null,
            content,
          });
        })
        .catch(error => {
          setTheoryState({
            isLoading: false,
            error: error instanceof Error ? error.message : 'Failed to load theory',
            content: null,
          });
        });
    }
  }, [state.isInitialized, theoryId, contentType, loadTheory]);

  return {
    ...state,
    theoryContent: theoryState.content,
    isLoadingTheory: theoryState.isLoading,
    theoryError: theoryState.error,
  };
}

/**
 * Hook to get a specific content item
 */
export function useContentItem(contentId: string) {
  const { state, getContentById } = useMathContent();
  const [itemState, setItemState] = useState<{
    isLoading: boolean;
    error: string | null;
    content: MathDocument | null;
  }>({
    isLoading: false,
    error: null,
    content: null,
  });

  useEffect(() => {
    if (state.isInitialized && contentId) {
      setItemState(prev => ({ ...prev, isLoading: true, error: null }));

      getContentById(contentId)
        .then(content => {
          setItemState({
            isLoading: false,
            error: null,
            content,
          });
        })
        .catch(error => {
          setItemState({
            isLoading: false,
            error: error instanceof Error ? error.message : 'Failed to load content',
            content: null,
          });
        });
    }
  }, [state.isInitialized, contentId, getContentById]);

  return {
    ...state,
    content: itemState.content,
    isLoadingContent: itemState.isLoading,
    contentError: itemState.error,
  };
} 