//! Two solid tori of middle-circle radius 3 and tube radius 0.5: one about the z axis at the
//! origin, one about the line through (3, 0, 0) parallel to y. United, keeping both: they are
//! linked once and never touch.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 3.0, 0.5)?;
    let second = make_torus_brep(p(3.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 3.0, 0.5)?;
    Ok(union(&first, &second)?)
}
