//! The box from the origin to (20, 10, 10) with a slot cut into its side y = 0 top to bottom, its
//! flats x = 8 and x = 12 running in to y = 5 and closed there by the half cylinder of radius 2
//! about the line through (10, 5); the slot's three faces then deleted, the side y = 0 running on
//! across the slot's mouth and the top and the bottom closing over its outline.
//!
//! Refused. cad-kernel builds the slotted block: the box a `prism` cut by the slot, its outline of
//! lines and an arc swept up by `prism`. What is missing is the edit. cad-kernel deletes no face of
//! a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a
//! boundary and runs the faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Joining the slot's prism back in would be the result drawn again, not the slot deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the slot's flats \
         x = 8 and x = 12 and its half cylinder out and runs the side y = 0 on across the slot's \
         mouth, the top and the bottom closing over the slot's outline"
        .into())
}
