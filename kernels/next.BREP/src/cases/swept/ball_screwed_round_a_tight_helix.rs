//! Everything a ball of radius 2 passes through as its centre runs two turns along the helix of
//! radius 1.5 about the z axis rising 3 a turn, from (1.5, 0, 0) to (1.5, 0, 6).
//!
//! next.BREP has no sweep of a solid. A ball's sweep is, by elementary geometry, the tube of the
//! ball's radius about its centre's path with a ball at each end: the tube is the kernel's helical
//! sweep of the ball's great circle (which starts its helix on +y, so it is turned a quarter turn
//! back about z), and the balls are its spheres. Here the tube is wider than the helix's radius of
//! curvature, so it folds through itself, and the kernel's helical sweep says whether it builds
//! that.
use crate::solids::p;
use crate::solids::sweep::{circle, moved, turn_z};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_sphere_brep, sweep_profile_helix, BooleanOperation, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let section = circle(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 2.0)?;
    let coil = sweep_profile_helix(&section, p(0.0, 0.0, 0.0), up, 1.5, 3.0, 2.0, None)?;
    let tube = moved(&coil, turn_z(-FRAC_PI_2, p(0.0, 0.0, 0.0))?)?;
    let start = make_sphere_brep(p(1.5, 0.0, 0.0), 2.0, up)?;
    let end = make_sphere_brep(p(1.5, 0.0, 6.0), 2.0, up)?;
    Ok(boolean_operation_nary(&[tube, start, end], BooleanOperation::Union)?)
}
