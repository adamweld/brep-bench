//! The half above z = 0 of the ring torus about the z axis whose tube has radius 2 about the circle
//! of radius 6, its flat the ring 4 ≤ r ≤ 8 at z = 0, its outer rim rounded to radius 1/2 by a
//! piece of the torus about the circle of radius 6 + √2 at z = 1/2; the round then deleted, the
//! flat running on outward and the torus down until they meet in the circle of radius 8 at z = 0.
//!
//! Refused. cad-kernel builds the rounded half torus: a `build::torus` `kept` above z = 0, its
//! outer rim rolled by `fillet::roll` and the round's piece cut away. What is missing is the edit.
//! cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or
//! `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Joining back the ring the round took would be the result drawn
//! again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the outer rim's \
         torus of tube radius 1/2 out and runs the flat on outward and the half torus down until \
         they meet in the circle of radius 8 at z = 0"
        .into())
}
