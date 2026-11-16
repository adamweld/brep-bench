//! A rounded puck hollowed with walls thicker than its rounds: the cylinder of radius 6 from z = 0 to 10, both rims rounded to radius 2,
//! hollowed to walls 3 thick.
//!
//! The rims are rounded by the kernel's fillet; then its closed shell, which drops a round turned
//! inside out.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, offset_shell_closed, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let puck = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 10.0)?;
    let rounded = fillet_edges(&puck, &[p(0.0, -6.0, 0.0), p(0.0, -6.0, 10.0)], None, 2.0, false, None)?;
    Ok(offset_shell_closed(&rounded, 3.0)?.solid)
}
