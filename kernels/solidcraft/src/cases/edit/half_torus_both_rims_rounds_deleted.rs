//! The half above z = 0 of the torus about the z axis whose tube has radius 2 about the circle of
//! radius 6, both its rims, the circles of radius 8 and 4 at z = 0, rounded to radius 1/2; then
//! both rounds deleted, the flat and the torus run on until they meet in the rims again.
use crate::bench::common;
use crate::shared::edit::{delete_rounds, round_the_rim, rounded};
use crate::Refusal;
use solidcore_ops::{make_box, make_torus, Body};

pub fn build() -> Result<Body, Refusal> {
    let above = make_box([-9.0, -9.0, 0.0], [18.0, 18.0, 3.0])?;
    let half = common(make_torus([0.0; 3], 6.0, 2.0)?, above)?;
    // the rims' edges and then their rounds, by points all round each
    let rims = [round_the_rim(8.0, 0.0), round_the_rim(4.0, 0.0)].concat();
    let half = rounded(&half, &rims, 0.5)?;
    delete_rounds(&half, &rims)
}
