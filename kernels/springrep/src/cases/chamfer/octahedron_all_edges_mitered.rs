//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges chamfered √6 on both faces, one after
//! another, each chamfer ending on the faces at its ends, so the four at each corner meet in a
//! point.
use springrep::blend::chamfer;
use springrep::topo::Brep;

use crate::shared::chamfer::{octahedron, octahedron_edges};
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let mut solid = octahedron()?;
    for (i, (a, b)) in octahedron_edges().iter().enumerate() {
        solid = chamfer(&solid, &[(a, b)], 6f64.sqrt(), &format!("chamfer{i}")).map_err(e)?;
    }
    Ok(solid)
}
