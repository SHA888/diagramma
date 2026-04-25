//! SVG theme and style block generation.
//!
//! Provides CSS `<style>` blocks for embedded SVG rendering, including
//! color variable definitions, `prefers-color-scheme` media queries,
//! and class-based color rules for style-free element rendering.

use crate::tokens::{COLOR_RAMPS, ColorRamp, ColorRampExt, ColorStop, SemanticRole, ThemeMode};
use std::fmt::Write;

/// Controls whether colors are applied via inline `style` attributes
/// or purely through CSS classes with externally-defined rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StyleMode {
    /// Inline `style="fill: var(...)"` attributes on every element (current default).
    #[default]
    Inline,
    /// No inline color styles; colors come from CSS class rules in a `<style>` block.
    ClassOnly,
}

impl StyleMode {
    /// Returns `true` if inline styles should be generated.
    #[must_use]
    pub fn is_inline(self) -> bool {
        matches!(self, StyleMode::Inline)
    }
}

/// Generates a complete `<style>` element for embedding in an SVG document.
///
/// Contains:
/// - CSS custom property definitions for all color ramps and stops
/// - Semantic variable mappings (`--dm-{ramp}-fill`, `--dm-{ramp}-stroke`, etc.)
/// - `prefers-color-scheme: dark` media query for automatic theme switching
/// - `[data-theme="dark"]` manual override
/// - Class rules mapping `.dm-{ramp}-{role}` to CSS properties
#[must_use]
pub fn style_block() -> String {
    let mut css = String::new();

    // --- CSS variable definitions ---
    css.push_str("/* Color ramp value variables */\n:root {\n");
    for ramp in COLOR_RAMPS {
        for stop in ColorStop::ALL {
            let value = ramp.stop(stop);
            let prefix = ramp.css_prefix();
            let suffix = stop.suffix();
            let _ = writeln!(css, "  {prefix}-{suffix}: {value};");
        }
    }

    // Semantic mappings for light mode
    css.push_str("\n  /* Semantic mappings - Light */\n");
    for ramp in COLOR_RAMPS {
        let prefix = ramp.css_prefix();
        let _ = writeln!(css, "  {prefix}-fill: var({prefix}-50);");
        let _ = writeln!(css, "  {prefix}-stroke: var({prefix}-600);");
        let _ = writeln!(css, "  {prefix}-title: var({prefix}-800);");
        let _ = writeln!(css, "  {prefix}-subtitle: var({prefix}-600);");
        let _ = writeln!(css, "  {prefix}-edge: var({prefix}-600);");
        let _ = writeln!(css, "  {prefix}-arrow: var({prefix}-600);");
        let _ = writeln!(css, "  {prefix}-text-on-color: var({prefix}-900);");
        let _ = writeln!(css, "  {prefix}-bg: var({prefix}-50);");
    }
    css.push_str("}\n\n");

    // Dark mode: prefers-color-scheme
    css.push_str("@media (prefers-color-scheme: dark) {\n");
    css.push_str("  :root {\n");
    css.push_str("    /* Semantic mappings - Dark */\n");
    for ramp in COLOR_RAMPS {
        let prefix = ramp.css_prefix();
        let _ = writeln!(css, "    {prefix}-fill: var({prefix}-800);");
        let _ = writeln!(css, "    {prefix}-stroke: var({prefix}-200);");
        let _ = writeln!(css, "    {prefix}-title: var({prefix}-100);");
        let _ = writeln!(css, "    {prefix}-subtitle: var({prefix}-200);");
        let _ = writeln!(css, "    {prefix}-edge: var({prefix}-200);");
        let _ = writeln!(css, "    {prefix}-arrow: var({prefix}-200);");
        let _ = writeln!(css, "    {prefix}-text-on-color: var({prefix}-100);");
        let _ = writeln!(css, "    {prefix}-bg: var({prefix}-800);");
    }
    css.push_str("  }\n");
    css.push_str("}\n\n");

    // Manual dark mode override via data-theme attribute
    css.push_str("[data-theme=\"dark\"] {\n");
    css.push_str("  /* Semantic mappings - Dark (manual override) */\n");
    for ramp in COLOR_RAMPS {
        let prefix = ramp.css_prefix();
        let _ = writeln!(css, "  {prefix}-fill: var({prefix}-800);");
        let _ = writeln!(css, "  {prefix}-stroke: var({prefix}-200);");
        let _ = writeln!(css, "  {prefix}-title: var({prefix}-100);");
        let _ = writeln!(css, "  {prefix}-subtitle: var({prefix}-200);");
        let _ = writeln!(css, "  {prefix}-edge: var({prefix}-200);");
        let _ = writeln!(css, "  {prefix}-arrow: var({prefix}-200);");
        let _ = writeln!(css, "  {prefix}-text-on-color: var({prefix}-100);");
        let _ = writeln!(css, "  {prefix}-bg: var({prefix}-800);");
    }
    css.push_str("}\n\n");

    // --- Class-based color rules ---
    css.push_str("/* Class-based color application */\n");
    for ramp in COLOR_RAMPS {
        let ramp_name = ramp_name(ramp);
        let prefix = ramp.css_prefix();

        // Fill-based roles
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-fill    {{ fill: var({prefix}-fill); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-title    {{ fill: var({prefix}-title); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-subtitle {{ fill: var({prefix}-subtitle); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-text-on-color {{ fill: var({prefix}-text-on-color); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-bg       {{ fill: var({prefix}-bg); }}"
        );

        // Stroke-based roles
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-stroke   {{ stroke: var({prefix}-stroke); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-edge     {{ stroke: var({prefix}-edge); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-arrow    {{ stroke: var({prefix}-arrow); }}"
        );
    }

    format!("<style>\n{css}</style>")
}

/// Generates CSS class rules only (without variable definitions),
/// useful when the variables are already defined elsewhere (e.g. an HTML page
/// hosting the SVG).
#[must_use]
pub fn class_rules_only() -> String {
    let mut css = String::new();
    for ramp in COLOR_RAMPS {
        let ramp_name = ramp_name(ramp);
        let prefix = ramp.css_prefix();

        let _ = writeln!(
            css,
            ".dm-{ramp_name}-fill    {{ fill: var({prefix}-fill); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-title    {{ fill: var({prefix}-title); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-subtitle {{ fill: var({prefix}-subtitle); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-text-on-color {{ fill: var({prefix}-text-on-color); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-bg       {{ fill: var({prefix}-bg); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-stroke   {{ stroke: var({prefix}-stroke); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-edge     {{ stroke: var({prefix}-edge); }}"
        );
        let _ = writeln!(
            css,
            ".dm-{ramp_name}-arrow    {{ stroke: var({prefix}-arrow); }}"
        );
    }
    css
}

/// Wraps raw CSS in an SVG `<style>` element.
#[must_use]
pub fn wrap_style_block(css: &str) -> String {
    format!("<style>\n{css}</style>")
}

/// Returns a style attribute string for the given semantic role,
/// or an empty string when in `ClassOnly` mode.
///
/// In `Inline` mode: `" style=\"fill: var(--dm-blue-50)\""`
/// In `ClassOnly` mode: `""` (color comes from the class)
#[must_use]
pub fn maybe_style_attr(
    mode: StyleMode,
    ramp: ColorRamp,
    role: SemanticRole,
    theme: ThemeMode,
    property: &str,
    extra: &str,
) -> String {
    if mode.is_inline() {
        let var = crate::tokens::css_var(ramp, role, theme);
        if extra.is_empty() {
            format!(r#" style="{property}: var({var})""#)
        } else {
            format!(r#" style="{property}: var({var}); {extra}""#)
        }
    } else {
        String::new()
    }
}

fn ramp_name(ramp: ColorRamp) -> &'static str {
    match ramp {
        ColorRamp::Purple => "purple",
        ColorRamp::Teal => "teal",
        ColorRamp::Coral => "coral",
        ColorRamp::Pink => "pink",
        ColorRamp::Blue => "blue",
        ColorRamp::Green => "green",
        ColorRamp::Amber => "amber",
        ColorRamp::Red => "red",
        _ => "gray", // Fallback for non_exhaustive enum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_block_contains_variables() {
        let block = style_block();
        assert!(block.starts_with("<style>"));
        assert!(block.ends_with("</style>"));
        assert!(block.contains("--dm-blue-50"));
        assert!(block.contains("--dm-purple-900"));
    }

    #[test]
    fn test_style_block_has_semantic_mappings() {
        let block = style_block();
        assert!(block.contains("--dm-blue-fill"));
        assert!(block.contains("--dm-blue-stroke"));
        assert!(block.contains("--dm-blue-title"));
        assert!(block.contains("--dm-blue-text-on-color"));
    }

    #[test]
    fn test_style_block_has_dark_media_query() {
        let block = style_block();
        assert!(block.contains("@media (prefers-color-scheme: dark)"));
    }

    #[test]
    fn test_style_block_has_manual_dark_override() {
        let block = style_block();
        assert!(block.contains("[data-theme=\"dark\"]"));
    }

    #[test]
    fn test_style_block_has_class_rules() {
        let block = style_block();
        assert!(block.contains(".dm-blue-fill"));
        assert!(block.contains("fill: var(--dm-blue-fill)"));
        assert!(block.contains(".dm-blue-stroke"));
        assert!(block.contains("stroke: var(--dm-blue-stroke)"));
        assert!(block.contains(".dm-blue-edge"));
        assert!(block.contains("stroke: var(--dm-blue-edge)"));
    }

    #[test]
    fn test_class_rules_only_omits_variables() {
        let rules = class_rules_only();
        assert!(!rules.contains(":root"));
        assert!(!rules.contains("@media"));
        assert!(rules.contains(".dm-teal-fill"));
        assert!(rules.contains("fill: var(--dm-teal-fill)"));
    }

    #[test]
    fn test_wrap_style_block() {
        let wrapped = wrap_style_block(".foo { fill: red; }");
        assert_eq!(wrapped, "<style>\n.foo { fill: red; }</style>");
    }

    #[test]
    fn test_maybe_style_attr_inline() {
        let attr = maybe_style_attr(
            StyleMode::Inline,
            ColorRamp::Blue,
            SemanticRole::Fill,
            ThemeMode::Light,
            "fill",
            "",
        );
        assert!(attr.contains("style="));
        assert!(attr.contains("fill:"));
        assert!(attr.contains("var(--dm-blue-50)"));
    }

    #[test]
    fn test_maybe_style_attr_class_only() {
        let attr = maybe_style_attr(
            StyleMode::ClassOnly,
            ColorRamp::Blue,
            SemanticRole::Fill,
            ThemeMode::Light,
            "fill",
            "",
        );
        assert!(attr.is_empty());
    }

    #[test]
    fn test_maybe_style_attr_with_extra() {
        let attr = maybe_style_attr(
            StyleMode::Inline,
            ColorRamp::Red,
            SemanticRole::Stroke,
            ThemeMode::Dark,
            "stroke",
            "stroke-width: 0.5",
        );
        assert!(attr.contains("stroke:"));
        assert!(attr.contains("stroke-width: 0.5"));
        assert!(attr.contains("var(--dm-red-200)"));
    }
}
