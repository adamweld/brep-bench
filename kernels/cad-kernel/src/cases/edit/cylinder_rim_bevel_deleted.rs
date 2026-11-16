//! The cylinder of radius 6 on the z axis from z = 0 to z = 8 with the frustum from radius 6 at
//! z = 8 to radius 4 at z = 10 on it, a cylinder to z = 10 whose top rim is beveled 2 back on each
//! face; the bevel, the frustum's side, then deleted, the top running on from radius 4 and the side
//! from z = 8 until they meet in the circle of radius 6 at z = 10.
//!
//! Refused. cad-kernel builds the beveled cylinder: a `prism` of a `disc` joined to the frustum
//! `build::cone`, or the rim taken off by `fillet::flat_along`. What is missing is the edit.
//! cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or
//! `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Joining back the ring the bevel took would be the result drawn
//! again, not the bevel deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the bevel's cone \
         out and runs the top on outward from radius 4 and the side up from z = 8 until they meet \
         in the circle of radius 6 at z = 10"
        .into())
}
