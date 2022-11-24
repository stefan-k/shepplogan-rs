// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod boundingbox;
mod ellipse;
mod rectangle;

use boundingbox::BoundingBox;
use ellipse::{Ellipse, EllipseOnCanvas};
use rectangle::{Rectangle, RectangleOnCanvas};

/// Representation of a shape.
///
/// A shape is defined on a 2D area where both x- and y-direction are in [-1.0, 1.0].
/// The shape will later be scaled onto the actual canvas given by the desired dimensions of the
/// phantom.
#[derive(Clone, PartialEq, Debug)]
pub struct Shape {
    intensity: f64,
    kind: ShapeKind,
}

/// Represents the kind of shape
#[derive(PartialEq, Clone, Debug)]
enum ShapeKind {
    Ellipse(Ellipse),
    Rectangle(Rectangle),
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

    /// Create a rectangle
    ///
    /// The canvas for defining rectangles is square and ranges from -1 to 1 on both axes `x` and `y`.
    ///
    /// # Parameters
    ///
    /// * `center_x`: x component of center on the canvas
    /// * `center_y`: y component of center on the canvas
    /// * `width`: width of rectangle
    /// * `height`: height of rectangle
    /// * `theta`: Rotation angle of the rectangle in degrees
    ///
    /// # Example
    ///
    /// ```
    /// # use shepplogan::Shape;
    /// let rectangle = Shape::rectangle(0.1, -0.4, 0.6, 0.2, 20.0, 1.0);
    /// ```
    pub fn rectangle(
        center_x: f64,
        center_y: f64,
        width: f64,
        height: f64,
        theta: f64,
        intensity: f64,
    ) -> Shape {
        Shape {
            intensity,
            kind: ShapeKind::Rectangle(Rectangle::new(center_x, center_y, width, height, theta)),
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
                ShapeKind::Rectangle(shape) => {
                    ShapeKindOnCanvas::Rectangle(shape.on_canvas(nx, ny))
                }
            },
        }
    }
}

/// A shape scaled onto a canvas given by the phantom dimensions
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct ShapeOnCanvas {
    intensity: f64,
    kind: ShapeKindOnCanvas,
}

/// All possible shapes on canvases
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum ShapeKindOnCanvas {
    Ellipse(EllipseOnCanvas),
    Rectangle(RectangleOnCanvas),
}

impl ShapeOnCanvas {
    /// Checks if a point is inside a shape
    #[inline(always)]
    pub(crate) fn inside(&self, x: f64, y: f64) -> bool {
        match &self.kind {
            ShapeKindOnCanvas::Ellipse(shape) => shape.inside(x, y),
            ShapeKindOnCanvas::Rectangle(shape) => shape.inside(x, y),
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
            ShapeKindOnCanvas::Rectangle(shape) => shape.bounding_box(),
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

        matches!(
            kind,
            ShapeKind::Ellipse(kind)
            if kind.center_x.to_ne_bytes() == center_x_input.to_ne_bytes()
                && kind.center_y.to_ne_bytes() == center_y_input.to_ne_bytes()
                && kind.major_axis.to_ne_bytes() == major_axis_input.to_ne_bytes()
                && kind.minor_axis.to_ne_bytes() == minor_axis_input.to_ne_bytes()
                && kind.theta.to_ne_bytes() == theta_input.to_ne_bytes()
                && intensity.to_ne_bytes() == intensity_input.to_ne_bytes()
        )
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

        matches!(
            ellipse_on_canvas.kind,
            ShapeKindOnCanvas::Ellipse(_x)
            if ellipse_on_canvas.intensity().to_ne_bytes() == intensity_input.to_ne_bytes()
        )
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

        matches!(
            ellipse_on_canvas.kind,
            ShapeKindOnCanvas::Ellipse(kind)
            if ellipse_on_canvas.bounding_box() == kind.bounding_box()
                && ellipse_on_canvas.inside(x, y) == kind.inside(x, y)
        )
    }

    #[quickcheck]
    fn test_shape_rectangle(
        center_x_input: FloatNotNan,
        center_y_input: FloatNotNan,
        width_input: FloatNotNan,
        height_input: FloatNotNan,
        theta_input: FloatNotNan,
        intensity_input: FloatNotNan,
    ) -> bool {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let width_input = width_input.0;
        let height_input = height_input.0;
        let theta_input = theta_input.0;
        let intensity_input = intensity_input.0;
        let Shape { intensity, kind } = Shape::rectangle(
            center_x_input,
            center_y_input,
            width_input,
            height_input,
            theta_input,
            intensity_input,
        );

        matches!(
            kind,
            ShapeKind::Rectangle(kind)
            if kind.center_x.to_ne_bytes() == center_x_input.to_ne_bytes()
                && kind.center_y.to_ne_bytes() == center_y_input.to_ne_bytes()
                && kind.width.to_ne_bytes() == width_input.to_ne_bytes()
                && kind.height.to_ne_bytes() == height_input.to_ne_bytes()
                && kind.theta.to_ne_bytes() == theta_input.to_ne_bytes()
                && intensity.to_ne_bytes() == intensity_input.to_ne_bytes()
        )
    }

    #[quickcheck]
    // Add a reason why this lint is allowed once the feature `lint_reasons` is stabilized.
    #[allow(clippy::too_many_arguments)]
    fn test_shape_rectangle_on_canvas(
        center_x_input: FloatNotNan,
        center_y_input: FloatNotNan,
        width_input: FloatNotNan,
        height_input: FloatNotNan,
        theta_input: FloatNotNan,
        intensity_input: FloatNotNan,
        nx: u32,
        ny: u32,
    ) -> bool {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let width_input = width_input.0;
        let height_input = height_input.0;
        let theta_input = theta_input.0;
        let intensity_input = intensity_input.0;
        let rectangle_on_canvas = Shape::rectangle(
            center_x_input,
            center_y_input,
            width_input,
            height_input,
            theta_input,
            intensity_input,
        )
        .on_canvas(nx, ny);

        matches!(
            rectangle_on_canvas.kind,
            ShapeKindOnCanvas::Rectangle(_kind)
            if rectangle_on_canvas.intensity().to_ne_bytes() == intensity_input.to_ne_bytes()
        )
    }

    #[quickcheck]
    // Add a reason why this lint is allowed once the feature `lint_reasons` is stabilized.
    #[allow(clippy::too_many_arguments)]
    fn test_shape_rectangle_on_canvas_inside_and_bbox(
        center_x_input: FloatNotNan,
        center_y_input: FloatNotNan,
        width_input: FloatNotNan,
        height_input: FloatNotNan,
        theta_input: FloatNotNan,
        intensity_input: FloatNotNan,
        x: FloatNotNan,
        y: FloatNotNan,
    ) -> bool {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let width_input = width_input.0;
        let height_input = height_input.0;
        let theta_input = theta_input.0;
        let intensity_input = intensity_input.0;
        let x = x.0;
        let y = y.0;
        let rectangle_on_canvas = Shape::rectangle(
            center_x_input,
            center_y_input,
            width_input,
            height_input,
            theta_input,
            intensity_input,
        )
        .on_canvas(128, 128);

        matches!(
            rectangle_on_canvas.kind,
            ShapeKindOnCanvas::Rectangle(kind)
            if rectangle_on_canvas.bounding_box() == kind.bounding_box()
                && rectangle_on_canvas.inside(x, y) == kind.inside(x, y)
        )
    }
}
