//! An L-shaped plate, the polygon (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded 8
//! along +z, shelled inward to a wall 1 thick, open at its top: the wall is every point within 1 of
//! its faces other than the top, so at the concave edge x = 5, y = 5 its inside is the quarter of
//! the cylinder of radius 1 about that edge.
//!
//! springrep's shell runs the moved walls on until they meet, which leaves the inside corner sharp,
//! at x = 4, y = 4. Rounding that edge of the wall to radius 1, the wall's thickness, puts the round
//! about the concave edge itself, tangent to x = 4 and y = 4: the points within 1 of the edge.
use springrep::blend::fillet_edges;
use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::shell::{edges_where, prism, shelled};

pub fn build() -> Result<Brep, String> {
    let plate = prism(
        &[[0., 0.], [20., 0.], [20., 5.], [5., 5.], [5., 15.], [0., 15.]],
        &["front", "right", "step", "riser", "back", "left"],
        8.,
        "plate",
    )?;
    let sharp = shelled(&plate, &["plate.end"], 1., Wall::Inward, "shell")?;
    // the inside corner the moved walls met in, the vertical edge at x = 4, y = 4
    let corner = edges_where(&sharp, |p| (p.x - 4.).abs() < 1e-9 && (p.y - 4.).abs() < 1e-9);
    fillet_edges(&sharp, &corner, 1., "round").map_err(e)
}
