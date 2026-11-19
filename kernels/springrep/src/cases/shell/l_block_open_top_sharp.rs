//! An L-shaped plate, the polygon (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded 8
//! along +z, shelled inward to a wall 1 thick, open at its top, the moved walls at its concave edge
//! extended until they meet: the cavity is the polygon (1, 1), (19, 1), (19, 4), (4, 4), (4, 14),
//! (1, 14) from z = 1 up.
use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::shared::shell::{prism, shelled};

pub fn build() -> Result<Brep, String> {
    let plate = prism(
        &[[0., 0.], [20., 0.], [20., 5.], [5., 5.], [5., 15.], [0., 15.]],
        &["front", "right", "step", "riser", "back", "left"],
        8.,
        "plate",
    )?;
    // springrep's shell moves every face and runs the moved faces on until they meet, so the
    // concave edge's walls meet sharp, in the line x = 4, y = 4
    shelled(&plate, &["plate.end"], 1., Wall::Inward, "shell")
}
