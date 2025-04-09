import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import HomePage from "./pages/HomePage/HomePage";
import MathPage from "./pages/MathPage/MathPage";
import DeveloperPage from "./pages/DeveloperPage/DeveloperPage";
import NotFound from "./pages/NotFound/NotFound";
import Navigation from "./components/navigation";
import usePreventNavigationGesture from "./hooks/usePreventNavigationGesture";
import styles from "./App.module.scss";

export const App: React.FC = () => {
  // Use the custom hook to prevent two-finger swipe navigation
  usePreventNavigationGesture();

  return (
    <BrowserRouter>
      <div className={styles.appContainer}>
        <Navigation />
        <main className={styles.mainContent}>
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/math" element={<MathPage />} />
            <Route path="/developer" element={<DeveloperPage />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
};
