//! A block, the box from the origin to (20, 10, 10), drilled right through along z by a hole of
//! radius 2 on the vertical line through (10, 5), and countersunk at the top by the cone about
//! that line from radius 2 at z = 8 to radius 4 at z = 10. Then the countersink deleted: the
//! hole's wall runs on up and the top closes in until they meet in the circle of radius 2 at
//! z = 10.
use crate::bench::cut_all;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_box, make_cone, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [20.0, 10.0, 10.0])?;
    let drill = make_cylinder([10.0, 5.0, -1.0], 2.0, 12.0)?;
    // the countersink's cone, of slope 1, run on past the top to radius 5 at z = 11
    let sink = make_cone([10.0, 5.0, 8.0], 2.0, 5.0, 3.0)?;
    let block = cut_all(block, [drill, sink])?;
    // the cone, halfway up it
    delete_faces(&block, &[[10.0, 8.0, 9.0]])
}
