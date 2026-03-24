use crate::types::{LayoutNode, Point};
use diagramma_core::Direction;

const CLEARANCE: f64 = 16.0;
const STEP: f64 = 24.0;
const EPSILON: f64 = 1e-9;

/// Route an edge with obstacle avoidance and directional connection points.
#[must_use]
pub fn route_edge(
    from: &LayoutNode,
    to: &LayoutNode,
    direction: Direction,
    obstacles: &[LayoutNode],
) -> Vec<Point> {
    let (start, end) = connection_points(direction, from, to);
    let (start, end) = nudge_connection_points(direction, start, end, from, to, obstacles);

    if !segments_hit_obstacles(&[(start, end)], from, to, obstacles) {
        return vec![start, end];
    }

    // Try vertical-then-horizontal L bend.
    let mut bend_y = f64::midpoint(start.y, end.y);
    bend_y = adjust_horizontal_clearance(bend_y, start.x, end.x, from, to, obstacles);
    let mut candidate = vec![
        start,
        Point::new(start.x, bend_y),
        Point::new(end.x, bend_y),
        end,
    ];
    if !segments_hit_obstacles(
        &[
            (start, Point::new(start.x, bend_y)),
            (Point::new(start.x, bend_y), Point::new(end.x, bend_y)),
            (Point::new(end.x, bend_y), end),
        ],
        from,
        to,
        obstacles,
    ) {
        return candidate;
    }

    // Try horizontal-then-vertical L bend.
    let mut bend_x = f64::midpoint(start.x, end.x);
    bend_x = adjust_vertical_clearance(bend_x, start.y, end.y, from, to, obstacles);
    candidate = vec![
        start,
        Point::new(bend_x, start.y),
        Point::new(bend_x, end.y),
        end,
    ];
    if !segments_hit_obstacles(
        &[
            (start, Point::new(bend_x, start.y)),
            (Point::new(bend_x, start.y), Point::new(bend_x, end.y)),
            (Point::new(bend_x, end.y), end),
        ],
        from,
        to,
        obstacles,
    ) {
        return candidate;
    }

    // Fallback: dogleg around obstacles by offsetting vertically then horizontally.
    bend_y = find_clear_line(start.y, 1.0, |y| {
        !horizontal_hits_obstacle(y, start.x, end.x, from, to, obstacles)
    });
    vec![
        start,
        Point::new(start.x, bend_y),
        Point::new(end.x, bend_y),
        end,
    ]
}

fn connection_points(direction: Direction, from: &LayoutNode, to: &LayoutNode) -> (Point, Point) {
    match direction {
        Direction::TopDown => (bottom_center(from), top_center(to)),
        Direction::BottomUp => (top_center(from), bottom_center(to)),
        Direction::LeftRight => (right_center(from), left_center(to)),
        Direction::RightLeft => (left_center(from), right_center(to)),
    }
}

fn bottom_center(node: &LayoutNode) -> Point {
    Point::new(node.x + node.width / 2.0, node.y + node.height + CLEARANCE)
}

fn top_center(node: &LayoutNode) -> Point {
    Point::new(node.x + node.width / 2.0, node.y - CLEARANCE)
}

fn left_center(node: &LayoutNode) -> Point {
    Point::new(node.x - CLEARANCE, node.y + node.height / 2.0)
}

fn right_center(node: &LayoutNode) -> Point {
    Point::new(node.x + node.width + CLEARANCE, node.y + node.height / 2.0)
}

fn adjust_horizontal_clearance(
    mut y: f64,
    x1: f64,
    x2: f64,
    from: &LayoutNode,
    to: &LayoutNode,
    obstacles: &[LayoutNode],
) -> f64 {
    let mut attempts = 0;
    while horizontal_hits_obstacle(y, x1, x2, from, to, obstacles) && attempts < 12 {
        y += STEP;
        attempts += 1;
    }
    y
}

fn adjust_vertical_clearance(
    mut x: f64,
    y1: f64,
    y2: f64,
    from: &LayoutNode,
    to: &LayoutNode,
    obstacles: &[LayoutNode],
) -> f64 {
    let mut attempts = 0;
    while vertical_hits_obstacle(x, y1, y2, from, to, obstacles) && attempts < 12 {
        x += STEP;
        attempts += 1;
    }
    x
}

fn find_clear_line<F>(mut value: f64, step_sign: f64, predicate: F) -> f64
where
    F: Fn(f64) -> bool,
{
    let mut attempts = 0;
    while !predicate(value) && attempts < 20 {
        value += STEP * step_sign;
        attempts += 1;
    }
    value
}

