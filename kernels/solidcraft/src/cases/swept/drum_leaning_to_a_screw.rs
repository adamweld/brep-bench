//! Refused. The drum leans 25° to the screw's axis, so its side meets the screw's motion square-on
//! along a curve that runs along the drum as it goes round it (the side's line at angle ψ about the
//! drum's axis touches at a distance along it proportional to tan ψ), no line of the side and in no
//! plane. So no section of the drum screwed by the kernel's helical sweep (`sweep_helix`) carries
//! it. solidcraft-kernel has no sweep of a solid and no envelope, and that curve computed here
//! would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the leaning drum's side \
         meets the screw square-on along a space curve, in no plane, so no planar section \
         screwed by `sweep_helix` makes the groove's walls"
        .into())
}
