import React, { useState } from "react";
import styles from "./DeveloperPage.module.scss";

const DeveloperPage: React.FC = () => {
  const [showAlert, setShowAlert] = useState(false);
  const [showTooltip, setShowTooltip] = useState(false);

  return (
    <div className={styles.developerPage}>
      <h1>Project Roadmap</h1>

      <div className={styles.card}>
        <h2>Project Vision</h2>
        <p>
          Turn-Formal is a unified framework for formal systems across
          mathematics, logic, law, and algorithms. It allows formal reasoning to
          be verified across different foundational theories such as Type
          Theory, Set Theory, and Category Theory.
        </p>
      </div>

      <div className={styles.card}>
        <h2>Current Development Focus</h2>
        <ul className={styles.roadmapList}>
          <li>
            <h3>Foundation-Agnostic Formal Proof Language</h3>
            <p>
              Creating a formal proof language that works with definitions and
              connectives rather than directly with a specific theory. This
              approach allows proofs to be converted to various formalisms like
              topos theory, cubical type theory, or category theory, providing
              multiple perspectives on the same formal reasoning.
            </p>
          </li>
          <li>
            <h3>Formal Subject Domains</h3>
            <p>Building formalized definitions for various subject domains:</p>
            <ul className={styles.subList}>
              <li>Mathematics: Number theory, analysis, algebra</li>
              <li>
                Logic: Propositional, predicate, modal, and non-classical logics
              </li>
              <li>Law: Formalized legal reasoning and jurisprudence</li>
              <li>
                Algorithms: Verified algorithms with proofs of correctness
              </li>
            </ul>
          </li>
          <li>
            <h3>Verification Systems</h3>
            <p>
              Developing automatic verification tools that can check formal
              proofs across multiple foundational theories, ensuring consistency
              and correctness.
            </p>
          </li>
        </ul>
      </div>

      <div className={styles.card}>
        <h2>UI Components Demo</h2>
        <div className={styles.buttonsDemo}>
          <button
            className={styles.button}
            onClick={() => setShowAlert(!showAlert)}
          >
            Toggle Alert
          </button>

          <sl-tooltip open={showTooltip} placement="top">
            <div slot="content">Interactive tooltip with Shoelace</div>
            <button
              className={`${styles.button} ${styles.secondary}`}
              onMouseEnter={() => setShowTooltip(true)}
              onMouseLeave={() => setShowTooltip(false)}
            >
              Hover me
            </button>
          </sl-tooltip>
        </div>

        {showAlert && (
          <div className={styles.alert}>
            This is a simple alert component in React!
          </div>
        )}
      </div>
    </div>
  );
};

export default DeveloperPage;
