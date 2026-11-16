//! A ball standing in a plate, its root filleted, grown past the fillet's radius: a plate (radius 8, z = −2 to 0) and the ball of radius 3 about (0, 0, 1) joined,
//! the concave root filleted to radius 1, grown by 2. cadrs's cylinder and ball united, filleted, then its offset.
use crate::bench::{ball, cylinder, p3, union, v3};
use crate::shared::fillet::fillet;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

const DISTANCE: f64 = 2.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let plate = cylinder(k, p3(0.0, 0.0, -2.0), z, 8.0, 2.0)?;
    let boss = ball(k, p3(0.0, 0.0, 1.0), 3.0)?;
    let joined = union(k, plate, boss)?;
    let root = k.edge_at(joined, p3(0.0, -(8f64).sqrt(), 0.0))?;
    let filleted = fillet(k, joined, &[root], 1.0)?;
    grown(k, filleted, DISTANCE)
}
