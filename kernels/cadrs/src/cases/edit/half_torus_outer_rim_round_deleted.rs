//! A half torus's rounded outer rim deleted: the flat and the torus meet again: the half torus (tube radius 2 about the circle of radius 6, above z = 0), its outer rim
//! rounded to radius 1/2; then the round deleted. cadrs's torus in common with a block, filleted; then Delete face.
use crate::bench::{block, common, p3, torus, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let t = torus(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 2.0)?;
    let above = block(k, p3(-10.0, -10.0, 0.0), p3(10.0, 10.0, 5.0))?;
    let half = common(k, t, above)?;
    let rims = k.edges_at(half, &[p3(0.0, -8.0, 0.0)])?;
    let rounded = fillet(k, half, &rims, 0.5)?;
    // each round halfway round its tube, toward the old rim
    let rounds = k.faces_at(rounded, &[p3(0.0, -7.794515558701765, 0.1753919415842729)])?;
    deleted(k, rounded, &rounds)
}
