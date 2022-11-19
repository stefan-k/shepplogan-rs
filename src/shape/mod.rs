// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod boundingbox;
mod ellipse;

use boundingbox::BoundingBox;
use ellipse::{Ellipse, EllipseOnCanvas};

/// Representation of a shape.
///
/// A shape is defined on a 2D area where both x- and y-direction are in [-1.0, 1.0].
/// The shape will later be scaled onto the actual canvas given by the desired dimensions of the
/// phantom.
#[derive(Clone, PartialEq)]
pub struct Shape {
    intensity: f64,
    kind: ShapeKind,
}

/// Represents the kind of shape
#[derive(PartialEq, Clone)]
enum ShapeKind {
    Ellipse(Ellipse),
}

impl Shape {
    /// Create an ellipse
    ///
    /// The canvas for defining ellipses is square and ranges from -1 to 1 on both axes `x` and `y`.
    ///
    /// # Parameters
    ///
    /// * `center_x`: x component of center on the canvas
    /// * `center_y`: y component of center on the canvas
    /// * `major_axis`: major axis length
    /// * `minor_axis`: minor axis length
    /// * `theta`: Rotation angle of the ellipse in degrees
    ///
    /// # Example
    ///
    /// ```
    /// # use shepplogan::Shape;
    /// let ellipse = Shape::ellipse(0.1, -0.4, 0.6, 0.2, 20.0, 1.0);
    /// ```
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

    /// Transforms the shape onto the canvas size given by the dimensions `nx` and `ny` of the final
    /// phantom.
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

/// A shape scaled onto a canvas given by the phantom dimensions
#[derive(Clone, PartialEq)]
pub(crate) struct ShapeOnCanvas {
    intensity: f64,
    kind: ShapeKindOnCanvas,
}

/// All possible shapes on canvases
#[derive(Clone, PartialEq)]
pub(crate) enum ShapeKindOnCanvas {
    Ellipse(EllipseOnCanvas),
}

impl ShapeOnCanvas {
    /// Checks if a point is inside a shape
    #[inline(always)]
    pub(crate) fn inside(&self, x: f64, y: f64) -> bool {
        match &self.kind {
            ShapeKindOnCanvas::Ellipse(shape) => shape.inside(x, y),
        }
    }

    /// Return intensity of the shape
    #[inline(always)]
    pub(crate) fn intensity(&self) -> f64 {
        self.intensity
    }

    /// Return the bounding box of the ellipse
    #[inline(always)]
    pub(crate) fn bounding_box(&self) -> BoundingBox {
        match &self.kind {
            ShapeKindOnCanvas::Ellipse(shape) => shape.bounding_box(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Shape, ShapeKind, ShapeKindOnCanvas};

    #[derive(Debug, Copy, Clone)]
    struct FloatNotNan(f64);

    impl quickcheck::Arbitrary for FloatNotNan {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let val = f64::arbitrary(g);
                if !val.is_nan() && val.is_finite() {
                    return FloatNotNan(val);
                }
            }
        }
    }

    #[quickcheck]
    fn test_shape_ellipse(
        center_x_input: FloatNotNan,
        center_y_input: FloatNotNan,
        major_axis_input: FloatNotNan,
        minor_axis_input: FloatNotNan,
        theta_input: FloatNotNan,
        intensity_input: FloatNotNan,
    ) -> bool {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let major_axis_input = major_axis_input.0;
        let minor_axis_input = minor_axis_input.0;
        let theta_input = theta_input.0;
        let intensity_input = intensity_input.0;
        let Shape { intensity, kind } = Shape::ellipse(
            center_x_input,
            center_y_input,
            major_axis_input,
            minor_axis_input,
            theta_input,
            intensity_input,
        );

        #[allow(irrefutable_let_patterns)]
        if let ShapeKind::Ellipse(kind) = kind {
            kind.center_x.to_ne_bytes() == center_x_input.to_ne_bytes()
                && kind.center_y.to_ne_bytes() == center_y_input.to_ne_bytes()
                && kind.major_axis.to_ne_bytes() == major_axis_input.to_ne_bytes()
                && kind.minor_axis.to_ne_bytes() == minor_axis_input.to_ne_bytes()
                && kind.theta.to_ne_bytes() == theta_input.to_ne_bytes()
                && intensity.to_ne_bytes() == intensity_input.to_ne_bytes()
        } else {
            false
        }
    }

    #[quickcheck]
    // Add a reason why this lint is allowed once the feature `lint_reasons` is stabilized.
    #[allow(clippy::too_many_arguments)]
    fn test_shape_ellipse_on_canvas(
        center_x_input: FloatNotNan,
        center_y_input: FloatNotNan,
        major_axis_input: FloatNotNan,
        minor_axis_input: FloatNotNan,
        theta_input: FloatNotNan,
        intensity_input: FloatNotNan,
        nx: u32,
        ny: u32,
    ) -> bool {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let major_axis_input = major_axis_input.0;
        let minor_axis_input = minor_axis_input.0;
        let theta_input = theta_input.0;
        let intensity_input = intensity_input.0;
        let ellipse_on_canvas = Shape::ellipse(
            center_x_input,
            center_y_input,
            major_axis_input,
            minor_axis_input,
            theta_input,
            intensity_input,
        )
        .on_canvas(nx, ny);

        #[allow(irrefutable_let_patterns)]
        if let ShapeKindOnCanvas::Ellipse(_) = ellipse_on_canvas.kind {
            ellipse_on_canvas.intensity().to_ne_bytes() == intensity_input.to_ne_bytes()
        } else {
            false
        }
    }

    #[quickcheck]
    // Add a reason why this lint is allowed once the feature `lint_reasons` is stabilized.
    #[allow(clippy::too_many_arguments)]
    fn test_shape_ellipse_on_canvas_inside_and_bbox(
        center_x_input: FloatNotNan,
        center_y_input: FloatNotNan,
        major_axis_input: FloatNotNan,
        minor_axis_input: FloatNotNan,
        theta_input: FloatNotNan,
        intensity_input: FloatNotNan,
        x: FloatNotNan,
        y: FloatNotNan,
    ) -> bool {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let major_axis_input = major_axis_input.0;
        let minor_axis_input = minor_axis_input.0;
        let theta_input = theta_input.0;
        let intensity_input = intensity_input.0;
        let x = x.0;
        let y = y.0;
        let ellipse_on_canvas = Shape::ellipse(
            center_x_input,
            center_y_input,
            major_axis_input,
            minor_axis_input,
            theta_input,
            intensity_input,
        )
        .on_canvas(128, 128);

        #[allow(irrefutable_let_patterns)]
        if let ShapeKindOnCanvas::Ellipse(kind) = ellipse_on_canvas.kind.clone() {
            ellipse_on_canvas.bounding_box() == kind.bounding_box()
                && ellipse_on_canvas.inside(x, y) == kind.inside(x, y)
        } else {
            false
        }
    }
}
