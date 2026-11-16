//! The upper half of a ball of radius 5 centred on the origin, the rim where its flat face meets
//! the ball chamfered by a distance and an angle: 1 in from the rim on the flat face, the chamfer
//! rising from there at 45° to the flat toward the ball. The chamfer is the cone that line sweeps
//! about the z axis; the flat shrinks to the disc of radius 4.
//!
//! The hemisphere is a solid of revolution, so it is built as one: its meridian, a region of the
//! (ρ, z) half-plane, cut at its corner and turned once about the z axis. cad-kernel's chamfers
//! take two legs, each a distance (`corners::Leg`), and no angle; so the corner is cut here by the
//! kernel's own walk of a meridian's corners (`corners::corners`) with a flat given by a distance
//! and an angle (`by_distance_and_angle`), where the line meets the other face found by the
//! kernel's meeting of the two carriers. The other chamfers by a distance and an angle of a turned
//! solid's rim use it too.
use cad_kernel::build::revolve;
use cad_kernel::corners::{corners, Carrier, Cut};
use cad_kernel::planar::{Region, Seg, Step};
use cad_kernel::plane2::{add, cross, dot, scale, sub, unit};
use cad_kernel::recipe::region_profile;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, TAU};

const RADIUS: f64 = 5.;
const DISTANCE: f64 = 1.;
const ANGLE: f64 = 45.;

// the faces of the meridian, by tag (0 is none)
const FLAT: u32 = 1;
const BALL: u32 = 2;
const AXIS: u32 = 3;
const CHAMFER: u32 = 4;

pub fn build() -> Result<Brep, String> {
    hemisphere(DISTANCE, ANGLE)
}

/// The hemisphere with its rim chamfered `distance` in on the flat face and at `angle` degrees to
/// it.
pub fn hemisphere(distance: f64, angle: f64) -> Result<Brep, String> {
    // the meridian, counter-clockwise: out along the flat face, up round the ball's quarter arc,
    // and back down the axis
    let step = |seg, tag| Step { seg, chart: None, tag };
    let half = Region::new(vec![vec![
        step(Seg::Line { a: [0., 0.], b: [RADIUS, 0.] }, FLAT),
        step(Seg::Arc { c: [0., 0.], r: RADIUS, a0: 0., sweep: FRAC_PI_2 }, BALL),
        step(Seg::Line { a: [0., RADIUS], b: [0., 0.] }, AXIS),
    ]]);
    let cut = by_distance_and_angle(&half, FLAT, BALL, distance, angle.to_radians(), CHAMFER)?;
    turned(&cut)
}

/// The meridian `region` turned once about the z axis, from the half-plane of x and z.
pub fn turned(region: &Region) -> Result<Brep, String> {
    let (origin, z) = ([0., 0., 0.], [0., 0., 1.]);
    revolve(&region_profile(region, origin, [1., 0., 0.], z, &|_| String::new()), origin, z, TAU)
}

/// `region` with each corner where the straight face `a` meets the face `b` cut by a distance and
/// an angle: from the point `distance` along `a` from the corner, the line making `angle` (in
/// radians) with `a`'s line back toward the corner, turned toward the side `b` leaves the corner
/// on, to where it first meets `b`'s line or circle. The cut is tagged `tag`.
pub fn by_distance_and_angle(region: &Region, a: u32, b: u32, distance: f64, angle: f64, tag: u32)
    -> Result<Region, String> {
    corners(region, &[a], &[b], tag, &|s: &Step, n: &Step, _side: f64| -> Result<Cut, String> {
        // which of the two steps is `a`, and whether it arrives at the corner or leaves it
        let arrives = s.tag == a;
        let (on, other) = if arrives { (s, n) } else { (n, s) };
        if !matches!(on.seg, Seg::Line { .. }) {
            return Err("a chamfer's angle is measured from a straight face".into());
        }
        let corner = s.seg.end();
        // each face's way away from the corner
        let back = |t: [f64; 2]| scale(t, -1.);
        let (from_on, from_other) = if arrives {
            (back(s.seg.tangent(1.)), n.seg.tangent(0.))
        } else {
            (n.seg.tangent(0.), back(s.seg.tangent(1.)))
        };
        // the chamfer's start, and the way from it back toward the corner turned by the angle
        // toward the other face
        let start = add(corner, scale(unit(from_on), distance));
        let toward = unit(back(from_on));
        let turn = cross(toward, from_other).signum() * angle;
        let (sin, cos) = turn.sin_cos();
        let way = [cos * toward[0] - sin * toward[1], sin * toward[0] + cos * toward[1]];
        // where the line first meets the other face's carrier, ahead of the start
        let end = Carrier::Line { p: start, d: way }.meet(Carrier::of(&other.seg)).into_iter()
            .filter(|&q| dot(sub(q, start), way) > 0.)
            .min_by(|p, q| dot(sub(*p, start), way).total_cmp(&dot(sub(*q, start), way)))
            .ok_or("the chamfer's line never meets the other face")?;
        let (t_on, t_other) = (on.seg.param(start), other.seg.param(end));
        Ok(if arrives {
            (t_on, t_other, Seg::Line { a: start, b: end })
        } else {
            (t_other, t_on, Seg::Line { a: end, b: start })
        })
    })
}
