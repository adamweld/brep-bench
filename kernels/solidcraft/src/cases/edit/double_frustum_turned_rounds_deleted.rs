//! Two cone frustums on the z axis joined, the first from radius 3 at z = 0 to radius 5 at z = 4,
//! the second from radius 5 at z = 4 to radius 2 at z = 8 and turned by 2 radians about the z
//! axis before the two are joined, so that the ridge where they meet is two edges; the ridge
//! rounded to radius 1, two faces, then both deleted, the two cones run on until they meet in
//! the ridge again.
use crate::bench::{fuse, rotated};
use crate::shared::edit::{delete_rounds, round_the_rim, rounded};
use crate::Refusal;
use solidcore_ops::{make_cone, Body};

pub fn build() -> Result<Body, Refusal> {
    let upper = rotated(make_cone([0.0, 0.0, 4.0], 5.0, 2.0, 4.0)?, [0.0; 3], [0.0, 0.0, 1.0], 2f64.to_degrees())?;
    let two = fuse(make_cone([0.0; 3], 3.0, 5.0, 4.0)?, upper)?;
    // the ridge's edges and then its rounds, by points all round it
    let ridge = round_the_rim(5.0, 4.0);
    let two = rounded(&two, &ridge, 1.0)?;
    delete_rounds(&two, &ridge)
}
