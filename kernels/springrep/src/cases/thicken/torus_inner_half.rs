//! The inner half of the torus about the z axis whose tube has radius 2 about a middle circle of
//! radius 6 (its points no farther than 6 from the axis), a sheet, thickened by 1 toward the axis:
//! between the inner halves of the tori of tube radii 2 and 3.
use std::f64::consts::{FRAC_PI_2, PI};

use springrep::topo::Brep;

use crate::shared::thicken::{arc, line, thickened, turned_whole};

pub fn build() -> Result<Brep, String> {
    // the sheet is the inner face of the tube's half toward the axis, closed by the cylinder of
    // radius 6 through the tube's top and bottom circles, where the sheet's normal is along z
    let half_tube = turned_whole(
        vec![
            arc([6., 0.], 2., FRAC_PI_2, PI, "skin"),
            line([6., -2.], [6., 2.], "ring"),
        ],
        "tube",
    )?;
    thickened(&half_tube, &["tube.skin"], 1.)
}
