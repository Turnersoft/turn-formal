import React from "react";
import SlCard from "@shoelace-style/shoelace/dist/react/card";

import styles from "./toc.module.scss";

interface Content {
  name: string;
  id: string;
}

interface TOCProps {
  contents: Content[];
  onClick: (id: string) => void;
}

export const TOC: React.FC<TOCProps> = ({ contents, onClick }) => {
  return (
    <SlCard className={styles.toc}>
      {contents.map((content) => (
        <div
          key={`toc-item-${content.id}`}
          id={`toc-item-${content.id}`}
          className={styles.tocItem}
        >
          <button onClick={() => onClick(content.id)}>{content.name}</button>
        </div>
      ))}
    </SlCard>
  );
};
