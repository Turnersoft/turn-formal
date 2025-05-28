import React from 'react';
import { FlexibleDocument, DocumentType } from '../types';
import styles from './styles.module.css';

interface DocumentTypeRendererProps {
  document: FlexibleDocument;
  children: React.ReactNode;
}

export const DocumentTypeRenderer: React.FC<DocumentTypeRendererProps> = ({
  document: _flexibleDocument, // Renamed to indicate it's intentionally unused for now
  children
}) => {
  const getDocumentClassName = (docType: DocumentType): string => {
    switch (docType) {
      case 'ScientificPaper':
        return styles.scientificPaper;
      case 'BlogPost':
        return styles.blogPost;
      case 'TooltipSummary':
        return styles.tooltipSummary;
      case 'AnimatedPresentation':
        return styles.animatedPresentation;
      case 'ResourcePanel':
        return styles.resourcePanel;
      case 'ComparisonPage':
        return styles.comparisonPage;
      case 'InteractivePlayground':
        return styles.interactivePlayground;
      case 'WikiPage':
        return styles.wikiPage;
      case 'Textbook':
        return styles.textbook;
      case 'PersonalNotes':
        return styles.personalNotes;
      case 'MathematicianNotes':
        return styles.mathematicianNotes;
      case 'StudyNotes':
        return styles.studyNotes;
      case 'TypeMappingDisplay':
        return styles.typeMappingDisplay;
      case 'TransformationMapping':
        return styles.transformationMapping;
      default:
        return styles.defaultDocument;
    }
  };

  const getLayoutStyle = (): React.CSSProperties => {
    const config = _flexibleDocument.presentation_config;
    
    // Document types that manage their own layout internally
    const hasOwnLayout = [
      'ResourcePanel', 
      'Textbook', 
      'AnimatedPresentation',
      'ComparisonPage',
      'TypeMappingDisplay',
      'TransformationMapping'
    ].includes(_flexibleDocument.document_type);
    
    if (!config || hasOwnLayout) {
      return { width: '100%' };
    }

    const style: React.CSSProperties = {
      width: '100%'
    };

    // Apply layout style only to documents that don't manage their own layout
    switch (config.layout_style) {
      case 'SingleColumn':
        style.width = '100%';
        style.margin = '0';
        style.padding = '2rem';
        break;
      case 'TwoColumn':
        style.columnCount = 2;
        style.columnGap = '3rem';
        style.width = '100%';
        break;
      case 'Sidebar':
        style.display = 'grid';
        style.gridTemplateColumns = '300px 1fr';
        style.gap = '3rem';
        style.width = '100%';
        break;
      case 'Dashboard':
        style.display = 'grid';
        style.gridTemplateColumns = 'repeat(auto-fit, minmax(350px, 1fr))';
        style.gap = '2rem';
        style.width = '100%';
        break;
      case 'Presentation':
        style.height = '100vh';
        style.display = 'flex';
        style.flexDirection = 'column';
        style.overflow = 'hidden';
        style.width = '100%';
        break;
      case 'Compact':
        style.fontSize = '0.9em';
        style.lineHeight = '1.4';
        style.width = '100%';
        style.padding = '1rem';
        break;
    }

    return style;
  };

  const renderWithDocumentTypeSpecifics = (): React.ReactNode => {
    switch (_flexibleDocument.document_type) {
      case 'AnimatedPresentation':
        return (
          <PresentationWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </PresentationWrapper>
        );
      
      case 'TooltipSummary':
        return (
          <TooltipWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </TooltipWrapper>
        );
      
      case 'ResourcePanel':
        return (
          <ResourcePanelWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </ResourcePanelWrapper>
        );
      
      case 'BlogPost':
        return (
          <BlogPostWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </BlogPostWrapper>
        );
      
      case 'ScientificPaper':
        return (
          <ScientificPaperWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </ScientificPaperWrapper>
        );

      case 'WikiPage':
        return (
          <WikiPageWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </WikiPageWrapper>
        );

      case 'Textbook':
        return (
          <TextbookWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </TextbookWrapper>
        );

      case 'PersonalNotes':
        return (
          <PersonalNotesWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </PersonalNotesWrapper>
        );

      case 'MathematicianNotes':
        return (
          <MathematicianNotesWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </MathematicianNotesWrapper>
        );

      case 'StudyNotes':
        return (
          <StudyNotesWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </StudyNotesWrapper>
        );

      case 'ComparisonPage':
        return (
          <ComparisonPageWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </ComparisonPageWrapper>
        );

      case 'TypeMappingDisplay':
        return (
          <TypeMappingDisplayWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </TypeMappingDisplayWrapper>
        );

      case 'TransformationMapping':
        return (
          <TransformationMappingWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </TransformationMappingWrapper>
        );

      case 'InteractivePlayground':
        return (
          <InteractivePlaygroundWrapper flexibleDocument={_flexibleDocument}>
            {children}
          </InteractivePlaygroundWrapper>
        );
      
      default:
        return children;
    }
  };

  return (
    <div 
      className={`${styles.documentTypeRenderer} ${getDocumentClassName(_flexibleDocument.document_type)}`}
      style={getLayoutStyle()}
      data-document-type={_flexibleDocument.document_type}
      data-formality-level={_flexibleDocument.presentation_config?.formality_level}
      data-audience-level={_flexibleDocument.presentation_config?.target_audience}
    >
      {renderWithDocumentTypeSpecifics()}
    </div>
  );
};

