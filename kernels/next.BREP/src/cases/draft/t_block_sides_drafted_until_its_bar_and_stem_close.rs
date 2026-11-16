//! A T-block with its sides drafted until they close over its bar and its stem: the outline through
//! (0, 0), (20, 0), (20, 4), (13, 4), (13, 16), (7, 16), (7, 4), (0, 4) extruded from z = 0 to z =
//! 8, every side drafted 45°, pull +z, about the neutral plane z = 0.
//!
//! The kernel's drafted extrusion carried past the taper that closes the section
//! (`extrude_profile_brep_draft_roof`): the outline, extruded 8 with its sides leaning in 45°.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep_draft_roof, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let outline = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 4.0, 0.0), p(13.0, 4.0, 0.0),
        p(13.0, 16.0, 0.0), p(7.0, 16.0, 0.0), p(7.0, 4.0, 0.0), p(0.0, 4.0, 0.0),
    ])?;
    let up = p(0.0, 0.0, 1.0);
    Ok(extrude_profile_brep_draft_roof(&outline, up, 8.0, 45.0f64.to_radians())?)
}
