//! Text element rendering for labels and subtitles.
//!
//! Implements themed text rendering with proper CSS class application.

use crate::elements::class_attr;
use crate::tokens::{ColorRamp, SemanticRole, ThemeMode, css_var};

/// Renders a text label with specified styling.
///
/// # Arguments
/// * `text` - The text content to render
/// * `x` - X coordinate (anchor point, typically center for node labels)
/// * `y` - Y coordinate (baseline position)
/// * `color` - Color ramp for theming
/// * `theme` - Current theme mode
/// * `class` - CSS class for the text element
/// * `font_size` - Font size in pixels
#[must_use]
pub fn render_label(
    text: &str,
    x: f64,
    y: f64,
    color: ColorRamp,
    theme: ThemeMode,
    class: &str,
    font_size: u32,
) -> String {
    let text_color = css_var(color, SemanticRole::Title, theme);
    let classes = format!("{class} dm-text");

    // Escape special XML characters in text
    let escaped_text = escape_xml(text);

    format!(
        r#"<text x="{:.1}" y="{:.1}" text-anchor="middle" font-size="{}px"{} style="fill: var({})">{}</text>"#,
        x,
        y,
        font_size,
        class_attr(&[&classes]),
        text_color,
        escaped_text
    )
}

/// Renders subtitle text with smaller font size.
#[must_use]
pub fn render_subtitle(text: &str, x: f64, y: f64, color: ColorRamp, theme: ThemeMode) -> String {
    render_label(text, x, y, color, theme, "dm-node-subtitle", 12)
}

/// Renders a container label (typically positioned above the container).
#[must_use]
pub fn render_container_label(
    text: &str,
    x: f64,
    y: f64,
    color: ColorRamp,
    theme: ThemeMode,
) -> String {
    render_label(text, x, y, color, theme, "dm-container-label", 14)
}

/// Escapes special XML characters in text content.
#[must_use]
pub fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Calculates approximate text width for layout purposes.
/// Uses a rough heuristic: average character width is about 0.6× font size.
#[must_use]
pub fn estimate_width(text: &str, font_size: u32) -> f64 {
    let char_width = f64::from(font_size) * 0.6;
    let char_count = text.chars().count();
    // Precision loss is acceptable for text width estimation
    #[allow(clippy::cast_precision_loss)]
    let count_f64 = char_count as f64;
    count_f64 * char_width
}

/// Calculates the y position for vertically centered text.
#[must_use]
pub fn center_y(y: f64, height: f64, font_size: u32) -> f64 {
    y + height / 2.0 + f64::from(font_size) * 0.35 // Approximate baseline adjustment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_label_produces_text_element() {
        let svg = render_label(
            "Hello",
            100.0,
            50.0,
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm-title",
            14,
        );
        assert!(svg.starts_with("<text"));
        assert!(svg.contains("</text>"));
        assert!(svg.contains(r#"x="100.0""#));
        assert!(svg.contains(r#"y="50.0""#));
        assert!(svg.contains(r#"font-size="14px""#));
        assert!(svg.contains(r#"text-anchor="middle""#));
    }

    #[test]
    fn test_render_label_includes_content() {
        let svg = render_label(
            "Test Label",
            100.0,
            50.0,
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm-title",
            14,
        );
        assert!(svg.contains(">Test Label</text>"));
    }

    #[test]
    fn test_render_label_uses_css_variable() {
        let svg = render_label(
            "Hello",
            100.0,
            50.0,
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm-title",
            14,
        );
        assert!(svg.contains("var(--dm-blue-800)"));
    }

    #[test]
    fn test_escape_xml_escapes_special_chars() {
        assert_eq!(escape_xml("<script>"), "&lt;script&gt;");
        assert_eq!(escape_xml("&"), "&amp;");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_escape_xml_preserves_normal_text() {
        assert_eq!(escape_xml("Hello World"), "Hello World");
        assert_eq!(escape_xml("Test 123"), "Test 123");
    }

    #[test]
    fn test_render_subtitle_uses_12px_font() {
        let svg = render_subtitle("Subtitle", 100.0, 50.0, ColorRamp::Teal, ThemeMode::Light);
        assert!(svg.contains(r#"font-size="12px""#));
        assert!(svg.contains("dm-node-subtitle"));
    }

    #[test]
    fn test_estimate_width_scales_with_length() {
        let w1 = estimate_width("A", 14);
        let w5 = estimate_width("AAAAA", 14);
        assert!(w5 > w1);
        assert!((w5 - w1 * 5.0).abs() < 0.1);
    }

    #[test]
    fn test_estimate_width_scales_with_font_size() {
        let small = estimate_width("test", 12);
        let large = estimate_width("test", 24);
        assert!(large > small);
        assert!((large / small - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_render_label_with_xml_special_chars() {
        let svg = render_label(
            "<Test>",
            100.0,
            50.0,
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm-title",
            14,
        );
        assert!(svg.contains("&lt;Test&gt;"));
        assert!(!svg.contains("<Test>")); // Should not contain unescaped
    }
}
