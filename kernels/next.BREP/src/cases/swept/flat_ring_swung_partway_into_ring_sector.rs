//! The ring sector 8 ≤ ρ ≤ 12, −1 ≤ z ≤ 1, 0° to 60° about the z axis, less everything a ring lying
//! flat passes through as it turns rigidly about the z axis from −140° to +30°: the torus about the
//! upright line through (10, 0), the circle through its tube's centres of radius 1 in z = 0, its
//! tube of radius 0.4.
//!
//! next.BREP's sweep of a solid builds a turn of a solid of revolution only where both its ends
//! stay clear of the region asked for, and this turn stops inside the block. By elementary
//! geometry: in each level the ring is an annulus about (10, 0), which a circle about the z axis
//! meets in the angles α with k₀ ≤ cos α ≤ k₁: one arc [−b, b], or two, [−b, −a] and [a, b]. Turned
//! from θ₀ to θ₁ the two arcs cover [θ₀ − b, θ₁ − a] and [θ₀ + a, θ₁ + b], which join into
//! [θ₀ − b, θ₁ + b] because the turn, 170°, is more than 2a (a < b < asin(1.4/8.6) < 10°): the
//! ring sweeps exactly what the ring with its hole filled (the disc of the outer radius in each
//! level) sweeps. That filled ring is an upright solid of revolution, whose turn is the filled ring
//! where it starts and where it stops and its section turned between them
//! (`solids::swept::upright_turned`). All of it is cut from the block.
use crate::solids::swept::{ring_sector, upright_turned, Meridian, Piece};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, BooleanOperation, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = ring_sector(8.0, 12.0, -1.0, 1.0, 0.0, 60f64.to_radians())?;
    let filled = Meridian(vec![
        Piece::Line { from: [0.0, -0.4], to: [1.0, -0.4] },
        Piece::Arc { centre: [1.0, 0.0], radius: 0.4, start: -FRAC_PI_2, end: FRAC_PI_2 },
        Piece::Line { from: [1.0, 0.4], to: [0.0, 0.4] },
    ]);
    let mut operands = vec![block];
    operands.extend(upright_turned(&filled, 10.0, (-140f64).to_radians(), 30f64.to_radians())?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
