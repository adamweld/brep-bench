//! A hemisphere with its rim rounded, hollowed with walls thicker than the round: the half ball of radius 5 above z = 0, its rim rounded to radius 1, then
//! hollowed to walls 2 thick.
//!
//! The half ball is the kernel's ball in common with a box above z = 0, its rim filleted; then the kernel's
//! closed shell, which drops the round turned inside out.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, make_sphere_brep, offset_shell_closed, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let half = common(&ball, &make_box_brep(p(-6.0, -6.0, 0.0), 12.0, 12.0, 6.0)?)?;
    let rounded = fillet_edges(&half, &[p(0.0, -5.0, 0.0)], None, 1.0, false, None)?;
    Ok(offset_shell_closed(&rounded, 2.0)?.solid)
}
