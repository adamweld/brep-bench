//! A cone frustum about the z axis, radius 6 at z = 0 to 3 at z = 20, with a ring groove turned
//! into it (everything outside radius 4 from z = 8 to z = 12 cut away), the groove's floor and both
//! walls deleted together: the frustum's cone below and above the groove is one surface, which runs
//! on across it.
//!
//! The body and the ring are made and the ring cut away; cadrs's Delete Face deletes the groove's
//! three faces, picked by a point inside each.
use crate::bench::{cone, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let body = cone(k, p3(0.0, 0.0, 0.0), z, 6.0, 3.0, 20.0)?;
    let outer = cylinder(k, p3(0.0, 0.0, 8.0), z, 7.0, 4.0)?;
    let inner = cylinder(k, p3(0.0, 0.0, 7.0), z, 4.0, 6.0)?;
    let ring = cut(k, outer, inner)?;
    let grooved = cut(k, body, ring)?;
    let groove = k.faces_at(grooved, &[p3(0.0, -4.0, 10.0), p3(0.0, -4.5, 8.0), p3(0.0, -4.1, 12.0)])?;
    deleted(k, grooved, &groove)
}
