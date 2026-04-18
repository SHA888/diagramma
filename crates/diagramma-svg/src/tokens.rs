//! Design tokens for themed SVG generation.
//!
//! Defines color ramps, theme mappings, and CSS variable generation.

pub use diagramma_core::ColorRamp;

use std::fmt::Write;

/// Extension trait for `ColorRamp` with SVG-specific methods.
pub trait ColorRampExt {
    /// Returns the CSS custom property prefix for this ramp.
    fn css_prefix(self) -> &'static str;
    /// Returns the color value at a specific stop.
    fn stop(self, stop: ColorStop) -> &'static str;
}

/// All available color ramps.
pub const COLOR_RAMPS: [ColorRamp; 9] = [
    ColorRamp::Purple,
    ColorRamp::Teal,
    ColorRamp::Coral,
    ColorRamp::Pink,
    ColorRamp::Gray,
    ColorRamp::Blue,
    ColorRamp::Green,
    ColorRamp::Amber,
    ColorRamp::Red,
];

impl ColorRampExt for ColorRamp {
    fn css_prefix(self) -> &'static str {
        match self {
            ColorRamp::Purple => "--dm-purple",
            ColorRamp::Teal => "--dm-teal",
            ColorRamp::Coral => "--dm-coral",
            ColorRamp::Pink => "--dm-pink",
            ColorRamp::Blue => "--dm-blue",
            ColorRamp::Green => "--dm-green",
            ColorRamp::Amber => "--dm-amber",
            ColorRamp::Red => "--dm-red",
            _ => "--dm-gray", // Fallback for non_exhaustive enum (includes Gray)
        }
    }

    fn stop(self, stop: ColorStop) -> &'static str {
        // All stop values are pre-calculated based on OKLCH color space adjustments
        // for perceptually uniform lightness scaling
        match (self, stop) {
            // Purple ramp
            (ColorRamp::Purple, ColorStop::S50) => "#faf5ff",
            (ColorRamp::Purple, ColorStop::S100) => "#f3e8ff",
            (ColorRamp::Purple, ColorStop::S200) => "#e9d5ff",
            (ColorRamp::Purple, ColorStop::S400) => "#c084fc",
            (ColorRamp::Purple, ColorStop::S600) => "#9333ea",
            (ColorRamp::Purple, ColorStop::S800) => "#6b21a8",
            (ColorRamp::Purple, ColorStop::S900) => "#3b0764",

            // Teal ramp
            (ColorRamp::Teal, ColorStop::S50) => "#f0fdfa",
            (ColorRamp::Teal, ColorStop::S100) => "#ccfbf1",
            (ColorRamp::Teal, ColorStop::S200) => "#99f6e4",
            (ColorRamp::Teal, ColorStop::S400) => "#2dd4bf",
            (ColorRamp::Teal, ColorStop::S600) => "#0d9488",
            (ColorRamp::Teal, ColorStop::S800) => "#115e59",
            (ColorRamp::Teal, ColorStop::S900) => "#042f2e",

            // Coral ramp
            (ColorRamp::Coral, ColorStop::S50) => "#fff7ed",
            (ColorRamp::Coral, ColorStop::S100) => "#ffedd5",
            (ColorRamp::Coral, ColorStop::S200) => "#fed7aa",
            (ColorRamp::Coral, ColorStop::S400) => "#fb923c",
            (ColorRamp::Coral, ColorStop::S600) => "#ea580c",
            (ColorRamp::Coral, ColorStop::S800) => "#9a3412",
            (ColorRamp::Coral, ColorStop::S900) => "#431407",

            // Pink ramp
            (ColorRamp::Pink, ColorStop::S50) => "#fdf2f8",
            (ColorRamp::Pink, ColorStop::S100) => "#fce7f3",
            (ColorRamp::Pink, ColorStop::S200) => "#fbcfe8",
            (ColorRamp::Pink, ColorStop::S400) => "#f472b6",
            (ColorRamp::Pink, ColorStop::S600) => "#db2777",
            (ColorRamp::Pink, ColorStop::S800) => "#9d174d",
            (ColorRamp::Pink, ColorStop::S900) => "#500724",

            // Gray ramp
            (ColorRamp::Gray, ColorStop::S50) => "#f9fafb",
            (ColorRamp::Gray, ColorStop::S100) => "#f3f4f6",
            (ColorRamp::Gray, ColorStop::S200) => "#e5e7eb",
            (ColorRamp::Gray, ColorStop::S400) => "#9ca3af",
            (ColorRamp::Gray, ColorStop::S600) => "#4b5563",
            (ColorRamp::Gray, ColorStop::S800) => "#1f2937",
            (ColorRamp::Gray, ColorStop::S900) => "#030712",

            // Blue ramp
            (ColorRamp::Blue, ColorStop::S50) => "#eff6ff",
            (ColorRamp::Blue, ColorStop::S100) => "#dbeafe",
            (ColorRamp::Blue, ColorStop::S200) => "#bfdbfe",
            (ColorRamp::Blue, ColorStop::S400) => "#60a5fa",
            (ColorRamp::Blue, ColorStop::S600) => "#2563eb",
            (ColorRamp::Blue, ColorStop::S800) => "#1e40af",
            (ColorRamp::Blue, ColorStop::S900) => "#172554",

            // Green ramp
            (ColorRamp::Green, ColorStop::S50) => "#f0fdf4",
            (ColorRamp::Green, ColorStop::S100) => "#dcfce7",
            (ColorRamp::Green, ColorStop::S200) => "#bbf7d0",
            (ColorRamp::Green, ColorStop::S400) => "#4ade80",
            (ColorRamp::Green, ColorStop::S600) => "#16a34a",
            (ColorRamp::Green, ColorStop::S800) => "#166534",
            (ColorRamp::Green, ColorStop::S900) => "#052e16",

            // Amber ramp
            (ColorRamp::Amber, ColorStop::S50) => "#fffbeb",
            (ColorRamp::Amber, ColorStop::S100) => "#fef3c7",
            (ColorRamp::Amber, ColorStop::S200) => "#fde68a",
            (ColorRamp::Amber, ColorStop::S400) => "#fbbf24",
            (ColorRamp::Amber, ColorStop::S600) => "#d97706",
            (ColorRamp::Amber, ColorStop::S800) => "#92400e",
            (ColorRamp::Amber, ColorStop::S900) => "#451a03",

            // Red ramp
            (ColorRamp::Red, ColorStop::S50) => "#fef2f2",
            (ColorRamp::Red, ColorStop::S100) => "#fee2e2",
            (ColorRamp::Red, ColorStop::S200) => "#fecaca",
            (ColorRamp::Red, ColorStop::S400) => "#f87171",
            (ColorRamp::Red, ColorStop::S600) => "#dc2626",
            (ColorRamp::Red, ColorStop::S800) => "#991b1b",
            (ColorRamp::Red, ColorStop::S900) => "#450a0a",
            // Fallback for non_exhaustive enum
            _ => "#6b7280", // Gray-500 as fallback
        }
    }
}

