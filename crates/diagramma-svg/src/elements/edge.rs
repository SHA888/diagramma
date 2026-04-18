//! Edge and arrow rendering.
//!
//! Implements path rendering with fill="none" and 0.5px stroke, plus arrow markers.

use crate::elements::{class_attr, text};
use crate::tokens::{ColorRamp, SemanticRole, ThemeMode, color_class, css_var};
use diagramma_layout::{LayoutEdge, Point};

/// Renders an edge path with styling.
///
/// Paths have `fill="none"` and 0.5px stroke as per the design spec.
#[must_use]
pub fn render_path(
    edge: &LayoutEdge,
    color: ColorRamp,
    theme: ThemeMode,
    class_prefix: &str,
) -> String {
    if edge.path.len() < 2 {
        return String::new(); // Need at least 2 points for a path
    }

    let path_data = build_path_data(&edge.path);
    let stroke_class = color_class(color, SemanticRole::Edge);
    let classes = format!("{class_prefix} {stroke_class} dm-edge");

    let style = format!(
        "fill: none; stroke: var({}); stroke-width: 0.5",
        css_var(color, SemanticRole::Edge, theme)
    );

    let path = format!(
        r#"<path d="{}" fill="none"{} style="{}"/>"#,
        path_data,
        class_attr(&[&classes]),
        style
    );

    // Render arrow head
    let arrow = render_arrow_head(&edge.arrow_pos, color, theme, class_prefix);

    format!("{path}{arrow}")
}

/// Builds SVG path data from a series of points.
/// Uses 'M' (move) for the first point and 'L' (line) for subsequent points.
#[must_use]
pub fn build_path_data(points: &[Point]) -> String {
    use std::fmt::Write;

    if points.is_empty() {
        return String::new();
    }

    let mut result = format!("M {:.1},{:.1}", points[0].x, points[0].y);

    for point in points.iter().skip(1) {
        let _ = write!(result, " L {:.1},{:.1}", point.x, point.y);
    }

    result
}

/// Renders an open chevron arrow head at the specified position.
///
/// The arrow head points in the direction of the last path segment.
#[must_use]
pub fn render_arrow_head(
    pos: &Point,
    color: ColorRamp,
    theme: ThemeMode,
    class_prefix: &str,
) -> String {
    let arrow_class = color_class(color, SemanticRole::Arrow);
    let classes = format!("{class_prefix} {arrow_class} dm-arrow");

    let style = format!(
        "fill: none; stroke: var({}); stroke-width: 0.5",
        css_var(color, SemanticRole::Arrow, theme)
    );

    // Open chevron: two lines forming a V shape
    // Size: 6px wide, 6px tall (pointing right by default)
    let chevron_size = 6.0;
    let half_size = chevron_size / 2.0;

    // Points for the open chevron (left-pointing, can be rotated if needed)
    let p1 = Point::new(pos.x - chevron_size, pos.y - half_size);
    let p2 = Point::new(pos.x, pos.y);
    let p3 = Point::new(pos.x - chevron_size, pos.y + half_size);

    let path_data = format!(
        "M {:.1},{:.1} L {:.1},{:.1} L {:.1},{:.1}",
        p1.x, p1.y, p2.x, p2.y, p3.x, p3.y
    );

    format!(
        r#"<path d="{}" fill="none"{} style="{}"/>"#,
        path_data,
        class_attr(&[&classes]),
        style
    )
}

/// Renders an orthogonal edge with right-angle bends.
#[must_use]
pub fn render_orthogonal(
    edge: &LayoutEdge,
    color: ColorRamp,
    theme: ThemeMode,
    class_prefix: &str,
) -> String {
    // For orthogonal edges, the path is already routed with L-bends
    // Just render it the same way as a regular path
    render_path(edge, color, theme, class_prefix)
}

/// Renders a straight (direct) edge.
#[must_use]
pub fn render_direct(
    from: &Point,
    to: &Point,
    color: ColorRamp,
    theme: ThemeMode,
    class_prefix: &str,
) -> String {
    let edge = LayoutEdge {
        id: "direct".to_string(),
        path: vec![*from, *to],
        arrow_pos: *to,
    };
    render_path(&edge, color, theme, class_prefix)
}

/// Renders an edge label positioned along the path.
#[must_use]
pub fn render_edge_label(
    label: &str,
    midpoint: &Point,
    color: ColorRamp,
    theme: ThemeMode,
) -> String {
    // Use text rendering with a small offset for better readability
    let y_offset = -4.0;
    text::render_label(
        label,
        midpoint.x,
        midpoint.y + y_offset,
        color,
        theme,
        "dm-edge-label",
        10,
    )
}

