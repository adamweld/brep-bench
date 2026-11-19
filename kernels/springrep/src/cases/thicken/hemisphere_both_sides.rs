//! The upper half of the sphere of radius 5 about the origin, a sheet, thickened by 1 to each side:
//! the dome between radii 4 and 6.
//!
//! springrep's thicken goes to one side of a face; the two sides are thickened apart and united.
use std::f64::consts::FRAC_PI_2;
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::thicken::{arc, line, thickened, turned_whole};
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    // the sheet is the dome of a half ball, its flat face at z = 0 holding the sheet's normals
    // at the equator
    let half = turned_whole(
        vec![
            line([0., 0.], [5., 0.], "flat"),
            arc([0., 0.], 5., 0., FRAC_PI_2, "dome"),
            line([0., 5.], [0., 0.], "axis"),
        ],
        "half",
    )?;
    // out of the half ball, and into it
    let outside = thickened(&half, &["half.dome"], 1.)?;
    let inside = thickened(&half, &["half.dome"], -1.)?;
    union(&outside, &inside).map_err(e)
}
