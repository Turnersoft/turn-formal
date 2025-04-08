/// <reference types="vite/client" />

declare module "*.module.css" {
  const classes: { [key: string]: string };
  export default classes;
}

declare module "*.css" {
  const css: string;
  export default css;
}

declare module "*.module.scss" {
  const classes: { [key: string]: string };
  export default classes;
}

declare module "*.scss" {
  const scss: string;
  export default scss;
}

declare module "@shoelace-style/shoelace/*" {
  const value: any;
  export default value;
}

declare module "@shoelace-style/shoelace/dist/utilities/base-path" {
  export function setBasePath(path: string): void;
}

// Add Shoelace custom elements to JSX
declare namespace JSX {
  interface IntrinsicElements {
    "sl-button": any;
    "sl-icon": any;
    "sl-alert": any;
    "sl-card": any;
    "sl-dialog": any;
    "sl-spinner": any;
    "sl-input": any;
    "sl-select": any;
    "sl-option": any;
    "sl-tab-group": any;
    "sl-tab": any;
    "sl-tab-panel": any;
    "sl-divider": any;
    "sl-badge": any;
    "sl-tooltip": any;
  }
}
