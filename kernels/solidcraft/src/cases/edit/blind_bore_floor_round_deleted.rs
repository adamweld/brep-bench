//! A block, the box from (-10, -10, 0) to (10, 10, 10), with a blind bore of radius 4 about the
//! z axis sunk from its top to a flat floor at z = 4, the concave edge where the floor meets the
//! wall rounded to radius 1; then the round deleted, the floor and the wall run on until they
//! meet in the sharp edge again.
use crate::bench::cut;
use crate::shared::edit::{delete_rounds, rounded};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-10.0, -10.0, 0.0], [20.0, 20.0, 10.0])?;
    let bored = cut(block, make_cylinder([0.0, 0.0, 4.0], 4.0, 7.0)?)?;
    let floor = [0.0, 4.0, 4.0];
    let bored = rounded(&bored, &[floor], 1.0)?;
    // the round: the face nearest where the floor's edge was
    delete_rounds(&bored, &[floor])
}
