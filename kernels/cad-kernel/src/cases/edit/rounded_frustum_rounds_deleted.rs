//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its side the cone
//! 2r + z = 12, both rims rounded to radius 3/2 by pieces of tori tangent to each end and to the
//! cone; both rounds then deleted together, the top and the base running on outward and the cone up
//! and down until they meet in the circles of radius 3 at z = 6 and radius 6 at z = 0.
//!
//! Refused. cad-kernel builds the rounded frustum: a `build::cone`, each rim rolled by
//! `fillet::roll` and its piece cut away. What is missing is the edit. cad-kernel deletes no face
//! of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a
//! boundary and runs the faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Joining back the rings the rounds took would be the result drawn again, not the rounds deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes both rims' tori \
         out and runs the top and the base on outward and the cone up and down until they meet in \
         the circles of radius 3 at z = 6 and radius 6 at z = 0"
        .into())
}
