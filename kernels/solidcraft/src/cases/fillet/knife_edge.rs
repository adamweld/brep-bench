use crate::solids::*;

/// The wedge from z = 0 to 6 on the triangle (0, 0), (10, 0), (10, 10 tan 5°), its knife edge of
/// 5° along the z axis rounded with a radius of 0.1, picked at its middle.
pub fn build() -> R<Body> {
    let section = polygon(&[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0 * 5f64.to_radians().tan())]);
    let wedge = extrude(&Plane::XY, section, 0.0, 6.0)?;
    solidcraft_kernel::fillet(&wedge, &[v3(0.0, 0.0, 3.0)], 0.1)
}
