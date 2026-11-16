//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its top rim rounded
//! to radius 1 by a piece of the torus whose tube has radius 1 about the circle of radius
//! (7 − √5)/2 at z = 5; the round then deleted, the top running on outward and the cone up until
//! they meet in the circle of radius 3 at z = 6.
//!
//! Refused. cad-kernel builds the rounded frustum: a `build::cone`, its top rim rolled by
//! `fillet::roll` and the round's piece cut away. What is missing is the edit. cad-kernel deletes
//! no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out
//! of a boundary and runs the faces round them on along their own surfaces until they close the
//! gap, and `offset::normal_sweep` sweeps a face along its normal without extending any face beside
//! it. Joining back the ring the round took would be the result drawn again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the rim's torus \
         out and runs the top on outward and the cone up until they meet in the circle of radius \
         3 at z = 6"
        .into())
}
