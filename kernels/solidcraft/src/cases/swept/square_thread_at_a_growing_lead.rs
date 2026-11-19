//! Refused. The bar has risen 6s + 0.3s² when it has turned s whole turns, so no screw carries it,
//! and the groove's section changes at every height. The helical sweeps of all three layers
//! (solidcraft-kernel's `sweep_helix`, solidcore-ops' `screw_sweep` and solidcore's screw sweep
//! beneath them) rise a constant pitch, and their radial laws (`sweep_helix_radial`) only move the
//! section off the axis as it turns; the path sweeps follow lines, arcs and Bézier or spline
//! curves, none of which is that path exactly, with frames that follow the path rather than the
//! turn. With no sweep of a solid either, nothing carries even a face of the bar along this motion.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or a sweep along a path of growing lead: \
         every layer's helical sweep rises a constant pitch (`sweep_helix_radial` only moves the section \
         off the axis), and its path sweeps take lines, arcs and splines, none of which is the \
         path that has risen 6s + 0.3s² after s turns"
        .into())
}
