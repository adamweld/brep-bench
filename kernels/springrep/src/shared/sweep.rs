//! Helpers for the sweep cases: a solid cut back to one side of a plane.
use springrep::boolean::split;
use springrep::math::Vec3;
use springrep::topo::Brep;

use super::e;

/// The part of `s` on the side of the plane through `origin` away from `normal`.
pub fn behind(s: &Brep, origin: [f64; 3], normal: [f64; 3]) -> Result<Brep, String> {
    let (_, kept) = split(s, Vec3::from_array(origin), Vec3::from_array(normal)).map_err(e)?;
    Ok(kept)
}

/// The pipe of radius 1 along three right-handed turns of the helix of radius 10 about the z axis
/// rising 5 a turn from (10, 0, 0), its section square to the path. springrep has no helix curve
/// (its screw sweep keeps a section in the axial plane), so the path is the kernel's spline
/// through points of the helix, 64 a turn, swept along by `along_curve`.
pub fn helical_pipe() -> Result<Brep, String> {
    use springrep::geom::{BSpline, Curve};
    use springrep::math::{Frame, Vec2};
    use springrep::profile::Profile;
    use springrep::sweep::along_curve;
    use std::f64::consts::TAU;

    let per_turn = 64;
    let points: Vec<Vec3> = (0..=3 * per_turn)
        .map(|i| {
            let t = TAU * i as f64 / per_turn as f64;
            Vec3::new(10. * t.cos(), 10. * t.sin(), 5. * t / TAU)
        })
        .collect();
    let path = Curve::BSpline(BSpline::interpolate(&points).map_err(e)?);
    let (lo, hi) = match &path {
        Curve::BSpline(s) => s.domain(),
        _ => unreachable!(),
    };
    let (start, heading) = path.d1(lo);
    let sketch = Frame::new(start, heading, Vec3::X).map_err(e)?;
    along_curve(
        &Profile::circle(Vec2::ZERO, 2.),
        &sketch,
        &path,
        (lo, hi),
        0.,
        "pipe",
    )
    .map_err(e)
}
