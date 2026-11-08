//! The ring sector 8 ≤ ρ ≤ 12, −1 ≤ z ≤ 1, 0° to 60° about the z axis, less everything a drill
//! swung into it passes through as it turns rigidly about the z axis from −140° to +30°: the solid
//! of revolution about the upright line through (10, 0): a cylinder of radius 1 from z = −0.8 +
//! 1/tan 59° to 3, ended below by a cone of 118° with its point at (10, 0, −0.8).
//!
//! next.BREP's sweep of a solid builds a turn of a solid of revolution only where both its ends
//! stay clear of the region asked for, and this turn stops inside the block. An upright solid of
//! revolution turned about a parallel upright line passes, by elementary geometry
//! (`solids::swept::upright_turned`), through the tool where it starts and where it stops and its
//! section by the half plane through both axes turned between them: each level of the tool is a
//! disc, which a circle about the z axis meets in one arc about the disc's centre's angle. All of
//! it is cut from the block.
use crate::solids::swept::{ring_sector, upright_turned, Meridian, Piece};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, BooleanOperation, BrepSolid};
#[allow(unused_imports)]
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = ring_sector(8.0, 12.0, -1.0, 1.0, 0.0, 60f64.to_radians())?;
    let tool = Meridian(vec![
        Piece::Line { from: [0.0, -0.8], to: [1.0, -0.8 + 1.0 / 59f64.to_radians().tan()] },
        Piece::Line { from: [1.0, -0.8 + 1.0 / 59f64.to_radians().tan()], to: [1.0, 3.0] },
        Piece::Line { from: [1.0, 3.0], to: [0.0, 3.0] },
    ]);
    let mut operands = vec![block];
    operands.extend(upright_turned(&tool, 10.0, (-140f64).to_radians(), 30f64.to_radians())?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
