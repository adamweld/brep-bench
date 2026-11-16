//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 2 about the
//! line x = 10, y = 5, the edge where the hole's wall meets the top rounded to radius 1 by a
//! quarter of the torus whose tube has radius 1 about the circle of radius 3 at z = 9; the round
//! then deleted, the top running on inward from radius 3 and the wall up from z = 9 until they meet
//! in the circle of radius 2 at z = 10.
//!
//! Refused. cad-kernel builds the rounded hole: the box a `prism` drilled by a `prism` of a `disc`,
//! the top rim rolled by `fillet::roll` and the round's piece cut away. What is missing is the
//! edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset`
//! or `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Joining back the ring the round took would be the result drawn
//! again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the top rim's \
         torus out and runs the top on inward from radius 3 and the hole's wall up from z = 9 \
         until they meet in the circle of radius 2 at z = 10"
        .into())
}
