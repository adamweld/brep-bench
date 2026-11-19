use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (0.001, 0.001, 0.001))?;
    // the copy turned 1.2° about the z axis, the line of the cube's edge through the origin
    let turned =
        solidcraft_kernel::transform(&cube, Vec3::ZERO, Vec3::ZERO, Vec3::Z, 1.2f64.to_radians())?;
    common(&cube, &turned)
}
