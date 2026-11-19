//! A turned post on the z axis, the cylinder of radius 5 from z = 0 to z = 6 joined to the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, its shoulder where the two meet rounded
//! to radius 1; then the round deleted, the cylinder and the cone run on until they meet in the
//! shoulder again.
use crate::bench::fuse;
use crate::shared::edit::{delete_rounds, round_the_rim, rounded};
use crate::Refusal;
use solidcore_ops::{make_cone, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let post = fuse(make_cylinder([0.0; 3], 5.0, 6.0)?, make_cone([0.0, 0.0, 6.0], 5.0, 2.0, 4.0)?)?;
    // the shoulder's edges and then its round, by points all round it
    let shoulder = round_the_rim(5.0, 6.0);
    let post = rounded(&post, &shoulder, 1.0)?;
    delete_rounds(&post, &shoulder)
}
