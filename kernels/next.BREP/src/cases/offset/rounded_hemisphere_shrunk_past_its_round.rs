//! A hemisphere with its rim rounded, shrunk past the round's radius: the half ball of radius 5 above z = 0, its rim rounded to radius 1, then
//! shrunk by 3/2.
//!
//! The half ball is the kernel's ball in common with a box above z = 0, its rim filleted; then the kernel's
//! rounded offset, which drops the round turned inside out.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use crate::solids::offset::parallel;
use brep_kernel::{fillet_edges, make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let half = common(&ball, &make_box_brep(p(-6.0, -6.0, 0.0), 12.0, 12.0, 6.0)?)?;
    let rounded = fillet_edges(&half, &[p(0.0, -5.0, 0.0)], None, 1.0, false, None)?;
    Ok(parallel(&rounded, -1.5)?)
}