// Specialized wrappers for different document types
const PresentationWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument, 
  children 
}) => {
  const [currentSlide, setCurrentSlide] = React.useState(0);
  const [isPlaying, setIsPlaying] = React.useState(flexibleDocument.presentation_config?.animation_config?.auto_play || false);

  React.useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      switch (e.key) {
        case 'ArrowRight':
        case ' ':
          e.preventDefault();
          // Navigate to next slide logic
          break;
        case 'ArrowLeft':
          e.preventDefault();
          // Navigate to previous slide logic
          break;
        case 'Escape':
          setIsPlaying(false);
          break;
      }
    };

    window.document.addEventListener('keydown', handleKeyPress);
    return () => window.document.removeEventListener('keydown', handleKeyPress);
  }, []);

  return (
    <div className={styles.presentationWrapper}>
      {flexibleDocument.presentation_config?.animation_config?.show_controls && (
        <div className={styles.presentationControls}>
          <button onClick={() => setCurrentSlide(Math.max(0, currentSlide - 1))}>
            Previous
          </button>
          <span>{currentSlide + 1} / {flexibleDocument.body.length}</span>
          <button onClick={() => setCurrentSlide(Math.min(flexibleDocument.body.length - 1, currentSlide + 1))}>
            Next
          </button>
          <button onClick={() => setIsPlaying(!isPlaying)}>
            {isPlaying ? 'Pause' : 'Play'}
          </button>
        </div>
      )}
      <div className={styles.presentationContent}>
        {children}
      </div>
    </div>
  );
};

const TooltipWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument, // Renamed to indicate it's intentionally unused
  children 
}) => {
  return (
    <div className={styles.tooltipWrapper}>
      <div className={styles.tooltipArrow} />
      <div className={styles.tooltipContent}>
        {children}
      </div>
    </div>
  );
};

const ResourcePanelWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument, // Renamed to indicate it's intentionally unused
  children 
}) => {
  const [searchTerm, setSearchTerm] = React.useState('');
  const [selectedCategory, setSelectedCategory] = React.useState('all');

  return (
    <div className={styles.resourcePanelWrapper}>
      <div className={styles.resourceSidebar}>
        <div className={styles.searchBox}>
          <input
            type="text"
            placeholder="Search resources..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className={styles.searchInput}
          />
        </div>
        <nav className={styles.categoryNav}>
          {/* Categories would be extracted from flexibleDocument metadata */}
          <button 
            className={selectedCategory === 'all' ? styles.active : ''}
            onClick={() => setSelectedCategory('all')}
          >
            All Resources
          </button>
          <button 
            className={selectedCategory === 'theorems' ? styles.active : ''}
            onClick={() => setSelectedCategory('theorems')}
          >
            Theorems
          </button>
          <button 
            className={selectedCategory === 'definitions' ? styles.active : ''}
            onClick={() => setSelectedCategory('definitions')}
          >
            Definitions
          </button>
        </nav>
      </div>
      <div className={styles.resourceMainContent}>
        {children}
      </div>
    </div>
  );
};

const BlogPostWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.blogPostWrapper}>
      <div className={styles.blogContainer}>
        <div className={styles.blogHeader}>
          <div className={styles.blogMeta}>
            {flexibleDocument.authors && (
              <div className={styles.authorInfo}>
                <div className={styles.authorAvatar}>
                  {flexibleDocument.authors[0]?.charAt(0) || 'A'}
                </div>
                <div className={styles.authorDetails}>
                  <div className={styles.authorName}>{flexibleDocument.authors[0]}</div>
                  <div className={styles.publishDate}>{flexibleDocument.date_published}</div>
                </div>
              </div>
            )}
            <div className={styles.shareButtons}>
              <button className={styles.shareBtn}>Share</button>
              <button className={styles.saveBtn}>Save</button>
            </div>
          </div>
        </div>
        <article className={styles.blogContent}>
          {children}
        </article>
        <div className={styles.blogFooter}>
          <div className={styles.tags}>
            {/* Tags would be extracted from metadata */}
          </div>
          <div className={styles.relatedPosts}>
            <h3>Related Posts</h3>
            {/* Related posts logic */}
          </div>
        </div>
      </div>
    </div>
  );
};

const ScientificPaperWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument, // Renamed to indicate it's intentionally unused
  children 
}) => {
  return (
    <div className={styles.scientificPaperWrapper}>
      {children}
    </div>
  );
};

const WikiPageWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.wikiPageWrapper}>
      <div className={styles.wikiContainer}>
        {children}
      </div>
    </div>
  );
};

const TextbookWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.textbookWrapper}>
      <div className={styles.textbookContainer}>
        {flexibleDocument.table_of_contents && (
          <div className={styles.textbookToc}>
            <h2>Table of Contents</h2>
            <nav>
              <a href={`#${flexibleDocument.table_of_contents.target_id}`}>
                {flexibleDocument.table_of_contents.title}
              </a>
              {flexibleDocument.table_of_contents.children.map((child, index) => (
                <div key={index} className={styles.tocSubItem}>
                  <a href={`#${child.target_id}`}>{child.title}</a>
                </div>
              ))}
            </nav>
          </div>
        )}
        <div className={styles.textbookContent}>
          {children}
        </div>
      </div>
    </div>
  );
};

const PersonalNotesWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.personalNotesWrapper}>
      <div className={styles.notesContainer}>
        {children}
      </div>
    </div>
  );
};

const MathematicianNotesWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.mathematicianNotesWrapper}>
      <div className={styles.researchContainer}>
        {children}
      </div>
    </div>
  );
};

const StudyNotesWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.studyNotesWrapper}>
      <div className={styles.studyContainer}>
        {children}
      </div>
    </div>
  );
};

const ComparisonPageWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.comparisonPageWrapper}>
      <div className={styles.comparisonContainer}>
        {children}
      </div>
    </div>
  );
};

const TypeMappingDisplayWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.typeMappingWrapper}>
      <div className={styles.typeMappingContainer}>
        {children}
      </div>
    </div>
  );
};

const TransformationMappingWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.transformationMappingWrapper}>
      <div className={styles.transformationContainer}>
        {children}
      </div>
    </div>
  );
};

const InteractivePlaygroundWrapper: React.FC<{ flexibleDocument: FlexibleDocument; children: React.ReactNode }> = ({ 
  flexibleDocument: _flexibleDocument,
  children 
}) => {
  return (
    <div className={styles.interactivePlaygroundWrapper}>
      <div className={styles.playgroundContainer}>
        {children}
      </div>
    </div>
  );
}; 