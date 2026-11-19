//! A block, the box from the origin to (20, 12, 8), its four upright edges rounded to radius 3,
//! then the whole rim of its top rounded to radius 1: four quarter cylinders along the sides and
//! four pieces of tori at the corners, a closed chain. Then the chain's eight faces deleted, the
//! top and the walls run on until they meet in the top's rim again.
use crate::shared::edit::{delete_rounds, rounded};
use crate::Refusal;
use solidcore_ops::{make_box, Body};
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> Result<Body, Refusal> {
    let uprights = [[0.0, 0.0, 4.0], [20.0, 0.0, 4.0], [20.0, 12.0, 4.0], [0.0, 12.0, 4.0]];
    let block = rounded(&make_box([0.0; 3], [20.0, 12.0, 8.0])?, &uprights, 3.0)?;
    // the top's rim: the middle of each side, and of each corner's quarter circle of radius 3
    let d = 3.0 - 3.0 * FRAC_1_SQRT_2;
    let rim = [
        [10.0, 0.0, 8.0],
        [20.0, 6.0, 8.0],
        [10.0, 12.0, 8.0],
        [0.0, 6.0, 8.0],
        [d, d, 8.0],
        [20.0 - d, d, 8.0],
        [20.0 - d, 12.0 - d, 8.0],
        [d, 12.0 - d, 8.0],
    ];
    let block = rounded(&block, &rim, 1.0)?;
    // the chain: the faces nearest where the rim was
    delete_rounds(&block, &rim)
}
