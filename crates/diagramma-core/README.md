# diagramma-core

[![crates.io](https://img.shields.io/crates/v/diagramma-core.svg)](https://crates.io/crates/diagramma-core)
[![docs.rs](https://docs.rs/diagramma-core/badge.svg)](https://docs.rs/diagramma-core)

Core type definitions for [diagramma](https://github.com/SHA888/diagramma) — the conversation-aware diagram rendering engine.

## What This Crate Provides

- **Node, Edge, Container** — the building blocks of any diagram
- **DiagramSpec** — the top-level enum covering all diagram types (flowchart, structural, illustrative, interactive)
- **Validation** — referential integrity checks, constraint enforcement, and meaningful error messages
- **Serialization** — `serde` support for JSON round-tripping, plus JSON schema generation

This crate defines the **data model** only. It has no layout logic and no rendering — those live in [`diagramma-layout`](https://crates.io/crates/diagramma-layout) and [`diagramma-svg`](https://crates.io/crates/diagramma-svg) respectively.

## Diagram Types

| Type               | Description                                                         |
| ------------------ | ------------------------------------------------------------------- |
| `FlowchartSpec`    | Sequential processes, decisions, data flow. Nodes + directed edges. |
| `StructuralSpec`   | Containment/nesting. Recursive containers with children.            |
| `IllustrativeSpec` | Freeform shapes, spatial metaphors, annotations.                    |
| `InteractiveSpec`  | Base diagram + UI controls (sliders, toggles) with state bindings.  |

## Key Types

```rust
use diagramma_core::{DiagramSpec, Node, Edge, ColorRamp, Theme};
```

- `NodeId` — typed identifier for nodes and containers
- `Node` — id, label, subtitle, color ramp, shape
- `Edge` — from, to, optional label, style, arrow type
- `Container` — id, label, color, recursive children
- `ColorRamp` — one of 9 named ramps (purple, teal, coral, pink, gray, blue, green, amber, red)
- `Theme` — light, dark, or auto
- `Direction` — top-down, left-right, bottom-up, right-left

## Status

Early development — type definitions and validation are being implemented. See the [project roadmap](https://github.com/SHA888/diagramma/blob/main/TODO.md) for details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
