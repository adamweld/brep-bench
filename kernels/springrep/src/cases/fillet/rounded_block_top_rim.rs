//! A block 20 by 12 by 5 with its four upright edges rounded to radius 3 (the rounded rectangle
//! extruded), the rim of its top rounded to radius 1: eight edges, each running on tangentially
//! into the next, rounded as one chain.
use std::f64::consts::FRAC_PI_2;

use springrep::blend::fillet;
use springrep::math::Vec2;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let outline = Profile::outline(vec![
        NamedSegment::line(p(-7., -6.), p(7., -6.), "front"),
        NamedSegment::arc(p(7., -3.), 3., -FRAC_PI_2, FRAC_PI_2, "corner"),
        NamedSegment::line(p(10., -3.), p(10., 3.), "right"),
        NamedSegment::arc(p(7., 3.), 3., 0., FRAC_PI_2, "corner"),
        NamedSegment::line(p(7., 6.), p(-7., 6.), "back"),
        NamedSegment::arc(p(-7., 3.), 3., FRAC_PI_2, FRAC_PI_2, "corner"),
        NamedSegment::line(p(-10., 3.), p(-10., -3.), "left"),
        NamedSegment::arc(p(-7., -3.), 3., 2. * FRAC_PI_2, FRAC_PI_2, "corner"),
    ]);
    let body = extrude(&outline, &at([0., 0., 0.]), 5., "block").map_err(e)?;
    // one edge of the rim picks the whole tangent chain
    fillet(&body, &[("block.end", "block.front")], 1., "round").map_err(e)
}
