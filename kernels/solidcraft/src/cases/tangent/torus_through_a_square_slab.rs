use crate::solids::*;

pub fn build() -> R<Body> {
    // the torus turned 45° about z, so its seam does not pass through a point of contact
    let torus = solidcraft_kernel::transform(
        &solidcraft_kernel::torus(Vec3::ZERO, 10.0, 3.0)?,
        Vec3::ZERO,
        Vec3::ZERO,
        Vec3::Z,
        std::f64::consts::FRAC_PI_4,
    )?;
    union(&block((-13.0, -13.0, -1.0), (13.0, 13.0, 1.0))?, &torus)
}
