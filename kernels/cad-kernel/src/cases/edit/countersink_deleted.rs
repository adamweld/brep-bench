//! The box from the origin to (20, 10, 10), drilled right through along z by a hole of radius 2
//! about the line through (10, 5) and countersunk at the top by the cone from radius 2 at z = 8 to
//! radius 4 at z = 10; the countersink's cone then deleted, the hole's wall running on up from
//! z = 8 and the top closing in from radius 4 until they meet in the circle of radius 2 at z = 10.
//!
//! Refused. cad-kernel builds the countersunk block: the box a `prism`, cut by the drill, a `prism`
//! of a `disc`, and by the frustum `build::cone`. What is missing is the edit. cad-kernel deletes
//! no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out
//! of a boundary and runs the faces round them on along their own surfaces until they close the
//! gap, and `offset::normal_sweep` sweeps a face along its normal without extending any face beside
//! it. Joining back the ring the countersink took would be the result drawn again, not the
//! countersink deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the countersink's \
         cone out and runs the hole's wall on up from z = 8 and the top in from radius 4 until \
         they meet in the circle of radius 2 at z = 10"
        .into())
}
