//! A post capped by a ball: the cylinder r 4 (z 0..6) under the cap above z 6 of the ball r 5 about (0, 0, 3), its rim rounded 1;
//! then the round deleted: the domed post. cadrs's primitives, the rim filleted; then Delete face.
use crate::bench::{ball, block, common, cylinder, p3, union, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b0 = ball(k, p3(0.0, 0.0, 3.0), 5.0)?;
    let above = block(k, p3(-6.0, -6.0, 6.0), p3(6.0, 6.0, 10.0))?;
    let cap = common(k, b0, above)?;
    let c = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 4.0, 6.0)?;
    let b = union(k, c, cap)?;
    let e = k.edge_at(b, p3(0.0, -4.0, 6.0))?;
    let r = fillet(k, b, &[e], 1.0)?;
    let f = k.faces_at(r, &[p3(0.0, -3.9426028923173684, 5.979667149854743)])?;
    deleted(k, r, &f)
}
