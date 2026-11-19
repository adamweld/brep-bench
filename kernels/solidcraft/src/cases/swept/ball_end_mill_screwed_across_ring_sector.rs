//! A sector of a ring about the z axis (radii 8 to 12, z from −2 to 2, polar angles −30° to 30°)
//! with a helical groove milled by a ball-end mill about the upright line through (10, 0): the
//! lower half of the ball of radius 1 about (10, 0, 0) under the cylinder of radius 1 from z = 0
//! to z = 3, carried along the screw about the z axis that rises 3 a turn, from a turn of −60° to
//! one of +60°.
//!
//! solidcraft-kernel has no sweep of a solid, so the mill's sweep is built by elementary geometry
//! from the kernel's helical sweep of planar regions (`sweep_helix`). Below its flat top, the
//! mill is every point within 1 of the upright segment from its ball's centre c to 3 above it;
//! screwed, those segments fill the strip of the cylinder of radius 10 from c's helix to 3 above
//! it, and the mill's top stays above the block. A point within 1 of the strip lies between radii
//! 9 and 11, and is within 1 either of the point of the cylinder straight out from the axis
//! through it, when that lies in the strip, or of the strip's edge, which in the block is c's
//! helix (the strip's top and ends stay clear of it). So in the block the sweep is the pipe of
//! radius 1 about c's helix, the ball's great disc square to it carried along it (the helix turns
//! no tighter than radius 10, so the pipe does not fold), with what lies between radii 9 and 11
//! from that helix to 3 above it: the screw sweep of the mill's section, from z = 0 to z = 3, in
//! the plane y = 0 through both axes.
use super::square_bar_screwed_through_rod::{screw, screwed};
use crate::solids::*;
use solidcraft_kernel::revolve;
use std::f64::consts::TAU;

pub fn build() -> R<Body> {
    let (lead, start, stop) = (3.0, -60.0, 60.0);
    // the sector: its section from radius 8 to 12 turned from −30° through 60°
    let sector = screwed(&xz(), 0.0, -30.0);
    let mut block = one(revolve(&sector, &[rect(8.0, -2.0, 12.0, 2.0)], Vec2::ZERO, Vec2::Y, -60f64.to_radians())?)?;
    // the ball's great disc square to its centre's helix where the screw starts it at (10, 0, 0)
    let along = v3(0.0, TAU * 10.0, lead);
    let disc_plane = Plane::from_normal(v3(10.0, 0.0, 0.0), along)
        .ok_or_else(|| KernelError::Invalid("the disc's plane".into()))?;
    block = cut(&block, &screw(&disc_plane, &disc((0.0, 0.0), 1.0), lead, start, stop)?)?;
    block = cut(&block, &screw(&xz(), &rect(9.0, 0.0, 11.0, 3.0), lead, start, stop)?)?;
    Ok(block)
}
