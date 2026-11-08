//! The common part of two balls of radius 5 centred at (−3, 0, 0) and (3, 0, 0), each bounded by
//! one NURBS face, the standard biquadratic sphere of `geometry/nurbs-sphere` moved to its centre.
use crate::cases::geometry::r#nurbs_sphere;
use crate::solids::p;
use crate::solids::sweep::{moved, turn_z};
use crate::Refusal;
use brep_kernel::{boolean_operation, BooleanOperation, BooleanOptions, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = nurbs_sphere::build()?;
    let left = moved(&ball, turn_z(0.0, p(-3.0, 0.0, 0.0))?)?;
    let right = moved(&ball, turn_z(0.0, p(3.0, 0.0, 0.0))?)?;
    Ok(boolean_operation(&left, &right, BooleanOperation::Intersect, &BooleanOptions::default())?)
}
