//! An L-block with its sides drafted until they close over its arms: the outline through (0, 0),
//! (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded from z = 0 to z = 8, every side drafted 30°,
//! pull +z, about the neutral plane z = 0.
//!
//! The kernel's drafted extrusion carried past the taper that closes the section
//! (`extrude_profile_brep_draft_roof`): the outline, extruded 8 with its sides leaning in 30°.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep_draft_roof, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let outline = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 5.0, 0.0), p(5.0, 5.0, 0.0),
        p(5.0, 15.0, 0.0), p(0.0, 15.0, 0.0),
    ])?;
    let up = p(0.0, 0.0, 1.0);
    Ok(extrude_profile_brep_draft_roof(&outline, up, 8.0, 30.0f64.to_radians())?)
}
