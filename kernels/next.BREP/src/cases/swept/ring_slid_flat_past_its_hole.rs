//! Everything a ring passes through as it slides without turning by 15 along x: the torus about the
//! z axis centred on the origin, the circle through its tube's centres of radius 6, its tube of
//! radius 2.
//!
//! next.BREP has no sweep of a solid along a line. A wheel slid square to its axis farther than its
//! hole is wide follows by elementary geometry (`solids::sweep::torus_slid_up`, proved there): the
//! wheel with its hole filled where it starts and where it stops, and between them its outline
//! seen along the slide slid from the start to the stop. That helper slides a wheel whose axis is
//! along y up the z axis; the rotation (x, y, z) ↦ (z, x, y) carries its axis onto z and its slide
//! onto x.
use crate::solids::p;
use crate::solids::sweep::{moved, torus_slid_up};
use crate::Refusal;
use brep_kernel::{AffineTransform, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let onto = AffineTransform::new([
        0.0, 0.0, 1.0, 0.0, //
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ])?;
    let mut parts = Vec::new();
    for part in torus_slid_up(p(0.0, 0.0, 0.0), 6.0, 2.0, 15.0)? {
        parts.push(moved(&part, onto)?);
    }
    let mut whole = parts[0].clone();
    for part in &parts[1..] {
        whole = crate::solids::fillet::union(&whole, part)?;
    }
    Ok(whole)
}
