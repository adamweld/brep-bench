//! The box from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 4 about the z axis sunk
//! from its top to a floor at z = 4, the edge where the floor meets the wall beveled: the wall
//! stops at z = 5 and a cone runs from there down and in to the floor, the disk of radius 3; the
//! bevel then deleted, the floor running on outward from radius 3 and the wall down from z = 5
//! until they meet in the circle of radius 4 at z = 4.
//!
//! Refused. cad-kernel builds the beveled bore: the block a `prism`, the cutter a `prism` of a
//! `disc` joined to the frustum `build::cone`, cut away by `boolean::combined`. What is missing is
//! the edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`,
//! `offset` or `sew` takes faces out of a boundary and runs the faces round them on along their own
//! surfaces until they close the gap, and `offset::normal_sweep` sweeps a face along its normal
//! without extending any face beside it. Cutting away the ring the bevel leaves would be the result
//! drawn again, not the bevel deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the bevel's cone \
         out of the bore and runs the floor on outward from radius 3 and the wall down from z = 5 \
         until they meet in the circle of radius 4 at z = 4"
        .into())
}
