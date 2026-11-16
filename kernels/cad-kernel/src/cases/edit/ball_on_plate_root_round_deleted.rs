//! A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), the concave edge where the plate's top meets the ball rounded to
//! radius 1 by a piece of the torus whose tube has radius 1 about the circle of radius 4 at z = 1;
//! the round then deleted, the top running on inward and the ball down until they meet in the
//! circle of radius √8 at z = 0.
//!
//! Refused. cad-kernel builds the rounded solid: the plate a `prism` of a `disc` joined to a
//! `sphere`, the circle where they meet rolled by `fillet::roll` and its piece joined into the
//! groove. What is missing is the edit. cad-kernel deletes no face of a solid: nothing in
//! `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary and runs the
//! faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Cutting away what the round added would be the result drawn again, not the round deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the round's torus \
         out and runs the plate's top on inward and the ball on down until they meet again in the \
         circle of radius √8 at z = 0"
        .into())
}
