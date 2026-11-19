//! Helpers shared by the offset cases: the inward offset springrep's `offset` declines, made from
//! its outward one, and the meridian plane the turned solids are drawn on.
use std::f64::consts::{FRAC_PI_2, TAU};

use springrep::blend::fillet;
use springrep::boolean::{difference, split, union};
use springrep::edit::offset;
use springrep::math::{Frame, Vec2, Vec3};
use springrep::measure;
use springrep::primitive::{block, cone, cylinder, sphere};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::{extrude, revolve};
use springrep::topo::Brep;

use crate::shared::geometry::ellipse;
use crate::shared::{at, e, frame};

/// `s` grown by `d`: every point within `d` of it, springrep's own offset (convex edges rounded,
/// concave ones run on until their faces meet, faces that close on the way followed there).
pub fn grown(s: &Brep, d: f64) -> Result<Brep, String> {
    offset(s, d, "grown").map_err(e)
}

/// `s` shrunk by `d`: every point of `s` at least `d` from its outside. springrep's `offset`
/// refuses an inward distance, but the points of `s` within `d` of its outside are the points of
/// `s` that its complement grown by `d` covers. So the complement, a block reaching well past `s`
/// with `s` taken out of it (a solid with a cavity), is grown by `d`, and taken from `s`: the
/// cavity's faces move into `s`, where `s`'s concave edges, the complement's convex ones, are
/// rounded and its convex edges stay sharp, as an inward offset makes them.
pub fn shrunk(s: &Brep, d: f64) -> Result<Brep, String> {
    let bounds = measure::bbox(s);
    let margin = 2. * d + 1.;
    let pad = Vec3::new(margin, margin, margin);
    let corner = bounds.min - pad;
    let size = bounds.max - bounds.min + pad * 2.;
    let around = block(&at(corner.to_array()), size, "around").map_err(e)?;
    let outside = difference(&around, s).map_err(e)?;
    let near = offset(&outside, d, "near").map_err(e)?;
    difference(s, &near).map_err(e)
}

/// The xz plane as a sketch plane: sketch x along world x, sketch y along world z, so a meridian
/// drawn on it with x as the distance from the z axis turns about z.
pub fn xz() -> Result<Frame, String> {
    frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])
}

/// The cylinder of radius 3 about the z axis from its flat bottom at z = −10 up to where the plane
/// z = x·tan α cuts it (α in degrees): the cylinder split by that plane, the part below kept.
pub fn slant_cut_cylinder(degrees: f64) -> Result<Brep, String> {
    let a = degrees.to_radians();
    let drum = cylinder(&at([0., 0., -10.]), 3., 20., "drum").map_err(e)?;
    // the plane's normal leans back from +z; the part on its side comes first
    let up = Vec3::new(-a.sin(), 0., a.cos());
    let (_, below) = split(&drum, Vec3::ZERO, up).map_err(e)?;
    Ok(below)
}

/// The ellipse x²/25 + y²/9 ≤ 1 in the plane z = 0, extruded up to z = 10.
pub fn elliptic_cylinder() -> Result<Brep, String> {
    let section = Profile::outline(ellipse(Vec2::ZERO, 5., 3., "side"));
    extrude(&section, &at([0., 0., 0.]), 10., "drum").map_err(e)
}

/// The L-shaped plate on the outline (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), extruded
/// 8 along +z.
pub fn l_block() -> Result<Brep, String> {
    let outline = [
        (0., 0.),
        (20., 0.),
        (20., 5.),
        (5., 5.),
        (5., 15.),
        (0., 15.),
    ]
    .map(|(x, y)| Vec2::new(x, y));
    let section = Profile::polygon(&outline, &["wall"; 6]);
    extrude(&section, &at([0., 0., 0.]), 8., "plate").map_err(e)
}

/// The cone on the z axis with its base of radius 6 at z = 0 and its apex at z = 12, its tip
/// rounded by a ball of radius 1 tangent to the side: the meridian (the base, the side up to the
/// circle where the ball touches it, the ball's arc to the axis) turned about z.
pub fn ball_tipped_cone() -> Result<Brep, String> {
    let s5 = 5f64.sqrt();
    // the side leans in 1 for every 2 it rises; the ball, of radius 1, sits on the axis √5 below
    // the apex and touches the side at the point 1 from its centre along the side's normal (2,
    // 1)/√5
    let centre = Vec2::new(0., 12. - s5);
    let touch = centre + Vec2::new(2., 1.) * (1. / s5);
    let meridian = Profile::outline(vec![
        NamedSegment::line(Vec2::new(0., 0.), Vec2::new(6., 0.), "base"),
        NamedSegment::line(Vec2::new(6., 0.), touch, "side"),
        NamedSegment::arc(
            centre,
            1.,
            (1f64 / 2.).atan(),
            FRAC_PI_2 - (1f64 / 2.).atan(),
            "tip",
        ),
        NamedSegment::line(centre + Vec2::new(0., 1.), Vec2::new(0., 0.), "axis"),
    ]);
    revolve(&meridian, &xz()?, Vec3::ZERO, Vec3::Z, TAU, "cone").map_err(e)
}

/// The cone frustum from radius 6 at z = 0 to radius 3 at z = 6, both its rims rounded to 1.5.
pub fn rounded_frustum() -> Result<Brep, String> {
    let frustum = cone(&at([0., 0., 0.]), 6., 3., 6., "frustum").map_err(e)?;
    let rims = [
        ("frustum.top", "frustum.side"),
        ("frustum.bottom", "frustum.side"),
    ];
    fillet(&frustum, &rims, 1.5, "round").map_err(e)
}

/// The cylinder of radius 5 from z = 0 to z = 6 topped by the frustum from radius 5 at z = 6 to
/// radius 2 at z = 10, joined, the shoulder between them rounded to radius 1.
pub fn turned_post() -> Result<Brep, String> {
    let shank = cylinder(&at([0., 0., 0.]), 5., 6., "shank").map_err(e)?;
    let tip = cone(&at([0., 0., 6.]), 5., 2., 4., "tip").map_err(e)?;
    let post = union(&shank, &tip).map_err(e)?;
    fillet(&post, &[("shank.side", "tip.side")], 1., "shoulder").map_err(e)
}

/// The plate of radius 8 from z = −2 to z = 0 and the ball of radius 3 about (0, 0, 1) standing in
/// it, joined, the concave edge where the ball meets the plate's top filleted to `radius`.
pub fn ball_on_plate(radius: f64) -> Result<Brep, String> {
    let plate = cylinder(&at([0., 0., -2.]), 8., 2., "plate").map_err(e)?;
    let ball = sphere(&at([0., 0., 1.]), 3., "ball").map_err(e)?;
    let joined = union(&plate, &ball).map_err(e)?;
    fillet(&joined, &[("plate.end", "ball.skin")], radius, "root").map_err(e)
}

/// The plate of radius 8 from z = −2 to z = 0 and the boss on it, the frustum from radius 4 at
/// z = 0 to radius 1 at z = 6, joined, the concave edge at the boss's foot filleted to `radius`.
pub fn cone_boss(radius: f64) -> Result<Brep, String> {
    let plate = cylinder(&at([0., 0., -2.]), 8., 2., "plate").map_err(e)?;
    let boss = cone(&at([0., 0., 0.]), 4., 1., 6., "boss").map_err(e)?;
    let joined = union(&plate, &boss).map_err(e)?;
    fillet(&joined, &[("plate.end", "boss.side")], radius, "root").map_err(e)
}
