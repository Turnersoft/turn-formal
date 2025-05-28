import React from 'react';
import { 
  SideBySideLayout, 
  PanelLayout, 
  AnnotationOverlay, 
  InteractiveControls 
} from '../types';
import { SectionContentRenderer } from '../SectionContentRenderer';
import { RichTextRenderer } from '../RichTextRenderer';
import styles from './styles.module.css';

interface SideBySideRendererProps {
  layout: SideBySideLayout;
}

export const SideBySideRenderer: React.FC<SideBySideRendererProps> = ({ layout }) => {
  const layoutStyle: React.CSSProperties = {
    gridTemplateColumns: layout.layout_config 
      ? `${layout.layout_config.left_width || '1fr'} ${layout.layout_config.right_width || '1fr'}`
      : '1fr 1fr',
    gap: layout.layout_config?.gap || '2rem'
  };

  return (
    <div 
      className={styles.sideBySideLayout}
      style={layoutStyle}
      data-sync-scrolling={layout.sync_scrolling}
      data-highlight-correspondence={layout.highlight_correspondence}
    >
      <SinglePanelRenderer panel={layout.left_panel} />
      <SinglePanelRenderer panel={layout.right_panel} />
    </div>
  );
};

interface PanelLayoutRendererProps {
  layout: PanelLayout;
}

export const PanelLayoutRenderer: React.FC<PanelLayoutRendererProps> = ({ layout }) => {
  const [activePanelId, setActivePanelId] = React.useState(
    layout.panels.find(p => p.initially_visible)?.id || layout.panels[0]?.id
  );

  const renderPanelContent = (panel: any) => (
    <div key={panel.id} className={styles.panelContent}>
      {panel.title && (
        <div className={styles.panelTitle}>
          <RichTextRenderer segments={panel.title.segments} />
        </div>
      )}
      <div className={styles.panelBody}>
        {panel.content.map((contentNode: any, index: number) => (
          <SectionContentRenderer 
            key={index}
            content={contentNode}
            contentIndex={index}
          />
        ))}
      </div>
    </div>
  );

  switch (layout.layout_type) {
    case 'Tabs':
      return (
        <div className={styles.tabsLayout}>
          <div className={styles.tabsList}>
            {layout.panels.map(panel => (
              <button
                key={panel.id}
                className={`${styles.tab} ${activePanelId === panel.id ? styles.tabActive : ''}`}
                onClick={() => setActivePanelId(panel.id)}
              >
                {panel.title ? (
                  <RichTextRenderer segments={panel.title.segments} />
                ) : (
                  panel.id
                )}
              </button>
            ))}
          </div>
          <div className={styles.tabContent}>
            {layout.panels.map(panel => 
              activePanelId === panel.id ? renderPanelContent(panel) : null
            )}
          </div>
        </div>
      );

    case 'Accordion':
      return (
        <div className={styles.accordionLayout}>
          {layout.panels.map(panel => (
            <details 
              key={panel.id} 
              className={styles.accordionPanel}
              open={panel.initially_visible}
            >
              <summary className={styles.accordionSummary}>
                {panel.title ? (
                  <RichTextRenderer segments={panel.title.segments} />
                ) : (
                  panel.id
                )}
              </summary>
              {renderPanelContent(panel)}
            </details>
          ))}
        </div>
      );

    default:
      if (typeof layout.layout_type === 'object' && layout.layout_type.type === 'Grid') {
        const gridStyle: React.CSSProperties = {
          gridTemplateColumns: `repeat(${layout.layout_type.columns}, 1fr)`,
          gap: '1rem'
        };
        
        return (
          <div className={styles.gridLayout} style={gridStyle}>
            {layout.panels.map(panel => (
              <div key={panel.id} className={styles.gridPanel}>
                {renderPanelContent(panel)}
              </div>
            ))}
          </div>
        );
      }
      
      return (
        <div className={styles.defaultLayout}>
          {layout.panels.map(panel => renderPanelContent(panel))}
        </div>
      );
  }
};

// Individual Panel renderer for single panels
const SinglePanelRenderer: React.FC<{ panel: any }> = ({ panel }) => {
  return (
    <div className={`${styles.panel} ${styles[`panel-${panel.panel_role.toLowerCase()}`]}`}>
      {panel.title && (
        <div className={styles.panelTitle}>
          <RichTextRenderer segments={panel.title.segments} />
        </div>
      )}
      <div className={styles.panelContent}>
        {panel.content.map((contentNode: any, index: number) => (
          <SectionContentRenderer 
            key={index}
            content={contentNode}
            contentIndex={index}
          />
        ))}
      </div>
    </div>
  );
};

interface AnnotationOverlayRendererProps {
  overlay: AnnotationOverlay;
}

