//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its side the cone
//! 2r + z = 12, both rims rounded to radius 3/2 by pieces of tori tangent to each end and to the
//! cone; the top round alone then deleted, the top running on outward and the cone up until they
//! meet in the circle of radius 3 at z = 6, the base's round staying.
//!
//! Refused. cad-kernel builds the rounded frustum: a `build::cone`, each rim rolled by
//! `fillet::roll` and its piece cut away. What is missing is the edit. cad-kernel deletes no face
//! of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a
//! boundary and runs the faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Rounding the base's rim alone would be the result drawn again, not the top round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the top rim's \
         torus out, leaving the base's, and runs the top on outward and the cone up until they \
         meet in the circle of radius 3 at z = 6"
        .into())
}
