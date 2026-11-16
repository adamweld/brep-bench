//! A filleted cone boss grown past its fillet's radius: a plate (radius 8, z = −2 to 0) and a cone boss (radius 4 at z = 0 to 1 at z = 6) joined,
//! the concave root filleted to radius 1, grown by 2. cadrs's cylinder and cone united, filleted, then its offset.
use crate::bench::{cone, cylinder, p3, union, v3};
use crate::shared::fillet::fillet;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

const DISTANCE: f64 = 2.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let plate = cylinder(k, p3(0.0, 0.0, -2.0), z, 8.0, 2.0)?;
    let boss = cone(k, p3(0.0, 0.0, 0.0), z, 4.0, 1.0, 6.0)?;
    let joined = union(k, plate, boss)?;
    let root = k.edge_at(joined, p3(0.0, -4.0, 0.0))?;
    let filleted = fillet(k, joined, &[root], 1.0)?;
    grown(k, filleted, DISTANCE)
}
