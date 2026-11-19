//! The solid a rounded block fills as it slides along (8, 5, 3): everything within 1 of the box
//! (1, 1, 1)–(9, 5, 3), slid.
//!
//! springrep has no sweep of a solid. The rounded block is convex, so a line along the slide meets
//! it in one chord, which the slide carries along itself: the sweep is the block where it starts and
//! stops and any flat pieces of it, carried along the slide, that together cover its shadow along
//! the slide. That shadow is the box's shadow, a hexagon, grown by 1: covered by the box's three
//! faces toward the slide, a strip 1 wide square to the slide beside each of the six edges the
//! hexagon's sides come from, and a disc of radius 1 square to the slide about each of their six
//! corners. Each carried along is a prism (`shared::swept::slid`); springrep's offset grows the box
//! into the rounded block.
use springrep::edit::offset;
use springrep::math::{Frame, Vec2};
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, shifted, slid, slid_face, unite, v3};

pub fn build() -> Result<Brep, String> {
    let v = v3(8.0, 5.0, 3.0);
    let core = boxed([1.0, 1.0, 1.0], [9.0, 5.0, 3.0], "block")?;
    let block = offset(&core, 1.0, "rounded").map_err(e)?;
    let mut parts = vec![block.clone(), shifted(&block, v)];
    let p = |x: f64, y: f64, z: f64| v3(x, y, z);
    // the faces toward the slide
    for face in [
        [p(9.0, 1.0, 1.0), p(9.0, 5.0, 1.0), p(9.0, 5.0, 3.0), p(9.0, 1.0, 3.0)],
        [p(1.0, 5.0, 1.0), p(1.0, 5.0, 3.0), p(9.0, 5.0, 3.0), p(9.0, 5.0, 1.0)],
        [p(1.0, 1.0, 3.0), p(9.0, 1.0, 3.0), p(9.0, 5.0, 3.0), p(1.0, 5.0, 3.0)],
    ] {
        parts.push(slid_face(&face, v, "swept")?);
    }
    // the corners the shadow's outline runs through, in order round it
    let corners = [
        p(9.0, 1.0, 1.0),
        p(9.0, 5.0, 1.0),
        p(1.0, 5.0, 1.0),
        p(1.0, 5.0, 3.0),
        p(1.0, 1.0, 3.0),
        p(9.0, 1.0, 3.0),
    ];
    let center = p(5.0, 3.0, 2.0);
    for i in 0..6 {
        let (a, b) = (corners[i], corners[(i + 1) % 6]);
        let mut out = (b - a).cross(v).unit().ok_or("an edge along the slide")?;
        if out.dot((a + b) * 0.5 - center) < 0.0 {
            out = -out;
        }
        parts.push(slid_face(&[a, b, b + out, a + out], v, "swept")?);
        let square = Frame::new(a, v, out).map_err(e)?;
        parts.push(slid(&Profile::circle(Vec2::ZERO, 2.0), &square, v, "swept")?);
    }
    let refs: Vec<&Brep> = parts.iter().collect();
    unite(&refs)
}
