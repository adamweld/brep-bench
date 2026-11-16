//! A rounded cone frustum shrunk past its rounds' radius: the frustum from radius 6 at z = 0 to radius 3 at z = 6, both rims
//! rounded to radius 1.5, shrunk by 2: every point at least 2 from its outside.
//!
//! The rims are rounded by the kernel's fillet and the solid shrunk by its offset, which drops a
//! round turned inside out and joins the faces on either side of it.
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cone_brep, BrepSolid};

const DISTANCE: f64 = 2.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let frustum = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    // the rounded rims, each by a point on it
    let rounded = fillet_edges(&frustum, &[p(0.0, -3.0, 6.0), p(0.0, -6.0, 0.0)], None, 1.5, false, None)?;
    Ok(parallel(&rounded, -DISTANCE)?)
}
