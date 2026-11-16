//! A turned post, the cylinder of radius 5 on the z axis from z = 0 to z = 6 and on it the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, its shoulder, the circle of radius 5 at
//! z = 6, rounded to radius 1 by a piece of a torus tangent to the cylinder and to the cone; the
//! round then deleted, the cylinder running on up and the cone down until they meet again in the
//! shoulder.
//!
//! Refused. cad-kernel builds the rounded post: the cylinder a `prism` of a `disc` joined to a
//! `build::cone`, the shoulder rolled by `fillet::roll` and the round's piece cut away. What is
//! missing is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`,
//! `rounds`, `offset` or `sew` takes faces out of a boundary and runs the faces round them on along
//! their own surfaces until they close the gap, and `offset::normal_sweep` sweeps a face along its
//! normal without extending any face beside it. Joining back the ring the round took would be the
//! result drawn again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the shoulder's \
         torus out and runs the cylinder on up and the cone down until they meet again in the \
         circle of radius 5 at z = 6"
        .into())
}
