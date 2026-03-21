# diagramma

[![npm](https://img.shields.io/npm/v/diagramma.svg)](https://www.npmjs.com/package/diagramma)

Conversation-aware diagram rendering engine.

Diagramma takes semantic diagram descriptions (nodes, edges, containers) and produces polished, interactive, theme-aware SVG with dark/light mode adaptation, clickable elements, and LLM conversation integration.

## Packages

This is the umbrella package. The actual functionality lives in scoped packages:

| Package                                                                | Purpose                                          |
| ---------------------------------------------------------------------- | ------------------------------------------------ |
| [`@diagramma/wasm`](https://www.npmjs.com/package/@diagramma/wasm)     | WASM bridge — Rust rendering core in the browser |
| [`@diagramma/theme`](https://www.npmjs.com/package/@diagramma/theme)   | Design tokens, color ramps, CSS variables        |
| [`@diagramma/react`](https://www.npmjs.com/package/@diagramma/react)   | `<Diagram spec={...} />` React component         |
| [`@diagramma/bridge`](https://www.npmjs.com/package/@diagramma/bridge) | LLM conversation ↔ diagram integration          |

## Quick Start

```bash
pnpm add @diagramma/react
```

```tsx
import { Diagram } from '@diagramma/react';

const spec = {
  type: 'flowchart',
  direction: 'top-down',
  nodes: [
    { id: 'a', label: 'Start', color: 'gray' },
    { id: 'b', label: 'Process', color: 'teal' },
    { id: 'c', label: 'End', color: 'green' },
  ],
  edges: [
    { from: 'a', to: 'b' },
    { from: 'b', to: 'c' },
  ],
};

function App() {
  return <Diagram spec={spec} theme="auto" onNodeClick={console.log} />;
}
```

## How It Works

```
JSON spec → Validate → Layout → SVG
             (Rust)    (Rust)   (Rust)
                                  ↓
                           WASM bridge
                                  ↓
                         React component
                                  ↓
                    Interactive SVG in the DOM
```

The Rust core handles validation, auto-layout, and SVG generation. It's compiled to WebAssembly and loaded in the browser via `@diagramma/wasm`. The React component wraps this pipeline with interactivity (click, hover, keyboard navigation) and theming.

## Repository

[github.com/SHA888/diagramma](https://github.com/SHA888/diagramma)

## Status

Early development. See the [project roadmap](https://github.com/SHA888/diagramma/blob/main/TODO.md) for details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
