//! The half above z = 0 of the torus about the z axis whose tube has radius 2 about the circle of
//! radius 6, its outer rim, the circle of radius 8 at z = 0, rounded to radius 1/2; then the
//! round deleted, the flat and the torus run on until they meet in the rim again.
use crate::bench::common;
use crate::shared::edit::{delete_rounds, round_the_rim, rounded};
use crate::Refusal;
use solidcore_ops::{make_box, make_torus, Body};

pub fn build() -> Result<Body, Refusal> {
    let above = make_box([-9.0, -9.0, 0.0], [18.0, 18.0, 3.0])?;
    let half = common(make_torus([0.0; 3], 6.0, 2.0)?, above)?;
    // the rim's edges and then its round, by points all round it
    let rim = round_the_rim(8.0, 0.0);
    let half = rounded(&half, &rim, 0.5)?;
    delete_rounds(&half, &rim)
}
