//! A tube about the z axis from z = 0 to z = 20, outer radius 10, bore radius 4, with a ring groove
//! turned into the bore (everything inside radius 6 from z = 8 to z = 12 cut away), the groove's
//! floor and both walls deleted together: the bore below and above the groove is one surface, which
//! runs on across it.
//!
//! The body is made and the groove cut away by booleans; cadrs's Delete Face deletes the groove's
//! faces, picked by a point inside each.
use crate::bench::{cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let outer = cylinder(k, p3(0.0, 0.0, 0.0), z, 10.0, 20.0)?;
    let bore = cylinder(k, p3(0.0, 0.0, -1.0), z, 4.0, 22.0)?;
    let tube = cut(k, outer, bore)?;
    let ring = cylinder(k, p3(0.0, 0.0, 8.0), z, 6.0, 4.0)?;
    let grooved = cut(k, tube, ring)?;
    let groove = k.faces_at(grooved, &[p3(0.0, -6.0, 10.0), p3(0.0, -5.0, 8.0), p3(0.0, -5.0, 12.0)])?;
    deleted(k, grooved, &groove)
}
