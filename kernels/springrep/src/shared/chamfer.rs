//! Helpers shared by the chamfer cases: solids turned about the z axis from a meridian drawn in
//! (ρ, z), solids cut by half-spaces, and the cone a chamfer given by a distance and an angle lies
//! on, turned the same way.
use std::f64::consts::TAU;

use springrep::boolean::{difference, union};
use springrep::math::{Frame, Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::e;

/// The plane y = 0 drawn on in (ρ, z): its x is the world's x and its y the world's z.
pub fn meridian_plane() -> Frame {
    Frame::new(Vec3::ZERO, Vec3::new(0., -1., 0.), Vec3::X).expect("the xz plane")
}

/// The solid `segments`, a closed meridian in (ρ, z), sweeps turned a whole turn about the z axis.
pub fn turned(segments: Vec<NamedSegment>, feature: &str) -> Result<Brep, String> {
    let plane = meridian_plane();
    revolve(
        &Profile::outline(segments),
        &plane,
        Vec3::ZERO,
        Vec3::Z,
        TAU,
        feature,
    )
    .map_err(e)
}

/// The solid the polygon `points` (in (ρ, z)) sweeps turned about the z axis.
pub fn turned_polygon(points: &[Vec2], feature: &str) -> Result<Brep, String> {
    let plane = meridian_plane();
    let profile = Profile::polygon(points, &[feature]);
    revolve(&profile, &plane, Vec3::ZERO, Vec3::Z, TAU, feature).map_err(e)
}

/// The block on the inner side of the plane through `point` with inward normal `inward`, `size`
/// across and deep, centered on `point` along the plane: a half-space as far as the cases reach.
/// Its face on the plane is `<feature>.start`.
pub fn half_space(point: Vec3, inward: Vec3, size: f64, feature: &str) -> Result<Brep, String> {
    let f = Frame::from_axis(point, inward).map_err(e)?;
    let corner = f.origin - f.x * (size / 2.) - f.y * (size / 2.);
    let f = Frame {
        origin: corner,
        ..f
    };
    block(&f, Vec3::new(size, size, size), feature).map_err(e)
}

/// `v` turned by `a` radians, counter-clockwise in (ρ, z): toward the axis from a direction
/// pointing up and out.
pub fn rotated(v: Vec2, a: f64) -> Vec2 {
    Vec2::new(v.x * a.cos() - v.y * a.sin(), v.x * a.sin() + v.y * a.cos())
}

/// The unit vector from `a` toward `b`.
pub fn toward(a: Vec2, b: Vec2) -> Vec2 {
    let d = b - a;
    d * (1. / d.norm())
}

/// Far enough to run past every solid the chamfer cases build.
const FAR: f64 = 30.;

/// A convex chamfer of a rim as a cut: `body` less the ring on the side of the chamfer's line
/// that holds the rim's `corner`, the line from `p` (where the chamfer leaves the face its first
/// distance is measured on) along `dir` (on across the solid, toward its other face), turned about
/// the z axis. `away` is a direction from `p` that leaves the solid across the first face's
/// surface: it bounds the ring there, so that face beyond `p` keeps its material.
pub fn cut_by_chamfer_cone(
    body: &Brep,
    p: Vec2,
    dir: Vec2,
    away: Vec2,
    corner: Vec2,
) -> Result<Brep, String> {
    let mut out = Vec2::new(dir.y, -dir.x);
    if out.dot(corner - p) < 0. {
        out = -out;
    }
    let far = p + dir * FAR;
    let ring = [p, p + away * FAR, far + out * FAR, far];
    let ring = turned_polygon(&off_the_axis(&ring), "chamfer")?;
    difference(body, &ring).map_err(e)
}

/// The point a chamfer distance `chord` from the edge point `edge` reaches on a face that is a
/// sphere about `center` in section: `edge` turned about `center` so the chord between them is
/// `chord` long, counter-clockwise in (ρ, z) when `ccw`.
pub fn along_sphere(center: Vec2, edge: Vec2, chord: f64, ccw: bool) -> Vec2 {
    let r = (edge - center).norm();
    let a = 2. * (chord / (2. * r)).asin();
    center + rotated(edge - center, if ccw { a } else { -a })
}

/// The part of the polygon `points` with ρ ≥ 0, which a turn about the z axis can sweep.
fn off_the_axis(points: &[Vec2]) -> Vec<Vec2> {
    let mut out = Vec::new();
    for (i, &a) in points.iter().enumerate() {
        let b = points[(i + 1) % points.len()];
        if a.x >= 0. {
            out.push(a);
        }
        if (a.x < 0.) != (b.x < 0.) {
            out.push(a + (b - a) * (a.x / (a.x - b.x)));
        }
    }
    out.dedup_by(|a, b| a.dist(*b) < 1e-12);
    out
}

/// A concave chamfer of a ball's root by a distance and an angle, as a fill: `body` with the
/// groove between the chamfer's line and the two faces filled, the line from `p` (on the face the
/// distance is measured on) along `dir` into the ball about the origin. Elsewhere the fill runs
/// within `body`: from the line's nearest point to the ball's center (or where the line reaches
/// the axis, if sooner) over to the axis, down it to `low` and back out to under `p`, all inside
/// the ball or the part it stands in.
pub fn fill_to_chamfer_cone(body: &Brep, p: Vec2, dir: Vec2, low: f64) -> Result<Brep, String> {
    let mut f = p + dir * -p.dot(dir);
    if f.x < 0. {
        f = p + dir * (p.x / -dir.x);
    }
    let mut pts = vec![p, f];
    if f.x > 1e-9 {
        pts.push(Vec2::new(0., f.y));
    }
    pts.push(Vec2::new(0., low));
    pts.push(Vec2::new(p.x, low));
    let ring = turned_polygon(&pts, "chamfer")?;
    union(body, &ring).map_err(e)
}

/// The ball of radius 5 about the origin.
pub fn ball(feature: &str) -> Result<Brep, String> {
    springrep::primitive::sphere(&Frame::WORLD, 5., feature).map_err(e)
}

/// The seat (the cylinder of radius 8 from z = −6 to 5, hollowed above the cone z = 1 + ρ/2, its
/// face `seat.cone`) with the ball of radius 5 about the origin (`ball.skin`) standing in it: they
/// meet in the concave circle of radius 4 at z = 3.
pub fn ball_in_conical_seat() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let seat = turned(
        vec![
            NamedSegment::line(p(0., -6.), p(8., -6.), "bottom"),
            NamedSegment::line(p(8., -6.), p(8., 5.), "side"),
            NamedSegment::line(p(8., 5.), p(0., 1.), "cone"),
            NamedSegment::line(p(0., 1.), p(0., -6.), "axis"),
        ],
        "seat",
    )?;
    union(&seat, &ball("ball")?).map_err(e)
}

