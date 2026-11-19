//! A golf ball: the ball of radius 10 about the origin less 62 balls of radius 1.2 whose centres
//! are 10.6 out toward the vertices, edge midpoints and face centres of a regular icosahedron,
//! each its own cut.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::e;

/// The 62 directions: the icosahedron's 12 vertices, the midpoints of its 30 edges and the centres
/// of its 20 faces, each pointing from its centre.
fn directions() -> Vec<Vec3> {
    let phi = (1. + 5f64.sqrt()) / 2.;
    let mut v = Vec::new();
    for s in [-1., 1.] {
        for t in [-1., 1.] {
            v.push(Vec3::new(0., s, t * phi));
            v.push(Vec3::new(s, t * phi, 0.));
            v.push(Vec3::new(s * phi, 0., t));
        }
    }
    // neighbouring vertices are 2 apart, the icosahedron's edge
    let edge = |a: Vec3, b: Vec3| (a.dist(b) - 2.).abs() < 1e-9;
    let mut out = v.clone();
    for i in 0..12 {
        for j in i + 1..12 {
            if !edge(v[i], v[j]) {
                continue;
            }
            out.push((v[i] + v[j]) * 0.5);
            for k in j + 1..12 {
                if edge(v[i], v[k]) && edge(v[j], v[k]) {
                    out.push((v[i] + v[j] + v[k]) * (1. / 3.));
                }
            }
        }
    }
    out.into_iter().map(|d| d.unit_or_zero()).collect()
}

pub fn build() -> Result<Brep, String> {
    let mut golf = ball([0., 0., 0.], 10., "ball")?;
    for d in directions() {
        // each dimple's ball has its poles on the vertical through its centre
        let dimple = ball((d * 10.6).to_array(), 1.2, "dimple")?;
        golf = difference(&golf, &dimple).map_err(e)?;
    }
    Ok(golf)
}
