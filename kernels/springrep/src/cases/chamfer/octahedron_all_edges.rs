//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges chamfered √6 on both faces, in one
//! operation: four chamfers meet at each corner, which is cut flat by a square.
use springrep::blend::chamfer;
use springrep::topo::Brep;

use crate::shared::chamfer::{octahedron, octahedron_edges};
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let solid = octahedron()?;
    let edges = octahedron_edges();
    let pairs: Vec<(&str, &str)> = edges.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
    chamfer(&solid, &pairs, 6f64.sqrt(), "chamfer").map_err(e)
}
