import React from 'react';
import styles from './StatsView.module.css';

interface StatsViewProps {
  selectedFile: string | null;
  onClose: () => void;
}

export const StatsView: React.FC<StatsViewProps> = ({ selectedFile, onClose }) => {
  return (
    <div className={styles.statsContainer}>
      <div className={styles.header}>
        <h2>📊 Data Statistics</h2>
        <button onClick={onClose} className={styles.closeButton}>✕</button>
      </div>
      <div className={styles.content}>
        <p>Statistics for: {selectedFile || 'All files'}</p>
        <p>This will show detailed statistics about the mathematical content.</p>
      </div>
    </div>
  );
};

export default StatsView; 