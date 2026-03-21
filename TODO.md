# diagramma — Development Roadmap

> Follows [Semantic Versioning 2.0.0](https://semver.org/). Pre-1.0 releases: each `0.x.0` bump introduces new features with possible breaking changes. Each `0.x.y` bump is a backward-compatible patch.

---

## Phase 0: Project Foundation (v0.0.x) — Scaffold & Planning

### v0.0.1 — Name Reservation ✅

- [x] Name selection and availability check (crates.io, npm, GitHub)
- [x] Monorepo scaffold (Cargo workspace + pnpm workspace)
- [x] Crate stubs: `diagramma-core`, `diagramma-layout`, `diagramma-svg`
- [x] Package stubs: `diagramma`, `@diagramma/wasm`, `@diagramma/theme`, `@diagramma/react`, `@diagramma/bridge`
- [x] License selection (MIT OR Apache-2.0) and distribution
- [x] Publish all crates and packages to reserve names
- [x] GitHub repo: `github.com/SHA888/diagramma`

### v0.0.2 — Documentation & CI Foundation

- [ ] **Documentation**
  - [x] `TODO.md` — this file
  - [x] `ARCHITECTURE.md` — system design and data flow
  - [x] `README.md` per crate and package (8 total)
  - [x] `CONTRIBUTING.md` — contribution guidelines, DCO sign-off
  - [x] `CODE_OF_CONDUCT.md`
  - [x] `CHANGELOG.md` — initialize with Keep a Changelog format
  - [x] `.github/ISSUE_TEMPLATE/` — bug report, feature request
  - [x] `.github/PULL_REQUEST_TEMPLATE.md`
- [x] **CI/CD**
  - [x] GitHub Actions: Rust CI (check, clippy, fmt, test) on `main` + PRs
  - [x] GitHub Actions: TypeScript CI (typecheck, lint, test) on `main` + PRs
  - [x] GitHub Actions: publish workflow (crates.io + npm) on tag push
  - [x] Dependabot or Renovate configuration
- [x] **Tooling**
  - [x] `rustfmt.toml` — project formatting rules
  - [x] `clippy.toml` — lint configuration (if overrides needed)
  - [x] `.editorconfig`
  - [x] ESLint + Prettier config for TypeScript packages
  - [x] `tsconfig.base.json` — shared TypeScript config
  - [x] Turborepo or custom build orchestration (Rust → WASM → TS)
  - [x] pre-commit hooks (lint, format, typecheck) for both Rust and TypeScript

---

## Phase 1: Core Types & Spec Format (v0.1.0)

> Goal: Define the diagram DSL. A spec can be written, validated, serialized/deserialized.

### diagramma-core

- [ ] **Data model**
  - [ ] `NodeId` — typed wrapper (`String` or `SmolStr`)
  - [ ] `Node` — id, label, subtitle, color (ramp name), shape (rect, pill, diamond, circle)
  - [ ] `Edge` — from, to, label (optional), style (solid, dashed), arrow (open, closed, none)
  - [ ] `Container` — id, label, color, children (recursive `Vec<Element>`)
  - [ ] `Element` enum — `Node | Container`
  - [ ] `ColorRamp` enum — purple, teal, coral, pink, gray, blue, green, amber, red
  - [ ] `Theme` enum — light, dark, auto
  - [ ] `Direction` enum — top-down, left-right, bottom-up, right-left
- [ ] **Diagram spec types**
  - [ ] `FlowchartSpec` — direction, nodes, edges, theme
  - [ ] `StructuralSpec` — containers (tree), edges, theme
  - [ ] `IllustrativeSpec` — shapes, annotations, spatial regions, theme
  - [ ] `InteractiveSpec` — base diagram + controls (sliders, toggles, state bindings)
  - [ ] `DiagramSpec` enum — unifying all four types
- [ ] **Validation**
  - [ ] Edge references resolve to existing node/container IDs
  - [ ] No duplicate IDs within a spec
  - [ ] Container nesting depth limit (configurable, default 6)
  - [ ] Color ramp name validation
  - [ ] Meaningful error messages (not just "invalid spec")
- [ ] **Serialization**
  - [ ] `serde::Serialize` + `serde::Deserialize` for all types
  - [ ] JSON schema generation (via `schemars` or manual)
  - [ ] Published JSON schema files for editor autocompletion
- [ ] **Tests**
  - [ ] Unit tests for all type constructors
  - [ ] Validation tests: valid specs pass, invalid specs produce correct errors
  - [ ] Round-trip serialization tests (struct → JSON → struct)
  - [ ] Property-based tests for spec validation (proptest or quickcheck)

### v0.1.0 Release Checklist

- [ ] All `diagramma-core` types implemented and tested
- [ ] JSON schema published alongside crate
- [ ] API docs (`cargo doc`) reviewed and complete
- [ ] CHANGELOG updated
- [ ] Tag `v0.1.0`, publish `diagramma-core` to crates.io

---

## Phase 2: Layout Engine (v0.2.0)

> Goal: Given a validated spec, compute positions (x, y, width, height) for every element.

### diagramma-layout

- [ ] **Layout result types**
  - [ ] `LayoutNode` — id, x, y, width, height, shape
  - [ ] `LayoutEdge` — id, path (Vec of points), arrow positions
  - [ ] `LayoutContainer` — id, x, y, width, height, children layout
  - [ ] `LayoutResult` — all positioned elements, viewBox dimensions
- [ ] **Text measurement**
  - [ ] Character-width estimation (monospace approximation: ~8px per char at 14px, ~7px at 12px)
  - [ ] Box auto-sizing: `width = max(title_chars × 8, subtitle_chars × 7) + 24`
  - [ ] Configurable font metrics (for WASM consumers providing real measurements)
- [ ] **Flowchart layout (hierarchical / layered)**
  - [ ] Layer assignment (longest path or network simplex)
  - [ ] Node ordering within layers (barycenter heuristic or median)
  - [ ] Coordinate assignment (Brandes-Köpf or similar)
  - [ ] Configurable spacing: inter-layer (60px default), intra-layer (40px default)
  - [ ] Direction support: top-down, left-right, bottom-up, right-left
  - [ ] Horizontal tier cap: max 4 nodes at full width, wrap or shrink beyond
- [ ] **Structural layout (tree packing)**
  - [ ] Recursive container sizing (children + padding)
  - [ ] Packing algorithm for sibling containers
  - [ ] Padding: 24px inner, 12px text-to-edge
  - [ ] Nesting-aware coordinate computation
- [ ] **Arrow routing**
  - [ ] Direct paths (straight line between connection points)
  - [ ] L-bend routing (horizontal-then-vertical or vice versa)
  - [ ] Obstacle avoidance: edges detour around unrelated boxes
  - [ ] Connection point selection (top, bottom, left, right based on direction)
  - [ ] Edge-edge crossing minimization (heuristic)
- [ ] **ViewBox computation**
  - [ ] Fixed width: 680px, content safe area x=40..640
  - [ ] Dynamic height based on content
  - [ ] Margin/padding around content bounds
- [ ] **Tests**
  - [ ] Snapshot tests: known specs → expected layout coordinates
  - [ ] No-overlap invariant: boxes don't intersect (property test)
  - [ ] ViewBox contains all elements (property test)
  - [ ] Benchmark: layout performance on specs with 10, 50, 200, 1000 nodes

### v0.2.0 Release Checklist

- [ ] Flowchart and structural layout implemented
- [ ] Arrow routing with obstacle avoidance working
- [ ] Layout benchmarks baselined
- [ ] API docs reviewed
- [ ] CHANGELOG updated
- [ ] Tag `v0.2.0`, publish `diagramma-core` + `diagramma-layout`

---

## Phase 3: SVG Generation (v0.3.0)

> Goal: Layout result → SVG string. Themed, accessible, spec-compliant.

### diagramma-svg

- [ ] **Design tokens (embedded)**
  - [ ] Color ramps: 9 ramps × 7 stops (50, 100, 200, 400, 600, 800, 900)
  - [ ] Light mode mapping: 50 fill + 600 stroke + 800 title / 600 subtitle
  - [ ] Dark mode mapping: 800 fill + 200 stroke + 100 title / 200 subtitle
  - [ ] CSS variable generation for theme switching
- [ ] **SVG elements**
  - [ ] Rect nodes: configurable rx (4 subtle, 8 emphasized, half-height pill)
  - [ ] Diamond nodes (rotated rect)
  - [ ] Circle/ellipse nodes
  - [ ] Container rendering (nested rects with labels)
  - [ ] Text rendering: 14px labels, 12px subtitles, explicit class on every element
  - [ ] Edge rendering: paths with `fill="none"`, 0.5px stroke
  - [ ] Arrow markers: open chevron, context-stroke coloring
- [ ] **Theme system**
  - [ ] `<style>` block with CSS variables for light/dark
  - [ ] `prefers-color-scheme` media query for `theme: auto`
  - [ ] Class-based color application (not inline styles)
  - [ ] Text-on-colored-background: same-ramp 800/900 stop (never black)
- [ ] **SVG structure**
  - [ ] ViewBox from layout result
  - [ ] `<defs>` section for markers and reusable elements
  - [ ] Ordered rendering: containers → edges → nodes (z-order)
  - [ ] `xmlns` and other required SVG attributes
- [ ] **Accessibility**
  - [ ] `<title>` and `<desc>` elements on diagram root
  - [ ] `aria-label` on interactive elements
  - [ ] Semantic grouping with `<g>` elements
- [ ] **Output modes**
  - [ ] Full SVG string (standalone file)
  - [ ] SVG fragment (for embedding in HTML)
  - [ ] Configurable: inline styles vs. class-based
- [ ] **Tests**
  - [ ] Snapshot tests: spec → layout → SVG (golden file comparison)
  - [ ] SVG validity: output parses as valid XML
  - [ ] Theme switching: same spec produces different fills for light vs. dark
  - [ ] All text elements have explicit class (no unstyled inheritance)
  - [ ] All paths have `fill="none"` (regression guard)
  - [ ] Visual regression tests (SVG → rasterized PNG diff)

### v0.3.0 Release Checklist

- [ ] All four node shapes rendering correctly
- [ ] Dark/light/auto theme working
- [ ] Container nesting renders properly
- [ ] Arrow markers and routing visible
- [ ] Visual regression baseline captured
- [ ] API docs reviewed
- [ ] CHANGELOG updated
- [ ] Tag `v0.3.0`, publish all three crates

---

## Phase 4: WASM Bridge (v0.4.0)

> Goal: Rust core runs in the browser via WebAssembly.

### @diagramma/wasm

- [ ] **wasm-pack setup**
  - [ ] `Cargo.toml` with `crate-type = ["cdylib", "rlib"]`
  - [ ] `wasm-bindgen` bindings for public API surface
  - [ ] `wasm-opt` integration for size optimization
- [ ] **Exported functions**
  - [ ] `validate_spec(json: &str) -> Result<(), JsValue>` — spec validation
  - [ ] `layout_spec(json: &str) -> Result<JsValue, JsValue>` — spec → layout result (JSON)
  - [ ] `render_svg(json: &str) -> Result<String, JsValue>` — spec → SVG string (full pipeline)
  - [ ] `render_svg_with_options(json: &str, options: &str) -> Result<String, JsValue>` — with theme/size overrides
- [ ] **TypeScript types**
  - [ ] Auto-generated `.d.ts` from wasm-bindgen
  - [ ] Hand-written type augmentations for spec types (if wasm-bindgen output insufficient)
- [ ] **Bundle**
  - [ ] ESM output (primary)
  - [ ] WASM binary size budget: <200KB gzipped target
  - [ ] Async initialization (`init()` / `initSync()`)
- [ ] **Tests**
  - [ ] Node.js integration tests (wasm-pack test --node)
  - [ ] Browser integration tests (wasm-pack test --headless --chrome)
  - [ ] Roundtrip: TS spec object → JSON → WASM → SVG → valid XML
  - [ ] Error propagation: invalid spec → meaningful JS error

### v0.4.0 Release Checklist

- [ ] WASM binary builds and loads in browser + Node.js
- [ ] All core functions exported and typed
- [ ] Binary size within budget
- [ ] CHANGELOG updated
- [ ] Tag `v0.4.0`, publish `@diagramma/wasm` to npm

---

## Phase 5: Theme Package & React Component (v0.5.0)

> Goal: Usable React component with full theming.

### @diagramma/theme

- [ ] **Design tokens (TypeScript)**
  - [ ] Color ramp definitions (all 9 × 7 stops)
  - [ ] Light/dark mode token sets
  - [ ] CSS variable stylesheet (importable CSS file)
  - [ ] TypeScript constants for programmatic access
- [ ] **Utilities**
  - [ ] `getColor(ramp, stop)` — type-safe color lookup
  - [ ] `getThemeTokens(mode)` — full token set for a mode
  - [ ] CSS custom property generator
- [ ] **Tests**
  - [ ] All color values match spec
  - [ ] Generated CSS is valid
  - [ ] Token lookup type safety (compile-time checks)

### @diagramma/react

- [ ] **Core component**
  - [ ] `<Diagram spec={...} />` — spec → rendered SVG
  - [ ] `theme` prop: `"light" | "dark" | "auto"`
  - [ ] `className` / `style` passthrough
  - [ ] `width` / `height` overrides (default: viewBox-based)
  - [ ] Error boundary: invalid spec → error UI (not crash)
- [ ] **Interactivity**
  - [ ] `onNodeClick(nodeId: string)` callback
  - [ ] `onNodeHover(nodeId: string | null)` callback
  - [ ] Hover effects: stroke thickening, subtle highlight
  - [ ] Keyboard navigation (tab between nodes, enter to select)
- [ ] **Performance**
  - [ ] Memoization: re-render only on spec/theme change
  - [ ] WASM initialization: lazy load, show loading state
  - [ ] `React.lazy` compatible
- [ ] **Tests**
  - [ ] Component renders without crashing (smoke test)
  - [ ] Spec change triggers re-render with new SVG
  - [ ] Click callback fires with correct node ID
  - [ ] Accessibility: keyboard navigation works
  - [ ] Snapshot tests for rendered output

### v0.5.0 Release Checklist

- [ ] `<Diagram />` renders all diagram types
- [ ] Click and hover interactivity working
- [ ] Theme switching (light/dark/auto) working
- [ ] Storybook or demo page with examples
- [ ] API docs (TSDoc) reviewed
- [ ] CHANGELOG updated
- [ ] Tag `v0.5.0`, publish `@diagramma/theme` + `@diagramma/react`

---

## Phase 6: Conversation Bridge (v0.6.0)

> Goal: LLM integration — context ingestion, click callbacks, streaming, incremental updates.

### @diagramma/bridge

- [ ] **Context ingestion**
  - [ ] `conversationToSpec(messages: Message[]) -> DiagramSpec` — LLM prompt builder
  - [ ] System prompt templates for each diagram type
  - [ ] Spec extraction from LLM response (JSON parsing with recovery)
- [ ] **Click callbacks**
  - [ ] `DiagramBridge` class — manages diagram ↔ conversation state
  - [ ] `onNodeClick` → generates follow-up prompt about clicked node
  - [ ] Configurable callback format (prompt template)
  - [ ] Callback debouncing
- [ ] **Incremental updates**
  - [ ] `patchSpec(base: DiagramSpec, patch: SpecPatch) -> DiagramSpec`
  - [ ] Patch operations: add node, remove node, update node, add edge, remove edge
  - [ ] Diff computation: old spec vs new spec → minimal patch
- [ ] **Streaming support**
  - [ ] Partial JSON parsing (streaming spec construction)
  - [ ] Progressive rendering: diagram updates as tokens arrive
  - [ ] Graceful degradation: incomplete spec renders what's available
- [ ] **Tests**
  - [ ] Context ingestion produces valid specs (mock LLM responses)
  - [ ] Patch operations maintain spec validity
  - [ ] Streaming parser handles truncated JSON correctly
  - [ ] Click callbacks generate expected prompts

### v0.6.0 Release Checklist

- [ ] Bridge connects `<Diagram />` to LLM conversation flow
- [ ] Streaming rendering demo working
- [ ] Incremental updates demo working
- [ ] API docs reviewed
- [ ] CHANGELOG updated
- [ ] Tag `v0.6.0`, publish `@diagramma/bridge`

---

## Phase 7: Illustrative & Interactive Diagrams (v0.7.0)

> Goal: Complete all four diagram types.

### diagramma-layout + diagramma-svg additions

- [ ] **Illustrative layout**
  - [ ] Freeform positioning with constraints
  - [ ] Spatial region definition (zones, layers)
  - [ ] Annotation placement (callouts, labels with leader lines)
  - [ ] Cross-section rendering support
- [ ] **Interactive diagram support**
  - [ ] Control definitions: slider, toggle, radio, dropdown
  - [ ] State bindings: control value → node property (color, label, visibility)
  - [ ] HTML+SVG hybrid output (controls are HTML, diagram is SVG)
  - [ ] State serialization/deserialization
- [ ] **Tests**
  - [ ] Illustrative layout snapshot tests
  - [ ] Interactive state binding tests
  - [ ] HTML+SVG output validity

### v0.7.0 Release Checklist

- [ ] All four diagram types fully implemented
- [ ] Interactive controls working in React component
- [ ] CHANGELOG updated
- [ ] Tag `v0.7.0`, publish all crates and packages

---

## Phase 8: Playground App (v0.8.0)

> Goal: Public demo and development tool.

### apps/playground

- [ ] **Editor**
  - [ ] JSON spec editor with syntax highlighting
  - [ ] JSON schema validation (inline errors)
  - [ ] Autocomplete from published JSON schema
- [ ] **Live preview**
  - [ ] Real-time rendering as spec is edited
  - [ ] Theme toggle (light/dark)
  - [ ] Zoom and pan
- [ ] **Examples gallery**
  - [ ] Flowchart examples (3+)
  - [ ] Structural examples (3+)
  - [ ] Illustrative examples (2+)
  - [ ] Interactive examples (2+)
- [ ] **Export**
  - [ ] Copy SVG to clipboard
  - [ ] Download SVG file
  - [ ] Copy spec JSON
- [ ] **Deployment**
  - [ ] Static site (Vite build)
  - [ ] Deploy to Vercel / Netlify / GitHub Pages
  - [ ] Custom domain (optional)

### v0.8.0 Release Checklist

- [ ] Playground deployed and publicly accessible
- [ ] All example diagrams render correctly
- [ ] Mobile-responsive editor
- [ ] CHANGELOG updated
- [ ] Tag `v0.8.0`

---

## Phase 9: Hardening & Pre-release (v0.9.0)

> Goal: Production-ready quality.

- [ ] **Performance**
  - [ ] Benchmarks for all layout algorithms (criterion)
  - [ ] WASM binary size audit and optimization
  - [ ] React component render profiling
  - [ ] Memory usage profiling (large diagrams: 500+ nodes)
- [ ] **Edge cases**
  - [ ] Empty spec handling
  - [ ] Single-node diagrams
  - [ ] Deeply nested containers (6+ levels)
  - [ ] Very long labels (truncation strategy)
  - [ ] Bidirectional edges
  - [ ] Self-referencing edges
  - [ ] Disconnected subgraphs
- [ ] **Documentation**
  - [ ] API reference (auto-generated, reviewed)
  - [ ] User guide: getting started, spec format, examples
  - [ ] Migration guide (for future breaking changes)
  - [ ] Architecture decision records (ADRs) for key decisions
- [ ] **Security**
  - [ ] SVG output sanitization (no script injection via labels)
  - [ ] JSON input size limits
  - [ ] Dependency audit (`cargo audit`, `pnpm audit`)
- [ ] **Compatibility**
  - [ ] Browser matrix: Chrome, Firefox, Safari, Edge (last 2 versions)
  - [ ] Node.js: current LTS + current
  - [ ] React: 18.x + 19.x
  - [ ] SSR compatibility (Next.js, Remix)

### v0.9.0 Release Checklist

- [ ] All benchmarks baselined and acceptable
- [ ] Zero `cargo audit` / `pnpm audit` advisories
- [ ] User guide complete
- [ ] SSR tested and working
- [ ] CHANGELOG updated
- [ ] Tag `v0.9.0`

---

## Phase 10: Stable Release (v1.0.0)

> Goal: Public API is stable. SemVer contract begins.

- [ ] **API freeze**
  - [ ] Review all public types and functions
  - [ ] Mark internal items as `pub(crate)` where appropriate
  - [ ] `#[non_exhaustive]` on enums that may grow
  - [ ] Document stability guarantees
- [ ] **Release**
  - [ ] Final CHANGELOG review
  - [ ] Announcement blog post / social media
  - [ ] Submit to Rust ecosystem listings (lib.rs, awesome-rust)
  - [ ] Submit to React ecosystem listings (if applicable)
  - [ ] Tag `v1.0.0`, publish all crates and packages
- [ ] **Post-release**
  - [ ] Monitor issues for first-week bugs
  - [ ] Set up discussion forum (GitHub Discussions)
  - [ ] Plan v1.1.0 based on community feedback

---

## Future (post-1.0)

- [ ] Sequence diagrams
- [ ] Entity-relationship diagrams
- [ ] Timeline / Gantt diagrams
- [ ] Animation support (transitions between spec states)
- [ ] Collaborative editing (CRDT-based spec merging)
- [ ] Plugin system for custom node shapes and layouts
- [ ] CLI tool (`diagramma render spec.json -o output.svg`)
- [ ] VS Code extension (preview pane)
- [ ] Server-side rendering (pure Rust, no browser needed)
