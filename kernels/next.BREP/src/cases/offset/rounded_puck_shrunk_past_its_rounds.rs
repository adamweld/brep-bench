//! A rounded puck shrunk past its rounds' radius. A puck: the cylinder of radius 6 on the z axis
//! from z = 0 to z = 10 with both its circular edges rounded to radius 2, each round a quarter of a
//! torus whose tube has radius 2 about the circle of radius 4 at z = 2 or z = 8. Shrink it by 3:
//! the solid of every point at least 3 from the puck's outside.
//!
//! The edges are rounded by the kernel's fillet and the solid shrunk by its offset, which drops a
//! round turned inside out (or shrunk to nothing) and joins the faces on either side of it.
use crate::solids::offset::{parallel};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, BrepSolid};

const DISTANCE: f64 = 3.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let puck = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 10.0)?;
    // both circular edges, each by a point on it
    let edges = [p(0.0, -6.0, 0.0), p(0.0, -6.0, 10.0)];
    let rounded = fillet_edges(&puck, &edges, None, 2.0, false, None)?;
    Ok(parallel(&rounded, -DISTANCE)?)
}
