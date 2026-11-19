//! Helpers the tangent cases share: a block between two corners, a rod along any axis, a block
//! standing for a half-space, and the solid paraboloid of revolution, the parabola turned about z.
use std::f64::consts::TAU;

use springrep::math::{Frame, Vec2, Vec3};
use springrep::primitive::{block, cylinder};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

/// The block between the corners `lo` and `hi`, its edges along the world's axes.
pub fn between(lo: [f64; 3], hi: [f64; 3], name: &str) -> Result<Brep, String> {
    let size = Vec3::from_array(hi) - Vec3::from_array(lo);
    block(&at(lo), size, name).map_err(e)
}

/// The rod of radius `r` from `base` along `axis` for `length`, its seam toward `seam`.
pub fn rod(
    base: [f64; 3],
    axis: [f64; 3],
    seam: [f64; 3],
    r: f64,
    length: f64,
    name: &str,
) -> Result<Brep, String> {
    cylinder(&frame(base, axis, seam)?, r, length, name).map_err(e)
}

/// The half-space on the side `normal` points to of the plane through `point`, as a cube of side
/// `2·reach` standing on the plane, centred over `point`: `reach` must take it past the other
/// solid on every side but the plane.
pub fn half_space(
    point: [f64; 3],
    normal: [f64; 3],
    reach: f64,
    name: &str,
) -> Result<Brep, String> {
    let n = Vec3::from_array(normal);
    let plane = Frame::from_axis(Vec3::from_array(point), n).map_err(e)?;
    let corner = plane.to_world(Vec3::new(-reach, -reach, 0.));
    let base = Frame::new(corner, plane.z, plane.x).map_err(e)?;
    block(&base, Vec3::new(2. * reach, 2. * reach, reach), name).map_err(e)
}

/// The solid paraboloid x² + y² ≤ 4f·z, z ≤ `top`, about the z axis from its vertex at the
/// origin: the parabola of the xz plane, from the vertex to the rim, turned once about z.
pub fn paraboloid(f: f64, top: f64, name: &str) -> Result<Brep, String> {
    let rim = (4. * f * top).sqrt();
    // the parabola's arc is the quadratic Bézier on its end tangents, which meet halfway out under
    // the rim; weights of one make it a parabola exactly
    let side = Segment::Bezier {
        ctrl: vec![
            Vec2::new(0., 0.),
            Vec2::new(rim / 2., 0.),
            Vec2::new(rim, top),
        ],
        weights: vec![1., 1., 1.],
    };
    let meridian = Profile::outline(vec![
        NamedSegment::new(side, "side"),
        NamedSegment::line(Vec2::new(rim, top), Vec2::new(0., top), "top"),
        NamedSegment::line(Vec2::new(0., top), Vec2::new(0., 0.), "axis"),
    ]);
    // the sketch plane is the xz plane: its x the world's x, its y the world's z
    let plane = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&meridian, &plane, Vec3::ZERO, Vec3::Z, TAU, name).map_err(e)
}
