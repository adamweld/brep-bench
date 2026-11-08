//! The octahedron |x| + |y| + |z| ≤ 10, all twelve of its edges rounded to radius 2 in one fillet,
//! which closes each corner where four rounds meet with a patch of the ball touching all four
//! faces there. Each edge is selected by its middle.
use crate::solids::fillet::{octahedron, octahedron_edge_middles};
use crate::Refusal;
use brep_kernel::{fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let solid = octahedron(10.0)?;
    Ok(fillet_edges(&solid, &octahedron_edge_middles(), None, 2.0, false, None)?)
}
