//! An L-shaped block, the polygon (0, 0), (20, 0), (20, 20), (15, 20), (15, 5), (0, 5) extruded 20
//! along +z, shelled outward to a wall 1 thick, open at the face y = 5 of its short leg, its edges kept
//! sharp. The wall of the face x = 15 stands on the opening's edge and keeps its foot.
use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::shared::shell::{prism, shelled};

pub fn build() -> Result<Brep, String> {
    let block = prism(
        &[[0., 0.], [20., 0.], [20., 20.], [15., 20.], [15., 5.], [0., 5.]],
        &["front", "right", "back", "riser", "step", "left"],
        20.,
        "l",
    )?;
    shelled(&block, &["l.step"], 1., Wall::Outward, "shell")
}
