//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, the edge where the dome meets
//! the side y = 0 (the patch's edge v = 0) rounded to radius 1 by the kernel's fillet, the edge
//! selected by its point at u = ½.
use crate::cases::nurbs::r#bezier_dome::dome;
use crate::solids::sweep::block_under;
use crate::Refusal;
use brep_kernel::{fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let top = dome()?;
    let block = block_under(&top)?;
    Ok(fillet_edges(&block, &[top.evaluate(0.5, 0.0)?], None, 1.0, false, None)?)
}
