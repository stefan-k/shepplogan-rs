// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod ellipse;

use ellipse::{Ellipse, EllipseOnCanvas};

/// A shape (TODO)
pub struct Shape {
    intensity: f64,
    kind: ShapeKind,
}

enum ShapeKind {
    Ellipse(Ellipse),
}

impl Shape {
    /// Create an ellipse
    ///
    /// Todo
    pub fn ellipse(
        center_x: f64,
        center_y: f64,
        major_axis: f64,
        minor_axis: f64,
        theta: f64,
        intensity: f64,
    ) -> Shape {
        Shape {
            intensity,
            kind: ShapeKind::Ellipse(Ellipse::new(
                center_x, center_y, major_axis, minor_axis, theta,
            )),
        }
    }

    pub(crate) fn on_canvas(&self, nx: u32, ny: u32) -> ShapeOnCanvas {
        let Self { intensity, kind } = self;
        ShapeOnCanvas {
            intensity: *intensity,
            kind: match kind {
                ShapeKind::Ellipse(shape) => ShapeKindOnCanvas::Ellipse(shape.on_canvas(nx, ny)),
            },
        }
    }
}

/// A shape (TODO)
pub(crate) struct ShapeOnCanvas {
    intensity: f64,
    kind: ShapeKindOnCanvas,
}

pub(crate) enum ShapeKindOnCanvas {
    Ellipse(EllipseOnCanvas),
}

impl ShapeOnCanvas {
    /// Checks if a point is inside a shape
    #[inline(always)]
    pub fn inside(&self, x: f64, y: f64) -> bool {
        match &self.kind {
            ShapeKindOnCanvas::Ellipse(shape) => shape.inside(x, y),
        }
    }

    /// Return intensity of the shape
    #[inline(always)]
    pub fn intensity(&self) -> f64 {
        self.intensity
    }

    /// Return the bounding box of the ellipse
    #[inline(always)]
    pub fn bounding_box(&self) -> BoundingBox {
        match &self.kind {
            ShapeKindOnCanvas::Ellipse(shape) => shape.bounding_box(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct BoundingBox {
    pub(crate) x_low: u32,
    pub(crate) x_high: u32,
    pub(crate) y_low: u32,
    pub(crate) y_high: u32,
}

impl From<(u32, u32, u32, u32)> for BoundingBox {
    fn from((x_low, x_high, y_low, y_high): (u32, u32, u32, u32)) -> BoundingBox {
        BoundingBox {
            x_low,
            x_high,
            y_low,
            y_high,
        }
    }
}
