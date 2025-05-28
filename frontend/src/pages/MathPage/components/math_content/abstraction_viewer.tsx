import React, { useState } from 'react';
import styles from './math_content.module.scss';
import { MathRender } from '../math_render/mathrender';

// Define types for our abstraction levels display
// (interface removed as it was unused)

interface DisplayMode {
  id: string;
  name: string;
  description: string;
}

interface AbstractionViewerProps {
  sectionData: Record<string, any>;
  defaultLevel?: number;
  defaultMode?: string;
}

const displayModes: DisplayMode[] = [
  {
    id: 'full',
    name: 'Full Wiki Page',
    description: 'Complete detailed view with all information'
  },
  {
    id: 'tooltip',
    name: 'Tooltip',
    description: 'Concise summary for hovering/quick reference'
  },
  {
    id: 'reference',
    name: 'Reference',
    description: 'Link-style reference for inline use'
  }
];

// Helper to extract the abstraction level (L1-L4) from a section key
const getLevelFromKey = (key: string): number => {
  if (key.includes('l1_')) return 1;
  if (key.includes('l2_')) return 2;
  if (key.includes('l3_')) return 3;
  if (key.includes('l4_')) return 4;
  return 0;
};

// Helper to extract the display mode from a section key
const getModeFromKey = (key: string): string => {
  if (key.includes('_full')) return 'full';
  if (key.includes('_tooltip')) return 'tooltip';
  if (key.includes('_reference')) return 'reference';
  return 'unknown';
};

// Helper to render section content
const renderSectionContent = (section: any): React.ReactNode => {
  if (!section) return null;
  
  // Handle title
  const title = section.title ? (
    <h3>{section.title.segments[0]?.text || 'Untitled'}</h3>
  ) : null;
  
  // Handle content
  const contentNodes = section.content.map((node: any, index: number) => {
    if (node.Paragraph) {
      return (
        <p key={`p-${index}`}>
          {node.Paragraph.segments.map((segment: any, segIndex: number) => {
            if (segment.Text) {
              return <span key={`seg-${segIndex}`}>{segment.Text}</span>;
            } else if (segment.Math) {
              return <MathRender key={`math-${segIndex}`} formula={segment.Math.content.Text || ''} />;
            } else if (segment.Link) {
              return <a key={`link-${segIndex}`} href="#">{segment.Link.content[0]?.Text || 'Link'}</a>;
            }
            return null;
          })}
        </p>
      );
    } else if (node.StructuredMath) {
      const structuredMath = node.StructuredMath;
      if (structuredMath.Definition) {
        return (
          <div key={`def-${index}`} className={styles.definition}>
            <h4>{structuredMath.Definition.label || 'Definition'}</h4>
            {structuredMath.Definition.body.map((bodyNode: any, _bodyIndex: number) => 
              renderSectionContent({ content: [bodyNode] })
            )}
          </div>
        );
      }
    } else if (node.SubSection) {
      return (
        <div key={`subsec-${index}`} className={styles.subSection}>
          {renderSectionContent(node.SubSection)}
        </div>
      );
    }
    return null;
  });
  
  return (
    <div className={styles.sectionContent}>
      {title}
      {contentNodes}
    </div>
  );
};

const AbstractionViewer: React.FC<AbstractionViewerProps> = ({ 
  sectionData,
  defaultLevel = 1,
  defaultMode = 'full'
}) => {
  const [currentLevel, setCurrentLevel] = useState<number>(defaultLevel);
  const [currentMode, setCurrentMode] = useState<string>(defaultMode);
  
  // Organize sections by level and mode
  const organizedSections: Record<number, Record<string, any>> = {
    1: {}, 2: {}, 3: {}, 4: {}
  };
  
  // Process all sections
  Object.entries(sectionData).forEach(([key, section]) => {
    const level = getLevelFromKey(key);
    const mode = getModeFromKey(key);
    
    if (level > 0 && mode !== 'unknown') {
      if (!organizedSections[level]) {
        organizedSections[level] = {};
      }
      organizedSections[level][mode] = section;
    }
  });
  
  // Get available levels
  const availableLevels = Object.entries(organizedSections)
    .filter(([_, modes]) => Object.keys(modes).length > 0)
    .map(([level, _]) => parseInt(level))
    .sort();
  
  // Get available modes for current level
  const availableModes = organizedSections[currentLevel] 
    ? Object.keys(organizedSections[currentLevel])
    : [];
  
  // Ensure current selections are valid
  React.useEffect(() => {
    if (availableLevels.length > 0 && !availableLevels.includes(currentLevel)) {
      setCurrentLevel(availableLevels[0]);
    }
    
    if (availableModes.length > 0 && !availableModes.includes(currentMode)) {
      setCurrentMode(availableModes[0]);
    }
  }, [sectionData, availableLevels, availableModes]);
  
  // Get current section to display
  const currentSection = 
    organizedSections[currentLevel] && 
    organizedSections[currentLevel][currentMode];
  
  const renderLevelSelector = () => (
    <div className={styles.abstractionLevelSelector}>
      <h3>Abstraction Level</h3>
      <div className={styles.levelButtons}>
        {availableLevels.map(level => (
          <button
            key={`level-${level}`}
            className={`${styles.levelButton} ${level === currentLevel ? styles.active : ''}`}
            onClick={() => setCurrentLevel(level)}
          >
            L{level}: {getLevelDescription(level)}
          </button>
        ))}
      </div>
    </div>
  );
  
  const renderModeSelector = () => (
    <div className={styles.displayModeSelector}>
      <h3>Display Mode</h3>
      <div className={styles.modeButtons}>
        {displayModes
          .filter(mode => availableModes.includes(mode.id))
          .map(mode => (
            <button
              key={`mode-${mode.id}`}
              className={`${styles.modeButton} ${mode.id === currentMode ? styles.active : ''}`}
              onClick={() => setCurrentMode(mode.id)}
              title={mode.description}
            >
              {mode.name}
            </button>
          ))}
      </div>
    </div>
  );

  return (
    <div className={styles.abstractionViewer}>
      <div className={styles.abstractionControls}>
        {renderLevelSelector()}
        {renderModeSelector()}
      </div>
      
      <div className={styles.abstractionDisplay}>
        {currentSection ? (
          <div className={`${styles.abstractionContent} ${styles[`mode-${currentMode}`]}`}>
            {renderSectionContent(currentSection)}
          </div>
        ) : (
          <div className={styles.noContentMessage}>
            No content available for Level {currentLevel} in {
              displayModes.find(m => m.id === currentMode)?.name || currentMode
            } mode
          </div>
        )}
      </div>
    </div>
  );
};

// Helper function to get description for each abstraction level
function getLevelDescription(level: number): string {
  switch(level) {
    case 1: return "Schema/Blueprint";
    case 2: return "Type/Specification";
    case 3: return "Constructor/Method";
    case 4: return "Concrete Instance";
    default: return "Unknown";
  }
}

export default AbstractionViewer; 