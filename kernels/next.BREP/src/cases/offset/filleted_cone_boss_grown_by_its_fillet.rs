//! A filleted cone boss grown by its fillet's radius: a plate (radius 8, z = −2 to 0) and a cone boss (radius 4 at z = 0 to 1 at z = 6) joined,
//! the concave root filleted to radius 1, grown by 1 with rounded edges.
//!
//! The root is filleted by the kernel's fillet and the solid grown by its offset, which drops a fillet
//! grown to or past its radius and joins the faces on either side of it.
use crate::solids::boolean::union;
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cone_brep, make_cylinder_brep, BrepSolid};

const DISTANCE: f64 = 1.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let plate = make_cylinder_brep(p(0.0, 0.0, -2.0), z, 8.0, 2.0)?;
    let boss = make_cone_brep(p(0.0, 0.0, 0.0), z, 4.0, 1.0, 6.0)?;
    let joined = union(&plate, &boss)?;
    // the root, by a point on it
    let filleted = fillet_edges(&joined, &[p(0.0, -4.0, 0.0)], None, 1.0, false, None)?;
    Ok(parallel(&filleted, DISTANCE)?)
}
