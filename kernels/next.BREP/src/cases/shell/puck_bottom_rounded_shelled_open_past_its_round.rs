//! A puck with its bottom rim rounded, shelled open at its top past the round's radius: the cylinder of radius 6 from z = 0 to 10, its bottom rim rounded to radius 2,
//! shelled inward open at its top to walls 3 thick.
//!
//! The rims are rounded by the kernel's fillet; then its shell, which drops a round turned
//! inside out.
use crate::solids::p;
use crate::Refusal;
use crate::solids::shell::shell;
use brep_kernel::{fillet_edges, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let puck = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 10.0)?;
    let rounded = fillet_edges(&puck, &[p(0.0, -6.0, 0.0)], None, 2.0, false, None)?;
    shell(&rounded, &[p(0.0, 0.0, 10.0)], 3.0)
}
