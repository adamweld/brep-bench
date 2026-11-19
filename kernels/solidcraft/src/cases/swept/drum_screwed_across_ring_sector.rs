//! A sector of a ring about the z axis (radii 8 to 12, z from −1 to 1, polar angles 0° to 60°)
//! with a helical groove milled by a drum, the cylinder of radius 1 about the upright line through
//! (10, 0) from z = −0.5 to z = 0.5, carried along the screw about the z axis that rises 0.5 a
//! turn, from a turn of −140° to one of +140°.
//!
//! solidcraft-kernel has no sweep of a solid, so the drum's sweep is built by elementary geometry
//! from the kernel's helical sweep of planar regions (`sweep_helix`). The drum's side meets the
//! screw's motion square-on along its two lines in the plane y = 0, through both axes. Split by
//! that plane, the half on y ≥ 0 moves away from a point only through its bottom and the cut (the
//! screw turns from +x toward +y), so a point it passes over and leaves before the end is left
//! through one of those; the half on y ≤ 0 likewise first reaches a point through its top or the
//! cut. So the drum's sweep is the drum where it starts and where it ends, clear of the block, with
//! the screw sweeps of its section by y = 0 and its two ends, each of which crosses the motion,
//! never moving along itself.
use super::square_bar_screwed_through_rod::screw;
use crate::solids::*;

pub fn build() -> R<Body> {
    let (lead, start, stop) = (0.5, -140.0, 140.0);
    let mut block = revolve_z(rect(8.0, -1.0, 12.0, 1.0), 60.0)?;
    block = cut(&block, &screw(&xz(), &rect(9.0, -0.5, 11.0, 0.5), lead, start, stop)?)?;
    for z in [0.5, -0.5] {
        let end = Plane { origin: v3(0.0, 0.0, z), x: Vec3::X, y: Vec3::Y };
        block = cut(&block, &screw(&end, &disc((10.0, 0.0), 1.0), lead, start, stop)?)?;
    }
    Ok(block)
}
