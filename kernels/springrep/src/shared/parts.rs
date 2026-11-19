//! Helpers for the parts cases: the xz plane to sketch turned and extruded outlines on, and an
//! outline from lines and arcs.
use springrep::math::{Frame, Vec2};
use springrep::profile::{NamedSegment, Profile};

use super::frame;

/// The xz plane through `y`, its sketch x along world x and its sketch y along world z: its
/// normal is −y.
pub fn xz(y: f64) -> Result<Frame, String> {
    frame([0., y, 0.], [0., -1., 0.], [1., 0., 0.])
}

/// A closed polygon outline through `points`, each edge named `name`.
pub fn polygon(points: &[(f64, f64)], name: &str) -> Profile {
    let pts: Vec<Vec2> = points.iter().map(|&(x, y)| Vec2::new(x, y)).collect();
    Profile::polygon(&pts, &[name])
}

/// A straight edge.
pub fn line(a: (f64, f64), b: (f64, f64), name: &str) -> NamedSegment {
    NamedSegment::line(Vec2::new(a.0, a.1), Vec2::new(b.0, b.1), name)
}
