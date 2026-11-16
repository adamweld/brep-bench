//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, its top rim rounded to radius 2 by a
//! quarter of the torus whose tube has radius 2 about the circle of radius 4 at z = 8; the round
//! then deleted, the top running on from radius 4 and the side from z = 8 until they meet in the
//! circle of radius 6 at z = 10.
//!
//! Refused. cad-kernel builds the rounded cylinder: a `prism` of a `disc`, its top rim rolled by
//! `fillet::roll` and the round's piece cut away. What is missing is the edit. cad-kernel deletes
//! no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out
//! of a boundary and runs the faces round them on along their own surfaces until they close the
//! gap, and `offset::normal_sweep` sweeps a face along its normal without extending any face beside
//! it. Joining back the ring the round took would be the result drawn again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the rim's torus \
         out and runs the top on outward from radius 4 and the side up from z = 8 until they meet \
         in the circle of radius 6 at z = 10"
        .into())
}
