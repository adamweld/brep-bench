//! The solid a ring fills as it slides 6 along x, square to its axis, less than its hole's width:
//! the torus about the z axis centred on the origin, the circle through its tube's centres of
//! radius 6, its tube of radius 2.
//!
//! next.BREP has no sweep of a solid along a line. The ring's sweep follows by elementary geometry,
//! level by level. The plane z = h (|h| < 2) cuts the ring in an annulus between the circles of
//! radius 6 − w and 6 + w about the ring's axis, w = √(4 − h²). Slid without turning, the outer
//! circle's disc sweeps a stadium, its two end discs joined by their common tangents; the disc
//! of the hole is left uncovered only where every position of it overlaps, and since a point's
//! distance from a centre moving along a line is convex in the centre's place, that is the lens
//! the hole's discs at the start and at the stop share. So the sweep is what `torus_slid_up` makes
//! of a ring slid farther than its hole (the ring with its hole filled at the start and at the stop,
//! and between them its outline seen along the slide), less the lens of the two holes: the solid
//! of revolution the tube's inner half bounds with the ring's axis, at the start and at the stop,
//! in common. The hole is 2·(6 − w) ≥ 8 wide at every level, more than the slide, so the lens
//! never empties and its walls are the ring's inner halves at those two places, meeting along two
//! curves. `torus_slid_up` slides a wheel whose axis is along y up the z axis; the rotation (x,
//! y, z) ↦ (z, x, y) carries its axis onto z and its slide onto x.
use crate::solids::boolean::{common, cut, union};
use crate::solids::p;
use crate::solids::sweep::{moved, torus_slid_up};
use crate::Refusal;
use brep_kernel::{make_arc, make_line, revolve_profile_brep, AffineTransform, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (major, minor, slide) = (6.0, 2.0, 6.0);
    let onto = AffineTransform::new([
        0.0, 0.0, 1.0, 0.0, //
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ])?;
    let mut parts = Vec::new();
    for part in torus_slid_up(p(0.0, 0.0, 0.0), major, minor, slide)? {
        parts.push(moved(&part, onto)?);
    }
    let mut whole = parts[0].clone();
    for part in &parts[1..] {
        whole = union(&whole, part)?;
    }

    // The hole at the start, in the ring's own frame (axis along z): the revolve about z of the
    // region between the axis and the tube's inner half, |z| ≤ 2 (the arc runs from the tube's
    // bottom through its innermost point to its top).
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let at = |r: f64, h: f64| p(r, 0.0, h);
    let meridian = vec![
        make_line(at(0.0, minor), at(0.0, -minor))?,
        make_line(at(0.0, -minor), at(major, -minor))?,
        make_arc(at(major, 0.0), x, p(0.0, 0.0, -1.0), minor, FRAC_PI_2, 3.0 * FRAC_PI_2)?,
        make_line(at(major, minor), at(0.0, minor))?,
    ];
    let hole = revolve_profile_brep(&meridian, p(0.0, 0.0, 0.0), z, TAU)?;
    let slid = AffineTransform::new([
        1.0, 0.0, 0.0, slide, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ])?;
    let lens = common(&hole, &moved(&hole, slid)?)?;
    Ok(cut(&whole, &lens)?)
}
