//! A cone frustum about the z axis, of radius 6 at z = 0 and 3 at z = 20, with a flat milled
//! along its side where y < −4; then the flat deleted, the cone closing over it and the base
//! growing back to a whole disc.
use crate::bench::cut;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_box, make_cone, Body};

pub fn build() -> Result<Body, Refusal> {
    let frustum = make_cone([0.0, 0.0, 0.0], 6.0, 3.0, 20.0)?;
    let below = make_box([-10.0, -10.0, -1.0], [20.0, 6.0, 22.0])?;
    let milled = cut(frustum, below)?;
    // the flat, by a point on it just above the base
    delete_faces(&milled, &[[0.0, -4.0, 1.0]])
}
