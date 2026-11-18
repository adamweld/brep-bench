//! A cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, both its rims
//! rounded to radius 3/2; then the top round only deleted, the top and the cone run on until
//! they meet in the top rim again, the base's round kept.
use crate::shared::edit::{delete_rounds, rounded};
use crate::Refusal;
use solidcore_ops::{make_cone, Body};

pub fn build() -> Result<Body, Refusal> {
    let (top, base) = ([0.0, 3.0, 6.0], [0.0, 6.0, 0.0]);
    let frustum = rounded(&make_cone([0.0; 3], 6.0, 3.0, 6.0)?, &[top, base], 1.5)?;
    // the top round: the face nearest where the top rim was
    delete_rounds(&frustum, &[top])
}
