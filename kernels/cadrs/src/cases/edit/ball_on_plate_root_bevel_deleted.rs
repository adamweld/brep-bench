//! A ball standing in a plate, its beveled root deleted: the plate and the ball meet again.
//!
//! The bevel is made as the case says, a cone face from cadrs's cone primitive joined or cut; then Delete face.
use crate::bench::{ball, cone, cylinder, p3, union, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let plate = cylinder(k, p3(0.0, 0.0, -2.0), z, 8.0, 2.0)?;
    let b = ball(k, p3(0.0, 0.0, 1.0), 3.0)?;
    let joined = union(k, plate, b)?;
    let ring = cone(k, p3(0.0, 0.0, 0.0), z, 4.0, 3.0, 1.0)?;
    let beveled = union(k, joined, ring)?;
    // the bevel, halfway along its slant
    let bevel = k.faces_at(beveled, &[p3(0.0, -3.5, 0.5)])?;
    deleted(k, beveled, &bevel)
}
