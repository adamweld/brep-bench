//! The half of the ball of radius 5 above z = 0, its rim rounded to radius 1, shrunk by 3/2: the
//! round vanishes, leaving the part of the ball of radius 7/2 above z = 3/2.
//! springrep 1.0.1 refuses it: its face offset cannot follow a round moved past its radius ("an
//! offset … closes a torus to nothing").
use springrep::blend::fillet_edges;
use springrep::boolean::split;
use springrep::math::Vec3;
use springrep::pick::convex_edges;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::offset::shrunk;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    let (dome, _) = split(&ball, Vec3::ZERO, Vec3::Z).map_err(e)?;
    // the rim, where the flat meets the ball, is the dome's one convex edge
    let rim = convex_edges(&dome).map_err(e)?;
    let rounded = fillet_edges(&dome, &rim, 1., "round").map_err(e)?;
    shrunk(&rounded, 1.5)
}
