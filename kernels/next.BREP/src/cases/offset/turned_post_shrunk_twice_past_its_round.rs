//! A turned post with its shoulder rounded, shrunk by 2, past the round: the cylinder of radius 5 from z = 0 to 6 topped by the frustum from radius 5 to 2 over z = 6 to 10,
//! the edge between them rounded to radius 1, shrunk by 2.
//!
//! The round is the kernel's fillet; the solid is shrunk by its rounded offset, which drops a round turned inside out.
use crate::solids::boolean::union;
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cone_brep, make_cylinder_brep, BrepSolid};

const DISTANCE: f64 = 2.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let post = union(&make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 6.0)?, &make_cone_brep(p(0.0, 0.0, 6.0), z, 5.0, 2.0, 4.0)?)?;
    let rounded = fillet_edges(&post, &[p(0.0, -5.0, 6.0)], None, 1.0, false, None)?;
    Ok(parallel(&rounded, -DISTANCE)?)
}
