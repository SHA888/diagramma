//! Node shape rendering (rect, pill, diamond, circle).
//!
//! Implements themed rendering for all node shapes with proper CSS class application.

use crate::elements::{class_attr, text};
use crate::theme::StyleMode;
use crate::tokens::{SemanticRole, ThemeMode, color_class, css_var};
use diagramma_core::Node;
use diagramma_layout::LayoutNode;

/// Renders a rectangle node with configurable corner radius.
///
/// # Arguments
/// * `rx` - Corner radius: 4 for subtle, 8 for emphasized
#[must_use]
pub fn render_rect(
    node: &Node,
    layout: &LayoutNode,
    theme: ThemeMode,
    class_prefix: &str,
    rx: f64,
    style_mode: StyleMode,
) -> String {
    let color = node.color;
    let fill_class = color_class(color, SemanticRole::Fill);
    let stroke_class = color_class(color, SemanticRole::Stroke);

    let style = if style_mode.is_inline() {
        format!(
            r#" style="fill: var({}); stroke: var({}); stroke-width: 0.5""#,
            css_var(color, SemanticRole::Fill, theme),
            css_var(color, SemanticRole::Stroke, theme)
        )
    } else {
        String::new()
    };

    let rect = format!(
        r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="{:.1}"{}{}/>"#,
        layout.x,
        layout.y,
        layout.width,
        layout.height,
        rx,
        class_attr(&[class_prefix, &fill_class, &stroke_class]),
        style
    );

    // Render text elements
    let text_content = render_node_text(node, layout, theme, style_mode);

    format!("{rect}{text_content}")
}

/// Renders a pill-shaped node (rect with half-height corner radius).
#[must_use]
pub fn render_pill(
    node: &Node,
    layout: &LayoutNode,
    theme: ThemeMode,
    class_prefix: &str,
    style_mode: StyleMode,
) -> String {
    let rx = layout.height / 2.0;
    render_rect(node, layout, theme, class_prefix, rx, style_mode)
}

/// Renders a diamond-shaped node (rotated square).
#[must_use]
pub fn render_diamond(
    node: &Node,
    layout: &LayoutNode,
    theme: ThemeMode,
    class_prefix: &str,
    style_mode: StyleMode,
) -> String {
    let color = node.color;
    let fill_class = color_class(color, SemanticRole::Fill);
    let stroke_class = color_class(color, SemanticRole::Stroke);

    // Calculate diamond points (centered in the layout box)
    let cx = layout.x + layout.width / 2.0;
    let cy = layout.y + layout.height / 2.0;
    let half_w = layout.width / 2.0;
    let half_h = layout.height / 2.0;

    // Diamond points: top, right, bottom, left
    let points = format!(
        "{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
        cx,
        cy - half_h, // top
        cx + half_w,
        cy, // right
        cx,
        cy + half_h, // bottom
        cx - half_w,
        cy // left
    );

    let style = if style_mode.is_inline() {
        format!(
            r#" style="fill: var({}); stroke: var({}); stroke-width: 0.5""#,
            css_var(color, SemanticRole::Fill, theme),
            css_var(color, SemanticRole::Stroke, theme)
        )
    } else {
        String::new()
    };

    let polygon = format!(
        r#"<polygon points="{}"{}{}/>"#,
        points,
        class_attr(&[class_prefix, &fill_class, &stroke_class]),
        style
    );

    // Render text elements (centered)
    let text_content = render_node_text(node, layout, theme, style_mode);

    format!("{polygon}{text_content}")
}

