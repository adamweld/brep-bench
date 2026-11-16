//! The box from the origin to (20, 10, 10), the edge where its top, z = 10, meets its side x = 20
//! beveled 3 back on each face, the bevel the plane through the lines x = 17, z = 10 and x = 20,
//! z = 7; the bevel then deleted, the top running on from x = 17 and the side from z = 7 until they
//! meet in the edge x = 20, z = 10.
//!
//! Refused. cad-kernel builds the beveled block: the pentagon in y = 0 swept 10 along y by `prism`,
//! or the box's edge taken off by `fillet::flat_along`. What is missing is the edit. cad-kernel
//! deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes
//! faces out of a boundary and runs the faces round them on along their own surfaces until they
//! close the gap, and `offset::normal_sweep` sweeps a face along its normal without extending any
//! face beside it. Joining back the prism the bevel took off would be the result drawn again, not
//! the bevel deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the bevel's plane \
         out and runs the top on from x = 17 and the side x = 20 up from z = 7 until they meet in \
         the edge x = 20, z = 10"
        .into())
}