fn nudge_connection_points(
    direction: Direction,
    mut start: Point,
    mut end: Point,
    from: &LayoutNode,
    to: &LayoutNode,
    obstacles: &[LayoutNode],
) -> (Point, Point) {
    match direction {
        Direction::LeftRight => {
            start.x = shift_clear_right(start.x, from, to, obstacles);
            end.x = shift_clear_left(end.x, from, to, obstacles);
        }
        Direction::RightLeft => {
            start.x = shift_clear_left(start.x, from, to, obstacles);
            end.x = shift_clear_right(end.x, from, to, obstacles);
        }
        Direction::TopDown => {
            start.y = shift_clear_down(start.y, from, to, obstacles);
            end.y = shift_clear_up(end.y, from, to, obstacles);
        }
        Direction::BottomUp => {
            start.y = shift_clear_up(start.y, from, to, obstacles);
            end.y = shift_clear_down(end.y, from, to, obstacles);
        }
    }

    (start, end)
}

fn shift_clear_right(x: f64, from: &LayoutNode, to: &LayoutNode, obstacles: &[LayoutNode]) -> f64 {
    let mut value = x;
    let mut attempts = 0;
    while vertical_hits_obstacle(
        value,
        from.y - CLEARANCE,
        from.y + from.height + CLEARANCE,
        from,
        to,
        obstacles,
    ) && attempts < 12
    {
        value += STEP;
        attempts += 1;
    }
    value
}

fn shift_clear_left(x: f64, from: &LayoutNode, to: &LayoutNode, obstacles: &[LayoutNode]) -> f64 {
    let mut value = x;
    let mut attempts = 0;
    while vertical_hits_obstacle(
        value,
        to.y - CLEARANCE,
        to.y + to.height + CLEARANCE,
        from,
        to,
        obstacles,
    ) && attempts < 12
    {
        value -= STEP;
        attempts += 1;
    }
    value
}

fn shift_clear_down(y: f64, from: &LayoutNode, to: &LayoutNode, obstacles: &[LayoutNode]) -> f64 {
    let mut value = y;
    let mut attempts = 0;
    while horizontal_hits_obstacle(
        value,
        from.x - CLEARANCE,
        from.x + from.width + CLEARANCE,
        from,
        to,
        obstacles,
    ) && attempts < 12
    {
        value += STEP;
        attempts += 1;
    }
    value
}

fn shift_clear_up(y: f64, from: &LayoutNode, to: &LayoutNode, obstacles: &[LayoutNode]) -> f64 {
    let mut value = y;
    let mut attempts = 0;
    while horizontal_hits_obstacle(
        value,
        to.x - CLEARANCE,
        to.x + to.width + CLEARANCE,
        from,
        to,
        obstacles,
    ) && attempts < 12
    {
        value -= STEP;
        attempts += 1;
    }
    value
}

fn segments_hit_obstacles(
    segments: &[(Point, Point)],
    from: &LayoutNode,
    to: &LayoutNode,
    obstacles: &[LayoutNode],
) -> bool {
    for obstacle in obstacles {
        if obstacle.id == from.id || obstacle.id == to.id {
            continue;
        }
        for (start, end) in segments {
            if segment_intersects_obstacle(*start, *end, obstacle) {
                return true;
            }
        }
    }
    false
}

fn horizontal_hits_obstacle(
    y: f64,
    x1: f64,
    x2: f64,
    from: &LayoutNode,
    to: &LayoutNode,
    obstacles: &[LayoutNode],
) -> bool {
    segments_hit_obstacles(
        &[(Point::new(x1, y), Point::new(x2, y))],
        from,
        to,
        obstacles,
    )
}

fn vertical_hits_obstacle(
    x: f64,
    y1: f64,
    y2: f64,
    from: &LayoutNode,
    to: &LayoutNode,
    obstacles: &[LayoutNode],
) -> bool {
    segments_hit_obstacles(
        &[(Point::new(x, y1), Point::new(x, y2))],
        from,
        to,
        obstacles,
    )
}

fn segment_intersects_obstacle(start: Point, end: Point, obstacle: &LayoutNode) -> bool {
    if (start.x - end.x).abs() < EPSILON {
        // Vertical segment
        let x = start.x;
        if x < obstacle.x - CLEARANCE || x > obstacle.x + obstacle.width + CLEARANCE {
            return false;
        }
        let (y1, y2) = if start.y < end.y {
            (start.y, end.y)
        } else {
            (end.y, start.y)
        };
        let obs_y1 = obstacle.y - CLEARANCE;
        let obs_y2 = obstacle.y + obstacle.height + CLEARANCE;
        return y2 > obs_y1 && y1 < obs_y2;
    } else if (start.y - end.y).abs() < EPSILON {
        // Horizontal segment
        let y = start.y;
        if y < obstacle.y - CLEARANCE || y > obstacle.y + obstacle.height + CLEARANCE {
            return false;
        }
        let (x1, x2) = if start.x < end.x {
            (start.x, end.x)
        } else {
            (end.x, start.x)
        };
        let obs_x1 = obstacle.x - CLEARANCE;
        let obs_x2 = obstacle.x + obstacle.width + CLEARANCE;
        return x2 > obs_x1 && x1 < obs_x2;
    }
    false
}