/// Renders a circular/elliptical node.
#[must_use]
pub fn render_circle(
    node: &Node,
    layout: &LayoutNode,
    theme: ThemeMode,
    class_prefix: &str,
    style_mode: StyleMode,
) -> String {
    let color = node.color;
    let fill_class = color_class(color, SemanticRole::Fill);
    let stroke_class = color_class(color, SemanticRole::Stroke);

    let cx = layout.x + layout.width / 2.0;
    let cy = layout.y + layout.height / 2.0;
    let rx = layout.width / 2.0;
    let ry = layout.height / 2.0;

    let style = if style_mode.is_inline() {
        format!(
            r#" style="fill: var({}); stroke: var({}); stroke-width: 0.5""#,
            css_var(color, SemanticRole::Fill, theme),
            css_var(color, SemanticRole::Stroke, theme)
        )
    } else {
        String::new()
    };

    // Use ellipse for flexibility (circle when width == height)
    let ellipse = format!(
        r#"<ellipse cx="{:.1}" cy="{:.1}" rx="{:.1}" ry="{:.1}"{}{}/>"#,
        cx,
        cy,
        rx,
        ry,
        class_attr(&[class_prefix, &fill_class, &stroke_class]),
        style
    );

    // Render text elements
    let text_content = render_node_text(node, layout, theme, style_mode);

    format!("{ellipse}{text_content}")
}

/// Title font size (14px) and subtitle font size (12px)
const TITLE_SIZE: u32 = 14;
const SUBTITLE_SIZE: u32 = 12;
const GAP: f64 = 2.0; // Gap between title and subtitle

