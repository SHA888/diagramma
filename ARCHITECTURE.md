# diagramma — Architecture

## Overview

Diagramma is a three-layer system that transforms semantic diagram descriptions into polished, interactive, theme-aware SVG. The layers are:

1. **Rust core** — type definitions, layout algorithms, SVG generation
2. **TypeScript packages** — WASM bridge, design tokens, React component
3. **Conversation bridge** — LLM integration for context-aware diagram generation

```
                        ┌─────────────────────────────────────────────┐
                        │              Conversation                   │
                        │                                             │
                        │  LLM response ──→ @diagramma/bridge         │
                        │                     │                       │
                        │                     ▼                       │
                        │               DiagramSpec (JSON)            │
                        │                     │                       │
                        └─────────────────────┼───────────────────────┘
                                              │
                        ┌─────────────────────┼───────────────────────┐
                        │              Browser / Node.js              │
                        │                     │                       │
                        │                     ▼                       │
                        │  @diagramma/react ─→ @diagramma/wasm        │
                        │  <Diagram spec />     │ (WASM boundary)     │
                        │                       │                     │
                        └───────────────────────┼─────────────────────┘
                                                │
                        ┌───────────────────────┼─────────────────────┐
                        │              Rust / WASM                    │
                        │                       │                     │
                        │                       ▼                     │
                        │    diagramma-core (validate)                │
                        │         │                                   │
                        │         ▼                                   │
                        │    diagramma-layout (position)              │
                        │         │                                   │
                        │         ▼                                   │
                        │    diagramma-svg (render)                   │
                        │         │                                   │
                        │         ▼                                   │
                        │      SVG string                             │
                        │                                             │
                        └─────────────────────────────────────────────┘
```

## Data Flow

A diagram goes through four stages, each owned by a distinct module:

```
JSON spec ──→ Validated spec ──→ Layout result ──→ SVG string
         parse/validate      compute positions    generate markup
         (diagramma-core)    (diagramma-layout)   (diagramma-svg)
```

### Stage 1: Parse & Validate (diagramma-core)

Input: JSON string conforming to the DiagramSpec schema.

Operations:

- Deserialize into typed Rust structs (serde)
- Validate referential integrity (edges reference existing nodes)
- Validate constraints (no duplicate IDs, nesting depth limits)
- Validate color ramp names

Output: `DiagramSpec` enum — one of `Flowchart`, `Structural`, `Illustrative`, `Interactive`.

### Stage 2: Layout (diagramma-layout)

Input: Validated `DiagramSpec`.

Operations (algorithm depends on diagram type):

- **Flowchart** — hierarchical layered layout (Sugiyama-style):
  1. Cycle removal (reverse back-edges)
  2. Layer assignment (longest path)
  3. Crossing minimization (barycenter heuristic)
  4. Coordinate assignment (Brandes-Köpf)
- **Structural** — recursive tree packing:
  1. Leaf-to-root: compute minimum bounding box per container
  2. Root-to-leaf: assign coordinates with padding
  3. Sibling packing within shared parent
- **Illustrative** — constraint-based freeform (later phase)
- **Interactive** — delegates to one of the above for the base diagram

All algorithms apply these constraints:

- ViewBox width: 680px, safe area x=40..640
- Box sizing: `width = max(title_chars × 8, subtitle_chars × 7) + 24`
- Minimum spacing: 60px between boxes
- Padding: 24px inside containers, 12px text-to-edge
- Horizontal tier max: 4 boxes at full width

Output: `LayoutResult` — every element has x, y, width, height; every edge has a routed path; viewBox dimensions are computed.

### Stage 3: SVG Generation (diagramma-svg)

Input: `LayoutResult` + theme selection (light/dark/auto).

Operations:

- Generate `<style>` block with CSS variables for selected theme
- Render containers (background rects with labels)
- Render edges (paths with `fill="none"`, 0.5px stroke, open chevron markers)
- Render nodes (rects/pills/diamonds/circles with text)
- Apply color ramp mapping per theme mode
- Add accessibility attributes (`<title>`, `<desc>`, `aria-label`)

