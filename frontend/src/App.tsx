import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import HomePage from "./pages/HomePage";
import MathPage from "./pages/MathPage";
import DeveloperPage from "./pages/DeveloperPage";
import NotFound from "./pages/NotFound";
import Navigation from "./components/Navigation";
import styles from "./App.module.css";

export const App: React.FC = () => {
  return (
    <BrowserRouter>
      <div className={styles.appContainer}>
        <Navigation />
        <main className={styles.mainContent}>
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/math" element={<MathPage />} />
            <Route path="/roadmap" element={<DeveloperPage />} />
            <Route path="/developer" element={<DeveloperPage />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
};
