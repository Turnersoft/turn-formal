import React from 'react';
import type { LinkTarget } from '../../turn-render/bindings/LinkTarget';
import type { RichTextSegment } from '../../turn-render/bindings/RichTextSegment';
import { RichTextRenderer } from './RichTextRenderer';
import { useMathNavigation } from '../../../services/mathNavigationService';
import styles from './LinkRenderer.module.css';

interface LinkRendererProps {
  content: RichTextSegment[];
  target: LinkTarget;
  tooltip?: string | null;
  className?: string;
}

/**
 * LinkRenderer component that handles different types of links
 * based on the LinkTarget type from the Rust bindings.
 */
export const LinkRenderer: React.FC<LinkRendererProps> = ({ 
  content, 
  target, 
  tooltip, 
  className 
}) => {
  const mathNavigation = useMathNavigation();

  const handleClick = (e: React.MouseEvent) => {
    e.preventDefault();
    
    if ('Url' in target) {
      window.open(target.Url, '_blank', 'noopener,noreferrer');
    } else if ('InternalPageId' in target) {
      // Navigate to internal page
      console.log('Navigate to internal page:', target.InternalPageId);
      // TODO: Implement internal navigation
    } else if ('DefinitionId' in target) {
      // Use navigation service to route to definition
      if (target.DefinitionId.term_id && target.DefinitionId.theory_context) {
        let termId = target.DefinitionId.term_id;
        
        // Handle common incorrect ID patterns
        if (termId.includes('-main-groupbasic-section') && !termId.includes('generic-main-groupbasic-section')) {
          console.log(`Redirecting incorrect basic group reference: ${termId} -> group_theory.generic-main-groupbasic-section`);
          termId = 'group_theory.generic-main-groupbasic-section';
        }
        // Handle other common patterns that should redirect to basic group
        else if (termId.includes('topological-main-groupbasic') || 
                 termId.includes('lie-main-groupbasic') || 
                 termId.includes('cyclic-main-groupbasic') ||
                 termId.includes('symmetric-main-groupbasic') ||
                 termId.includes('dihedral-main-groupbasic') ||
                 termId.includes('alternating-main-groupbasic') ||
                 termId.includes('product-main-groupbasic')) {
          console.log(`Redirecting specialized group reference to basic group: ${termId} -> group_theory.generic-main-groupbasic-section`);
          termId = 'group_theory.generic-main-groupbasic-section';
        }
        
        mathNavigation.navigateToDefinition({
          term_id: termId,
          theory_context: target.DefinitionId.theory_context
        });
      }
    } else if ('DefinitionAspect' in target) {
      // Use navigation service to route to definition aspect
      if (target.DefinitionAspect.term_id && target.DefinitionAspect.theory_context) {
        let termId = target.DefinitionAspect.term_id;
        
        // Handle common incorrect ID patterns
        if (termId.includes('-main-groupbasic-section') && !termId.includes('generic-main-groupbasic-section')) {
          console.log(`Redirecting incorrect basic group aspect reference: ${termId} -> group_theory.generic-main-groupbasic-section`);
          termId = 'group_theory.generic-main-groupbasic-section';
        }
        // Handle other common patterns that should redirect to basic group
        else if (termId.includes('topological-main-groupbasic') || 
                 termId.includes('lie-main-groupbasic') || 
                 termId.includes('cyclic-main-groupbasic') ||
                 termId.includes('symmetric-main-groupbasic') ||
                 termId.includes('dihedral-main-groupbasic') ||
                 termId.includes('alternating-main-groupbasic') ||
                 termId.includes('product-main-groupbasic')) {
          console.log(`Redirecting specialized group aspect reference to basic group: ${termId} -> group_theory.generic-main-groupbasic-section`);
          termId = 'group_theory.generic-main-groupbasic-section';
        }
        
        mathNavigation.navigateToDefinition({
          term_id: termId,
          theory_context: target.DefinitionAspect.theory_context
        });
      }
    } else if ('TheoremId' in target) {
      // Use navigation service to route to theorem
      const theoremTarget = target.TheoremId;
      if (typeof theoremTarget === 'string') {
        // If it's just a string, try to scroll to it on the current page
        console.log('Navigate to theorem (string):', theoremTarget);
        scrollToElement(theoremTarget);
      } else if (theoremTarget && typeof theoremTarget === 'object' && 'theorem_id' in theoremTarget && 'theory_context' in theoremTarget) {
        // If it's an object with theorem_id and theory_context
        const theorem = theoremTarget as { theorem_id: string; theory_context: string };
        mathNavigation.navigateToTheorem({
          theorem_id: theorem.theorem_id,
          theory_context: theorem.theory_context
        });
      } else {
        console.log('Navigate to theorem:', theoremTarget);
        scrollToElement(String(theoremTarget));
      }
    } else if ('GlossaryTerm' in target) {
      console.log('Navigate to glossary term:', target.GlossaryTerm);
      // TODO: Open glossary modal or navigate to glossary
    } else if ('InteractiveElementId' in target) {
      console.log('Trigger interactive element:', target.InteractiveElementId);
      // TODO: Trigger interactive element
    } else {
      console.log('Unsupported link target:', target);
    }
  };

  const scrollToElement = (elementId: string) => {
    // Try to find the element on the current page
    const element = document.getElementById(elementId);
    if (element) {
      element.scrollIntoView({ 
        behavior: 'smooth', 
        block: 'start' 
      });
      // Add highlight effect
      element.classList.add(styles.highlighted);
      setTimeout(() => {
        element.classList.remove(styles.highlighted);
      }, 2000);
    } else {
      console.log(`Element with id "${elementId}" not found on current page`);
    }
  };

  const getLinkClassName = () => {
    const baseClass = styles.link;
    const typeClass = getLinkTypeClass();
    return [baseClass, typeClass, className].filter(Boolean).join(' ');
  };

  const getLinkTypeClass = () => {
    if ('Url' in target) return styles.urlLink;
    if ('InternalPageId' in target) return styles.internalLink;
    if ('DefinitionId' in target) return styles.definitionLink;
    if ('DefinitionAspect' in target) return styles.definitionLink;
    if ('TheoremId' in target) return styles.theoremLink;
    if ('GlossaryTerm' in target) return styles.glossaryLink;
    return styles.defaultLink;
  };

  const getTooltipText = () => {
    if (tooltip) return tooltip;
    
    if ('DefinitionId' in target) {
      return `View definition: ${target.DefinitionId.term_id}`;
    }
    if ('TheoremId' in target) {
      return `View theorem: ${target.TheoremId}`;
    }
    if ('GlossaryTerm' in target) {
      return `View glossary term: ${target.GlossaryTerm}`;
    }
    if ('Url' in target) {
      return `External link: ${target.Url}`;
    }
    
    return 'Click to navigate';
  };

  return (
    <a
      href="#"
      className={getLinkClassName()}
      onClick={handleClick}
      title={getTooltipText()}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          handleClick(e as any);
        }
      }}
    >
      <RichTextRenderer segments={content} />
    </a>
  );
};

export default LinkRenderer; 