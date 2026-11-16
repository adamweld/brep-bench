//! A rounded cone frustum hollowed with walls thicker than its rounds: the frustum from radius 6 at z = 0 to radius 3 at z = 6, both rims rounded to radius 1.5,
//! hollowed to walls 2 thick. cadrs's cone, filleted, then its shell.
use crate::bench::{cone, p3, v3};
use crate::shared::fillet::fillet;
use crate::shared::shell::hollow;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    let edges = k.edges_at(frustum, &[p3(0.0, -3.0, 6.0), p3(0.0, -6.0, 0.0)])?;
    let rounded = fillet(k, frustum, &edges, 1.5)?;
    hollow(k, rounded, 2.0)
}
