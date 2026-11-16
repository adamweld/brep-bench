//! The upper half of a ball of radius 5 centred on the origin, the rim where its flat face meets
//! the ball chamfered with two distances: 5 across the flat face, the whole way to its centre, and
//! 1 on the ball, each a straight line from the edge in the plane through the z axis. The flat
//! shrinks to a point, and the solid is the ball inside the cone from the origin to the circle 1
//! from the rim: a spherical sector.
//!
//! The hemisphere is a solid of revolution, so it is built as one: its meridian, a region of the
//! (ρ, z) half-plane, chamfered at its corner and turned once about the z axis. The kernel's
//! chamfer of a meridian's corners (`corners::chamfer`) splices its flat in between what is left of
//! the two faces, and declines where nothing of one is left ("the cuts at both ends of face 1 meet
//! within it"); so the corner's flat is taken from the kernel as `corners::flat` makes it, its leg
//! a straight chord from the corner (`Leg::FromEdge`), and the piece between it and the corner is
//! taken off by a planar Boolean, which drops the face it consumes (`chamfered_at`). The other
//! chamfers that consume a face of a turned solid use it too.
use super::hemisphere_rim_distance_1_angle_45::turned;
use cad_kernel::corners::{flat, Leg};
use cad_kernel::planar::{boolean, Op, Region, Seg, Step};
use cad_kernel::plane2::{cross, dist, unit};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;

const RADIUS: f64 = 5.;
const ON_FLAT: f64 = RADIUS;
const ON_BALL: f64 = 1.;

// the faces of the meridian, by tag (0 is none)
const FLAT: u32 = 1;
const BALL: u32 = 2;
const AXIS: u32 = 3;
const CHAMFER: u32 = 4;

pub fn build() -> Result<Brep, String> {
    // the meridian, counter-clockwise: out along the flat face, up round the ball's quarter arc,
    // and back down the axis
    let step = |seg, tag| Step { seg, chart: None, tag };
    let half = Region::new(vec![vec![
        step(Seg::Line { a: [0., 0.], b: [RADIUS, 0.] }, FLAT),
        step(Seg::Arc { c: [0., 0.], r: RADIUS, a0: 0., sweep: FRAC_PI_2 }, BALL),
        step(Seg::Line { a: [0., RADIUS], b: [0., 0.] }, AXIS),
    ]]);
    // the rim, the corner at (5, 0), cut by the chord between the flat's centre, 5 from it, and
    // the point 1 from it on the ball
    let legs = [(FLAT, Leg::FromEdge(ON_FLAT)), (BALL, Leg::FromEdge(ON_BALL))];
    let cut = chamfered_at(&half, [RADIUS, 0.], legs, CHAMFER)?;
    // turned once about z
    turned(&cut)
}

/// `region` with its corner at `at` chamfered, each leg given with the tag of the face it is on:
/// the flat `corners::flat` cuts across the corner, and the piece between the flat and the corner,
/// bounded by the two faces as far as the legs, taken off where the corner is convex and added
/// where it is concave. The flat is tagged `tag`.
pub fn chamfered_at(region: &Region, at: [f64; 2], legs: [(u32, Leg); 2], tag: u32)
    -> Result<Region, String> {
    let size = region.loops.iter().flatten().map(|s| s.seg.start())
        .fold(0., |m: f64, p| m.max(p[0].abs()).max(p[1].abs()));
    let tol = 1e-9 * size;
    // the step arriving at the corner and the one leaving it
    let (s, n) = region.loops.iter()
        .find_map(|l| (0..l.len()).find(|&i| dist(l[i].seg.end(), at) <= tol)
            .map(|i| (l[i], l[(i + 1) % l.len()])))
        .ok_or("no corner of the meridian there")?;
    let leg = |on: &Step| legs.iter().find(|(t, _)| *t == on.tag).map(|&(_, l)| l)
        .ok_or_else(|| format!("no leg given on face {}", on.tag));
    // which way the boundary turns there: left (its material on the left) where it is convex
    let side = cross(unit(s.seg.tangent(1.)), unit(n.seg.tangent(0.))).signum();
    let (t_s, t_n, across) = flat(&s.seg, &n.seg, side, [leg(&s)?, leg(&n)?])?;
    let piece = Region::new(vec![vec![
        Step { seg: s.seg.part(t_s, 1.), ..s },
        Step { seg: n.seg.part(0., t_n), ..n },
        Step { seg: across.reversed(), chart: None, tag },
    ]]);
    boolean(region, &piece, if side > 0. { Op::Cut } else { Op::Union }, tol)
}
