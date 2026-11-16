//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss standing on
//! it, the cylinder of radius 3 from z = 0 to z = 8; the boss's side and top then deleted, the
//! plate's top closing over the circle of radius 3 at the boss's foot.
//!
//! Refused. cad-kernel builds the bossed plate: both cylinders `prism`s of `disc`s, joined by
//! `boolean::combined`. What is missing is the edit. cad-kernel deletes no face of a solid: nothing
//! in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary and runs the
//! faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Cutting the boss back off would be the result drawn again, not the boss deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the boss's side \
         and top out and closes the plate's top over the circle of radius 3 at the boss's foot"
        .into())
}