/// Color stop positions in a ramp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorStop {
    S50,
    S100,
    S200,
    S400,
    S600,
    S800,
    S900,
}

impl ColorStop {
    /// All available stops.
    pub const ALL: [ColorStop; 7] = [
        ColorStop::S50,
        ColorStop::S100,
        ColorStop::S200,
        ColorStop::S400,
        ColorStop::S600,
        ColorStop::S800,
        ColorStop::S900,
    ];

    /// Returns the numeric suffix for CSS variable naming.
    #[must_use]
    pub fn suffix(self) -> &'static str {
        match self {
            ColorStop::S50 => "50",
            ColorStop::S100 => "100",
            ColorStop::S200 => "200",
            ColorStop::S400 => "400",
            ColorStop::S600 => "600",
            ColorStop::S800 => "800",
            ColorStop::S900 => "900",
        }
    }
}

/// Theme mode for SVG rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
    Auto,
}

/// Semantic color roles for diagram elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticRole {
    /// Node/container fill color
    Fill,
    /// Border/stroke color
    Stroke,
    /// Title text color
    Title,
    /// Subtitle text color
    Subtitle,
    /// Edge path color
    Edge,
    /// Arrow marker color
    Arrow,
    /// Background color for the diagram
    Background,
    /// Text on colored backgrounds (same-ramp 800/900)
    TextOnColor,
}

