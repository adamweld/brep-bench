//! A block, the box from the origin to (20, 10, 10), drilled through along z by a hole of radius
//! 2 on the line x = 10, y = 5, the hole's rim at the top rounded to radius 1; then the round
//! deleted, the top and the hole's wall run on until they meet in the rim again.
use crate::shared::edit::{delete_rounds, drilled_box, rounded};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let rim = [10.0, 7.0, 10.0];
    let block = rounded(&drilled_box([20.0, 10.0, 10.0], 10.0, 5.0, 2.0)?, &[rim], 1.0)?;
    // the round: the face nearest where the rim was
    delete_rounds(&block, &[rim])
}
