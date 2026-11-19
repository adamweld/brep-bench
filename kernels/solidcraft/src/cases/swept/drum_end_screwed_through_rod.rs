//! Refused. The mill's axis runs across the screw's, so its side, a cylinder about the x axis,
//! meets the screw's motion square-on where x·y = −(6/2π)·z: a curve on the side that is no line of
//! it and in no plane, and the groove's walls near the rod's surface are what that curve sweeps. So
//! no section of the mill screwed by the kernel's helical sweep (`sweep_helix`) carries it.
//! solidcraft-kernel has no sweep of a solid and no envelope, and that curve computed here would be
//! geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the mill's side meets the \
         screw square-on along the space curve x·y = −(6/2π)·z, in no plane, so no planar \
         section screwed by either layer's screw sweep (`sweep_helix`, `screw_sweep`) makes the groove's walls"
        .into())
}
