//! The frustum on the z axis from radius 6 at z = 0 to radius 3.5 at z = 5, its side the cone
//! 2r + z = 12, joined to the steeper frustum from radius 3.5 at z = 5 to radius 2 at z = 6, which
//! bevels its top rim; the bevel's cone then deleted, the top running on outward from radius 2 and
//! the side up from z = 5 until they meet in the circle of radius 3 at z = 6.
//!
//! Refused. cad-kernel builds the beveled frustum: the two frustums `build::cone`s, joined by
//! `boolean::combined`. What is missing is the edit. cad-kernel deletes no face of a solid: nothing
//! in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary and runs the
//! faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Drawing the frustum to radius 3 at z = 6 would be the result drawn again, not the bevel deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the bevel's \
         steeper cone out and runs the top on outward from radius 2 and the cone 2r + z = 12 up \
         from z = 5 until they meet in the circle of radius 3 at z = 6"
        .into())
}
