//! The half of the ball of radius 5 about the origin above z = 0, its rim rounded to radius 1 by a
//! piece of the torus whose tube has radius 1 about the circle of radius √15 at z = 1; the round
//! then deleted, the flat running on outward and the ball down until they meet in the circle of
//! radius 5 at z = 0.
//!
//! Refused. cad-kernel builds the rounded hemisphere: a `sphere` `kept` above z = 0, its rim rolled
//! by `fillet::roll` and the round's piece cut away. What is missing is the edit. cad-kernel
//! deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes
//! faces out of a boundary and runs the faces round them on along their own surfaces until they
//! close the gap, and `offset::normal_sweep` sweeps a face along its normal without extending any
//! face beside it. Joining back the ring the round took would be the result drawn again, not the
//! round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the rim's torus \
         out and runs the flat on outward and the ball down until they meet in the circle of \
         radius 5 at z = 0"
        .into())
}
