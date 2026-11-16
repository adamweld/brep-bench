//! The box from the origin to (20, 10, 10), the edge where its top, z = 10, meets its side x = 20
//! rounded to radius 2 by the quarter cylinder about the line x = 18, z = 8; the round then
//! deleted, the top running on from x = 18 and the side from z = 8 until they meet in the edge
//! x = 20, z = 10, and the ends y = 0 and y = 10 getting their corners back.
//!
//! Refused. cad-kernel builds the rounded block: the box a `prism`, its edge rolled by
//! `fillet::roll` and the round's piece cut away. What is missing is the edit. cad-kernel deletes
//! no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out
//! of a boundary and runs the faces round them on along their own surfaces until they close the
//! gap, and `offset::normal_sweep` sweeps a face along its normal without extending any face beside
//! it. Joining back the piece the round took would be the result drawn again, not the round
//! deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the round's \
         quarter cylinder out and runs the top on from x = 18 and the side x = 20 up from z = 8 \
         until they meet in the edge x = 20, z = 10, the ends y = 0 and y = 10 closing their \
         corners"
        .into())
}
