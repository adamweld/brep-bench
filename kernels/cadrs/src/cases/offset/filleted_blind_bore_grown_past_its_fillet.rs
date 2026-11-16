//! A block with a blind bore, filleted at the bore's floor, grown past the fillet's radius. A
//! block, the cylinder of radius 10 on the z axis from z = 0 to z = 8, with a blind bore in its
//! top: the cylinder of radius 5 from z = 2 up through the top is cut away, leaving a flat floor at
//! z = 2. The concave edge where the bore's side meets its floor, the circle of radius 5 at z = 2,
//! is filleted to radius 2: a quarter of a torus whose tube has radius 2 about the circle of radius
//! 3 at z = 4, concave, tangent to the floor at radius 3 and to the bore's side at z = 4. Grow the
//! solid by 3: every point within 3 of it.
//!
//! The edges are rounded by cadrs's Fillet and the part grown by its offset. cadrs declines it: its
//! offset has no slab for a torus face moved to or past its tube's center line.
use crate::bench::{cut, cylinder, p3, v3};
use crate::shared::fillet::fillet;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

const DISTANCE: f64 = 3.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let block = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 10.0, 8.0)?;
    let bore = cylinder(k, p3(0.0, 0.0, 2.0), v3(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let bored = cut(k, block, bore)?;
    let edges = k.edges_at(bored, &[p3(0.0, -5.0, 2.0)])?;
    let rounded = fillet(k, bored, &edges, 2.0)?;
    grown(k, rounded, DISTANCE)
}
