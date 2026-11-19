//! A creased sheet, the floor from (0, 0, 0) to (10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10),
//! thickened by 1 to the inside of its crease: the moved faces cross and are trimmed where they meet.
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::thicken::thickened;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the floor, z = 0, and the wall, x = 0, are two faces of the block from the origin to
    // (10, 8, 10), their outward normals -z and -x: the outside of the crease
    let b = block(&at([0., 0., 0.]), Vec3::new(10., 8., 10.), "block").map_err(e)?;
    // a negative thickness thickens into the block, inside the crease
    thickened(&b, &["block.start", "block.u_min"], -1.)
}
