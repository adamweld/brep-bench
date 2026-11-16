//! The cube from the origin to (10, 10, 10), the three edges meeting at its corner (10, 10, 10)
//! rounded to radius 2, three quarter cylinders and the eighth of a ball of radius 2 about (8, 8,
//! 8) where they meet; all four then deleted, the faces x = 10, y = 10 and z = 10 running on until
//! they meet in the three edges and the corner.
//!
//! Refused. cad-kernel builds the rounded cube: the cube a `prism`, its three edges rounded
//! together by `rounds::rounded`, which closes their corner with the ball. What is missing is the
//! edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset`
//! or `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Building the cube without its rounds would be the result drawn
//! again, not the rounds deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the three quarter \
         cylinders and the corner's eighth of a ball out and runs the faces x = 10, y = 10 and \
         z = 10 on until they meet in the three edges and the corner (10, 10, 10)"
        .into())
}
