//! A post on a plate, filleted at its foot, grown past the fillet's radius. A plate, the cylinder
//! of radius 8 on the z axis from z = −4 to z = 0, with a post standing on it, the cylinder of
//! radius 3 from z = 0 to z = 8. The concave edge where the post meets the plate, the circle of
//! radius 3 at z = 0, is filleted to radius 2: a quarter of a torus whose tube has radius 2 about
//! the circle of radius 5 at z = 2, concave, tangent to the post's side at z = 2 and to the plate's
//! top at radius 5. Grow the solid by 3: every point within 3 of it.
//!
//! The edges are rounded by the kernel's fillet and the solid grown by its offset, which drops a
//! round turned inside out (or shrunk to nothing) and joins the faces on either side of it.
use crate::solids::offset::{parallel, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, BrepSolid};

const DISTANCE: f64 = 3.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let plate = make_cylinder_brep(p(0.0, 0.0, -4.0), z, 8.0, 4.0)?;
    let post = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 3.0, 8.0)?;
    // the post's foot, by a point on it
    let foot = p(0.0, -3.0, 0.0);
    let rounded = fillet_edges(&union(&plate, &post)?, &[foot], None, 2.0, false, None)?;
    Ok(parallel(&rounded, DISTANCE)?)
}
