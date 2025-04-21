import React from "react";
import { Link } from "react-router-dom";
import styles from "./HomePage.module.scss";

const HomePage: React.FC = () => {
  return (
    <div className={styles.homePage}>
      <h1>Turn-Formal</h1>
      <p className={styles.description}>
        A unified formal framework for mathematics, logic, law, and algorithms
        verified using various foundational theories.
      </p>
      <div className={styles.features}>
        <div className={styles.feature}>
          <h2>Formal Mathematics</h2>
          <p>
            Rigorous mathematical proofs and definitions verified across
            multiple foundational theories including Type Theory, Set Theory,
            and Category Theory.
          </p>
        </div>
        <div className={styles.feature}>
          <h2>Formal Logic</h2>
          <p>
            Logical reasoning systems that provide precise frameworks for
            deduction, verification, and philosophical analysis.
          </p>
        </div>
        <div className={styles.feature}>
          <h2>Formal Law</h2>
          <p>
            Formalized legal reasoning and arguments with precise semantics that
            can be analyzed across different jurisprudential foundations.
          </p>
        </div>
        <div className={styles.feature}>
          <h2>Verified Algorithms</h2>
          <p>
            Algorithms with formal proofs of correctness, complexity, and
            termination guaranteed by foundational theories.
          </p>
        </div>
      </div>
      <div className={styles.cta}>
        <Link to="/developer" className={styles.ctaButton}>
          Explore the Roadmap
        </Link>
      </div>
      <div className={styles.buttonContainer}>
        <Link to="/math" className={styles.actionButton}>
          <div className={styles.buttonIcon}>📚</div>
          <div className={styles.buttonText}>
            <h3>Mathematics</h3>
            <p>Explore mathematical concepts and theories</p>
          </div>
        </Link>
        
        <Link to="/math/theories/groups" className={styles.actionButton}>
          <div className={styles.buttonIcon}>🧮</div>
          <div className={styles.buttonText}>
            <h3>Group Theory</h3>
            <p>View group theory theorems and definitions</p>
          </div>
        </Link>
        
        <Link to="/developer" className={styles.actionButton}>
          <div className={styles.buttonIcon}>👨‍💻</div>
          <div className={styles.buttonText}>
            <h3>Developer Tools</h3>
            <p>Utilities and documentation for developers</p>
          </div>
        </Link>
      </div>
    </div>
  );
};

export default HomePage;
