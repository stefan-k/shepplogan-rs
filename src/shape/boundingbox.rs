// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// A bounding box around a shape
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

#[cfg(test)]
mod tests {
    use super::BoundingBox;

    #[quickcheck]
    fn test_bounding_box(x_low: u32, x_high: u32, y_low: u32, y_high: u32) -> bool {
        let bbox: BoundingBox = (x_low, x_high, y_low, y_high).into();
        bbox.x_low == x_low
            && bbox.x_high == x_high
            && bbox.y_low == y_low
            && bbox.y_high == y_high
            && BoundingBox {
                x_low,
                x_high,
                y_low,
                y_high,
            } == bbox
    }
}
