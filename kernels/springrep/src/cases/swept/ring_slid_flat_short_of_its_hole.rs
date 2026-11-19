//! The solid a ring fills as it slides 6 along x: the torus about the z axis, its tube's centers
//! 6 from it and its tube of radius 2.
//!
//! springrep has no sweep of a solid. The torus is everything within 2 of its tube's center
//! circle, which the slide carries over the stadium of radius 6 about the segment from (0, 0) to
//! (6, 0) less the lens its hole leaves open, the discs of radius 6 about the two ends in common.
//! Everything within 2 of that region is everything within 2 of the whole stadium (the stadium's
//! section square to the slide carried along, ended by half a filled ring), less the lens's
//! upright prism, but for the part of that prism within 2 of the lens's rim: the inner halves of
//! the torus where it starts and stops, which are added back.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::torus;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::swept::{arc, outline, slid_ring, unite};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let whole = slid_ring(Vec3::ZERO, Vec3::Z, Vec3::X, 6.0, 2.0, 6.0, "ring")?;
    // the lens: the arc of the first circle inside the second, then the second's inside the first
    let lens = outline(vec![
        arc((0.0, 0.0), 6.0, -PI / 3.0, 2.0 * PI / 3.0, "hole"),
        arc((6.0, 0.0), 6.0, 2.0 * PI / 3.0, 2.0 * PI / 3.0, "hole"),
    ]);
    let hole = extrude(&lens, &at([0.0, 0.0, -2.0]), 4.0, "hole").map_err(e)?;
    let start = torus(&at([0.0, 0.0, 0.0]), 6.0, 2.0, "ring").map_err(e)?;
    let stop = torus(&at([6.0, 0.0, 0.0]), 6.0, 2.0, "ring").map_err(e)?;
    let open = difference(&whole, &hole).map_err(e)?;
    unite(&[&open, &start, &stop])
}
