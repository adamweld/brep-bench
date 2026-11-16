//! A cylinder cut by a plane at 45° and grown by a half: the cylinder of radius 3 on the z axis from z = −10, cut by the plane z = x·tan 45°
//! (the part below it kept), grown by 0.5 with rounded edges.
//!
//! The cylinder is cadrs's primitive; the plane's half-space is a long prism on its trace in the
//! Front plane, extruded both ways, and the two are intersected. Then cadrs's offset with rounded
//! edges.
use crate::bench::{common, cylinder, extrude_symmetric, p3, v3};
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let t = (45f64).to_radians().tan();
    let c = cylinder(k, p3(0.0, 0.0, -10.0), v3(0.0, 0.0, 1.0), 3.0, 30.0)?;
    // the Front plane's sketch (x, y) is the model's (x, z)
    let trace = Loop::polygon(&[[-20.0, -20.0 * t], [20.0, 20.0 * t], [20.0, -40.0], [-20.0, -40.0]]);
    let below = extrude_symmetric(k, &Profile::of(Plane::xz(0.0), vec![trace]), 40.0)?;
    let cut = common(k, c, below)?;
    grown(k, cut, 0.5)
}
