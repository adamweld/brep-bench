//! A cylinder of radius 3 about the z axis from z = −10 up to the plane z = x·tan 45°, grown by 1:
//! every point within 1 of it, its bottom rim a piece of a torus and its elliptic top rim a tube
//! about the ellipse, the kernel's offset with its convex edges rounded.
use crate::bench::{cut, rotated};
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

/// The cylinder of radius 3 from z = −10, cut off by the plane z = x·tan `degrees`: it runs up
/// past the plane, and the half-space above the plane, a block on z > 0 turned −`degrees` about
/// the y axis (its face's normal then (−sin, 0, cos)), is cut from it.
pub fn slant_cut_cylinder(degrees: f64) -> Result<Body, Refusal> {
    let top = 3.0 * degrees.to_radians().tan();
    let cylinder = make_cylinder([0.0, 0.0, -10.0], 3.0, 11.0 + top)?;
    let above = make_box([-30.0, -30.0, 0.0], [60.0, 60.0, 30.0])?;
    cut(cylinder, rotated(above, [0.0; 3], [0.0, 1.0, 0.0], -degrees)?)
}

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&slant_cut_cylinder(45.0)?, 1.0)
}
