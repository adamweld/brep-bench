//! The triangle (0, 0), (10, 0), (10, 10 tan 5°) extruded along +z from z = 0 to z = 6: a wedge
//! whose edge along the z axis is a knife edge of 5°. That edge, selected by its middle, rounded to
//! radius 0.1.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let section = polygon(&[
        p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 10.0 * 5f64.to_radians().tan(), 0.0),
    ])?;
    let wedge = extrude_profile_brep(&section, p(0.0, 0.0, 1.0), 6.0)?;
    Ok(fillet_edges(&wedge, &[p(0.0, 0.0, 3.0)], None, 0.1, false, None)?)
}
