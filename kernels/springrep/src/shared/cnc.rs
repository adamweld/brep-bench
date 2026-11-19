//! Helpers for the machining cases. springrep has no sweep of a solid, so what a cutter passes
//! through is built from its other operations, each by elementary geometry:
//!
//! - an upright cutter moved straight up or down sweeps itself at its lowest tip with its side
//!   carried up to its top at the highest: a solid of revolution about its axis;
//! - an upright flat end mill of radius r slid round a closed path in a level plane sweeps
//!   everything within r of the path, from its tip up its length: the ring between the path's
//!   region grown by r and shrunk by r (the kernel's region offsets), extruded;
//! - an upright mill slid along a smooth level path whose curvature stays under 1/r sweeps its
//!   silhouette (its section through its axis) carried along the path square to it.
use std::f64::consts::{FRAC_PI_2, TAU};

use springrep::math::{Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::region::{Op, Region};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::{extrude, revolve};
use springrep::topo::Brep;

use super::{at, e, frame};

/// The box from `lo` to `hi`.
pub fn plate(lo: [f64; 3], hi: [f64; 3]) -> Result<Brep, String> {
    let size = Vec3::new(hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]);
    block(&at(lo), size, "plate").map_err(e)
}

/// An upright cutter's half section through its axis, x out from it and y up from its tip: from
/// the tip out along its end, up its side to `length`, and back in to the axis.
pub enum Cutter {
    /// A flat end mill: a cylinder.
    Flat { radius: f64, length: f64 },
    /// A bull-nose mill: a flat end of radius `radius − corner` rounded into its side.
    BullNose {
        radius: f64,
        corner: f64,
        length: f64,
    },
    /// A drill: a cone of half-angle 45° from its tip to its side.
    Drill { radius: f64, length: f64 },
}

impl Cutter {
    /// The half section with its tip at height 0, carried up `rise` more.
    fn half(&self, rise: f64) -> Profile {
        let p = Vec2::new;
        let line = |a: Vec2, b: Vec2| NamedSegment::line(a, b, "cutter");
        let (radius, length) = match *self {
            Cutter::Flat { radius, length } | Cutter::Drill { radius, length } => (radius, length),
            Cutter::BullNose { radius, length, .. } => (radius, length),
        };
        let top = length + rise;
        let mut segs = match *self {
            Cutter::Flat { .. } => vec![
                line(p(0., 0.), p(radius, 0.)),
                line(p(radius, 0.), p(radius, top)),
            ],
            Cutter::BullNose { corner, .. } => vec![
                line(p(0., 0.), p(radius - corner, 0.)),
                NamedSegment::arc(
                    p(radius - corner, corner),
                    corner,
                    -FRAC_PI_2,
                    FRAC_PI_2,
                    "cutter",
                ),
                line(p(radius, corner), p(radius, top)),
            ],
            Cutter::Drill { .. } => vec![
                line(p(0., 0.), p(radius, radius)),
                line(p(radius, radius), p(radius, top)),
            ],
        };
        segs.push(line(p(radius, top), p(0., top)));
        segs.push(line(p(0., top), p(0., 0.)));
        Profile::outline(segs)
    }

    /// The cutter's section through its axis, both sides, as a profile with sketch x across the
    /// axis and sketch y up from the tip: what it sweeps as it slides square to that plane.
    pub fn silhouette(&self) -> Result<Profile, String> {
        let half = Region::of(&self.half(0.)).map_err(e)?;
        let mut mirrored = self.half(0.);
        for lp in &mut mirrored.loops {
            for s in &mut lp.segments {
                mirror_x(&mut s.segment);
            }
        }
        let other = Region::of(&mirrored).map_err(e)?;
        let both = half.combine(&other, Op::Add).map_err(e)?;
        Ok(both.profiles().map_err(e)?.remove(0))
    }

    /// What the cutter sweeps moving straight up or down at `(x, y)` with its tip between `lo`
    /// and `hi`.
    pub fn plunge(&self, x: f64, y: f64, lo: f64, hi: f64) -> Result<Brep, String> {
        let plane = frame([x, y, lo], [0., -1., 0.], [1., 0., 0.])?;
        let axis = Vec3::new(x, y, lo);
        revolve(&self.half(hi - lo), &plane, axis, Vec3::Z, TAU, "plunge").map_err(e)
    }
}

/// A segment mirrored in the sketch's y axis (its direction reversed by the mirror).
fn mirror_x(s: &mut springrep::profile::Segment) {
    use springrep::profile::Segment;
    let m = |v: Vec2| Vec2::new(-v.x, v.y);
    match s {
        Segment::Line { a, b } => (*a, *b) = (m(*a), m(*b)),
        Segment::Arc {
            center,
            start,
            sweep,
            ..
        } => {
            *center = m(*center);
            *start = std::f64::consts::PI - *start;
            *sweep = -*sweep;
        }
        _ => unreachable!("a cutter's section is lines and arcs"),
    }
}

/// What an upright flat end mill of `radius` and `length` sweeps running once round the
/// boundary of `path` with its tip at height `z`: the ring of points within `radius` of it.
pub fn flat_lap(path: &Region, radius: f64, length: f64, z: f64) -> Result<Vec<Brep>, String> {
    let grown = path.offset(radius).map_err(e)?;
    let shrunk = path.offset(-radius).map_err(e)?;
    let ring = grown.combine(&shrunk, Op::Cut).map_err(e)?;
    ring.profiles()
        .map_err(e)?
        .iter()
        .map(|p| extrude(p, &at([0., 0., z]), length, "lap").map_err(e))
        .collect()
}

/// The region inside the polygon through `corners`, every corner rounded to `round`.
pub fn rounded(corners: &[[f64; 2]], round: f64) -> Result<Region, String> {
    let pts: Vec<Vec2> = corners.iter().map(|c| Vec2::new(c[0], c[1])).collect();
    // edges named a and b by turns, so every corner is where an a meets a b
    let names: Vec<&str> = (0..pts.len())
        .map(|i| if i % 2 == 0 { "a" } else { "b" })
        .collect();
    let mut region = Region::of(&Profile::polygon(&pts, &names)).map_err(e)?;
    region.fillet("a", "b", round).map_err(e)?;
    Ok(region)
}

/// What an upright flat end mill of `radius` and `length` sweeps sliding level from `a` to `b`
/// with its tip at height `z`: the slot of points within `radius` of the segment, extruded.
pub fn flat_slide(
    a: [f64; 2],
    b: [f64; 2],
    radius: f64,
    length: f64,
    z: f64,
) -> Result<Brep, String> {
    let (a, b) = (Vec2::new(a[0], a[1]), Vec2::new(b[0], b[1]));
    let d = b - a;
    let heading = d.y.atan2(d.x);
    let side = Vec2::new(-d.y, d.x) * (radius / d.norm());
    let half = std::f64::consts::PI;
    let slot = Profile::outline(vec![
        NamedSegment::line(a - side, b - side, "slot"),
        NamedSegment::arc(b, radius, heading - FRAC_PI_2, half, "slot"),
        NamedSegment::line(b + side, a + side, "slot"),
        NamedSegment::arc(a, radius, heading + FRAC_PI_2, half, "slot"),
    ]);
    extrude(&slot, &at([0., 0., z]), length, "slide").map_err(e)
}
