//! A cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, both its rims
//! rounded to radius 3/2; then both rounds deleted, the ends and the cone run on until they meet
//! in the rims again.
use crate::shared::edit::{delete_rounds, rounded};
use crate::Refusal;
use solidcore_ops::{make_cone, Body};

pub fn build() -> Result<Body, Refusal> {
    let rims = [[0.0, 3.0, 6.0], [0.0, 6.0, 0.0]];
    let frustum = rounded(&make_cone([0.0; 3], 6.0, 3.0, 6.0)?, &rims, 1.5)?;
    // the rounds: the faces nearest where the rims were
    delete_rounds(&frustum, &rims)
}
