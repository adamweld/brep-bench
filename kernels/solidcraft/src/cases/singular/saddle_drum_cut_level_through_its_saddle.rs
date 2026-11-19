//! The drum of `geometry/saddle-drum` (radius 8, floor z = 0, top the bilinear B-spline saddle
//! z = 10 + xy/8), less everything above the plane z = 10, which is tangent to the saddle at its
//! saddle point: a block whose floor is that plane, reaching past the drum on every other side.
use crate::bench::cut;
use crate::shared::geometry::saddle_drum;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let above = make_box([-20.0, -20.0, 10.0], [40.0, 40.0, 20.0])?;
    cut(saddle_drum()?, above)
}
