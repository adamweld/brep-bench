//! A cone frustum about the z axis, of radius 6 at z = 0 and 3 at z = 20, with a ring groove
//! turned into it: everything outside radius 4 from z = 8 to z = 12 cut away. Then the groove's
//! floor and both its walls deleted together: the cone below and above runs on across the groove.
use crate::bench::cut;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_cone, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let frustum = make_cone([0.0, 0.0, 0.0], 6.0, 3.0, 20.0)?;
    let ring = cut(make_cylinder([0.0, 0.0, 8.0], 10.0, 4.0)?, make_cylinder([0.0, 0.0, 7.0], 4.0, 6.0)?)?;
    let grooved = cut(frustum, ring)?;
    // the floor, and the walls at z = 8 (out to radius 4.8) and z = 12 (out to 4.2)
    delete_faces(&grooved, &[[0.0, 4.0, 10.0], [0.0, 4.4, 8.0], [0.0, 4.1, 12.0]])
}
