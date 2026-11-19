//! A creased sheet, the floor from (0, 0, 0) to (10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10),
//! thickened by 1 to the outside of its crease, its corner rounded by the quarter of the cylinder of
//! radius 1 about the crease.
//!
//! springrep's thicken extends the moved faces until they meet, leaving the corner sharp, at
//! x = -1, z = -1. Rounding that edge to radius 1, the thickness, puts the round about the crease.
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;
use springrep::blend::fillet_edges;
use crate::shared::thicken::{edges_where, thickened};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the floor, z = 0, and the wall, x = 0, are two faces of the block from the origin to
    // (10, 8, 10), their outward normals -z and -x: the outside of the crease
    let b = block(&at([0., 0., 0.]), Vec3::new(10., 8., 10.), "block").map_err(e)?;
    let sharp = thickened(&b, &["block.start", "block.u_min"], 1.)?;
    // the angle's outer corner, the edge along y at x = -1, z = -1
    let corner = edges_where(&sharp, |p| (p.x + 1.).abs() < 1e-9 && (p.z + 1.).abs() < 1e-9);
    fillet_edges(&sharp, &corner, 1., "round").map_err(e)
}
