import React, { useState, useEffect } from 'react';
import { mathDataService } from '../../../services/mathDataService';
import styles from './DataNavigationSidebar.module.css';

interface TheoryData {
  theory_id: string;
  theory_name: string;
  overview?: string;
  definitions?: string;
  theorems?: string;
  overviewCount?: number;
  definitionCount?: number;
  theoremCount?: number;
}

interface DataNavigationSidebarProps {
  selectedFile: string | null;
  onFileSelect: (filename: string) => void;
  onStatsView: () => void;
}

export const DataNavigationSidebar: React.FC<DataNavigationSidebarProps> = ({
  selectedFile,
  onFileSelect,
  onStatsView
}) => {
  const [theories, setTheories] = useState<TheoryData[]>([]);
  const [expandedTheories, setExpandedTheories] = useState<Set<string>>(new Set());
  const [loading, setLoading] = useState(true);

  // Load theories and organize by content type
  useEffect(() => {
    const loadTheories = async () => {
      console.log('📚 DataNavigationSidebar loading theories...');
      try {
        // Try to load from manifest first
        const response = await fetch('/manifest.json');
        if (response.ok) {
          console.log('✅ Manifest loaded successfully');
          const manifest = await response.json();
          console.log('📋 Manifest contains', manifest.theories.length, 'theories');
          const theoryData: TheoryData[] = [];
          
          for (const theory of manifest.theories) {
            console.log('🔍 Processing theory:', theory.theory_name, 'with', theory.files.length, 'files');
            const theoryInfo: TheoryData = {
              theory_id: theory.theory_id,
              theory_name: theory.theory_name,
            };
            
            // Find overview, definitions and theorems files
            for (const file of theory.files) {
              console.log('📄 Processing file:', file.file_path, 'type:', file.content_type);
              if (file.content_type === 'theory_overview') {
                console.log('🏠 Found overview file:', file.file_path);
                theoryInfo.overview = file.file_path;
                theoryInfo.overviewCount = file.item_count;
              } else if (file.content_type === 'definitions') {
                console.log('📖 Found definitions file:', file.file_path);
                theoryInfo.definitions = file.file_path;
                theoryInfo.definitionCount = file.item_count;
              } else if (file.content_type === 'theorems') {
                console.log('🔬 Found theorems file:', file.file_path);
                theoryInfo.theorems = file.file_path;
                theoryInfo.theoremCount = file.item_count;
              }
            }
            
            console.log('📊 Theory processed:', theoryInfo);
            theoryData.push(theoryInfo);
          }
          
          console.log('✅ Setting', theoryData.length, 'theories');
          setTheories(theoryData);
        } else {
          // Fallback to legacy file structure
          const availableFiles = await mathDataService.getAvailableDataFilesAsync();
          const legacyTheories: TheoryData[] = [];
          
          // Group legacy files by theory
          const groupTheoryFiles = availableFiles.filter(f => f.includes('group_theory'));
          if (groupTheoryFiles.length > 0) {
            const groupTheory: TheoryData = {
              theory_id: 'group_theory',
              theory_name: 'Group Theory'
            };
            
            groupTheoryFiles.forEach(file => {
              if (file.includes('definitions')) {
                groupTheory.definitions = file;
              } else if (file.includes('theorems')) {
                groupTheory.theorems = file;
              }
            });
            
            legacyTheories.push(groupTheory);
          }
          
          // Add other legacy files as individual theories
          const otherFiles = availableFiles.filter(f => !f.includes('group_theory'));
          otherFiles.forEach(file => {
            const theoryName = file.replace('.json', '').replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
            legacyTheories.push({
              theory_id: file.replace('.json', ''),
              theory_name: theoryName,
              definitions: file
            });
          });
          
          setTheories(legacyTheories);
        }
      } catch (error) {
        console.error('Failed to load theories:', error);
      } finally {
        setLoading(false);
      }
    };

    loadTheories();
  }, []);

  const toggleTheoryExpansion = (theoryId: string) => {
    const newExpanded = new Set(expandedTheories);
    if (newExpanded.has(theoryId)) {
      newExpanded.delete(theoryId);
    } else {
      newExpanded.add(theoryId);
    }
    setExpandedTheories(newExpanded);
  };

  const isFileSelected = (filename: string) => {
    return selectedFile === filename;
  };

  const getFileIcon = (contentType: 'overview' | 'definitions' | 'theorems') => {
    return contentType === 'overview' ? '🏠' : contentType === 'definitions' ? '📖' : '🔬';
  };

  const getFileDisplayName = (contentType: 'overview' | 'definitions' | 'theorems') => {
    return contentType === 'overview' ? 'Overview' : contentType === 'definitions' ? 'Definitions' : 'Theorems';
  };

  return (
    <div className={styles.sidebar}>
      {/* Header */}
      <div className={styles.header}>
        <h2>📚 Math Content Explorer</h2>
        <button 
          onClick={onStatsView}
          className={styles.statsButton}
          title="View Statistics"
        >
          📊 Stats
        </button>
      </div>

      {/* Theory Navigation */}
      <div className={styles.section}>
        <h3>🗂️ Theories</h3>
        {loading ? (
          <div className={styles.loadingFiles}>
            <p>🔄 Loading theories...</p>
          </div>
        ) : (
          <div className={styles.theoryList}>
            {theories.map(theory => (
              <div key={theory.theory_id} className={styles.theoryFolder}>
                {/* Theory Header */}
                <button
                  onClick={() => toggleTheoryExpansion(theory.theory_id)}
                  className={styles.theoryHeader}
                >
                  <span className={styles.expandIcon}>
                    {expandedTheories.has(theory.theory_id) ? '📂' : '📁'}
                  </span>
                  <span className={styles.theoryName}>{theory.theory_name}</span>
                  <span className={styles.theoryCount}>
                    {(theory.overviewCount || 0) + (theory.definitionCount || 0) + (theory.theoremCount || 0)} items
                  </span>
                </button>

                {/* Theory Content Files */}
                {expandedTheories.has(theory.theory_id) && (
                  <div className={styles.theoryContent}>
                    {theory.overview && (
                      <button
                        onClick={() => {
                          console.log('🏠 Overview button clicked for:', theory.theory_name, 'file:', theory.overview);
                          onFileSelect(theory.overview!);
                        }}
                        className={`${styles.contentFile} ${isFileSelected(theory.overview) ? styles.selected : ''}`}
                      >
                        <span className={styles.fileIcon}>
                          {getFileIcon('overview')}
                        </span>
                        <span className={styles.fileName}>
                          {getFileDisplayName('overview')}
                        </span>
                        {theory.overviewCount && (
                          <span className={styles.fileCount}>
                            {theory.overviewCount}
                          </span>
                        )}
                      </button>
                    )}
                    
                    {theory.definitions && (
                      <button
                        onClick={() => onFileSelect(theory.definitions!)}
                        className={`${styles.contentFile} ${isFileSelected(theory.definitions) ? styles.selected : ''}`}
                      >
                        <span className={styles.fileIcon}>
                          {getFileIcon('definitions')}
                        </span>
                        <span className={styles.fileName}>
                          {getFileDisplayName('definitions')}
                        </span>
                        {theory.definitionCount && (
                          <span className={styles.fileCount}>
                            {theory.definitionCount}
                          </span>
                        )}
                      </button>
                    )}
                    
                    {theory.theorems && (
                      <button
                        onClick={() => onFileSelect(theory.theorems!)}
                        className={`${styles.contentFile} ${isFileSelected(theory.theorems) ? styles.selected : ''}`}
                      >
                        <span className={styles.fileIcon}>
                          {getFileIcon('theorems')}
                        </span>
                        <span className={styles.fileName}>
                          {getFileDisplayName('theorems')}
                        </span>
                        {theory.theoremCount && (
                          <span className={styles.fileCount}>
                            {theory.theoremCount}
                          </span>
                        )}
                      </button>
                    )}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default DataNavigationSidebar; 