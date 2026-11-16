//! A rounded puck shrunk by exactly its rounds' radius. A puck: the cylinder of radius 6 on the z
//! axis from z = 0 to z = 10 with both its circular edges rounded to radius 2, each round a quarter
//! of a torus whose tube has radius 2 about the circle of radius 4 at z = 2 or z = 8. Shrink it by
//! 2: the solid of every point at least 2 from the puck's outside.
//!
//! The edges are rounded by cadrs's Fillet and the part shrunk by its offset. cadrs declines it:
//! its offset has no slab for a torus face moved to or past its tube's center line.
use crate::bench::{cylinder, p3, v3};
use crate::shared::fillet::fillet;
use crate::shared::offset::shrunk;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

const DISTANCE: f64 = 2.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let puck = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 10.0)?;
    let edges = k.edges_at(puck, &[p3(0.0, -6.0, 0.0), p3(0.0, -6.0, 10.0)])?;
    let rounded = fillet(k, puck, &edges, 2.0)?;
    shrunk(k, rounded, DISTANCE)
}
