import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import HomePage from "./pages/HomePage/HomePage";
import MathPage from "./pages/MathPage/MathPage";
import DeveloperPage from "./pages/DeveloperPage/DeveloperPage";
import NotFound from "./pages/NotFound/NotFound";
import Navigation from "./components/navigation";
import usePreventNavigationGesture from "./hooks/usePreventNavigationGesture";
import TestMathRendering from "./test-math-rendering";
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
            {/* Math section routes - include specific content routes */}
            <Route path="/math" element={<MathPage />} />
            <Route path="/math/definition/:theory/:termId" element={<MathPage />} />
            <Route path="/math/theorem/:theory/:theoremId" element={<MathPage />} />
            <Route path="/math/theory/:theoryName" element={<MathPage />} />
            <Route path="/math/*" element={<MathPage />} />
            {/* Developer section */}
            <Route path="/developer" element={<DeveloperPage />} />
            {/* Test routes */}
            <Route path="/test-math" element={<TestMathRendering />} />
            {/* Catch-all for not found pages */}
            <Route path="*" element={<NotFound />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
};
