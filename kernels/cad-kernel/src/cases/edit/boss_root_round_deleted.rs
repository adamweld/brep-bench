//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss on it, the
//! cylinder of radius 3 from z = 0 to z = 8, the concave edge at the boss's foot rounded to radius
//! 1 by a quarter of the torus whose tube has radius 1 about the circle of radius 4 at z = 1; the
//! round then deleted, the plate's top running in from radius 4 and the boss's side down from z = 1
//! until they meet in the circle of radius 3 at z = 0.
//!
//! Refused. cad-kernel builds the rounded boss: the cylinders `prism`s of `disc`s joined, the
//! circle at the boss's foot rolled by `fillet::roll` and the round's piece joined. What is missing
//! is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`,
//! `offset` or `sew` takes faces out of a boundary and runs the faces round them on along their own
//! surfaces until they close the gap, and `offset::normal_sweep` sweeps a face along its normal
//! without extending any face beside it. Cutting away the ring the round added would be the result
//! drawn again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the round's torus \
         out and runs the plate's top in from radius 4 and the boss's side down from z = 1 until \
         they meet in the circle of radius 3 at z = 0"
        .into())
}
