//! A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), the groove where they meet filled by a bevel, the frustum from radius
//! 4 at z = 0 to radius 3 at z = 1; the bevel's cone then deleted, the plate's top running on
//! inward and the ball down from its equator until they meet in the circle of radius √8 at z = 0.
//!
//! Refused. cad-kernel builds the beveled solid: the plate a `prism` of a `disc`, the ball a
//! `sphere`, the bevel the frustum `build::cone`, joined by `boolean::combined`. What is missing is
//! the edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`,
//! `offset` or `sew` takes faces out of a boundary and runs the faces round them on along their own
//! surfaces until they close the gap, and `offset::normal_sweep` sweeps a face along its normal
//! without extending any face beside it. Cutting the frustum back off would be the result drawn
//! again, not the bevel deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the bevel's cone \
         out and runs the plate's top on inward and the ball down from its equator until they \
         meet in the circle of radius √8 at z = 0"
        .into())
}
