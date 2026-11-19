//! A plate, the box from (−15, −15, 0) to (15, 15, 4), with a hole punched through it by an
//! ellipsoid, semi-axes 3, 2 and 1.5 along x, y and z about (−2, −1.5, −4), slid without turning
//! along (4, 3, 12).
//!
//! solidcraft-kernel has no sweep of a solid and no ellipsoid, so the sweep is built by elementary
//! geometry. The ellipsoid is the ball of radius 1 stretched by S = diag(3, 2, 1.5) and moved to
//! its centre, and a linear map carries a slide onto a slide: the ellipsoid slid along (4, 3, 12)
//! is the ball slid along S⁻¹(4, 3, 12) = (4/3, 1.5, 8), stretched. The ball's slide is the
//! cylinder of radius 1 about its centre's path with the ball at each end. The cylinder's ends,
//! and the balls, stretched, lie within the ellipsoid where it starts and where it stops, wholly
//! below z = −2.5 and above z = 6.5, clear of the plate; so the stretched cylinder, an oblique
//! elliptic one, is what the sweep takes from the plate. The kernel's `transform_matrix` scales.
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-15.0, -15.0, 0.0), (15.0, 15.0, 4.0))?;
    let path = v3(4.0 / 3.0, 1.5, 8.0);
    let tube = solidcraft_kernel::cylinder(Vec3::ZERO, path, 1.0, path.len())?;
    // column by column: x, y and z stretched, then moved to the ellipsoid's centre
    let stretch = [
        [3.0, 0.0, 0.0, 0.0],
        [0.0, 2.0, 0.0, 0.0],
        [0.0, 0.0, 1.5, 0.0],
        [-2.0, -1.5, -4.0, 1.0],
    ];
    cut(&plate, &solidcraft_kernel::transform_matrix(&tube, stretch)?)
}
