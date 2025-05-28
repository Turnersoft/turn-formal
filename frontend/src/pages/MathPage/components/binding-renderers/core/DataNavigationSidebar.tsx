import React, { useState } from 'react';
import { mathDataService } from '../../../services/mathDataService';
import styles from './DataNavigationSidebar.module.css';

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
  const [availableFiles] = useState(mathDataService.getAvailableDataFiles());

  const formatFileDisplayName = (filename: string): string => {
    return filename
      .replace('.json', '')
      .replace(/_/g, ' ')
      .replace(/\b\w/g, l => l.toUpperCase());
  };

  const getFileSize = (filename: string): string => {
    if (filename.includes('math_content')) return '400KB';
    if (filename.includes('l1_definitions')) return '364KB';
    if (filename.includes('theorems')) return '77KB';
    return '36KB';
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

      {/* File Selection */}
      <div className={styles.section}>
        <h3>📁 Data Files</h3>
        <div className={styles.fileList}>
          {availableFiles.map(filename => (
            <button
              key={filename}
              onClick={() => onFileSelect(filename)}
              className={`${styles.fileItem} ${selectedFile === filename ? styles.selected : ''}`}
            >
              <div className={styles.fileName}>
                {formatFileDisplayName(filename)}
              </div>
              <div className={styles.fileSize}>
                {getFileSize(filename)}
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Quick Help */}
      <div className={styles.section}>
        <h3>💡 Quick Guide</h3>
        <div className={styles.helpText}>
          <p>1. Select a data file above</p>
          <p>2. Choose content from the content panel</p>
          <p>3. View mathematical objects with clean notation</p>
        </div>
      </div>
    </div>
  );
};

export default DataNavigationSidebar; 