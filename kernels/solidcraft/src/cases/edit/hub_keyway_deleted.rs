//! A hub, the tube about the z axis from z = 0 to z = 20, outer radius 10, its bore of radius 4,
//! with a keyway cut along the bore's whole length: the box −1 ≤ x ≤ 1, −5 ≤ y ≤ 0. Then the
//! keyway's floor and two walls deleted together: the bore closes over it and the ends grow back
//! to whole rings.
use crate::bench::cut_all;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let outer = make_cylinder([0.0, 0.0, 0.0], 10.0, 20.0)?;
    let bore = make_cylinder([0.0, 0.0, -1.0], 4.0, 22.0)?;
    let key = make_box([-1.0, -5.0, -1.0], [2.0, 5.0, 22.0])?;
    let hub = cut_all(outer, [bore, key])?;
    // the floor y = −5, and the walls x = −1 and x = 1 outside the bore
    delete_faces(&hub, &[[0.0, -5.0, 10.0], [-1.0, -4.5, 10.0], [1.0, -4.5, 10.0]])
}
