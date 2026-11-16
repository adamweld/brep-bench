//! The box from the origin to (20, 10, 10), drilled right through along z by a hole of radius 3
//! about the line through (10, 5); the hole's wall then deleted, the top and the bottom closing
//! over the circles its two ends are.
//!
//! Refused. cad-kernel builds the drilled block: the box a `prism` cut by the drill, a `prism` of a
//! `disc`. What is missing is the edit. cad-kernel deletes no face of a solid: nothing in
//! `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary and runs the
//! faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Joining the drill back in would be the result drawn again, not the hole deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the hole's \
         cylindrical wall out and closes the top and the bottom over the circles of radius 3 its \
         two ends are"
        .into())
}
