//! The half above z = 0 of the ring torus about the z axis whose tube has radius 2 about the circle
//! of radius 6, its flat the ring 4 ≤ r ≤ 8 at z = 0, both rims rounded to radius 1/2 by pieces of
//! the tori about the circles of radius 6 + √2 and 6 − √2 at z = 1/2; both rounds then deleted
//! together, the flat running on outward and inward and the torus down until they meet in the
//! circles of radius 8 and 4 at z = 0.
//!
//! Refused. cad-kernel builds the rounded half torus: a `build::torus` `kept` above z = 0, each rim
//! rolled by `fillet::roll` and its piece cut away. What is missing is the edit. cad-kernel deletes
//! no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out
//! of a boundary and runs the faces round them on along their own surfaces until they close the
//! gap, and `offset::normal_sweep` sweeps a face along its normal without extending any face beside
//! it. Joining back the rings the rounds took would be the result drawn again, not the rounds
//! deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes both rims' tori \
         out and runs the flat on outward and inward and the half torus down until they meet in \
         the circles of radius 8 and 4 at z = 0"
        .into())
}
