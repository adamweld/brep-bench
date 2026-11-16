//! A block with a blind bore, filleted at the bore's floor, grown past the fillet's radius. A
//! block, the cylinder of radius 10 on the z axis from z = 0 to z = 8, with a blind bore in its
//! top: the cylinder of radius 5 from z = 2 up through the top is cut away, leaving a flat floor at
//! z = 2. The concave edge where the bore's side meets its floor, the circle of radius 5 at z = 2,
//! is filleted to radius 2: a quarter of a torus whose tube has radius 2 about the circle of radius
//! 3 at z = 4, concave, tangent to the floor at radius 3 and to the bore's side at z = 4. Grow the
//! solid by 3: every point within 3 of it.
//!
//! The edges are rounded by the kernel's fillet and the solid grown by its offset, which drops a
//! round turned inside out (or shrunk to nothing) and joins the faces on either side of it.
use crate::solids::offset::{parallel, subtract};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, BrepSolid};

const DISTANCE: f64 = 3.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let block = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 10.0, 8.0)?;
    let bore = make_cylinder_brep(p(0.0, 0.0, 2.0), z, 5.0, 10.0)?;
    // the bore's floor edge, by a point on it
    let edge = p(0.0, -5.0, 2.0);
    let rounded = fillet_edges(&subtract(&block, &bore)?, &[edge], None, 2.0, false, None)?;
    Ok(parallel(&rounded, DISTANCE)?)
}
