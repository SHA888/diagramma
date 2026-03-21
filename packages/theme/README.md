# @diagramma/theme

[![npm](https://img.shields.io/npm/v/@diagramma/theme.svg)](https://www.npmjs.com/package/@diagramma/theme)

Design tokens, color ramps, and CSS variables for [diagramma](https://github.com/SHA888/diagramma).

## What This Package Provides

The complete design system as consumable TypeScript constants and a CSS file. Use this package to integrate diagramma's visual language into your own UI, or to build custom renderers that match the diagramma aesthetic.

This package has **zero dependencies** — no WASM, no React, no runtime.

## Color Ramps

9 named ramps, each with 7 stops:

| Ramp | Usage guidance |
|---|---|
| `purple` | Primary/active elements |
| `teal` | Success, network, connectivity |
| `coral` | Warning, attention |
| `pink` | Accent, secondary highlight |
| `gray` | Neutral, inactive, structural |
| `blue` | Information, links, references |
| `green` | Success, positive, growth |
| `amber` | Caution, pending |
| `red` | Error, critical, destructive |

Each ramp has stops: 50 (lightest), 100, 200, 400, 600, 800, 900 (darkest).

## Usage

### TypeScript

```typescript
import { getColor, ramps, lightTokens, darkTokens } from '@diagramma/theme';

// Single color lookup
const teal600 = getColor('teal', 600); // '#0F6E56'

// Full ramp
const teal = ramps.teal; // { 50: '#E1F5EE', 100: '#9FE1CB', ... }

// Theme tokens
const light = lightTokens('teal');
// { fill: '#E1F5EE', stroke: '#0F6E56', title: '#085041', subtitle: '#0F6E56' }
```

### CSS

```css
@import '@diagramma/theme/diagramma.css';
```

This imports CSS custom properties for all color ramps and theme modes:

```css
:root {
  --dm-purple-50: #EEEDFE;
  --dm-purple-100: #CECBF6;
  /* ... all 63 color variables ... */
}
```

## Theme Mapping

| Element | Light mode | Dark mode |
|---|---|---|
| Fill | ramp-50 | ramp-800 |
| Stroke | ramp-600 | ramp-200 |
| Title text | ramp-800 | ramp-100 |
| Subtitle text | ramp-600 | ramp-200 |

## Status

Early development. See the [project roadmap](https://github.com/SHA888/diagramma/blob/main/TODO.md) for details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