Output: SVG string (standalone or fragment).

### Stage 4: Browser Integration (TypeScript packages)

The SVG string is consumed by the TypeScript layer:

- `@diagramma/wasm` — exposes the Rust pipeline as async JS functions
- `@diagramma/theme` — provides CSS variables and color constants for the host page
- `@diagramma/react` — wraps everything in `<Diagram spec={...} />` with interactivity
- `@diagramma/bridge` — connects diagram events to LLM conversation flow

## Crate Architecture

### diagramma-core

```
diagramma-core/
└── src/
    ├── lib.rs              # Public API re-exports
    ├── spec/
    │   ├── mod.rs          # DiagramSpec enum
    │   ├── flowchart.rs    # FlowchartSpec
    │   ├── structural.rs   # StructuralSpec
    │   ├── illustrative.rs # IllustrativeSpec
    │   └── interactive.rs  # InteractiveSpec
    ├── types/
    │   ├── mod.rs          # Re-exports
    │   ├── node.rs         # Node, NodeId, Shape
    │   ├── edge.rs         # Edge, EdgeStyle, ArrowType
    │   ├── container.rs    # Container, Element enum
    │   ├── color.rs        # ColorRamp enum, color stop values
    │   └── theme.rs        # Theme enum, Direction enum
    ├── validate/
    │   ├── mod.rs          # validate(spec) -> Result<(), Vec<Error>>
    │   └── error.rs        # Validation error types
    └── schema/
        └── mod.rs          # JSON schema generation
```

Dependencies: `serde`, `serde_json`, `thiserror`. Zero heavy dependencies.

### diagramma-layout

```
diagramma-layout/
└── src/
    ├── lib.rs              # Public API: layout(spec) -> LayoutResult
    ├── result.rs           # LayoutResult, LayoutNode, LayoutEdge, LayoutContainer
    ├── measure.rs          # Text measurement, box auto-sizing
    ├── flowchart/
    │   ├── mod.rs          # Flowchart layout entry point
    │   ├── layering.rs     # Layer assignment
    │   ├── ordering.rs     # Crossing minimization
    │   └── positioning.rs  # Coordinate assignment
    ├── structural/
    │   ├── mod.rs          # Structural layout entry point
    │   └── packing.rs      # Tree packing algorithm
    ├── routing/
    │   ├── mod.rs          # Arrow routing entry point
    │   ├── direct.rs       # Straight-line paths
    │   ├── orthogonal.rs   # L-bend routing
    │   └── avoidance.rs    # Obstacle avoidance
    └── viewbox.rs          # ViewBox computation
```

Dependencies: `diagramma-core`. Possibly `petgraph` for graph operations (evaluate if custom graph is lighter).

### diagramma-svg

```
diagramma-svg/
└── src/
    ├── lib.rs              # Public API: render(layout, theme) -> String
    ├── tokens.rs           # Color ramp values, theme mappings
    ├── style.rs            # CSS variable / <style> block generation
    ├── elements/
    │   ├── mod.rs          # Element rendering dispatch
    │   ├── node.rs         # Rect, pill, diamond, circle rendering
    │   ├── edge.rs         # Path + marker rendering
    │   ├── container.rs    # Nested container rendering
    │   └── text.rs         # Text element rendering (labels, subtitles)
    ├── markers.rs          # Arrow marker definitions (<defs>)
    ├── accessibility.rs    # <title>, <desc>, aria attributes
    └── output.rs           # Output modes: standalone, fragment, inline styles
```

Dependencies: `diagramma-core`, `diagramma-layout`. String building only — no XML crate needed (SVG output is simple enough for direct string construction with proper escaping).

## Package Architecture

### @diagramma/wasm

Thin binding layer. Compiled by `wasm-pack` from a dedicated `crates/diagramma-wasm/` crate (internal, not published to crates.io).

```
packages/wasm/
├── package.json
├── src/                   # wasm-pack output destination
│   ├── diagramma_wasm.js  # Generated JS glue
│   ├── diagramma_wasm.d.ts
│   └── diagramma_wasm_bg.wasm
└── index.ts               # Re-export with ergonomic API
```

