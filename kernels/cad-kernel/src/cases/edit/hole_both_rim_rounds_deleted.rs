//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 2 about the
//! line x = 10, y = 5, both of the hole's rims rounded to radius 1 by quarters of the tori whose
//! tubes have radius 1 about the circles of radius 3 at z = 9 and at z = 1; both rounds then
//! deleted together, the top and the bottom running on inward from radius 3 and the wall up from
//! z = 9 and down from z = 1 until they meet in the circles of radius 2 at z = 10 and at z = 0.
//!
//! Refused. cad-kernel builds the rounded hole: the box a `prism` drilled by a `prism` of a `disc`,
//! each rim rolled by `fillet::roll` and its piece cut away. What is missing is the edit.
//! cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or
//! `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Joining back the rings the rounds took would be the result drawn
//! again, not the rounds deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes both rims' tori \
         out of the hole and runs the top and the bottom on inward from radius 3 and the wall up \
         from z = 9 and down from z = 1 until they meet in the circles of radius 2 at z = 10 and \
         z = 0"
        .into())
}
