//! The box from the origin to (20, 16, 12) with every edge rounded to radius 2, twelve quarter
//! cylinders along its edges and eight eighths of a ball at its corners; all twenty rounds then
//! deleted, the six flat faces running on until they meet in the box's twelve edges and eight
//! corners.
//!
//! Refused. cad-kernel builds the rounded box: the box a `prism`, its twelve edges rounded together
//! by `rounds::rounded`, which closes each corner with its ball. What is missing is the edit.
//! cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or
//! `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Building the box without its rounds would be the result drawn
//! again, not the rounds deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the twelve quarter \
         cylinders and the eight eighths of a ball out and runs the six flat faces on until they \
         meet in the box's twelve edges and eight corners"
        .into())
}
