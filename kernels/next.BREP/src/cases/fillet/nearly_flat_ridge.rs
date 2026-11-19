//! The pentagon (0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°), (0, 5) extruded along +z from z = 0
//! to z = 6: a block whose top is two faces meeting at 179° along a ridge at x = 4. The ridge,
//! selected by its middle, rounded to radius 180.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let peak = 5.0 + 4.0 * 0.5f64.to_radians().tan();
    let section = polygon(&[
        p(0.0, 0.0, 0.0), p(8.0, 0.0, 0.0), p(8.0, 5.0, 0.0), p(4.0, peak, 0.0), p(0.0, 5.0, 0.0),
    ])?;
    let block = extrude_profile_brep(&section, p(0.0, 0.0, 1.0), 6.0)?;
    Ok(fillet_edges(&block, &[p(4.0, peak, 3.0)], None, 180.0, false, None)?)
}
