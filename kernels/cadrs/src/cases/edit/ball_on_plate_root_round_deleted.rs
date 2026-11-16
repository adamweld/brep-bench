//! The plate of radius 8 from z = −2 to 0 joined to the ball of radius 3 about (0, 0, 1), their
//! edge rounded to radius 1; then the round deleted. The plate and the ball united, the edge
//! filleted; then Delete face.
use crate::bench::{ball, cylinder, p3, union, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = cylinder(k, p3(0.0, 0.0, -2.0), v3(0.0, 0.0, 1.0), 8.0, 2.0)?;
    let b = ball(k, p3(0.0, 0.0, 1.0), 3.0)?;
    let joined = union(k, plate, b)?;
    let edge = k.edge_at(joined, p3(0.0, -(8f64).sqrt(), 0.0))?;
    let rounded = fillet(k, joined, &[edge], 1.0)?;
    let round = k.faces_at(rounded, &[p3(0.0, -3.2393960073426595, 0.3507838831685456)])?;
    deleted(k, rounded, &round)
}