/// The plate (the cylinder of radius 8 from z = −6 to 0, its top `plate.end`) with the ball of
/// radius 5 about the origin (`ball.skin`) standing in it: they meet in the concave circle of
/// radius 5 at z = 0.
pub fn ball_on_plate() -> Result<Brep, String> {
    let at = Frame::WORLD.translated(Vec3::new(0., 0., -6.));
    let plate = springrep::primitive::cylinder(&at, 8., 6., "plate").map_err(e)?;
    union(&plate, &ball("ball")?).map_err(e)
}

/// The frustum from radius 2 at z = 0 to 4 at z = 3, capped by the ball of radius 5 about the
/// origin above z = 3: the cone `cap.side` meets the sphere `cap.dome` in the rim (4, 3).
pub fn capped_frustum() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let rim = 3f64.atan2(4.);
    turned(
        vec![
            NamedSegment::line(p(0., 0.), p(2., 0.), "bottom"),
            NamedSegment::line(p(2., 0.), p(4., 3.), "side"),
            NamedSegment::arc(p(0., 0.), 5., rim, TAU / 4. - rim, "dome"),
            NamedSegment::line(p(0., 5.), p(0., 0.), "axis"),
        ],
        "cap",
    )
}

/// The upper half of the ball of radius 5 about the origin: its flat face `half.flat` meets its
/// sphere `half.skin` in the rim (5, 0).
pub fn hemisphere() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    turned(
        vec![
            NamedSegment::line(p(0., 0.), p(5., 0.), "flat"),
            NamedSegment::arc(p(0., 0.), 5., 0., TAU / 4., "skin"),
            NamedSegment::line(p(0., 5.), p(0., 0.), "axis"),
        ],
        "half",
    )
}

/// The spherical sector of the ball of radius 5 about the origin inside the cone about the z axis
/// of half angle α, cos α = `cos_a`: its cone `sector.cone` meets its sphere `sector.skin` in the
/// rim at 5 from the origin.
pub fn sector(cos_a: f64) -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let a = cos_a.acos();
    let rim = p(5. * a.sin(), 5. * cos_a);
    turned(
        vec![
            NamedSegment::line(p(0., 0.), rim, "cone"),
            NamedSegment::arc(p(0., 0.), 5., TAU / 4. - a, a, "skin"),
            NamedSegment::line(p(0., 5.), p(0., 0.), "axis"),
        ],
        "sector",
    )
}

