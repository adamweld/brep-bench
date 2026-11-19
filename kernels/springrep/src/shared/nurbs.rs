//! Helpers for the nurbs cases: the block under the bicubic Bézier dome, and the block under the
//! weighted barrel vault.
use springrep::math::{Frame, Vec2};
use springrep::primitive::slab;
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use super::{at, e};

/// The block on the square 0 ≤ x, y ≤ 20 under the bicubic Bézier dome: a slab, whose top is the
/// B-spline height field with its control points over its knots' Greville abscissae, which for a
/// Bézier are the even grid 0, ⅓, ⅔, 1 the dome's control points stand on.
pub fn bezier_dome() -> Result<Brep, String> {
    let bezier = [0., 0., 0., 0., 1., 1., 1., 1.];
    let heights = vec![
        vec![4., 6., 7., 5.],
        vec![6., 14., 12., 7.],
        vec![5., 13., 16., 8.],
        vec![3., 6., 8., 6.],
    ];
    slab(
        &at([0., 0., 0.]),
        [20., 20.],
        (&bezier, &bezier),
        &heights,
        "dome",
    )
    .map_err(e)
}

/// The block on −12 ≤ x, y ≤ 12 under the barrel vault that arches across the sketch's x: its
/// section, the floor, the sides and the rational quadratic arc (−12, 10), (0, 26), (12, 10) with
/// weights 1, 3/5, 1, extruded 24 along `frame`'s normal from its origin.
pub fn vault(frame: &Frame) -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let arch = Segment::Bezier {
        ctrl: vec![p(12., 10.), p(0., 26.), p(-12., 10.)],
        weights: vec![1., 0.6, 1.],
    };
    let section = Profile::outline(vec![
        NamedSegment::line(p(-12., 0.), p(12., 0.), "floor"),
        NamedSegment::line(p(12., 0.), p(12., 10.), "side"),
        NamedSegment::new(arch, "vault"),
        NamedSegment::line(p(-12., 10.), p(-12., 0.), "side"),
    ]);
    extrude(&section, frame, 24., "vault").map_err(e)
}
