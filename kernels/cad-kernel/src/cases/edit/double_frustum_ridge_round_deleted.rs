//! Two cone frustums on the z axis made one solid, from radius 3 at z = 0 to radius 5 at z = 4 and
//! from there to radius 2 at z = 8, their ridge, the circle of radius 5 at z = 4, rounded to radius
//! 1 by a piece of a torus tangent to both cones; the round then deleted, the lower cone running on
//! up and the upper one down until they meet again in the ridge.
//!
//! Refused. cad-kernel builds the rounded frustums: two `build::cone`s joined, the ridge rolled by
//! `fillet::roll` and the round's piece cut away. What is missing is the edit. cad-kernel deletes
//! no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out
//! of a boundary and runs the faces round them on along their own surfaces until they close the
//! gap, and `offset::normal_sweep` sweeps a face along its normal without extending any face beside
//! it. Joining back the ring the round took would be the result drawn again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the ridge's torus \
         out and runs the lower cone on up and the upper cone down until they meet again in the \
         circle of radius 5 at z = 4"
        .into())
}
