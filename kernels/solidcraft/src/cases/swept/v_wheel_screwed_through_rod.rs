//! Refused. The V groove is what a wheel with a sharp V rim, set at the helix angle, passes through
//! as it is screwed along the rod. The kernel's helical sweep (`sweep_helix`) carries a planar
//! profile along a screw, but no plane section of the wheel is the groove's section: the
//! groove's flanks are the envelope of the wheel's conical flanks over the screw. solidcraft-kernel
//! has no sweep of a solid and no envelope, and that section computed here would be geometry made
//! outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: no layer's screw sweep (`sweep_helix`, `screw_sweep`) does more than screw a planar \
         profile, but the V groove's flanks are the envelope of the leaning wheel's cones over \
         the screw, which is no plane section of it"
        .into())
}
