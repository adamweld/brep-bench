//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with an open keyway:
//! the box −1 ≤ x ≤ 1, y ≤ −3, z ≥ 8 cut away, out through the top. Then the keyway's four faces
//! deleted together: the cylinder closes over it and the top grows back to a whole disc.
use crate::bench::cut;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let shaft = make_cylinder([0.0, 0.0, 0.0], 5.0, 20.0)?;
    let key = make_box([-1.0, -6.0, 8.0], [2.0, 3.0, 13.0])?;
    let keyed = cut(shaft, key)?;
    // the floor y = −3, the walls x = −1 and x = 1, and the end z = 8
    delete_faces(&keyed, &[[0.0, -3.0, 14.0], [-1.0, -4.0, 14.0], [1.0, -4.0, 14.0], [0.0, -4.0, 8.0]])
}
