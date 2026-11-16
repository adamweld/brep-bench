//! A post, the cylinder of radius 4 on the z axis from z = 0 to z = 6, capped by the part above
//! z = 6 of the ball of radius 5 about (0, 0, 3), the rim where they meet, the circle of radius 4
//! at z = 6, rounded to radius 1 by a piece of a torus tangent to both; the round then deleted, the
//! cylinder running on up and the dome down until they meet again in that circle.
//!
//! Refused. cad-kernel builds the rounded post: the cylinder a `prism` of a `disc` joined to a
//! `sphere` `kept` above z = 6, the rim rolled by `fillet::roll` and the round's piece cut away.
//! What is missing is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`,
//! `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary and runs the faces round
//! them on along their own surfaces until they close the gap, and `offset::normal_sweep` sweeps a
//! face along its normal without extending any face beside it. Joining back the ring the round took
//! would be the result drawn again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the rim's torus \
         out and runs the cylinder on up and the dome down until they meet again in the circle of \
         radius 4 at z = 6"
        .into())
}
