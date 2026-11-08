//! The block from (0, −10, −10) to (30, 10, 0), less everything a V bit passes through as it slides
//! without turning along x by 15: the cone about the upright line through (5, 0) with its point at
//! (5, 0, −2), opening at 90° to a flat top of radius 5 at z = 3.
//!
//! next.BREP has no sweep of a solid along a line. An upright solid of revolution slid along a
//! level line passes, by elementary geometry (`solids::swept::upright_slid`), through the tool where
//! it starts and where it stops and its section through its axis, square to the slide, slid between
//! them: here the triangle |y| ≤ z + 2, −2 ≤ z ≤ 3, extruded 15 along x. All of it is cut from the
//! block.
use crate::solids::p;
use crate::solids::swept::{upright_slid, Meridian, Piece};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_box_brep, BooleanOperation, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, -10.0, -10.0), 30.0, 20.0, 10.0)?;
    let bit = Meridian(vec![
        Piece::Line { from: [0.0, -2.0], to: [5.0, 3.0] },
        Piece::Line { from: [5.0, 3.0], to: [0.0, 3.0] },
    ]);
    let mut operands = Vec::new();
    operands.extend(upright_slid(&bit, p(5.0, 0.0, 0.0), p(15.0, 0.0, 0.0))?);
    let swept = boolean_operation_nary(&operands, BooleanOperation::Union)?;
    Ok(crate::solids::fillet::subtract(&block, &swept)?)
}
