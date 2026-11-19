//! Two cylinders of radius 0.003 and length 0.02 crossing at right angles at the origin, one
//! along the x axis from x = −0.01 to 0.01 and one along the y axis from y = −0.01 to 0.01: the
//! solid common to both. Each is a circle extruded.
use crate::bench::{common, cylinder, p3, V3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let a = cylinder(k, p3(-0.01, 0.0, 0.0), V3::x(), 0.003, 0.02)?;
    let b = cylinder(k, p3(0.0, -0.01, 0.0), V3::y(), 0.003, 0.02)?;
    common(k, a, b)
}
