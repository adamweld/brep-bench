//! A turned post's rounded shoulder deleted: the cylinder r 5 (z 0..6) under the frustum 5 -> 2 (z 6..10), its shoulder rounded 1;
//! then the round deleted: the post. cadrs's primitives, the rim filleted; then Delete face.
use crate::bench::{cone, cylinder, p3, union, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let c = cylinder(k, p3(0.0, 0.0, 0.0), z, 5.0, 6.0)?;
    let f = cone(k, p3(0.0, 0.0, 6.0), z, 5.0, 2.0, 4.0)?;
    let b = union(k, c, f)?;
    let e = k.edge_at(b, p3(0.0, -5.0, 6.0))?;
    let r = fillet(k, b, &[e], 1.0)?;
    let face = k.faces_at(r, &[p3(0.0, -4.948683298050514, 5.982894432683505)])?;
    deleted(k, r, &face)
}
