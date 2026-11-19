use crate::solids::*;

/// The prism from z = 0 to 6 on the pentagon (0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°), (0, 5):
/// a block whose top is two faces meeting at 179° along a ridge over x = 4. The ridge rounded
/// with a radius of 180, picked at its middle.
pub fn build() -> R<Body> {
    let peak = 5.0 + 4.0 * 0.5f64.to_radians().tan();
    let section = polygon(&[(0.0, 0.0), (8.0, 0.0), (8.0, 5.0), (4.0, peak), (0.0, 5.0)]);
    let prism = extrude(&Plane::XY, section, 0.0, 6.0)?;
    solidcraft_kernel::fillet(&prism, &[v3(4.0, peak, 3.0)], 180.0)
}
