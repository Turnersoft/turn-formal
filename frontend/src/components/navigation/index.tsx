import React, { useState } from "react";
import { Link, useLocation } from "react-router-dom";
import styles from "./navigation.module.scss";

const Navigation: React.FC = () => {
  const location = useLocation();
  const [isMobileMenuOpen] = useState(false);

  const isActive = (path: string) => location.pathname === path;

  return (
    <nav className={styles.navigation}>
      <div className={styles.container}>
        <div className={styles.logo}>
          <Link to="/">Turn-Formal</Link>
        </div>
        <ul
          className={`${styles.navLinks} ${
            isMobileMenuOpen ? styles.open : ""
          }`}
        >
          <li>
            <Link
              to="/foundations"
              className={isActive("/foundations") ? styles.active : ""}
            >
              <span>Foundations</span>
            </Link>
          </li>
          <li>
            <Link to="/math" className={isActive("/math") ? styles.active : ""}>
              <span>Math</span>
            </Link>
          </li>
          <li>
            <Link
              to="/logic"
              className={isActive("/logic") ? styles.active : ""}
            >
              <span>Logic</span>
            </Link>
          </li>
          <li>
            <Link
              to="/developer"
              className={isActive("/developer") ? styles.active : ""}
            >
              <span>Developer</span>
            </Link>
          </li>
        </ul>
      </div>
    </nav>
  );
};

export default Navigation;
