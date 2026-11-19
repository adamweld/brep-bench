//! A ball of radius 5 about the origin hollowed to a wall 1 thick with no opening: the ball less the
//! ball of radius 4.
use springrep::edit::Wall;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    shelled(&ball, &[], 1., Wall::Inward, "shell")
}
