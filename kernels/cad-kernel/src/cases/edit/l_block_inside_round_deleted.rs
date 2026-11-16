//! The L-shaped block through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), 8 high, its
//! inside corner along x = 5, y = 5 rounded to radius 2 by the quarter cylinder about the line
//! x = 7, y = 7; the round then deleted, the face x = 5 running back from y = 7 and the face y = 5
//! from x = 7 until they meet in the edge x = 5, y = 5.
//!
//! Refused. cad-kernel builds the rounded L-block: `sketch::l_block`, its inside edge rolled by
//! `fillet::roll` and the round's piece joined into the corner. What is missing is the edit.
//! cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or
//! `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Cutting away what the round added would be the result drawn again,
//! not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the inside round's \
         quarter cylinder out and runs the face x = 5 back from y = 7 and the face y = 5 back \
         from x = 7 until they meet in the edge x = 5, y = 5"
        .into())
}
