//! A cone frustum with its bottom rim rounded, shelled open at its top past the round's radius: the frustum from radius 6 at z = 0 to radius 3 at z = 6, its bottom rim rounded to radius 1.5,
//! shelled inward open at its top to walls 2 thick.
//!
//! The rims are rounded by the kernel's fillet; then the kernel's shell, which drops a
//! round turned inside out.
use crate::solids::p;
use crate::Refusal;
use crate::solids::shell::shell;
use brep_kernel::{fillet_edges, make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let frustum = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    let rounded = fillet_edges(&frustum, &[p(0.0, -6.0, 0.0)], None, 1.5, false, None)?;
    shell(&rounded, &[p(0.0, 0.0, 6.0)], 2.0)
}
