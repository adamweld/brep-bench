//! The plate (0, −5, 0)–(20, 5, 4) less everything a square bar passes through as it slides by 8
//! along (0.3, 0, √0.91): the prism 6 ≤ x ≤ 14, |y| + |z + 6| ≤ 1.
//!
//! springrep has no sweep of a solid. A point the bar passes through lies in the bar where it
//! starts, or behind one of the faces it leads with, which it has carried along the slide: its end
//! x = 14 and its two upper sides. So the sweep is the bar and those faces carried along the slide,
//! oblique prisms (`shared::swept::slid_face`).
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, polygon, slid_face, unite, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 4.0], "plate")?;
    // the section in (y, z), drawn on the plane x = 6 and carried 8 along x
    let diamond = polygon(&[(0.0, -7.0), (1.0, -6.0), (0.0, -5.0), (-1.0, -6.0)], "side");
    let bar = extrude(&diamond, &frame([6.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?, 8.0, "bar")
        .map_err(e)?;
    let v = v3(0.3, 0.0, 0.91f64.sqrt()) * 8.0;
    let p = |x: f64, y: f64, z: f64| v3(x, y, z);
    let faces: [Vec<Vec3>; 3] = [
        vec![p(14.0, 0.0, -7.0), p(14.0, 1.0, -6.0), p(14.0, 0.0, -5.0), p(14.0, -1.0, -6.0)],
        vec![p(6.0, 1.0, -6.0), p(14.0, 1.0, -6.0), p(14.0, 0.0, -5.0), p(6.0, 0.0, -5.0)],
        vec![p(6.0, -1.0, -6.0), p(6.0, 0.0, -5.0), p(14.0, 0.0, -5.0), p(14.0, -1.0, -6.0)],
    ];
    let mut parts = vec![bar];
    for f in &faces {
        parts.push(slid_face(f, v, "pocket")?);
    }
    let refs: Vec<&Brep> = parts.iter().collect();
    difference(&plate, &unite(&refs)?).map_err(e)
}
