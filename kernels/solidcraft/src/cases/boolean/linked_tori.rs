use crate::solids::*;

pub fn build() -> R<Body> {
    let first = solidcraft_kernel::torus(Vec3::ZERO, 3.0, 0.5)?;
    // the kernel's tori are about z: the second turned a quarter turn about the line through its
    // centre along x, its middle circle into the plane y = 0
    let centre = v3(3.0, 0.0, 0.0);
    let second = solidcraft_kernel::transform(
        &solidcraft_kernel::torus(centre, 3.0, 0.5)?,
        Vec3::ZERO,
        centre,
        Vec3::X,
        std::f64::consts::FRAC_PI_2,
    )?;
    union(&first, &second)
}
