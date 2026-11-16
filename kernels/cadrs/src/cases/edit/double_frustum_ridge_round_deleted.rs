//! Two cone frustums base to base (3 -> 5 over z 0..4, 5 -> 2 over z 4..8), their ridge rounded 1;
//! then the round deleted: the two frustums. cadrs's primitives, the rim filleted; then Delete face.
use crate::bench::{cone, p3, union, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let c = cone(k, p3(0.0, 0.0, 0.0), z, 3.0, 5.0, 4.0)?;
    let f = cone(k, p3(0.0, 0.0, 4.0), z, 5.0, 2.0, 4.0)?;
    let b = union(k, c, f)?;
    let e = k.edge_at(b, p3(0.0, -5.0, 4.0))?;
    let r = fillet(k, b, &[e], 1.0)?;
    let face = k.faces_at(r, &[p3(0.0, -4.825138920703175, 3.984232786315833)])?;
    deleted(k, r, &face)
}
