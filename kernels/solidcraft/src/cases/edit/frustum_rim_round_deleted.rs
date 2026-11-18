//! A cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its top rim rounded
//! to radius 1; then the round deleted, the top and the cone run on until they meet in the rim
//! again.
use crate::shared::edit::{delete_rounds, rounded};
use crate::Refusal;
use solidcore_ops::{make_cone, Body};

pub fn build() -> Result<Body, Refusal> {
    let rim = [0.0, 3.0, 6.0];
    let frustum = rounded(&make_cone([0.0; 3], 6.0, 3.0, 6.0)?, &[rim], 1.0)?;
    // the round: the face nearest where the rim was
    delete_rounds(&frustum, &[rim])
}
