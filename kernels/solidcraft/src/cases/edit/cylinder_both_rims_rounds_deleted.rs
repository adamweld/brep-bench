//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, both its rims rounded to radius 2;
//! then both rounds deleted, the top, the bottom and the side run on until they meet in the
//! rims again.
use crate::shared::edit::{delete_rounds, rounded};
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let rims = [[0.0, 6.0, 10.0], [0.0, 6.0, 0.0]];
    let cylinder = rounded(&make_cylinder([0.0; 3], 6.0, 10.0)?, &rims, 2.0)?;
    // the rounds: the faces nearest where the rims were
    delete_rounds(&cylinder, &rims)
}
