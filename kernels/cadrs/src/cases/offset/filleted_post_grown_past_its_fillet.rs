//! A post on a plate, filleted at its foot, grown past the fillet's radius. A plate, the cylinder
//! of radius 8 on the z axis from z = −4 to z = 0, with a post standing on it, the cylinder of
//! radius 3 from z = 0 to z = 8. The concave edge where the post meets the plate, the circle of
//! radius 3 at z = 0, is filleted to radius 2: a quarter of a torus whose tube has radius 2 about
//! the circle of radius 5 at z = 2, concave, tangent to the post's side at z = 2 and to the plate's
//! top at radius 5. Grow the solid by 3: every point within 3 of it.
//!
//! The edges are rounded by cadrs's Fillet and the part grown by its offset. cadrs declines it: its
//! offset has no slab for a torus face moved to or past its tube's center line.
use crate::bench::{cylinder, p3, union, v3};
use crate::shared::fillet::fillet;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

const DISTANCE: f64 = 3.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = cylinder(k, p3(0.0, 0.0, -4.0), v3(0.0, 0.0, 1.0), 8.0, 4.0)?;
    let post = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 3.0, 8.0)?;
    let joined = union(k, plate, post)?;
    let edges = k.edges_at(joined, &[p3(0.0, -3.0, 0.0)])?;
    let rounded = fillet(k, joined, &edges, 2.0)?;
    grown(k, rounded, DISTANCE)
}
