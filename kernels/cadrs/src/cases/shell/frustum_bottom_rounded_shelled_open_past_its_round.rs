//! A cone frustum with its bottom rim rounded, shelled open at its top past the round's radius: the frustum from radius 6 at z = 0 to radius 3 at z = 6, its bottom rim rounded to radius 1.5,
//! shelled inward open at its top to walls 2 thick. cadrs's cone, filleted, then its shell.
use crate::bench::{cone, p3, v3};
use crate::shared::fillet::fillet;
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    let edges = k.edges_at(frustum, &[p3(0.0, -6.0, 0.0)])?;
    let rounded = fillet(k, frustum, &edges, 1.5)?;
    shell_inward(k, rounded, &[p3(0.0, 0.0, 6.0)], 2.0, Joins::Rounded)
}
