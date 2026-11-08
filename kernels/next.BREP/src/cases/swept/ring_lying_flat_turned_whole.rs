//! Everything a ring passes through as it turns a whole turn about the z axis: the torus about the
//! upright line through (8, 0), the circle through its tube's centres of radius 3 in z = 0, its
//! tube of radius 1.
//!
//! next.BREP's sweep of a solid builds a turn only where both its ends stay clear of the region
//! asked for, and a whole turn starts and stops on the ring. A whole turn about the z axis sweeps
//! exactly the solid of revolution of the distances from the axis the ring reaches in each level.
//! At height z the ring is the annulus of radii 3 ∓ √(1 − z²) about (8, 0), a connected set that
//! reaches every distance from the z axis between 5 − √(1 − z²) and 11 + √(1 − z²): its section is
//! the stadium |z| ≤ 1 between 5 and 11 out, ended by half discs of radius 1, which the kernel
//! revolves a whole turn.
use crate::solids::p;
use crate::solids::swept::{planar, Piece};
use crate::Refusal;
use brep_kernel::{revolve_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let stadium = planar(&[
        Piece::Line { from: [5.0, -1.0], to: [11.0, -1.0] },
        Piece::Arc { centre: [11.0, 0.0], radius: 1.0, start: -FRAC_PI_2, end: FRAC_PI_2 },
        Piece::Line { from: [11.0, 1.0], to: [5.0, 1.0] },
        Piece::Arc { centre: [5.0, 0.0], radius: 1.0, start: FRAC_PI_2, end: 3.0 * FRAC_PI_2 },
    ], p(0.0, 0.0, 0.0), x, z)?;
    Ok(revolve_profile_brep(&stadium, p(0.0, 0.0, 0.0), z, TAU)?)
}
