# diagramma

Conversation-aware diagram rendering engine.

Diagramma takes semantic diagram descriptions (nodes, edges, containers) and produces polished, interactive, theme-aware SVG with dark/light mode adaptation, clickable elements, and LLM conversation integration.

## Why

Existing diagram tools fall short in different ways. Mermaid produces static SVG from a text DSL with no interactivity or theming. Draw.io and Excalidraw are manual canvas editors with no programmatic generation. D3.js is low-level SVG manipulation with no diagram-level abstractions.

Diagramma bridges the gap: a **declarative spec** in, a **polished interactive SVG** out — with conversation-awareness built into the architecture.

## Architecture

Diagramma is a three-layer system:

**Rust core** (compiled to WASM for browser use):

| Crate | Purpose |
|---|---|
| [`diagramma-core`](crates/diagramma-core/) | Type definitions, spec validation, serialization |
| [`diagramma-layout`](crates/diagramma-layout/) | Auto-layout algorithms — hierarchical, tree packing, arrow routing |
| [`diagramma-svg`](crates/diagramma-svg/) | Layout result → themed, accessible SVG output |

**TypeScript packages** (browser/Node.js):

| Package | Purpose |
|---|---|
| [`@diagramma/wasm`](packages/wasm/) | WASM bridge — Rust core in the browser |
| [`@diagramma/theme`](packages/theme/) | Design tokens, color ramps, CSS variables |
| [`@diagramma/react`](packages/react/) | `<Diagram spec={...} />` React component |
| [`@diagramma/bridge`](packages/bridge/) | LLM conversation ↔ diagram integration |
| [`diagramma`](packages/diagramma/) | Umbrella package |

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed system design and data flow.

## Diagram Types

Diagramma supports four diagram types, each with a dedicated layout algorithm:

- **Flowchart** — sequential processes, decisions, data flow. Hierarchical layered layout.
- **Structural** — containment/nesting (e.g., AWS VPC diagrams). Recursive tree packing.
- **Illustrative** — freeform shapes for building intuition. Constraint-based positioning.
- **Interactive** — HTML+SVG widgets with controls (sliders, toggles) that manipulate the diagram.

## Design System

Diagramma enforces a consistent visual language:

- **9 color ramps** (purple, teal, coral, pink, gray, blue, green, amber, red) × 7 stops each
- **Dark/light mode** via CSS variables — runtime switching without re-render
- **Flat aesthetic** — no gradients, no shadows, 0.5px strokes
- **Typography** — 14px labels, 12px subtitles, sentence case
- **Auto-sizing** — boxes sized to fit text content with proper padding

## Status

Early development. See [TODO.md](TODO.md) for the full roadmap.

Currently at **v0.0.1** — name reservation and project scaffold.

## Development

Prerequisites: Rust 1.85+, pnpm 10+, wasm-pack.

```bash
# Rust
cargo check
cargo test
cargo clippy

# TypeScript
pnpm install
pnpm build
pnpm test
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
