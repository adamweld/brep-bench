//! The side of the cone frustum about the z axis whose radius runs from 6 at z = 0 to 3 at z = 4, a
//! sheet, thickened by 1 along its normal away from the axis: a band whose rims sweep cones.
use springrep::topo::Brep;

use crate::shared::thicken::{line, thickened, turned_whole};

pub fn build() -> Result<Brep, String> {
    // The sheet is the side of a solid of revolution whose faces at its rims run along its normal
    // there, (4, 3)/5 in (distance from the axis, z): the cones from each rim circle in along that
    // normal to the axis, which the line from (6, 0) meets at z = -4.5 and the line from (3, 4)
    // at z = 1.75.
    let solid = turned_whole(
        vec![
            line([0., -4.5], [6., 0.], "below"),
            line([6., 0.], [3., 4.], "side"),
            line([3., 4.], [0., 1.75], "above"),
            line([0., 1.75], [0., -4.5], "axis"),
        ],
        "frustum",
    )?;
    thickened(&solid, &["frustum.side"], 1.)
}
