//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 4, more than the hole's radius, to its parallel body: the hole fills, and
//! each end of it leaves a dimple in the grown face, the inner part of a spindle torus.
//!
//! The hole is cut by a cylinder running past both faces of the plate.
use crate::solids::offset::{parallel, subtract};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 4.0)?;
    let drill = make_cylinder_brep(p(0.0, 0.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 6.0)?;
    let drilled = subtract(&plate, &drill)?;
    Ok(parallel(&drilled, 4.0)?)
}
