# diagramma-svg

[![crates.io](https://img.shields.io/crates/v/diagramma-svg.svg)](https://crates.io/crates/diagramma-svg)
[![docs.rs](https://docs.rs/diagramma-svg/badge.svg)](https://docs.rs/diagramma-svg)

SVG generation for [diagramma](https://github.com/SHA888/diagramma) — the conversation-aware diagram rendering engine.

## What This Crate Provides

Takes a `LayoutResult` from [`diagramma-layout`](https://crates.io/crates/diagramma-layout) and produces a themed, accessible SVG string. This is the final stage of the Rust pipeline: geometry in, markup out.

## Features

### Theming

SVG output uses CSS variables for colors, enabling runtime theme switching without re-running the layout pipeline.

- **Light mode**: ramp-50 fill, ramp-600 stroke, ramp-800 title, ramp-600 subtitle
- **Dark mode**: ramp-800 fill, ramp-200 stroke, ramp-100 title, ramp-200 subtitle
- **Auto mode**: `prefers-color-scheme` media query — adapts to system setting

### Color Ramps

9 named ramps, each with 7 stops (50, 100, 200, 400, 600, 800, 900):

purple, teal, coral, pink, gray, blue, green, amber, red

Colors encode **meaning** (category, intensity), not sequence. Max 2–3 colors per diagram.

### Visual Style

- **Flat aesthetic** — no gradients, no shadows, no glow
- **0.5px strokes** — thin is refined
- **Corners**: rx=4 subtle, rx=8 emphasized, rx=half-height for pills
- **Arrow markers**: open chevron with context-stroke coloring
- **Text on colored backgrounds**: same-ramp 800/900 stop (never black)
- **Every text element**: explicit CSS class (no unstyled inheritance)
- **Every path**: `fill="none"` (SVG defaults fill to black)

### Node Shapes

Rect, pill (rounded rect), diamond, circle — each with auto-sizing based on text content.

### Accessibility

- `<title>` and `<desc>` elements on the diagram root
- `aria-label` on interactive elements
- Semantic `<g>` grouping

### Output Modes

- Standalone SVG (complete file with `xmlns`)
- SVG fragment (for embedding in HTML)
- Configurable: class-based or inline styles

## Key Types

```rust
use diagramma_svg::{render, OutputMode, ThemeOverride};
```

## Status

Early development — SVG generation is being implemented. See the [project roadmap](https://github.com/SHA888/diagramma/blob/main/TODO.md) for details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
