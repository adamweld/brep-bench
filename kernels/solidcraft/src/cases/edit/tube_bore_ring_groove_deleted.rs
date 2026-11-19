//! A tube about the z axis from z = 0 to z = 20, outer radius 10, its bore of radius 4, with a ring
//! groove turned into the bore: everything inside radius 6 from z = 8 to z = 12 cut away. Then the
//! groove's floor and both its walls deleted together: the bore below and above runs on across
//! the groove.
use crate::bench::cut_all;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let outer = make_cylinder([0.0, 0.0, 0.0], 10.0, 20.0)?;
    let bore = make_cylinder([0.0, 0.0, -1.0], 4.0, 22.0)?;
    let groove = make_cylinder([0.0, 0.0, 8.0], 6.0, 4.0)?;
    let tube = cut_all(outer, [bore, groove])?;
    // the floor, and the walls at z = 8 and z = 12
    delete_faces(&tube, &[[0.0, 6.0, 10.0], [0.0, 5.0, 8.0], [0.0, 5.0, 12.0]])
}
