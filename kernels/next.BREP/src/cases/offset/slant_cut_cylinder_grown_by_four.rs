//! A cylinder cut by a plane at 45° and grown by 4: the cylinder of radius 3 on the z axis from z = −10, cut by the plane z = x·tan 45°
//! (the part below it kept), grown by 4.0 with rounded edges.
//!
//! The cylinder is the kernel's; the plane's half-space is a long prism on its trace in the plane
//! y = −10, extruded 20 along +y, and the two are intersected. Then the kernel's rounded offset.
use crate::solids::boolean::common;
use crate::solids::offset::parallel;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let t = (45f64).to_radians().tan();
    let cylinder = make_cylinder_brep(p(0.0, 0.0, -10.0), p(0.0, 0.0, 1.0), 3.0, 30.0)?;
    let trace = polygon(&[
        p(-20.0, -10.0, -20.0 * t),
        p(20.0, -10.0, 20.0 * t),
        p(20.0, -10.0, -40.0),
        p(-20.0, -10.0, -40.0),
    ])?;
    let below = extrude_profile_brep(&trace, p(0.0, 1.0, 0.0), 20.0)?;
    Ok(parallel(&common(&cylinder, &below)?, 4.0)?)
}
