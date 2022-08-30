use crate::{BrushType, Color, LayerColor, Line, Page, Point};
use core::f32::{INFINITY, NEG_INFINITY};

pub(crate) struct BoundingBox {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl BoundingBox {
    pub fn new() -> BoundingBox {
        BoundingBox {
            min_x: INFINITY,
            min_y: INFINITY,
            max_x: NEG_INFINITY,
            max_y: NEG_INFINITY,
        }
    }

    pub fn enclose_point(mut self, point: &Point) -> BoundingBox {
        let radius = 0.5 * point.width;
        self.min_x = self.min_x.min(point.x - radius);
        self.min_y = self.min_y.min(point.y - radius);
        self.max_x = self.max_x.max(point.x + radius);
        self.max_y = self.max_y.max(point.y + radius);
        self
    }

    pub fn enclose_line(mut self, line: &Line) -> BoundingBox {
        for point in line.points.iter() {
            self = self.enclose_point(point);
        }
        self
    }

    pub fn enclose_page(mut self, page: &Page) -> BoundingBox {
        for layer in page.layers.iter() {
            for line in layer.lines.iter() {
                self = self.enclose_line(line);
            }
        }
        self
    }
}

pub fn line_to_css_color(line: &Line, layer_idx: usize, layer_colors: &[LayerColor]) -> String {
    // If no layer color is provided for this layer, default to the last layer we have colors for.
    let layer_colors = layer_colors.get(layer_idx).cloned().unwrap_or_default();
    match line.brush_type {
        BrushType::Highlighter => "rgb(240, 220, 40)".to_string(),
        _ => match line.color {
            Color::Black => layer_colors.black,
            Color::Grey => layer_colors.grey,
            Color::White => layer_colors.white,
            Color::Blue => layer_colors.blue,
            Color::Red => layer_colors.red,
            Color::Yellow => layer_colors.yellow,
            Color::Green => layer_colors.green,
            Color::Pink => layer_colors.pink,
            Color::GrayOverlapping => layer_colors.gray_overlapping,
        },
    }
}

/// Creates a vector of quadrilateral coordinates enclosing each segment of the
/// line. The length of the returned vector is always a multiple of 8 (4 points
/// à 2 coordinates per quadrilateral.)
pub(crate) fn segment_quads(line: &Line) -> Vec<f32> {
    let points = &line.points;
    let offset_distance = if points.is_empty() {
        0.0
    } else {
        points[0].width * 0.5
    };

    let offsets = line.offsets(offset_distance);

    offsets.iter().enumerate().fold(
        Vec::with_capacity(8 * (points.len() - 1)),
        |mut coords, (i, offset)| {
            let p1 = &points[i];
            let p2 = &points[i + 1];

            coords.extend_from_slice(&[
                p1.x + offset.x,
                p1.y + offset.y,
                p2.x + offset.x,
                p2.y + offset.y,
                p2.x - offset.x,
                p2.y - offset.y,
                p1.x - offset.x,
                p1.y - offset.y,
            ]);
            coords
        },
    )
}

#[test]
fn test_segment_quads() {
    let line = Line::with_points(
        Point {
            width: 10.0,
            ..Default::default()
        },
        &vec![(0.0, 0.0), (3.0, 4.0), (6.0, 4.0)][..],
    );
    assert_eq!(
        segment_quads(&line),
        vec![-4.0, 3.0, -1.0, 7.0, 7.0, 1.0, 4.0, -3.0, 3.0, 9.0, 6.0, 9.0, 6.0, -1.0, 3.0, -1.0]
    );
}
