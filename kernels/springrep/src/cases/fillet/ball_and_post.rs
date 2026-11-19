//! A ball of radius 10 about the origin joined to a post of radius 4 from z = 0 to 15, the
//! concave circle where the post leaves the ball rounded to radius 2, adding material.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::primitive::{cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 10., "ball").map_err(e)?;
    let post = cylinder(&at([0., 0., 0.]), 4., 15., "post").map_err(e)?;
    let body = union(&ball, &post).map_err(e)?;
    fillet(&body, &[("ball.skin", "post.side")], 2., "round").map_err(e)
}
