# diagramma

Conversation-aware diagram rendering engine.

Takes semantic diagram descriptions (nodes, edges, containers) and produces polished, interactive, theme-aware SVG with dark/light mode, clickable elements, and LLM conversation integration.

## Architecture

```
crates/
├── diagramma-core/        # Type definitions: nodes, edges, containers, specs
├── diagramma-layout/      # Auto-layout: hierarchical, tree packing, arrow routing
└── diagramma-svg/         # SVG generation: layout → themed interactive SVG

packages/
├── diagramma/             # Umbrella npm package
├── @diagramma/wasm/       # WASM bridge (Rust → browser)
├── @diagramma/theme/      # Design tokens, color ramps, CSS variables
├── @diagramma/react/      # React component: <Diagram spec={...} />
└── @diagramma/bridge/     # LLM conversation context ↔ diagram callbacks

apps/
└── playground/            # Demo app / visual spec editor
```

## Status

Early development.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
