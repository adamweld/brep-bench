//! The half of the ball of radius 5 about the origin above z = 0, its rim rounded to radius 1;
//! then the round deleted, the flat and the ball run on until they meet in the rim again.
use crate::bench::common;
use crate::shared::edit::{delete_rounds, round_the_rim, rounded};
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let above = make_box([-6.0, -6.0, 0.0], [12.0, 12.0, 6.0])?;
    let half = common(make_sphere([0.0; 3], 5.0)?, above)?;
    // the rim's edges and then its round, by points all round it: the ball's faces split it
    let rim = round_the_rim(5.0, 0.0);
    let half = rounded(&half, &rim, 1.0)?;
    delete_rounds(&half, &rim)
}
