//! A cone frustum with its top rim rounded, shrunk past the round's radius: the frustum from radius 6 at z = 0 to radius 3 at z = 6, its top rim
//! rounded to radius 1.5, shrunk by 2. cadrs's cone primitive, filleted, then its offset (shrunk).
use crate::bench::{cone, p3, v3};
use crate::shared::fillet::fillet;
use crate::shared::offset::shrunk;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

const DISTANCE: f64 = 2.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    let edges = k.edges_at(frustum, &[p3(0.0, -3.0, 6.0)])?;
    let rounded = fillet(k, frustum, &edges, 1.5)?;
    shrunk(k, rounded, DISTANCE)
}
