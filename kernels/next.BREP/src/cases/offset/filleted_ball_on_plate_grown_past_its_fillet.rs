//! A ball standing in a plate, its root filleted, grown past the fillet's radius: a plate (radius 8, z = −2 to 0) and the ball of radius 3 about (0, 0, 1) joined,
//! the concave root filleted to radius 1, grown by 2 with rounded edges.
//!
//! The root is filleted by the kernel's fillet and the solid grown by its offset, which drops a fillet
//! grown to or past its radius and joins the faces on either side of it.
use crate::solids::boolean::union;
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, make_sphere_brep, BrepSolid};

const DISTANCE: f64 = 2.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let plate = make_cylinder_brep(p(0.0, 0.0, -2.0), z, 8.0, 2.0)?;
    let boss = make_sphere_brep(p(0.0, 0.0, 1.0), 3.0, z)?;
    let joined = union(&plate, &boss)?;
    // the root, by a point on it
    let filleted = fillet_edges(&joined, &[p(0.0, -(8f64).sqrt(), 0.0)], None, 1.0, false, None)?;
    Ok(parallel(&filleted, DISTANCE)?)
}
