//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges rounded to radius 2, in one
//! operation: four rounds meet at each corner, closed by the ball touching all four faces.
use springrep::blend::fillet_edges;
use springrep::pick::convex_edges;
use springrep::topo::Brep;

use crate::shared::chamfer::octahedron;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let solid = octahedron()?;
    let edges = convex_edges(&solid).map_err(e)?;
    fillet_edges(&solid, &edges, 2., "round").map_err(e)
}
