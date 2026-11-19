//! Helpers for the spline cases: a cubic Bézier path, and the frame square to a path's start.
use springrep::geom::{BSpline, Curve};
use springrep::math::{Frame, Vec3};

use super::e;

/// The cubic Bézier on four control points, as the clamped B-spline with no inner knot.
pub fn bezier_path(ctrl: [[f64; 3]; 4]) -> Result<Curve, String> {
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    let points = ctrl.iter().map(|&p| Vec3::from_array(p)).collect();
    Ok(Curve::BSpline(BSpline::new(3, knots, points).map_err(e)?))
}

/// The sketch plane square to `path` at its start, its first axis along `x_hint` (squared up).
pub fn start_frame(path: &Curve, x_hint: [f64; 3]) -> Result<Frame, String> {
    let (at, d) = path.d1(0.);
    Frame::new(at, d, Vec3::from_array(x_hint)).map_err(e)
}
