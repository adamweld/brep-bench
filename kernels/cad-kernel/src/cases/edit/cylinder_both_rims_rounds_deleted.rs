//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, both rims rounded to radius 2 by
//! quarters of the tori whose tubes have radius 2 about the circles of radius 4 at z = 8 and at
//! z = 2; both rounds then deleted together, the top and the bottom running on outward from radius
//! 4 and the side up from z = 8 and down from z = 2 until they meet in the circles of radius 6 at
//! z = 10 and at z = 0.
//!
//! Refused. cad-kernel builds the rounded cylinder: a `prism` of a `disc`, each rim rolled by
//! `fillet::roll` and its piece cut away. What is missing is the edit. cad-kernel deletes no face
//! of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a
//! boundary and runs the faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Joining back the rings the rounds took would be the result drawn again, not the rounds deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes both rims' tori \
         out and runs the top and the bottom on outward from radius 4 and the side up from z = 8 \
         and down from z = 2 until they meet in the circles of radius 6 at z = 10 and z = 0"
        .into())
}
