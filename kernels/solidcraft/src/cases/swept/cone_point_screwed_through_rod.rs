//! Refused. The groove's walls are what the cone's side sweeps where it meets the screw's motion
//! square-on. On a cone pointing across the screw's axis that contact is a curve on the side, a
//! point on each of its lines found from the screw's lead, which is neither a line of the cone nor
//! in any plane, so no section of the cone screwed by the kernel's helical sweep (`sweep_helix`)
//! carries it. solidcraft-kernel has no sweep of a solid and no envelope, and that curve computed
//! here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the cone's side meets the \
         screw square-on along a space curve, no line of it and in no plane, so no planar \
         section screwed by either layer's screw sweep (`sweep_helix`, `screw_sweep`) makes the groove's walls"
        .into())
}
