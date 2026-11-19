//! Helpers for the loft cases: a solid whose level sections are rectangles centred on the z axis,
//! the half-widths along x and y each set by a planar Bézier guide.
use springrep::boolean::intersection;
use springrep::math::Vec2;
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use super::{e, frame};

/// The region in a vertical sketch plane (sketch x across, sketch y up) between the cubic Bézier
/// `(w, z)` through `guide` and its mirror image `(−w, z)`, closed by the floor and the top (where the guide ends off the axis).
fn between(guide: [(f64, f64); 4]) -> Profile {
    let p = |(w, z): (f64, f64)| Vec2::new(w, z);
    let q = |(w, z): (f64, f64)| Vec2::new(-w, z);
    let cubic = |pts: [Vec2; 4]| Segment::Bezier {
        ctrl: pts.to_vec(),
        weights: vec![1.; 4],
    };
    let [a, b, c, d] = guide;
    let mut outline = vec![
        NamedSegment::line(q(a), p(a), "floor"),
        NamedSegment::new(cubic([p(a), p(b), p(c), p(d)]), "wall"),
    ];
    // a guide that ends on the axis closes the region in a point, with no top
    if d.0 != 0. {
        outline.push(NamedSegment::line(p(d), q(d), "top"));
    }
    outline.push(NamedSegment::new(cubic([q(d), q(c), q(b), q(a)]), "wall"));
    Profile::outline(outline)
}

/// The solid whose section at each height is the rectangle |x| ≤ p, |y| ≤ q, where the guide
/// through the corner with x, y > 0 is the cubic Bézier `(p, q, z)` on `guide`.
///
/// Each wall is swept by a level segment parallel to y (or x) between two guides, so it is the
/// guide's curve in the xz plane (or the yz plane) carried straight along y (or x): the solid is
/// the common part of the region under the (p, z) curve extruded along y and the region under the
/// (q, z) curve extruded along x.
pub fn on_guides(guide: [[f64; 3]; 4], feature: &str) -> Result<Brep, String> {
    let reach = 4.
        * guide
            .iter()
            .flat_map(|g| [g[0].abs(), g[1].abs()])
            .fold(0., f64::max);
    let in_x = guide.map(|g| (g[0], g[2]));
    let in_y = guide.map(|g| (g[1], g[2]));
    // the xz plane moved back to y = −reach (sketch x along x, sketch y up), extruded along +y
    let xz = frame([0., -reach, 0.], [0., 1., 0.], [-1., 0., 0.])?;
    // the yz plane moved back to x = −reach (sketch x along y, sketch y up), extruded along +x
    let yz = frame([-reach, 0., 0.], [1., 0., 0.], [0., 1., 0.])?;
    let across_x = extrude(&between(in_x), &xz, 2. * reach, feature).map_err(e)?;
    let across_y = extrude(&between(in_y), &yz, 2. * reach, feature).map_err(e)?;
    intersection(&across_x, &across_y).map_err(e)
}
