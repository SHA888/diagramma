//! SVG element rendering dispatch.
//!
//! Provides rendering functions for nodes, edges, containers, and text elements.

use crate::tokens::{ColorRamp, ThemeMode};
use diagramma_core::{Node, NodeShape};
use diagramma_layout::{LayoutContainer, LayoutEdge, LayoutNode};

pub mod container;
pub mod edge;
pub mod node;
pub mod text;

/// Renders a layout node to SVG.
#[must_use]
pub fn render_node(
    node: &Node,
    layout: &LayoutNode,
    theme: ThemeMode,
    class_prefix: &str,
) -> String {
    match node.shape {
        NodeShape::Rect => node::render_rect(node, layout, theme, class_prefix, 4.0),
        NodeShape::Pill => node::render_pill(node, layout, theme, class_prefix),
        NodeShape::Diamond => node::render_diamond(node, layout, theme, class_prefix),
        NodeShape::Circle => node::render_circle(node, layout, theme, class_prefix),
    }
}

/// Renders a layout edge to SVG.
#[must_use]
pub fn render_edge(
    edge: &LayoutEdge,
    color: ColorRamp,
    theme: ThemeMode,
    class_prefix: &str,
) -> String {
    edge::render_path(edge, color, theme, class_prefix)
}

/// Renders a layout container to SVG.
#[must_use]
pub fn render_container(
    container: &LayoutContainer,
    label: &str,
    color: ColorRamp,
    theme: ThemeMode,
    class_prefix: &str,
    children_svg: &str,
) -> String {
    container::render(container, label, color, theme, class_prefix, children_svg)
}

/// Helper to format a CSS class attribute.
#[must_use]
pub fn class_attr(classes: &[&str]) -> String {
    if classes.is_empty() {
        String::new()
    } else {
        format!(r#" class="{}""#, classes.join(" "))
    }
}

/// Helper to build a style attribute from key-value pairs.
#[must_use]
pub fn style_attr(styles: &[(&str, &str)]) -> String {
    if styles.is_empty() {
        String::new()
    } else {
        let style_str: Vec<String> = styles.iter().map(|(k, v)| format!("{k}: {v}")).collect();
        format!(r#" style="{}""#, style_str.join("; "))
    }
}

/// Renders a complete SVG element group.
#[must_use]
pub fn render_group(id: &str, content: &str, class: &str) -> String {
    format!(r#"<g id="{id}" class="{class}">{content}</g>"#)
}

#[cfg(test)]
mod tests {
    use super::*;
    use diagramma_core::{ColorRamp, NodeId, NodeShape};

    fn test_node() -> Node {
        Node {
            id: NodeId::new("test"),
            label: "Test".into(),
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
    fn test_render_rect_produces_svg() {
        let node = test_node();
        let layout = test_layout();
        let svg = render_node(&node, &layout, ThemeMode::Light, "dm");
        assert!(svg.contains("<rect"));
        assert!(svg.contains(r#"x="10.0""#));
        assert!(svg.contains(r#"y="20.0""#));
        assert!(svg.contains(r#"width="100.0""#));
        assert!(svg.contains(r#"height="50.0""#));
    }

    #[test]
    fn test_class_attr_formats_correctly() {
        assert_eq!(class_attr(&[]), "");
        assert_eq!(class_attr(&["dm-blue-fill"]), r#" class="dm-blue-fill""#);
        assert_eq!(
            class_attr(&["dm-blue-fill", "dm-blue-stroke"]),
            r#" class="dm-blue-fill dm-blue-stroke""#
        );
    }

    #[test]
    fn test_style_attr_formats_correctly() {
        assert_eq!(style_attr(&[]), "");
        assert_eq!(style_attr(&[("fill", "red")]), r#" style="fill: red""#);
        assert_eq!(
            style_attr(&[("fill", "red"), ("stroke", "blue")]),
            r#" style="fill: red; stroke: blue""#
        );
    }
}
