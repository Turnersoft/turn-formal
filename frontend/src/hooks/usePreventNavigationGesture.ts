import { useEffect } from "react";

/**
 * A custom hook that prevents the browser's back/forward navigation when using
 * two-finger swipe gestures on a Mac trackpad, without affecting normal scrolling.
 */
export function usePreventNavigationGesture() {
  useEffect(() => {
    // Create an initial history state to return to
    window.history.pushState(null, "", window.location.pathname);

    // This handler intercepts back/forward navigation attempts
    const handlePopState = () => {
      // Push another state to prevent actual navigation
      window.history.pushState(null, "", window.location.pathname);
    };

    // Add event listener for history changes
    window.addEventListener("popstate", handlePopState);

    // Clean up on unmount
    return () => {
      window.removeEventListener("popstate", handlePopState);
    };
  }, []);
}

export default usePreventNavigationGesture;
