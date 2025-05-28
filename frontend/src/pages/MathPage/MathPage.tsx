import React, { useState, useEffect } from 'react';
import { DataNavigationSidebar } from './components/binding-renderers/core/DataNavigationSidebar';
import { DocumentRenderer } from './components/binding-renderers/core/DocumentRenderer';
import { StatsView } from './components/binding-renderers/core/StatsView';
import { mathDataService, type MathDataExport } from './services/mathDataService';
import type { MathematicalContent } from './components/binding-renderers';
import styles from './MathPage.module.scss';

type ViewMode = 'content' | 'stats';

export const MathPage: React.FC = () => {
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

  // Load content when file/content selection changes
  useEffect(() => {
    if (selectedFile && selectedContentId) {
      loadSpecificContent();
    } else if (selectedFile) {
      loadFileData();
    }
  }, [selectedFile, selectedContentId]);

  const loadFileData = async () => {
    if (!selectedFile) return;

      setLoading(true);
    setError(null);
    try {
      const data = await mathDataService.loadMathData(selectedFile);
      setDataExport(data);
      
      // Load content index for the content sidebar
      const index = await mathDataService.getContentIndex(selectedFile);
      setContentIndex(index);
      
      // Auto-select first content item if none selected
      if (!selectedContentId && Object.keys(data.content).length > 0) {
        const firstContentId = Object.keys(data.content)[0];
        setSelectedContentId(firstContentId);
        setCurrentContent(data.content[firstContentId]);
      }
    } catch (err) {
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
    setSelectedFile(filename);
    setSelectedContentId(null);
    setCurrentContent(null);
    setViewMode('content');
  };

  const handleContentSelect = (contentId: string) => {
    setSelectedContentId(contentId);
    setViewMode('content');
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
