import React, { useState, useEffect } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { DataNavigationSidebar } from './components/binding-renderers/core/DataNavigationSidebar';
import { DocumentRenderer } from './components/binding-renderers/core/DocumentRenderer';
import { StatsView } from './components/binding-renderers/core/StatsView';
import { mathDataService, type MathDataExport } from './services/mathDataService';
import { MathNavigationService, useMathNavigationState, useMathNavigation } from './services/mathNavigationService';
import type { MathematicalContent } from './components/binding-renderers';
import styles from './MathPage.module.scss';

type ViewMode = 'content' | 'stats';

export const MathPage: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const navigationTarget = useMathNavigationState(location);
  const mathNavigation = useMathNavigation();
  
  // Data state
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [selectedContentId, setSelectedContentId] = useState<string | null>(null);
  const [currentContent, setCurrentContent] = useState<MathematicalContent | null>(null);
  const [dataExport, setDataExport] = useState<MathDataExport | null>(null);
  
  // UI state
  const [viewMode, setViewMode] = useState<ViewMode>('content');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [contentIndex, setContentIndex] = useState<any[]>([]);

  // Handle URL-based navigation
  useEffect(() => {
    console.log('🎯 URL navigation useEffect - navigationTarget:', navigationTarget);
    if (navigationTarget) {
      handleNavigationTarget(navigationTarget);
    }
  }, [navigationTarget]);

  // Load content when file/content selection changes (only for non-URL navigation)
  useEffect(() => {
    console.log('🔄 File loading useEffect triggered - navigationTarget:', navigationTarget, 'selectedFile:', selectedFile, 'selectedContentId:', selectedContentId);
    
    if (!navigationTarget && selectedFile && selectedContentId) {
      console.log('📄 Calling loadSpecificContent()');
      loadSpecificContent();
    } else if (!navigationTarget && selectedFile) {
      console.log('📁 Calling loadFileData()');
      loadFileData();
    } else {
      console.log('⚠️ useEffect conditions not met:', {
        navigationTargetExists: !!navigationTarget,
        selectedFileExists: !!selectedFile,
        selectedContentIdExists: !!selectedContentId
      });
    }
  }, [selectedFile, selectedContentId, navigationTarget]);

  const handleNavigationTarget = async (target: any) => {
    console.log('🎯 handleNavigationTarget called with target:', target);
    console.log('🔍 Target type:', target.type, 'Target theory:', target.theory);
    
    setLoading(true);
    setError(null);
    
    try {
      let dataFile: string;
      
      if (target.type === 'theory') {
        console.log('🏛️ Handling theory overview navigation for:', target.theory);
        // For theory overview, load the overview file
        const theoryName = target.theory.toLowerCase().replace('theory', '').replace(/([a-z])([A-Z])/g, '$1_$2').toLowerCase();
        dataFile = `${theoryName}_theory.overview.json`;
        
        // Load the overview data file
        const data = await mathDataService.loadMathData(dataFile);
        setDataExport(data);
        setSelectedFile(dataFile);
        
        // Load content index
        const index = await mathDataService.getContentIndex(dataFile);
        setContentIndex(index);
        
        // Auto-select first content if available
        if (Object.keys(data.content).length > 0) {
          const firstContentId = Object.keys(data.content)[0];
          setSelectedContentId(firstContentId);
          setCurrentContent(data.content[firstContentId]);
        }
        
        setLoading(false);
        return;
      }
      
      if (target.type === 'definition') {
        dataFile = MathNavigationService.getDataFileForTheory(target.theory);
      } else if (target.type === 'theorem') {
        dataFile = MathNavigationService.getTheoremFileForTheory(target.theory);
      } else {
        throw new Error(`Unknown navigation target type: ${target.type}`);
      }
      
      // Load the appropriate data file
      const data = await mathDataService.loadMathData(dataFile);
      setDataExport(data);
      setSelectedFile(dataFile);
      
      // Load content index
      const index = await mathDataService.getContentIndex(dataFile);
      setContentIndex(index);
      
      // Find and load the specific content
      const contentId = target.id;
      if (data.content[contentId]) {
        setSelectedContentId(contentId);
        setCurrentContent(data.content[contentId]);
      } else {
        // If exact ID not found, try to find a matching one
        const matchingId = Object.keys(data.content).find(id => 
          id.includes(contentId) || contentId.includes(id)
        );
        if (matchingId) {
          setSelectedContentId(matchingId);
          setCurrentContent(data.content[matchingId]);
        } else {
          setError(`Content not found: ${contentId} in ${dataFile}`);
        }
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load content');
    } finally {
      setLoading(false);
    }
  };

  const loadFileData = async () => {
    if (!selectedFile) {
      console.log('⚠️ loadFileData called but no selectedFile');
      return;
    }

    console.log('📁 Loading file data for:', selectedFile);
    setLoading(true);
    setError(null);
    try {
      const data = await mathDataService.loadMathData(selectedFile);
      console.log('✅ File data loaded successfully:', selectedFile, 'with', Object.keys(data.content).length, 'items');
      setDataExport(data);
      
      // Load content index for the content sidebar
      const index = await mathDataService.getContentIndex(selectedFile);
      console.log('📋 Content index loaded with', index.length, 'items');
      setContentIndex(index);
      
      // Auto-select first content item if none selected
      if (!selectedContentId && Object.keys(data.content).length > 0) {
        const firstContentId = Object.keys(data.content)[0];
        console.log('🎯 Auto-selecting first content:', firstContentId);
        setSelectedContentId(firstContentId);
        setCurrentContent(data.content[firstContentId]);
      }
    } catch (err) {
      console.error('❌ Failed to load file data for:', selectedFile, err);
      setError(err instanceof Error ? err.message : 'Failed to load data');
      setDataExport(null);
      setCurrentContent(null);
      setContentIndex([]);
    } finally {
      setLoading(false);
    }
  };

  const loadSpecificContent = async () => {
    if (!selectedFile || !selectedContentId) return;

    setLoading(true);
    setError(null);
    try {
      const content = await mathDataService.getContentById(selectedFile, selectedContentId);
      setCurrentContent(content);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load content');
      setCurrentContent(null);
    } finally {
      setLoading(false);
    }
  };

  const handleFileSelect = (filename: string) => {
    console.log('🔍 Primary sidebar file selected:', filename);
    console.log('🔍 File type checks:', {
      includesOverview: filename.includes('overview'),
      endsWithJson: filename.endsWith('.json'),
      actualFilename: filename
    });
    
    if (filename.includes('overview')) {
      // For overview files, navigate to theory URL
      console.log('🏠 Handling overview file via URL navigation');
      const theoryContext = getTheoryContextFromFilename(filename);
      console.log('🎯 Theory context determined:', theoryContext);
      console.log('📍 About to call mathNavigation.navigateToTheory with:', theoryContext);
      mathNavigation.navigateToTheory(theoryContext);
    } else if (filename.endsWith('.json')) {
      // For definitions/theorems files, clear navigation state first
      console.log('📂 Handling JSON file selection, clearing navigation target first');
      
      // Navigate to a clean URL to clear the navigation target
      navigate('/math'); // Clear the navigation target
      
      // Then set the file after a brief delay to let the navigation target clear
      setTimeout(() => {
        setSelectedFile(filename);
        setSelectedContentId(null);
        setCurrentContent(null);
        setViewMode('content');
        console.log('📄 File state set after navigation clear, useEffect should trigger loadFileData()');
      }, 50);
    } else {
      // For other legacy files
      console.log('🏠 Handling overview/legacy file directly:', filename);
      
      // Clear any existing navigation by navigating to a clean URL using React Router
      console.log('🔄 Clearing URL navigation to allow direct file loading');
      
      // Add a small delay to let the navigation state update
      setTimeout(() => {
        setSelectedFile(filename);
        setSelectedContentId(null);
        setCurrentContent(null);
        setViewMode('content');
        console.log('📄 Overview file state set after URL clear, useEffect should trigger loadFileData()');
      }, 50);
    }
  };

  const handleContentSelect = async (contentId: string) => {
    console.log('🔍 Sidebar clicked for content:', contentId, 'selectedFile:', selectedFile);
    
    if (selectedFile) {
      const theoryContext = getTheoryContextFromFilename(selectedFile);
      
      try {
        if (selectedFile.includes('definitions')) {
          await mathNavigation.navigateToDefinition({
            term_id: contentId,
            theory_context: theoryContext
          });
        } else if (selectedFile.includes('theorems')) {
          await mathNavigation.navigateToTheorem({
            theorem_id: contentId,
            theory_context: theoryContext
          });
        } else {
          // Legacy behavior for non-routed content
          setSelectedContentId(contentId);
          setViewMode('content');
        }
      } catch (error) {
        console.error('❌ Navigation failed:', error);
        // Fallback to local state update
        setSelectedContentId(contentId);
        setViewMode('content');
      }
    }
  };

  const getTheoryContextFromFilename = (filename: string): string => {
    if (filename.includes('group_theory')) return 'GroupTheory';
    if (filename.includes('probability_theory')) return 'ProbabilityTheory';
    if (filename.includes('field_theory')) return 'FieldTheory';
    if (filename.includes('ring_theory')) return 'RingTheory';
    
    // Extract theory name from filename as fallback
    const match = filename.match(/^([a-z_]+)_theory\./);
    if (match) {
      const theoryName = match[1].split('_').map(word => 
        word.charAt(0).toUpperCase() + word.slice(1)
      ).join('') + 'Theory';
      return theoryName;
    }
    
    return 'GroupTheory'; // final fallback
  };

  const loadFileAndNavigateToFirst = async (filename: string, theoryContext: string, type: 'definition' | 'theorem') => {
    try {
      const data = await mathDataService.loadMathData(filename);
      const firstContentId = Object.keys(data.content)[0];
      
      if (firstContentId) {
        if (type === 'definition') {
          await mathNavigation.navigateToDefinition({
            term_id: firstContentId,
            theory_context: theoryContext
          });
        } else {
          await mathNavigation.navigateToTheorem({
            theorem_id: firstContentId,
            theory_context: theoryContext
          });
        }
      }
    } catch (error) {
      console.error('Failed to load file:', error);
      // Fallback to legacy behavior
      setSelectedFile(filename);
      setSelectedContentId(null);
      setCurrentContent(null);
      setViewMode('content');
    }
  };

  const handleStatsView = () => {
    setViewMode('stats');
  };

  const renderMainContent = () => {
    if (viewMode === 'stats') {
      return (
        <StatsView 
          selectedFile={selectedFile}
          onClose={() => setViewMode('content')}
        />
      );
    }

    if (loading) {
      return (
        <div className={styles.loadingState}>
          <h3>🔄 Loading Content...</h3>
          <p>Fetching mathematical content from the data files...</p>
        </div>
      );
    }

    if (error) {
      return (
        <div className={styles.errorState}>
          <h3>❌ Error Loading Content</h3>
          <p>{error}</p>
          <button 
            onClick={() => selectedFile && handleFileSelect(selectedFile)}
            className={styles.retryButton}
          >
            🔄 Retry
          </button>
        </div>
      );
    }

    if (!selectedFile) {
      return (
        <div className={styles.welcomeState}>
          <div className={styles.welcomeContent}>
            <h2>🚀 Welcome to the Binding-Based Math Explorer</h2>
            <p>
              This application demonstrates the new binding-renderer architecture 
              where components directly consume Rust binding types with zero runtime transformation.
            </p>
            
            <div className={styles.features}>
              <div className={styles.feature}>
                <h3>📚 Comprehensive Data</h3>
                <p>Browse through multiple mathematical content files with thousands of definitions, theorems, and examples.</p>
              </div>
              
              <div className={styles.feature}>
                <h3>🔍 Advanced Navigation</h3>
                <p>Navigate through mathematical objects with clean mathematical notation like GL(2, 𝔽).</p>
              </div>
              
              <div className={styles.feature}>
                <h3>⚡ Type-Safe Rendering</h3>
                <p>All components use exact Rust binding types, ensuring perfect synchronization between frontend and backend.</p>
              </div>
            </div>

            <div className={styles.getStarted}>
              <h3>🎯 Get Started</h3>
              <p>Select a data file from the left sidebar to begin exploring mathematical content.</p>
            </div>
          </div>
        </div>
      );
    }

    if (!currentContent) {
      return (
        <div className={styles.noContentState}>
          <h3>📄 No Content Selected</h3>
          <p>
            {dataExport ? 
              `Select a document from the content list to view it here.` :
              'Select content from the sidebar to view it here.'
            }
          </p>
          {dataExport && (
            <div className={styles.fileInfo}>
              <h4>📊 File Information</h4>
              <p><strong>Theory:</strong> {dataExport.theory_name}</p>
              <p><strong>Version:</strong> {dataExport.version}</p>
              <p><strong>Documents:</strong> {Object.keys(dataExport.content).length}</p>
              <p><strong>Exported:</strong> {new Date(parseInt(dataExport.exported_at) * 1000).toLocaleDateString()}</p>
            </div>
          )}
        </div>
      );
    }

    // Render the actual mathematical content using binding-based renderers
    return (
      <div className={styles.contentRenderer}>
        <div className={styles.contentHeader}>
          <div className={styles.breadcrumb}>
            <span className={styles.fileName}>
              {selectedFile?.replace('.json', '').replace(/_/g, ' ')}
            </span>
            {selectedContentId && (
              <>
                <span className={styles.separator}>→</span>
                <span className={styles.contentId}>{selectedContentId}</span>
              </>
            )}
          </div>
        </div>

        <div className={styles.documentContent}>
          <DocumentRenderer content={currentContent} />
        </div>
      </div>
    );
  };

  const renderContentSidebar = () => {
    if (!selectedFile || contentIndex.length === 0) {
      return null;
    }

    return (
      <div className={styles.contentSidebar}>
        <div className={styles.contentHeader}>
          <h3>📄 {selectedFile.replace('.json', '').replace(/_/g, ' ')}</h3>
          <div className={styles.contentCount}>
            {contentIndex.length} objects
          </div>
        </div>
        
        <div className={styles.contentList}>
          {contentIndex.map(item => (
            <div
              key={item.id}
              onClick={() => handleContentSelect(item.id)}
              className={`${styles.contentItem} ${selectedContentId === item.id ? styles.selected : ''}`}
            >
              <div className={styles.contentTitle}>{item.title}</div>
              <div className={styles.contentMeta}>
                {item.level && <span className={styles.level}>L{item.level}</span>}
                <span className={styles.sections}>{item.section_count} sections</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  };

  return (
    <div className={styles.mathPage}>
      <DataNavigationSidebar
        selectedFile={selectedFile}
        onFileSelect={handleFileSelect}
        onStatsView={handleStatsView}
      />
      
      {renderContentSidebar()}
      
      <main className={styles.mainContent}>
        {renderMainContent()}
      </main>
    </div>
  );
};

export default MathPage;
