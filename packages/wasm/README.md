# @diagramma/wasm

[![npm](https://img.shields.io/npm/v/@diagramma/wasm.svg)](https://www.npmjs.com/package/@diagramma/wasm)
[![TypeScript CI](https://github.com/SHA888/diagramma/workflows/TypeScript%20CI/badge.svg)](https://github.com/SHA888/diagramma/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

WASM bridge for [diagramma](https://github.com/SHA888/diagramma) — runs the Rust rendering core in the browser and Node.js.

## What This Package Provides

This package wraps the entire Rust pipeline (validation → layout → SVG generation) as async JavaScript functions. It's the bridge between the Rust crates and the TypeScript ecosystem.

You typically don't use this package directly — [`@diagramma/react`](https://www.npmjs.com/package/@diagramma/react) consumes it. Use this if you need diagramma in a non-React context (vanilla JS, Vue, Svelte, server-side).

## Usage

```typescript
import init, { validateSpec, renderSvg } from '@diagramma/wasm';

// Initialize WASM (once, at app startup)
await init();

const spec = JSON.stringify({
  type: 'flowchart',
  direction: 'top-down',
  nodes: [
    { id: 'a', label: 'Input', color: 'gray' },
    { id: 'b', label: 'Process', color: 'teal' },
  ],
  edges: [{ from: 'a', to: 'b' }],
});

// Validate
const errors = validateSpec(spec);

// Render
const svg = renderSvg(spec); // returns SVG string
```

## API

| Function                              | Input               | Output                                               |
| ------------------------------------- | ------------------- | ---------------------------------------------------- |
| `init()`                              | —                   | Loads WASM binary. Call once before other functions. |
| `validateSpec(json)`                  | Spec JSON string    | Validation errors or null                            |
| `layoutSpec(json)`                    | Spec JSON string    | Layout result as JSON                                |
| `renderSvg(json)`                     | Spec JSON string    | Complete SVG string                                  |
| `renderSvgWithOptions(json, options)` | Spec + options JSON | SVG with theme/size overrides                        |

## Build

This package is compiled from Rust source using `wasm-pack`. To rebuild:

```bash
# From monorepo root
cd crates/diagramma-wasm
wasm-pack build --target web --out-dir ../../packages/wasm/src
```

## Binary Size

Target: <200KB gzipped. WASM binary is optimized with `wasm-opt`.

## Status

Early development. See the [project roadmap](https://github.com/SHA888/diagramma/blob/main/TODO.md) for details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
