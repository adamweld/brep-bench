//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a ring groove
//! turned into it: everything between radius 4 and 5 from z = 8 to z = 12 cut away. Then the
//! groove's floor and both its walls deleted together: the cylinder below and above runs on
//! across the groove.
use crate::bench::cut;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let shaft = make_cylinder([0.0, 0.0, 0.0], 5.0, 20.0)?;
    let ring = cut(make_cylinder([0.0, 0.0, 8.0], 6.0, 4.0)?, make_cylinder([0.0, 0.0, 7.0], 4.0, 6.0)?)?;
    let grooved = cut(shaft, ring)?;
    // the floor, and the walls at z = 8 and z = 12
    delete_faces(&grooved, &[[0.0, 4.0, 10.0], [0.0, 4.5, 8.0], [0.0, 4.5, 12.0]])
}
