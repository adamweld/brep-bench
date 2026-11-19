//! The half above z = 0 of the ball of radius 5, the rim of its flat rounded to radius 1; the round
//! then deleted, giving the hemisphere back.
use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, sphere};
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    let below = block(&at([-6., -6., -6.]), Vec3::new(12., 12., 6.), "cut").map_err(e)?;
    let half = difference(&ball, &below).map_err(e)?;
    let rounded = fillet(&half, &[("cut.end", "ball.skin")], 1., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
