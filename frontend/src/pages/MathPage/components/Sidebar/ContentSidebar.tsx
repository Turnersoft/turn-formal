import React from 'react';
import styles from './ContentSidebar.module.scss';

interface ContentItem {
  id: string;
  title: string;
  level?: string | number;
  section_count: number;
}

interface ContentSidebarProps {
  selectedFile: string | null;
  contentIndex: ContentItem[];
  selectedContentId: string | null;
  onContentSelect: (contentId: string) => void;
  className?: string;
}

export const ContentSidebar: React.FC<ContentSidebarProps> = ({
  selectedFile,
  contentIndex,
  selectedContentId,
  onContentSelect,
  className = ''
}) => {
  if (!selectedFile || contentIndex.length === 0) {
    return null;
  }

  return (
    <div className={`${styles.contentSidebar} ${className}`}>
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
            onClick={() => onContentSelect(item.id)}
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

export default ContentSidebar; 