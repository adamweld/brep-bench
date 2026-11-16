//! The box from the origin to (20, 10, 10), the two long edges of its top rounded to radius 2 by
//! the quarter cylinders about the lines y = 2, z = 8 and y = 8, z = 8; both rounds then deleted
//! together, the top running on from y = 2 and from y = 8 and the sides y = 0 and y = 10 up from
//! z = 8 until they meet in the edges at z = 10, and the ends x = 0 and x = 20 getting their
//! corners back.
//!
//! Refused. cad-kernel builds the rounded block: the box a `prism`, each edge rolled by
//! `fillet::roll` and its piece cut away. What is missing is the edit. cad-kernel deletes no face
//! of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a
//! boundary and runs the faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Joining back the pieces the rounds took would be the result drawn again, not the rounds deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the two quarter \
         cylinders out and runs the top on outward from y = 2 and y = 8 and the sides y = 0 and \
         y = 10 up from z = 8 until they meet in the edges y = 0, z = 10 and y = 10, z = 10"
        .into())
}
