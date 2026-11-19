//! A block, the box from the origin to (20, 10, 10), drilled through along z by a hole of radius
//! 2 on the line x = 10, y = 5, both of the hole's rims rounded to radius 1; then both rounds
//! deleted, the top, the bottom and the hole's wall run on until they meet in the rims again.
use crate::shared::edit::{delete_rounds, drilled_box, rounded};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let rims = [[10.0, 7.0, 10.0], [10.0, 7.0, 0.0]];
    let block = rounded(&drilled_box([20.0, 10.0, 10.0], 10.0, 5.0, 2.0)?, &rims, 1.0)?;
    // the rounds: the faces nearest where the rims were
    delete_rounds(&block, &rims)
}