export const AnnotationOverlayRenderer: React.FC<AnnotationOverlayRendererProps> = ({ overlay }) => {
  return (
    <div className={styles.annotationOverlay} data-overlay-style={overlay.overlay_style}>
      <div className={styles.baseContent}>
        {overlay.base_content.map((contentNode, index) => (
          <SectionContentRenderer 
            key={index}
            content={contentNode}
            contentIndex={index}
          />
        ))}
      </div>
      <div className={styles.annotations}>
        {overlay.annotations.map((annotation, _index) => (
          <div 
            key={annotation.id}
            className={`${styles.annotation} ${styles[`annotation-${annotation.annotation_type.toLowerCase()}`]}`}
            data-target={annotation.target_selector}
            style={{
              position: annotation.position ? 'absolute' : 'relative',
              left: annotation.position?.x,
              top: annotation.position?.y,
              ...annotation.styling
            }}
          >
            <RichTextRenderer segments={annotation.annotation_content} />
          </div>
        ))}
      </div>
    </div>
  );
};

interface InteractiveControlsRendererProps {
  controls: InteractiveControls;
}

export const InteractiveControlsRenderer: React.FC<InteractiveControlsRendererProps> = ({ controls }) => {
  const [controlValues, setControlValues] = React.useState<Record<string, string>>(() => {
    const initial: Record<string, string> = {};
    controls.controls.forEach(control => {
      initial[control.id] = control.default_value;
    });
    return initial;
  });

  const handleControlChange = (controlId: string, value: string) => {
    setControlValues(prev => ({ ...prev, [controlId]: value }));
    // Emit change event for target content
    console.log('Control changed:', controlId, value, 'targets:', controls.target_content_ids);
  };

  const renderControl = (control: any) => {
    const value = controlValues[control.id] || control.default_value;

    if (typeof control.control_type === 'string') {
      switch (control.control_type) {
        case 'Toggle':
          return (
            <label className={styles.toggleControl}>
              <input
                type="checkbox"
                checked={value === 'true'}
                onChange={(e) => handleControlChange(control.id, e.target.checked.toString())}
              />
              <span className={styles.controlLabel}>{control.label}</span>
            </label>
          );
        case 'ColorPicker':
          return (
            <label className={styles.colorControl}>
              <span className={styles.controlLabel}>{control.label}</span>
              <input
                type="color"
                value={value}
                onChange={(e) => handleControlChange(control.id, e.target.value)}
              />
            </label>
          );
      }
    } else if (control.control_type.type) {
      switch (control.control_type.type) {
        case 'Slider':
          return (
            <label className={styles.sliderControl}>
              <span className={styles.controlLabel}>{control.label}</span>
              <input
                type="range"
                min={control.control_type.min}
                max={control.control_type.max}
                step={control.control_type.step}
                value={value}
                onChange={(e) => handleControlChange(control.id, e.target.value)}
              />
              <span className={styles.controlValue}>{value}</span>
            </label>
          );
        case 'Dropdown':
          return (
            <label className={styles.dropdownControl}>
              <span className={styles.controlLabel}>{control.label}</span>
              <select
                value={value}
                onChange={(e) => handleControlChange(control.id, e.target.value)}
              >
                {control.control_type.options.map((option: string) => (
                  <option key={option} value={option}>
                    {option}
                  </option>
                ))}
              </select>
            </label>
          );
        case 'NumberInput':
          return (
            <label className={styles.numberControl}>
              <span className={styles.controlLabel}>{control.label}</span>
              <input
                type="number"
                min={control.control_type.min}
                max={control.control_type.max}
                value={value}
                onChange={(e) => handleControlChange(control.id, e.target.value)}
              />
            </label>
          );
        case 'Button':
          return (
            <button
              className={styles.buttonControl}
              onClick={() => handleControlChange(control.id, 'clicked')}
            >
              {control.label}
            </button>
          );
      }
    }

    return <div>Unknown control type</div>;
  };

  const getLayoutClass = () => {
    if (typeof controls.layout === 'string') {
      return styles[`layout-${controls.layout.toLowerCase()}`];
    } else if (controls.layout.type === 'Grid') {
      return styles.layoutGrid;
    }
    return styles.layoutHorizontal;
  };

  const getLayoutStyle = (): React.CSSProperties => {
    if (typeof controls.layout === 'object' && controls.layout.type === 'Grid') {
      return {
        gridTemplateColumns: `repeat(${controls.layout.columns}, 1fr)`,
        gap: '1rem'
      };
    }
    return {};
  };

  return (
    <div className={`${styles.interactiveControls} ${getLayoutClass()}`} style={getLayoutStyle()}>
      {controls.controls.map(control => (
        <div key={control.id} className={styles.controlWrapper}>
          {renderControl(control)}
          {control.description && (
            <div className={styles.controlDescription}>{control.description}</div>
          )}
        </div>
      ))}
    </div>
  );
};

// Export all renderers as a namespace
export const LayoutRenderers = {
  SideBySideRenderer,
  PanelLayoutRenderer,
  AnnotationOverlayRenderer,
  InteractiveControlsRenderer
}; 