//! A drum, the cylinder of radius 8 on the z axis from z = 0 to z = 6, with a flange round its
//! side out to radius 11 from z = 2.2 to z = 3.8, shrunk by 1: every point at least 1 from its
//! outside. The flange's faces, moved 1 in from each side, cross and are trimmed away by the
//! kernel's offset inward (its self-intersections merged), and the flange's inside edges, where
//! the moved faces part, become concave tori.
use crate::bench::fuse;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let drum = make_cylinder([0.0, 0.0, 0.0], 8.0, 6.0)?;
    let flange = make_cylinder([0.0, 0.0, 2.2], 11.0, 1.6)?;
    offset_rounded(&fuse(drum, flange)?, -1.0)
}
