//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, its top rim rounded to radius 2;
//! then the round deleted, the top and the side run on until they meet in the rim again: the
//! kernel's fillet removal, its round's neighbours a plane and a cylinder.
use crate::shared::edit::{delete_rounds, rounded};
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let rim = [0.0, 6.0, 10.0];
    let cylinder = rounded(&make_cylinder([0.0; 3], 6.0, 10.0)?, &[rim], 2.0)?;
    // the round: the face nearest where the rim was
    delete_rounds(&cylinder, &[rim])
}
