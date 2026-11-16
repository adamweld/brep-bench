//! A blind bore's beveled floor deleted: the floor and the wall meet again.
//!
//! The bevel is made as the case says, a cone face from cadrs's cone primitive joined or cut; then Delete face.
use crate::bench::{block, cone, cut, cylinder, p3, union, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let b = block(k, p3(-10.0, -10.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let wall = cylinder(k, p3(0.0, 0.0, 5.0), z, 4.0, 6.0)?;
    let bevel = cone(k, p3(0.0, 0.0, 4.0), z, 3.0, 4.0, 1.0)?;
    let cutter = union(k, wall, bevel)?;
    let beveled = cut(k, b, cutter)?;
    // the bevel, halfway along its slant
    let bevel = k.faces_at(beveled, &[p3(0.0, -3.5, 4.5)])?;
    deleted(k, beveled, &bevel)
}
