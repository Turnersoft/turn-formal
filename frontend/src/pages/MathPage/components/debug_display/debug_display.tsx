import React, { useState } from "react";
import styles from "./debug_display.module.scss";

interface DebugDisplayProps {
  data: any;
  title: string;
}

/**
 * Component for displaying JSON data for debugging purposes
 */
const DebugDisplay: React.FC<DebugDisplayProps> = ({ data, title }) => {
  const [show, setShow] = useState(false);

  return (
    <div className={styles.debugContainer}>
      <button onClick={() => setShow(!show)} className={styles.debugButton}>
        {show ? "Hide" : "Show"} {title} Data
      </button>

      {show && (
        <pre className={styles.debugPre}>{JSON.stringify(data, null, 2)}</pre>
      )}
    </div>
  );
};

export default DebugDisplay;
