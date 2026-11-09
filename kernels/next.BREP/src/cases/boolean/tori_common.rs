//! The common part of two solid tori about the z axis, each of middle-circle radius 5 and tube
//! radius 2, their middle circles in z = −1 and z = 1.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let lower = make_torus_brep(p(0.0, 0.0, -1.0), p(0.0, 0.0, 1.0), 5.0, 2.0)?;
    let upper = make_torus_brep(p(0.0, 0.0, 1.0), p(0.0, 0.0, 1.0), 5.0, 2.0)?;
    Ok(common(&lower, &upper)?)
}
