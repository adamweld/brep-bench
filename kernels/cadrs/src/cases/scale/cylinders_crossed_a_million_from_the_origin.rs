//! Two cylinders of radius 3 and length 20 crossing square a million from the origin, both
//! centered on c = (600000, −800000, 0), one along (2, 3, 6)/7 and one along (3, −6, 2)/7, each
//! from c − 10u to c + 10u: the solid common to both. Each is a circle on the plane square to its
//! axis through c − 10u, extruded 20.
use crate::bench::{common, cylinder, p3, P3, V3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

/// The cylinder of radius 3 along `dir` from `c` − 10u to `c` + 10u.
fn crossing(k: &mut DefaultKernel, c: P3, dir: V3) -> Result<BodyId, Refusal> {
    let u = dir.normalize();
    cylinder(k, c - 10.0 * u, u, 3.0, 20.0)
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let c = p3(600000.0, -800000.0, 0.0);
    let a = crossing(k, c, V3::new(2.0, 3.0, 6.0))?;
    let b = crossing(k, c, V3::new(3.0, -6.0, 2.0))?;
    common(k, a, b)
}
