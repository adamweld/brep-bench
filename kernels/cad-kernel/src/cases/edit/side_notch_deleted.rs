//! The box from the origin to (20, 10, 10) with a half-round notch in its side y = 0, the cylinder
//! of radius 3 about the line through (10, 0) cut away top to bottom; the notch's half cylinder
//! then deleted, the side running on from x = 7 and from x = 13 until it is one rectangle again,
//! and the top and the bottom closing over the half disks the notch took.
//!
//! Refused. cad-kernel builds the notched block: the box a `prism` cut by a `prism` of a `disc`.
//! What is missing is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`,
//! `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary and runs the faces round
//! them on along their own surfaces until they close the gap, and `offset::normal_sweep` sweeps a
//! face along its normal without extending any face beside it. Joining the cylinder back in would
//! be the result drawn again, not the notch deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the notch's half \
         cylinder out and runs the side y = 0 on from x = 7 and x = 13 until it is one rectangle, \
         the top and the bottom closing over the half disks the notch took"
        .into())
}
