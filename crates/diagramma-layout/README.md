# diagramma-layout

[![crates.io](https://img.shields.io/crates/v/diagramma-layout.svg)](https://crates.io/crates/diagramma-layout)
[![docs.rs](https://docs.rs/diagramma-layout/badge.svg)](https://docs.rs/diagramma-layout)

Auto-layout algorithms for [diagramma](https://github.com/SHA888/diagramma) — the conversation-aware diagram rendering engine.

## What This Crate Provides

Given a validated `DiagramSpec` from [`diagramma-core`](https://crates.io/crates/diagramma-core), this crate computes positions (x, y, width, height) for every element and routes edge paths between them. The output is a `LayoutResult` consumed by [`diagramma-svg`](https://crates.io/crates/diagramma-svg) for rendering.

This crate contains the computationally intensive parts of the pipeline — it's why diagramma's core is written in Rust.

## Layout Algorithms

| Diagram Type | Algorithm | Description |
|---|---|---|
| Flowchart | Sugiyama-style hierarchical | Layer assignment → crossing minimization → coordinate assignment |
| Structural | Recursive tree packing | Leaf-to-root sizing → root-to-leaf coordinate assignment |
| Illustrative | Constraint-based freeform | Zone definitions + annotation placement (planned) |
| Interactive | Delegates to base type | Uses flowchart or structural layout for the underlying diagram |

## Layout Constraints

The layout engine enforces diagramma's design system constraints:

- **ViewBox**: 680px wide, content safe area x=40..640, dynamic height
- **Box sizing**: `width = max(title_chars × 8, subtitle_chars × 7) + 24`
- **Spacing**: 60px minimum between boxes, 24px padding inside containers
- **Horizontal tier**: max 4 boxes at full width (~140px each)

## Arrow Routing

Edge paths support three routing strategies:

- **Direct** — straight line between connection points
- **Orthogonal** — L-bend routing (horizontal-then-vertical or vice versa)
- **Obstacle-aware** — paths detour around unrelated boxes

## Key Types

```rust
use diagramma_layout::{layout, LayoutResult, LayoutNode, LayoutEdge};
```

- `LayoutResult` — positioned elements + viewBox dimensions
- `LayoutNode` — id, x, y, width, height, shape
- `LayoutEdge` — id, routed path (Vec of points), arrow positions
- `LayoutContainer` — id, bounds, children layout

## Status

Early development — layout algorithms are being implemented. See the [project roadmap](https://github.com/SHA888/diagramma/blob/main/TODO.md) for details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
