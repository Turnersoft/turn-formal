import React from "react";
import styles from "./definition_detail.module.scss";
import { Definition, Member } from "../../models/math";
import TypeLink from "../type_link/type_link";
import { renderMathText } from "../../utils/formatters";

interface DefinitionDetailProps {
  definition: Definition;
  definitionMap: Map<string, Definition>;
  onScrollToDefinition: (name: string) => void;
}

/**
 * Component for rendering a detailed view of a definition
 */
const DefinitionDetail: React.FC<DefinitionDetailProps> = ({
  definition,
  definitionMap,
  onScrollToDefinition,
}) => {
  return (
    <div className={styles.definition} id={`definition-${definition.name}`}>
      <h3 className={styles.definitionName}>
        {definition.name}
        <span className={styles.typeTag}>{definition.kind}</span>
      </h3>
      {definition.docs && (
        <p className={styles.definitionDocs}>
          {renderMathText(definition.docs)}
        </p>
      )}

      {definition.members && definition.members.length > 0 && (
        <div className={styles.members}>
          <h4>Members</h4>
          <table className={styles.membersTable}>
            <thead>
              <tr>
                <th>Name</th>
                <th>Type</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              {definition.members.map((member: Member, idx: number) => (
                <tr key={idx}>
                  <td>{member.name}</td>
                  <td className={styles.typeCell}>
                    <code>
                      <TypeLink
                        typeStr={member.type}
                        definitionMap={definitionMap}
                        onScrollToDefinition={onScrollToDefinition}
                      />
                    </code>
                  </td>
                  <td>{renderMathText(member.docs)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};

export default DefinitionDetail;
