//! A block, the box from (−10, −10, 0) to (10, 10, 10), with a blind bore of radius 4 about the z
//! axis whose floor's edge is beveled: the cutter is the cylinder of radius 4 from z = 5 up
//! through the top joined to the frustum from radius 3 at z = 4 to radius 4 at z = 5. Then the
//! bevel deleted: the floor runs on outward and the wall down until they meet in the circle of
//! radius 4 at z = 4.
use crate::bench::{cut, fuse};
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_box, make_cone, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-10.0, -10.0, 0.0], [20.0, 20.0, 10.0])?;
    // the bore's cylinder runs on past the top, so the cut leaves no face lying on the top's
    let bore = make_cylinder([0.0, 0.0, 5.0], 4.0, 6.0)?;
    let bevel = make_cone([0.0, 0.0, 4.0], 3.0, 4.0, 1.0)?;
    let bored = cut(block, fuse(bore, bevel)?)?;
    // the bevel, halfway down it
    delete_faces(&bored, &[[0.0, 3.5, 4.5]])
}