/// Calculates the midpoint of a path for label positioning.
#[must_use]
pub fn calculate_midpoint(points: &[Point]) -> Point {
    if points.len() < 2 {
        return points.first().copied().unwrap_or(Point::new(0.0, 0.0));
    }

    // For even number of points, use the middle segment's midpoint
    // For odd number, use the middle point
    let mid_index = points.len() / 2;

    if points.len() % 2 == 0 {
        // Even: return midpoint of segment at mid_index - 1
        let p1 = &points[mid_index - 1];
        let p2 = &points[mid_index];
        Point::new(f64::midpoint(p1.x, p2.x), f64::midpoint(p1.y, p2.y))
    } else {
        // Odd: return the middle point
        points[mid_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_edge() -> LayoutEdge {
        LayoutEdge {
            id: "test-edge".to_string(),
            path: vec![
                Point::new(0.0, 0.0),
                Point::new(50.0, 0.0),
                Point::new(50.0, 50.0),
            ],
            arrow_pos: Point::new(50.0, 50.0),
        }
    }

    #[test]
    fn test_render_path_produces_svg() {
        let edge = test_edge();
        let svg = render_path(&edge, ColorRamp::Blue, ThemeMode::Light, "dm");
        assert!(svg.contains("<path"));
        assert!(svg.contains('d'));
        assert!(svg.contains(r#"fill="none""#));
        assert!(svg.contains("stroke-width: 0.5"));
    }

    #[test]
    fn test_build_path_data_formats_correctly() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(50.0, 0.0),
            Point::new(50.0, 50.0),
        ];
        let data = build_path_data(&points);
        assert_eq!(data, "M 0.0,0.0 L 50.0,0.0 L 50.0,50.0");
    }

    #[test]
    fn test_build_path_data_empty_returns_empty() {
        let data = build_path_data(&[]);
        assert!(data.is_empty());
    }

    #[test]
    fn test_build_path_data_single_point() {
        let points = vec![Point::new(10.0, 20.0)];
        let data = build_path_data(&points);
        assert_eq!(data, "M 10.0,20.0");
    }

    #[test]
    fn test_render_path_uses_css_variables() {
        let edge = test_edge();
        let svg = render_path(&edge, ColorRamp::Blue, ThemeMode::Light, "dm");
        assert!(svg.contains("var(--dm-blue-600)"));
    }

    #[test]
    fn test_render_arrow_head_produces_chevron() {
        let pos = Point::new(100.0, 50.0);
        let svg = render_arrow_head(&pos, ColorRamp::Teal, ThemeMode::Light, "dm");
        assert!(svg.contains("<path"));
        assert!(svg.contains(r#"fill="none""#));
        // Should be an open chevron (two line segments)
        assert!(svg.contains("M "));
        assert!(svg.contains(" L "));
    }

    #[test]
    fn test_render_direct_creates_straight_path() {
        let from = Point::new(0.0, 0.0);
        let to = Point::new(100.0, 0.0);
        let svg = render_direct(&from, &to, ColorRamp::Gray, ThemeMode::Light, "dm");
        assert!(svg.contains("M 0.0,0.0"));
        assert!(svg.contains("L 100.0,0.0"));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_calculate_midpoint_odd_count() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(50.0, 0.0),
            Point::new(100.0, 0.0),
        ];
        let mid = calculate_midpoint(&points);
        assert_eq!(mid.x, 50.0);
        assert_eq!(mid.y, 0.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_calculate_midpoint_even_count() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(50.0, 0.0),
            Point::new(100.0, 0.0),
            Point::new(150.0, 0.0),
        ];
        let mid = calculate_midpoint(&points);
        assert_eq!(mid.x, 75.0); // midpoint of segment 1-2
        assert_eq!(mid.y, 0.0);
    }

    #[test]
    fn test_render_path_includes_arrow() {
        let edge = test_edge();
        let svg = render_path(&edge, ColorRamp::Blue, ThemeMode::Light, "dm");
        // Should contain two paths: one for the edge, one for the arrow
        let path_count = svg.matches("<path").count();
        assert_eq!(path_count, 2);
    }

    #[test]
    fn test_render_edge_uses_class_based_coloring() {
        let edge = test_edge();
        let svg = render_path(&edge, ColorRamp::Green, ThemeMode::Light, "dm");
        assert!(svg.contains("dm-green-edge"));
        assert!(svg.contains("dm-green-arrow"));
    }
}
