//! Refused. The bar's sides are two planes and two half cones. Its planes meet the screw's motion
//! square-on along lines, but its half cones, whose axes run across the screw's, meet it along
//! curves that are neither lines of them nor in any plane, and the groove's walls are partly what
//! those curves sweep. So no section of the bar screwed by the kernel's helical sweep
//! (`sweep_helix`) carries them. solidcraft-kernel has no sweep of a solid and no envelope, and
//! those curves computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the bar's half-conical ends \
         meet the screw square-on along space curves, in no plane, so no planar section screwed \
         by either layer's screw sweep (`sweep_helix`, `screw_sweep`) makes the groove's walls"
        .into())
}