/// The sector's rim chamfered by `distance` down its cone and the chamfer at `angle` (radians) to
/// the cone's line continued toward the rim, turned toward the axis.
pub fn sector_rim_by_angle(cos_a: f64, distance: f64, angle: f64) -> Result<Brep, String> {
    let body = sector(cos_a)?;
    let up = Vec2::new((1. - cos_a * cos_a).sqrt(), cos_a);
    let p = up * (5. - distance);
    // off the cone's side, square to it and away from the sector
    let away = Vec2::new(up.y, -up.x);
    cut_by_chamfer_cone(&body, p, rotated(up, angle), away, up * 5.)
}

/// The capped frustum's rim chamfered by `distance` down its cone and the chamfer at `angle` to
/// the cone's line continued toward the rim, turned toward the axis.
pub fn capped_frustum_rim_by_angle(distance: f64, angle: f64) -> Result<Brep, String> {
    let body = capped_frustum()?;
    let (rim, foot) = (Vec2::new(4., 3.), Vec2::new(2., 0.));
    let up = toward(foot, rim);
    let p = rim - up * distance;
    cut_by_chamfer_cone(&body, p, rotated(up, angle), Vec2::new(up.y, -up.x), rim)
}

/// The hemisphere's rim chamfered by `distance` in across its flat face and the chamfer rising at
/// `angle` from it toward the rim.
pub fn hemisphere_rim_by_angle(distance: f64, angle: f64) -> Result<Brep, String> {
    let body = hemisphere()?;
    let p = Vec2::new(5. - distance, 0.);
    let (dir, away) = (rotated(Vec2::new(1., 0.), angle), Vec2::new(0., -1.));
    cut_by_chamfer_cone(&body, p, dir, away, Vec2::new(5., 0.))
}

/// The groove where the ball stands in the conical seat chamfered by `distance` up the seat's cone
/// and the chamfer at `angle` to the cone's line back toward the edge, turned upward.
pub fn conical_seat_by_angle(distance: f64, angle: f64) -> Result<Brep, String> {
    let body = ball_in_conical_seat()?;
    let (edge, rim) = (Vec2::new(4., 3.), Vec2::new(8., 5.));
    let up = toward(edge, rim);
    let p = edge + up * distance;
    fill_to_chamfer_cone(&body, p, rotated(-up, -angle), 2.)
}

/// The groove where the ball stands on the plate chamfered by `distance` out across the plate's
/// top and the chamfer rising at `angle` from it back toward the ball.
pub fn plate_rim_by_angle(distance: f64, angle: f64) -> Result<Brep, String> {
    let body = ball_on_plate()?;
    let p = Vec2::new(5. + distance, 0.);
    fill_to_chamfer_cone(&body, p, rotated(Vec2::new(-1., 0.), -angle), -1.)
}

/// The sign of each axis for each of the octahedron's eight faces, face `i` named `f<i>`.
pub const OCTANTS: [[f64; 3]; 8] = [
    [1., 1., 1.],
    [-1., 1., 1.],
    [1., -1., 1.],
    [-1., -1., 1.],
    [1., 1., -1.],
    [-1., 1., -1.],
    [1., -1., -1.],
    [-1., -1., -1.],
];

/// The octahedron |x| + |y| + |z| ≤ 10, the common part of the eight half-spaces its faces bound;
/// the face in the octant of `OCTANTS[i]` is `f<i>.start`.
pub fn octahedron() -> Result<Brep, String> {
    let mut solid: Option<Brep> = None;
    for (i, s) in OCTANTS.iter().enumerate() {
        let n = Vec3::from_array(*s) * (1. / 3f64.sqrt());
        let side = half_space(n * (10. / 3f64.sqrt()), -n, 60., &format!("f{i}"))?;
        solid = Some(match solid {
            None => side,
            Some(s) => springrep::boolean::intersection(&s, &side).map_err(e)?,
        });
    }
    Ok(solid.expect("eight faces"))
}

/// The octahedron's twelve edges, as the pairs of its faces' names that meet in them: faces whose
/// octants differ in one sign.
pub fn octahedron_edges() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for i in 0..8 {
        for j in i + 1..8 {
            let differ = (0..3).filter(|&k| OCTANTS[i][k] != OCTANTS[j][k]).count();
            if differ == 1 {
                out.push((format!("f{i}.start"), format!("f{j}.start")));
            }
        }
    }
    out
}
