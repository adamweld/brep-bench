//! A four-armed star handle: an outline of four tip arcs (radius 4, centres 33.1 out on the axes),
//! four concave fillet arcs (radius 3, centres 15.9 out on the diagonals) and the eight straight
//! sides tangent between them, sketched in the xz plane and extruded 12 along +y with a 2° draft.
use std::f64::consts::{FRAC_PI_2, SQRT_2, TAU};

use springrep::edit::draft;
use springrep::math::Vec2;
use springrep::pick::faces_named;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::{Brep, FaceId};

use crate::shared::parts::xz;
use crate::shared::{e, frame};

/// `p` turned by `a` about the origin.
fn turned(p: Vec2, a: f64) -> Vec2 {
    let (s, c) = a.sin_cos();
    Vec2::new(c * p.x - s * p.y, s * p.x + c * p.y)
}

pub fn build() -> Result<Brep, String> {
    let (tip_r, fillet_r) = (4., 3.);
    let tip = Vec2::new(33.1, 0.);
    let fillet = Vec2::new(15.9 / SQRT_2, 15.9 / SQRT_2);
    // The side between the tip at 0° and the fillet at 45° crosses between the circles: with n its
    // normal, n·(tip − fillet) = r₁ + r₂; it touches the tip circle at tip − r₁n and the fillet
    // circle at fillet + r₂n, on the tip's side facing the fillet.
    let d = tip - fillet;
    let (len, toward) = (d.norm(), d.y.atan2(d.x));
    let spread = ((tip_r + fillet_r) / len).acos();
    let n = Vec2::new((toward - spread).cos(), (toward - spread).sin());
    let n = if (tip - n * tip_r).y > 0. { n } else { Vec2::new((toward + spread).cos(), (toward + spread).sin()) };
    let on_tip = tip - n * tip_r;
    let on_fillet = fillet + n * fillet_r;
    // the tip arc runs ±a about its axis, the fillet arc clockwise from b about its diagonal
    let a = (on_tip - tip).y.atan2((on_tip - tip).x);
    let b = (on_fillet - fillet).y.atan2((on_fillet - fillet).x).rem_euclid(TAU);
    // it ends at the mirror image of its start across the diagonal, at 90° − b
    let fillet_sweep = (FRAC_PI_2 - b).rem_euclid(TAU) - b;
    let mut segments = Vec::new();
    for k in 0..4 {
        let q = k as f64 * FRAC_PI_2;
        let mirror = |p: Vec2| turned(Vec2::new(p.y, p.x), q); // across the arm's 45° diagonal
        segments.push(NamedSegment::arc(turned(tip, q), tip_r, q - a, 2. * a, "tip"));
        segments.push(NamedSegment::line(turned(on_tip, q), turned(on_fillet, q), "flank"));
        segments.push(NamedSegment::arc(turned(fillet, q), fillet_r, q + b, fillet_sweep, "fillet"));
        segments.push(NamedSegment::line(mirror(on_fillet), mirror(on_tip), "flank"));
    }
    let outline = Profile::outline(segments);
    // the xz plane faces −y, so the solid runs to y = 12 against its normal
    let handle = extrude(&outline, &xz(0.)?, -12., "star").map_err(e)?;
    // every side tilted 2° about where it meets y = 0, narrowing the star toward +y
    let sides: Vec<FaceId> = ["star.tip", "star.flank", "star.fillet"]
        .iter()
        .flat_map(|name| faces_named(&handle, name))
        .collect();
    let neutral = frame([0., 0., 0.], [0., 1., 0.], [1., 0., 0.])?;
    draft(&handle, &sides, &neutral, 2f64.to_radians()).map_err(e)
}
