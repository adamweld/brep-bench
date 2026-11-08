//! The block from (0, −5, 0) to (20, 5, 10), less everything a V wheel passes through as it slides
//! without turning by 36 along x: the solid of revolution, about the line through (−8, 0, 14)
//! along y, of the triangle with corners (y, z) = (0, 8) and (±3 tan 30°, 11).
//!
//! next.BREP has no sweep of a solid along a line. By elementary geometry: the wheel lies within 6
//! of its axis along x, so it is clear of the block where it starts (x ≤ −2) and where it stops
//! (x ≥ 22), and a point of the block is passed exactly when the line through it along x meets the
//! wheel (the wheel's point on that line, at most 6 from the axis, reaches the block's point at
//! some place of the slide between them). In the plane y = c the wheel is the annulus of radii 3 to
//! 6 − √3|c| about the axis (|c| ≤ √3), whose shadow along x is |z − 14| ≤ 6 − √3|c|, so the block
//! loses its part inside the hexagon with corners (y, z) = (0, 8), (±√3, 11), (±√3, 17), (0, 20)
//! extruded along x: the kernel's extrude, cut from the block.
use crate::solids::fillet::subtract;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 10.0)?;
    let r3 = 3f64.sqrt();
    let outline = polygon(&[p(-8.0, 0.0, 8.0), p(-8.0, r3, 11.0), p(-8.0, r3, 17.0),
        p(-8.0, 0.0, 20.0), p(-8.0, -r3, 17.0), p(-8.0, -r3, 11.0)])?;
    let shadow = extrude_profile_brep(&outline, p(1.0, 0.0, 0.0), 36.0)?;
    Ok(subtract(&block, &shadow)?)
}
