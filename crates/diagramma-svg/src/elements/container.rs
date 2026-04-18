//! Container rendering (nested rects with labels).
//!
//! Implements themed container rendering with background and border.

use crate::elements::{class_attr, text};
use crate::tokens::{ColorRamp, SemanticRole, ThemeMode, color_class, css_var};
use diagramma_layout::LayoutContainer;

/// Renders a container with nested children.
///
/// # Arguments
/// * `container` - The layout container data
/// * `label` - The container label text
/// * `color` - Color ramp for theming
/// * `theme` - Current theme mode
/// * `class_prefix` - Prefix for CSS class names
/// * `children_svg` - Pre-rendered SVG content for children
#[must_use]
pub fn render(
    container: &LayoutContainer,
    label: &str,
    color: ColorRamp,
    theme: ThemeMode,
    class_prefix: &str,
    children_svg: &str,
) -> String {
    let fill_class = color_class(color, SemanticRole::Fill);
    let stroke_class = color_class(color, SemanticRole::Stroke);
    let classes = format!("{class_prefix} {fill_class} {stroke_class} dm-container");

    // Container background rect with subtle styling
    let style = format!(
        "fill: var({}); stroke: var({}); stroke-width: 0.5; stroke-dasharray: 2,2",
        css_var(color, SemanticRole::Fill, theme),
        css_var(color, SemanticRole::Stroke, theme)
    );

    let rect = format!(
        r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="4"{} style="{}"/>"#,
        container.x,
        container.y,
        container.width,
        container.height,
        class_attr(&[&classes]),
        style
    );

    // Container label (positioned at top with padding)
    let label_padding = 8.0;
    let label_x = container.x + container.width / 2.0;
    let label_y = container.y + label_padding + 10.0; // 10px approximates font baseline

    let label_svg = text::render_container_label(label, label_x, label_y, color, theme);

    // Group container background, label, and children
    format!(r#"<g class="dm-container-group">{rect}{label_svg}{children_svg}</g>"#)
}

/// Renders a container without children (for simple container nodes).
#[must_use]
pub fn render_simple(
    container: &LayoutContainer,
    label: &str,
    color: ColorRamp,
    theme: ThemeMode,
    class_prefix: &str,
) -> String {
    render(container, label, color, theme, class_prefix, "")
}

/// Renders a nested container structure recursively.
#[must_use]
pub fn render_nested<F>(
    container: &LayoutContainer,
    label: &str,
    color: ColorRamp,
    theme: ThemeMode,
    class_prefix: &str,
    render_child: &mut F,
) -> String
where
    F: FnMut(&diagramma_layout::LayoutElement) -> String,
{
    // Render all children first
    let children_svg: String = container
        .children
        .iter()
        .map(render_child)
        .collect::<String>();

    render(container, label, color, theme, class_prefix, &children_svg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use diagramma_core::NodeId;
    use diagramma_layout::{LayoutContainer, LayoutElement, LayoutNode};

    fn test_container() -> LayoutContainer {
        LayoutContainer {
            id: NodeId::new("container-1"),
            x: 10.0,
            y: 20.0,
            width: 200.0,
            height: 150.0,
            children: vec![LayoutElement::Node(LayoutNode {
                id: NodeId::new("child-1"),
                x: 30.0,
                y: 50.0,
                width: 50.0,
                height: 30.0,
            })],
        }
    }

    #[test]
    fn test_render_produces_container_svg() {
        let container = test_container();
        let svg = render_simple(
            &container,
            "Container",
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm",
        );
        assert!(svg.contains("<rect"));
        assert!(svg.contains("<text"));
        assert!(svg.contains("Container"));
        assert!(svg.contains("dm-container"));
    }

    #[test]
    fn test_render_includes_position() {
        let container = test_container();
        let svg = render_simple(
            &container,
            "Container",
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm",
        );
        assert!(svg.contains(r#"x="10.0""#));
        assert!(svg.contains(r#"y="20.0""#));
        assert!(svg.contains(r#"width="200.0""#));
        assert!(svg.contains(r#"height="150.0""#));
    }

    #[test]
    fn test_render_uses_css_variables() {
        let container = test_container();
        let svg = render_simple(
            &container,
            "Container",
            ColorRamp::Teal,
            ThemeMode::Light,
            "dm",
        );
        assert!(svg.contains("var(--dm-teal-50)")); // fill
        assert!(svg.contains("var(--dm-teal-600)")); // stroke
    }

    #[test]
    fn test_render_has_dashed_stroke() {
        let container = test_container();
        let svg = render_simple(
            &container,
            "Container",
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm",
        );
        assert!(svg.contains("stroke-dasharray: 2,2"));
    }

    #[test]
    fn test_render_uses_subtle_corner_radius() {
        let container = test_container();
        let svg = render_simple(
            &container,
            "Container",
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm",
        );
        assert!(svg.contains(r#"rx="4""#));
    }

    #[test]
    fn test_render_label_positioned_near_top() {
        let container = test_container();
        let svg = render_simple(
            &container,
            "Container",
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm",
        );
        // Label should be centered horizontally (x=110) and near top (y≈38)
        assert!(svg.contains(r#"x="110.0""#));
    }

    #[test]
    fn test_render_wraps_in_group() {
        let container = test_container();
        let svg = render_simple(
            &container,
            "Container",
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm",
        );
        assert!(svg.starts_with("<g"));
        assert!(svg.ends_with("</g>"));
    }

    #[test]
    fn test_render_with_children_includes_them() {
        let container = test_container();
        let children_svg = r#"<rect x="30" y="50" width="50" height="30"/>"#;
        let svg = render(
            &container,
            "Container",
            ColorRamp::Blue,
            ThemeMode::Light,
            "dm",
            children_svg,
        );
        assert!(svg.contains(r#"<rect x="30" y="50""#));
    }

    #[test]
    fn test_render_uses_class_based_coloring() {
        let container = test_container();
        let svg = render_simple(
            &container,
            "Container",
            ColorRamp::Purple,
            ThemeMode::Light,
            "dm",
        );
        assert!(svg.contains("dm-purple-fill"));
        assert!(svg.contains("dm-purple-stroke"));
    }

    #[test]
    fn test_render_dark_mode_uses_dark_values() {
        let container = test_container();
        let svg = render_simple(
            &container,
            "Container",
            ColorRamp::Blue,
            ThemeMode::Dark,
            "dm",
        );
        assert!(svg.contains("var(--dm-blue-800)")); // fill in dark
        assert!(svg.contains("var(--dm-blue-200)")); // stroke in dark
    }
}
