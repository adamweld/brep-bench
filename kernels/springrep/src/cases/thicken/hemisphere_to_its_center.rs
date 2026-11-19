//! The upper half of the sphere of radius 5 about the origin, a sheet, thickened by 5 toward the
//! center, its whole radius: the half ball.
use std::f64::consts::FRAC_PI_2;

use springrep::topo::Brep;

use crate::shared::thicken::{arc, line, thickened, turned_whole};


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
    // a negative thickness thickens into the half ball; moved 5 in, the dome closes on the center
    thickened(&half, &["half.dome"], -5.)
}