/// Renders text for a node (label and subtitle).
fn render_node_text(
    node: &Node,
    layout: &LayoutNode,
    theme: ThemeMode,
    style_mode: StyleMode,
) -> String {
    let color = node.color;
    let mut result = String::new();
    let center_x = layout.x + layout.width / 2.0;
    let node_center_y = layout.y + layout.height / 2.0;

    // Calculate Y positions based on whether we have a subtitle
    let (title_y, subtitle_y) = if node.subtitle.is_some() {
        // Two-line layout: position title and subtitle around center
        // Total text block height ≈ TITLE_SIZE + GAP + SUBTITLE_SIZE = 28px
        // Title baseline should be slightly above center
        let title_baseline_offset = (f64::from(TITLE_SIZE) + GAP + f64::from(SUBTITLE_SIZE)) / 2.0
            - f64::from(TITLE_SIZE) * 0.3; // Adjust for baseline
        let subtitle_baseline_offset =
            title_baseline_offset + f64::from(TITLE_SIZE) + GAP - f64::from(SUBTITLE_SIZE) * 0.3;
        (
            node_center_y - title_baseline_offset,
            node_center_y + subtitle_baseline_offset,
        )
    } else {
        // Single line: center the title
        (text::center_y(layout.y, layout.height, TITLE_SIZE), 0.0)
    };

    // Text sits on colored background → use TextOnColor for same-ramp contrast
    // (never black; always from the node's own ramp for visual harmony)
    let text_color = if style_mode.is_inline() {
        css_var(color, SemanticRole::TextOnColor, theme)
    } else {
        String::new()
    };
    let text_class = color_class(color, SemanticRole::TextOnColor);

    result.push_str(&text::render_label_on_color(
        &node.label,
        center_x,
        title_y,
        &text_color,
        &text_class,
        "dm-node-title",
        TITLE_SIZE,
        style_mode,
    ));

    // Subtitle text if present
    if let Some(ref subtitle) = node.subtitle {
        result.push_str(&text::render_label_on_color(
            subtitle,
            center_x,
            subtitle_y,
            &text_color,
            &text_class,
            "dm-node-subtitle",
            SUBTITLE_SIZE,
            style_mode,
        ));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use diagramma_core::{ColorRamp, NodeId, NodeShape};

    fn test_node() -> Node {
        Node {
            id: NodeId::new("test"),
            label: "Test Node".into(),
            subtitle: None,
            color: ColorRamp::Blue,
            shape: NodeShape::Rect,
        }
    }

    fn test_layout() -> LayoutNode {
        LayoutNode {
            id: NodeId::new("test"),
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
        }
    }

    #[test]
    fn test_render_rect_includes_position() {
        let node = test_node();
        let layout = test_layout();
        let svg = render_rect(
            &node,
            &layout,
            ThemeMode::Light,
            "dm",
            4.0,
            StyleMode::Inline,
        );
        assert!(svg.contains(r#"x="10.0""#));
        assert!(svg.contains(r#"y="20.0""#));
        assert!(svg.contains(r#"width="100.0""#));
        assert!(svg.contains(r#"height="50.0""#));
    }

    #[test]
    fn test_render_rect_uses_css_variables() {
        let node = test_node();
        let layout = test_layout();
        let svg = render_rect(
            &node,
            &layout,
            ThemeMode::Light,
            "dm",
            4.0,
            StyleMode::Inline,
        );
        assert!(svg.contains("var(--dm-blue-50)")); // fill
        assert!(svg.contains("var(--dm-blue-600)")); // stroke
    }

    #[test]
    fn test_render_pill_uses_half_height_radius() {
        let node = test_node();
        let layout = test_layout();
        let svg = render_pill(&node, &layout, ThemeMode::Light, "dm", StyleMode::Inline);
        assert!(svg.contains(r#"rx="25.0""#)); // 50/2 = 25
    }

    #[test]
    fn test_render_diamond_produces_polygon() {
        let node = test_node();
        let layout = test_layout();
        let svg = render_diamond(&node, &layout, ThemeMode::Light, "dm", StyleMode::Inline);
        assert!(svg.contains("<polygon"));
        assert!(svg.contains("points="));
        // Should have 4 points (8 coordinates)
        assert!(svg.contains("60.0,20.0")); // top
        assert!(svg.contains("110.0,45.0")); // right
        assert!(svg.contains("60.0,70.0")); // bottom
        assert!(svg.contains("10.0,45.0")); // left
    }

    #[test]
    fn test_render_circle_produces_ellipse() {
        let node = test_node();
        let layout = test_layout();
        let svg = render_circle(&node, &layout, ThemeMode::Light, "dm", StyleMode::Inline);
        assert!(svg.contains("<ellipse"));
        assert!(svg.contains(r#"cx="60.0""#)); // 10 + 100/2
        assert!(svg.contains(r#"cy="45.0""#)); // 20 + 50/2
        assert!(svg.contains(r#"rx="50.0""#));
        assert!(svg.contains(r#"ry="25.0""#));
    }

    #[test]
    fn test_render_rect_includes_text() {
        let node = test_node();
        let layout = test_layout();
        let svg = render_rect(
            &node,
            &layout,
            ThemeMode::Light,
            "dm",
            4.0,
            StyleMode::Inline,
        );
        assert!(svg.contains("<text"));
        assert!(svg.contains("Test Node"));
    }

    #[test]
    fn test_node_with_subtitle_renders_both() {
        let mut node = test_node();
        node.subtitle = Some("Subtitle".into());
        let layout = test_layout();
        let svg = render_rect(
            &node,
            &layout,
            ThemeMode::Light,
            "dm",
            4.0,
            StyleMode::Inline,
        );
        assert!(svg.contains("Test Node"));
        assert!(svg.contains("Subtitle"));
        // Should have two text elements
        let text_count = svg.matches("<text").count();
        assert_eq!(text_count, 2);
    }

    #[test]
    fn test_emphasized_rect_uses_larger_radius() {
        let node = test_node();
        let layout = test_layout();
        let subtle = render_rect(
            &node,
            &layout,
            ThemeMode::Light,
            "dm",
            4.0,
            StyleMode::Inline,
        );
        let emphasized = render_rect(
            &node,
            &layout,
            ThemeMode::Light,
            "dm",
            8.0,
            StyleMode::Inline,
        );
        assert!(subtle.contains(r#"rx="4.0""#));
        assert!(emphasized.contains(r#"rx="8.0""#));
    }
}
