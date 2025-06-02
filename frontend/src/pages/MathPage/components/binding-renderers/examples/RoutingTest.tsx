import React from 'react';
import { useMathNavigation } from '../../../services/mathNavigationService';
import styles from './RoutingTest.module.css';

/**
 * Test component demonstrating the mathematical content routing system
 */
export const RoutingTest: React.FC = () => {
  const mathNavigation = useMathNavigation();

  const testNavigations = [
    {
      type: 'definition',
      title: 'Basic Group Definition',
      description: 'Navigate to the fundamental group definition',
      action: () => mathNavigation.navigateToDefinition({
        term_id: 'group_theory.generic-main-groupbasic-section',
        theory_context: 'GroupTheory'
      })
    },
    {
      type: 'definition',
      title: 'Cyclic Group',
      description: 'Navigate to the cyclic group definition',
      action: () => mathNavigation.navigateToDefinition({
        term_id: 'group_theory.cyclic-main-cyclicgroup-section',
        theory_context: 'GroupTheory'
      })
    },
    {
      type: 'definition',
      title: 'Symmetric Group',
      description: 'Navigate to the symmetric group definition',
      action: () => mathNavigation.navigateToDefinition({
        term_id: 'group_theory.symmetric-main-symmetricgroup-section',
        theory_context: 'GroupTheory'
      })
    },
    {
      type: 'theorem',
      title: 'Group Theorem',
      description: 'Navigate to a group theory theorem',
      action: () => mathNavigation.navigateToTheorem({
        theorem_id: 'inverse_uniqueness',
        theory_context: 'GroupTheory'
      })
    },
    {
      type: 'theory',
      title: 'Group Theory Overview',
      description: 'Navigate to Group Theory overview',
      action: () => mathNavigation.navigateToTheory('GroupTheory')
    }
  ];

  return (
    <div className={styles.routingTest}>
      <h2>🧭 Mathematical Content Routing Test</h2>
      <p>
        This component demonstrates the routing system for mathematical content.
        Click the buttons below to navigate to specific definitions, theorems, and theory overviews.
      </p>

      <div className={styles.navigationGrid}>
        {testNavigations.map((nav, index) => (
          <div key={index} className={styles.navigationCard}>
            <h3 className={styles.navigationTitle}>
              {nav.type === 'definition' && '📖'}
              {nav.type === 'theorem' && '⚡'}
              {nav.type === 'theory' && '🏛️'}
              {' '}{nav.title}
            </h3>
            <p className={styles.navigationDescription}>
              {nav.description}
            </p>
            <button 
              className={styles.navigationButton}
              onClick={nav.action}
            >
              Navigate →
            </button>
          </div>
        ))}
      </div>

      <div className={styles.routingInfo}>
        <h3>🔍 How the Routing Works</h3>
        <ul>
          <li><strong>Definitions:</strong> <code>/math/definition/&#123;theory&#125;/&#123;termId&#125;</code></li>
          <li><strong>Theorems:</strong> <code>/math/theorem/&#123;theory&#125;/&#123;theoremId&#125;</code></li>
          <li><strong>Theory Overview:</strong> <code>/math/theory/&#123;theoryName&#125;</code></li>
        </ul>
        
        <h4>📁 Theory to File Mapping</h4>
        <ul>
          <li><strong>GroupTheory:</strong> group_theory.definitions.json, group_theory.theorems.json</li>
          <li><strong>FieldTheory:</strong> field_theory.definitions.json, field_theory.theorems.json</li>
          <li><strong>RingTheory:</strong> ring_theory.definitions.json, ring_theory.theorems.json</li>
        </ul>
      </div>
    </div>
  );
};

export default RoutingTest; 