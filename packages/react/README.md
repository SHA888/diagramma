# @diagramma/react

[![npm](https://img.shields.io/npm/v/@diagramma/react.svg)](https://www.npmjs.com/package/@diagramma/react)

React component for [diagramma](https://github.com/SHA888/diagramma) — render interactive, themed diagrams from a declarative spec.

## What This Package Provides

A `<Diagram />` component that takes a JSON spec and renders a polished, interactive SVG with dark/light mode support, clickable nodes, hover effects, and keyboard navigation.

## Usage

```bash
pnpm add @diagramma/react @diagramma/wasm @diagramma/theme
```

```tsx
import { Diagram } from '@diagramma/react';

const spec = {
  type: 'flowchart',
  direction: 'top-down',
  nodes: [
    { id: 'input', label: 'User request', color: 'gray' },
    { id: 'auth', label: 'Authentication', subtitle: 'Validate JWT', color: 'blue' },
    { id: 'process', label: 'Process', subtitle: 'Business logic', color: 'teal' },
    { id: 'output', label: 'Response', color: 'green' },
  ],
  edges: [
    { from: 'input', to: 'auth' },
    { from: 'auth', to: 'process' },
    { from: 'process', to: 'output' },
  ],
};

function App() {
  return (
    <Diagram
      spec={spec}
      theme="auto"
      onNodeClick={(nodeId) => console.log('Clicked:', nodeId)}
      onNodeHover={(nodeId) => console.log('Hovered:', nodeId)}
    />
  );
}
```

## Props

| Prop | Type | Default | Description |
|---|---|---|---|
| `spec` | `DiagramSpec` | *required* | Diagram specification object |
| `theme` | `'light' \| 'dark' \| 'auto'` | `'auto'` | Color theme |
| `onNodeClick` | `(nodeId: string) => void` | — | Click callback |
| `onNodeHover` | `(nodeId: string \| null) => void` | — | Hover callback (null on leave) |
| `className` | `string` | — | CSS class on wrapper element |
| `style` | `CSSProperties` | — | Inline styles on wrapper |
| `width` | `number \| string` | ViewBox-based | Override width |
| `height` | `number \| string` | ViewBox-based | Override height |

## Features

- **Auto-layout** — no manual coordinate placement
- **Dark/light/auto theme** — adapts to system preference or explicit prop
- **Interactivity** — click and hover callbacks, keyboard navigation (tab + enter)
- **Error boundary** — invalid spec shows error UI, not a crash
- **Lazy WASM loading** — shows loading state while WASM initializes
- **Memoized rendering** — re-renders only on spec or theme change

## Peer Dependencies

- `react >= 18`
- `@diagramma/wasm`
- `@diagramma/theme`

## Status

Early development. See the [project roadmap](https://github.com/SHA888/diagramma/blob/main/TODO.md) for details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
