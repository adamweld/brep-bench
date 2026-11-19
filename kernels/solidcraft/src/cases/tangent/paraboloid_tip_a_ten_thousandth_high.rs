use super::paraboloid_tip_on_face::paraboloid;
use crate::solids::*;

pub fn build() -> R<Body> {
    // the block below the plane z = 10⁻⁴, reaching past the paraboloid on every side but its top
    common(&paraboloid(8.0, 8.0)?, &block((-9.0, -9.0, -1.0), (9.0, 9.0, 1e-4))?)
}