### @diagramma/theme

Pure TypeScript. No WASM dependency.

```
packages/theme/
├── package.json
├── src/
│   ├── index.ts           # Public API
│   ├── ramps.ts           # Color ramp definitions
│   ├── tokens.ts          # Light/dark token sets
│   └── css.ts             # CSS custom property generator
└── diagramma.css           # Pre-built CSS file (importable)
```

### @diagramma/react

React component library.

```
packages/react/
├── package.json
├── src/
│   ├── index.ts           # Public API
│   ├── Diagram.tsx        # Main component
│   ├── hooks/
│   │   ├── useDiagramma.ts # WASM initialization + rendering hook
│   │   └── useTheme.ts    # Theme detection (prefers-color-scheme)
│   ├── types.ts           # Component prop types
│   └── error.tsx          # Error boundary
└── stories/               # Storybook stories (optional)
```

Peer dependencies: `react >= 18`, `@diagramma/wasm`, `@diagramma/theme`.

### @diagramma/bridge

Conversation integration. Framework-agnostic (works with any LLM API).

```
packages/bridge/
├── package.json
├── src/
│   ├── index.ts           # Public API
│   ├── DiagramBridge.ts   # Main bridge class
│   ├── context.ts         # Conversation → spec prompt builder
│   ├── callbacks.ts       # Click → follow-up prompt generator
│   ├── patch.ts           # Spec diffing and patching
│   └── stream.ts          # Streaming JSON parser for progressive rendering
└── prompts/               # System prompt templates
```

## Design Decisions

### Why Rust for layout?

Layout algorithms are computationally intensive (graph traversal, crossing minimization is NP-hard — heuristics run in O(n²) to O(n³)). Rust compiled to WASM gives near-native performance in the browser with zero garbage collection pauses. For the server-side use case (CLI, SSR), the same code runs natively without a JS runtime.

### Why not use an existing layout library?

Evaluated options:

- **dagre** (JS) — unmaintained since 2018, poor TypeScript support, can't run in Rust
- **ELK** (Java/JS) — heavy runtime (~400KB), overkill for our constrained layout model
- **petgraph** (Rust) — graph data structure only, no layout algorithms
- **layout-rs** — minimal, doesn't cover our needs

The layout requirements are specific enough (fixed viewBox, design system constraints, obstacle-aware routing) that a custom implementation is more maintainable than adapting a general-purpose library. The algorithmic foundation (Sugiyama method) is well-documented in academic literature.

### Why separate SVG generation from layout?

Separation of concerns: layout computes geometry (numbers), SVG generation produces markup (strings). This allows:

- Testing layout correctness without parsing SVG
- Swapping SVG output for other formats later (Canvas, PDF)
- Independent optimization of each stage

### Why a WASM bridge package instead of bundling WASM into React?

Not all consumers use React. The WASM bridge is usable from vanilla JS, Vue, Svelte, or any framework. React is one consumer of the WASM package, not the only one.

### Why CSS variables for theming instead of generating two SVGs?

CSS variables allow runtime theme switching without re-running the layout pipeline. A single SVG adapts to light/dark mode via CSS alone, which enables `prefers-color-scheme` media queries and instant toggle without re-render.

## Build Pipeline

```
cargo build (workspace)
    ├── diagramma-core     → lib
    ├── diagramma-layout   → lib (depends on core)
    ├── diagramma-svg      → lib (depends on core + layout)
    └── diagramma-wasm     → cdylib (depends on all three)
                                │
                    wasm-pack build ──→ packages/wasm/src/
                                            │
                                pnpm build (workspace)
                                    ├── @diagramma/wasm     → ESM bundle
                                    ├── @diagramma/theme    → ESM + CSS
                                    ├── @diagramma/react    → ESM bundle
                                    └── @diagramma/bridge   → ESM bundle
```

CI runs: `cargo check` → `cargo clippy` → `cargo test` → `cargo fmt --check` → `wasm-pack build` → `pnpm install` → `pnpm build` → `pnpm test`.
