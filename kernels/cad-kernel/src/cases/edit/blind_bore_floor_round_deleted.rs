//! The box from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 4 about the z axis sunk
//! from its top to a flat floor at z = 4, the concave edge where the floor meets the wall rounded
//! to radius 1 by a quarter of the torus whose tube has radius 1 about the circle of radius 3 at
//! z = 5; the round then deleted, the floor running on outward from radius 3 and the wall down from
//! z = 5 until they meet in the circle of radius 4 at z = 4.
//!
//! Refused. cad-kernel builds the rounded bore: the block a `prism`, the bore a `prism` of a `disc`
//! cut away, and the circle at its floor rolled by `fillet::roll`, the round's piece joined. What
//! is missing is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`,
//! `rounds`, `offset` or `sew` takes faces out of a boundary and runs the faces round them on along
//! their own surfaces until they close the gap, and `offset::normal_sweep` sweeps a face along its
//! normal without extending any face beside it. Cutting away the ring the round added would be the
//! result drawn again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the round's torus \
         out of the bore's floor and runs the floor on outward from radius 3 and the wall down \
         from z = 5 until they meet in the circle of radius 4 at z = 4"
        .into())
}
