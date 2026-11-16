//! A rounded cone frustum hollowed with walls thicker than its rounds: the frustum from radius 6 at z = 0 to radius 3 at z = 6, both rims rounded to radius 1.5,
//! hollowed to walls 2 thick.
//!
//! The rims are rounded by the kernel's fillet; then the kernel's closed shell (hollow), which drops a
//! round turned inside out.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cone_brep, offset_shell_closed, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let frustum = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    let rounded = fillet_edges(&frustum, &[p(0.0, -3.0, 6.0), p(0.0, -6.0, 0.0)], None, 1.5, false, None)?;
    Ok(offset_shell_closed(&rounded, 2.0)?.solid)
}
