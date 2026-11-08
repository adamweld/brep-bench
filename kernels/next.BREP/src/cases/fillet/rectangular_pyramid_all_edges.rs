//! The pyramid on the rectangle from (−10, −6, 0) to (10, 6, 0) with its apex at (0, 0, 12), all
//! eight of its edges rounded to radius 3/2 in one fillet. Each edge is selected by its middle.
//!
//! next.BREP's pyramid stands on a regular polygon, so the pyramid on the rectangle is its pyramid
//! on the square from (−10, −10) to (10, 10), apex 12 high, stretched by 0.6 along y: an affine
//! map, which the kernel applies exactly to every pole.
use crate::solids::geometry::stretch;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_pyramid_brep, transform_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let square = make_pyramid_brep(p(-10.0, -10.0, 0.0), 20.0, 12.0, 4)?;
    let pyramid = transform_brep(&square, stretch(1.0, 0.6, 1.0)?, false)?;
    let mut middles = vec![p(0.0, -6.0, 0.0), p(10.0, 0.0, 0.0), p(0.0, 6.0, 0.0),
        p(-10.0, 0.0, 0.0)];
    for (x, y) in [(5.0, 3.0), (-5.0, 3.0), (-5.0, -3.0), (5.0, -3.0)] {
        middles.push(p(x, y, 6.0));
    }
    Ok(fillet_edges(&pyramid, &middles, None, 1.5, false, None)?)
}