/// Maps a semantic role to the appropriate color stop for the current theme.
#[must_use]
pub fn semantic_stop(role: SemanticRole, theme: ThemeMode) -> ColorStop {
    // Auto mode falls back to light for static generation
    // (browser handles prefers-color-scheme via CSS)
    match role {
        SemanticRole::Fill | SemanticRole::Background => match theme {
            ThemeMode::Light | ThemeMode::Auto => ColorStop::S50,
            ThemeMode::Dark => ColorStop::S800,
        },
        SemanticRole::Stroke
        | SemanticRole::Subtitle
        | SemanticRole::Edge
        | SemanticRole::Arrow => match theme {
            ThemeMode::Light | ThemeMode::Auto => ColorStop::S600,
            ThemeMode::Dark => ColorStop::S200,
        },
        SemanticRole::Title => match theme {
            ThemeMode::Light | ThemeMode::Auto => ColorStop::S800,
            ThemeMode::Dark => ColorStop::S100,
        },
        SemanticRole::TextOnColor => match theme {
            ThemeMode::Light | ThemeMode::Auto => ColorStop::S900,
            ThemeMode::Dark => ColorStop::S100,
        },
    }
}

/// Returns the CSS color variable reference for a semantic role.
///
/// Example: `var(--dm-purple-600)` for a purple node stroke.
#[must_use]
pub fn css_var(ramp: ColorRamp, role: SemanticRole, theme: ThemeMode) -> String {
    let stop = semantic_stop(role, theme);
    format!("var({}-{})", ramp.css_prefix(), stop.suffix())
}

/// Generates the complete CSS variable definitions for all color ramps.
///
/// Returns a CSS string with :root definitions for light mode and
/// @media (prefers-color-scheme: dark) for dark mode.
#[must_use]
pub fn generate_css_variables() -> String {
    let mut css = String::new();

    // Light mode root variables
    css.push_str(":root {\n");
    for ramp in COLOR_RAMPS {
        for stop in ColorStop::ALL {
            let value = ramp.stop(stop);
            let prefix = ramp.css_prefix();
            let suffix = stop.suffix();
            let _ = writeln!(css, "  {prefix}-{suffix}: {value};");
        }
    }

    // Semantic role variables for light mode
    css.push_str("\n  /* Semantic mappings - Light */\n");
    for ramp in COLOR_RAMPS {
        let prefix = ramp.css_prefix();
        let _ = writeln!(css, "  {prefix}-fill: var({prefix}-50);");
        let _ = writeln!(css, "  {prefix}-stroke: var({prefix}-600);");
        let _ = writeln!(css, "  {prefix}-title: var({prefix}-800);");
        let _ = writeln!(css, "  {prefix}-subtitle: var({prefix}-600);");
        let _ = writeln!(css, "  {prefix}-text-on-color: var({prefix}-900);");
    }
    css.push_str("}\n\n");

    // Dark mode media query
    css.push_str("@media (prefers-color-scheme: dark) {\n");
    css.push_str("  :root {\n");
    css.push_str("    /* Semantic mappings - Dark */\n");
    for ramp in COLOR_RAMPS {
        let prefix = ramp.css_prefix();
        let _ = writeln!(css, "    {prefix}-fill: var({prefix}-800);");
        let _ = writeln!(css, "    {prefix}-stroke: var({prefix}-200);");
        let _ = writeln!(css, "    {prefix}-title: var({prefix}-100);");
        let _ = writeln!(css, "    {prefix}-subtitle: var({prefix}-200);");
        let _ = writeln!(css, "    {prefix}-text-on-color: var({prefix}-100);");
    }
    css.push_str("  }\n");
    css.push_str("}\n");

    // Manual dark mode class override
    css.push_str("\n[data-theme=\"dark\"] {\n");
    css.push_str("  /* Semantic mappings - Dark (manual override) */\n");
    for ramp in COLOR_RAMPS {
        let prefix = ramp.css_prefix();
        let _ = writeln!(css, "  {prefix}-fill: var({prefix}-800);");
        let _ = writeln!(css, "  {prefix}-stroke: var({prefix}-200);");
        let _ = writeln!(css, "  {prefix}-title: var({prefix}-100);");
        let _ = writeln!(css, "  {prefix}-subtitle: var({prefix}-200);");
        let _ = writeln!(css, "  {prefix}-text-on-color: var({prefix}-100);");
    }
    css.push_str("}\n");

    css
}

/// Returns the CSS class name for applying color to an element.
#[must_use]
pub fn color_class(ramp: ColorRamp, role: SemanticRole) -> String {
    format!("dm-{}-{}", ramp_name(ramp), role_name(role))
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
        _ => "gray", // Fallback for non_exhaustive enum (includes Gray)
    }
}

