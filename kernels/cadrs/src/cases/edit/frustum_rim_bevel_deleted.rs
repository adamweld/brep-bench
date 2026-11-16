//! A cone frustum's beveled top rim deleted: the top and the cone meet again.
//!
//! The bevel is made as the case says, a cone face from cadrs's cone primitive joined or cut; then Delete face.
use crate::bench::{cone, p3, union, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let lower = cone(k, p3(0.0, 0.0, 0.0), z, 6.0, 3.5, 5.0)?;
    let bevel = cone(k, p3(0.0, 0.0, 5.0), z, 3.5, 2.0, 1.0)?;
    let beveled = union(k, lower, bevel)?;
    // the bevel, halfway along its slant
    let bevel = k.faces_at(beveled, &[p3(0.0, -2.75, 5.5)])?;
    deleted(k, beveled, &bevel)
}
