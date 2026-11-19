//! A transition from the square −20 ≤ x, y ≤ 20 at z = 0 to the octagon (the 28 × 28 square with
//! its corners cut, corners (±6, ±14), (±14, ±6)) at z = 30: four flat trapezoids from side to
//! side and four flat triangles from corner to cut side.
use std::f64::consts::FRAC_PI_2;

use springrep::boolean::split;
use springrep::math::{Transform, Vec3};
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // springrep's loft pairs sections of unequal edge counts by splitting both at every vertex's
    // polar angle, which makes ruled twisted walls, not a corner swept to a whole side; the walls
    // here are flat, so the hull is the block between the two planes cut by each wall's plane
    let mut hull = block(&at([-20., -20., 0.]), Vec3::new(40., 40., 30.), "hood").map_err(e)?;
    let v = |x: f64, y: f64, z: f64| Vec3::new(x, y, z);
    // the walls on the side y < 0 and the corner (20, −20), each as three of its points
    let walls = [
        [v(-20., -20., 0.), v(20., -20., 0.), v(6., -14., 30.)],
        [v(20., -20., 0.), v(14., -6., 30.), v(6., -14., 30.)],
    ];
    for k in 0..4 {
        let turn = Transform::rotation(Vec3::ZERO, Vec3::Z, k as f64 * FRAC_PI_2);
        for [a, b, c] in walls {
            let (a, b, c) = (turn.point(a), turn.point(b), turn.point(c));
            // the plane through the three, its normal pointing out of the hull (away from the axis)
            let mut n = (b - a).cross(c - a);
            if n.dot(a - Vec3::new(0., 0., a.z)) < 0. {
                n = -n;
            }
            hull = split(&hull, a, n).map_err(e)?.1;
        }
    }
    Ok(hull)
}