fn role_name(role: SemanticRole) -> &'static str {
    match role {
        SemanticRole::Fill => "fill",
        SemanticRole::Stroke => "stroke",
        SemanticRole::Title => "title",
        SemanticRole::Subtitle => "subtitle",
        SemanticRole::Edge => "edge",
        SemanticRole::Arrow => "arrow",
        SemanticRole::Background => "bg",
        SemanticRole::TextOnColor => "text-on-color",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_stop_values() {
        // Verify purple ramp values
        assert_eq!(ColorRamp::Purple.stop(ColorStop::S50), "#faf5ff");
        assert_eq!(ColorRamp::Purple.stop(ColorStop::S600), "#9333ea");
        assert_eq!(ColorRamp::Purple.stop(ColorStop::S900), "#3b0764");
    }

    #[test]
    fn test_semantic_mapping_light() {
        assert_eq!(
            semantic_stop(SemanticRole::Fill, ThemeMode::Light),
            ColorStop::S50
        );
        assert_eq!(
            semantic_stop(SemanticRole::Stroke, ThemeMode::Light),
            ColorStop::S600
        );
        assert_eq!(
            semantic_stop(SemanticRole::Title, ThemeMode::Light),
            ColorStop::S800
        );
        assert_eq!(
            semantic_stop(SemanticRole::Subtitle, ThemeMode::Light),
            ColorStop::S600
        );
    }

    #[test]
    fn test_semantic_mapping_dark() {
        assert_eq!(
            semantic_stop(SemanticRole::Fill, ThemeMode::Dark),
            ColorStop::S800
        );
        assert_eq!(
            semantic_stop(SemanticRole::Stroke, ThemeMode::Dark),
            ColorStop::S200
        );
        assert_eq!(
            semantic_stop(SemanticRole::Title, ThemeMode::Dark),
            ColorStop::S100
        );
        assert_eq!(
            semantic_stop(SemanticRole::Subtitle, ThemeMode::Dark),
            ColorStop::S200
        );
    }

    #[test]
    fn test_css_var_generation() {
        let var = css_var(ColorRamp::Blue, SemanticRole::Stroke, ThemeMode::Light);
        assert_eq!(var, "var(--dm-blue-600)");
    }

    #[test]
    fn test_semantic_mapping_auto() {
        // Auto mode should behave like Light mode for static generation
        assert_eq!(
            semantic_stop(SemanticRole::Fill, ThemeMode::Auto),
            ColorStop::S50
        );
        assert_eq!(
            semantic_stop(SemanticRole::Stroke, ThemeMode::Auto),
            ColorStop::S600
        );
        assert_eq!(
            semantic_stop(SemanticRole::Title, ThemeMode::Auto),
            ColorStop::S800
        );
        assert_eq!(
            semantic_stop(SemanticRole::Subtitle, ThemeMode::Auto),
            ColorStop::S600
        );
    }

    #[test]
    fn test_background_role_mapping() {
        // Background should follow same mapping as Fill
        assert_eq!(
            semantic_stop(SemanticRole::Background, ThemeMode::Light),
            ColorStop::S50
        );
        assert_eq!(
            semantic_stop(SemanticRole::Background, ThemeMode::Dark),
            ColorStop::S800
        );
        assert_eq!(
            semantic_stop(SemanticRole::Background, ThemeMode::Auto),
            ColorStop::S50
        );
    }

    #[test]
    fn test_all_color_stops_accessible() {
        use super::ColorRampExt;
        // Verify all 9 ramps have all 7 stops accessible
        for ramp in COLOR_RAMPS {
            for stop in ColorStop::ALL {
                let value = ramp.stop(stop);
                // Verify it's a valid hex color
                assert!(
                    value.starts_with('#') && value.len() == 7,
                    "Invalid color value for {ramp:?} {stop:?}: {value}"
                );
            }
        }
    }

    #[test]
    fn test_css_variables_output() {
        let css = generate_css_variables();
        // Should contain all ramps
        assert!(css.contains("--dm-purple-50"));
        assert!(css.contains("--dm-teal-100"));
        assert!(css.contains("--dm-blue-900"));

        // Should contain semantic mappings
        assert!(css.contains("--dm-purple-fill"));
        assert!(css.contains("--dm-blue-stroke"));

        // Should contain dark mode media query
        assert!(css.contains("@media (prefers-color-scheme: dark)"));

        // Should contain manual dark mode override
        assert!(css.contains("[data-theme=\"dark\"]"));
    }

    #[test]
    fn test_color_class() {
        assert_eq!(
            color_class(ColorRamp::Teal, SemanticRole::Fill),
            "dm-teal-fill"
        );
        assert_eq!(
            color_class(ColorRamp::Red, SemanticRole::TextOnColor),
            "dm-red-text-on-color"
        );
    }
}
