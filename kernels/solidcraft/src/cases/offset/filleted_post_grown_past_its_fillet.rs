//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a post on it, the
//! cylinder of radius 3 from z = 0 to z = 8, the concave edge where they meet filleted to radius
//! 2, grown by 3: every point within 3 of it, the kernel's offset with its convex edges rounded.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::bench::fuse;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_cylinder([0.0, 0.0, -4.0], 8.0, 4.0)?;
    let post = make_cylinder([0.0, 0.0, 0.0], 3.0, 8.0)?;
    let filleted = round_circles(fuse(plate, post)?, &[(3.0, 0.0)], 2.0)?;
    offset_rounded(&filleted, 3.0)
}
