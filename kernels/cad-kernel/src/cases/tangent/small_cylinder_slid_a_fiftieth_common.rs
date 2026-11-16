//! A small cylinder, of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
//! it slid 0.000015 along x, a fiftieth of its radius, the two sharing their end planes and their
//! sides crossing in two vertical lines in the plane x = 0.0000075: their common part, the prism over the lens the two circles share.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;

const UP: [f64; 3] = [0., 0., 1.];
const RADIUS: f64 = 0.00075;
const HEIGHT: f64 = 0.0015;
const SLIDE: f64 = RADIUS / 50.;

pub fn build() -> Result<Brep, String> {
    let cylinder = prism(&disc([0., 0., 0.], UP, RADIUS), 0., HEIGHT)?;
    let slid = cylinder.moved(&Rigid { t: [SLIDE, 0., 0.], ..Rigid::identity() });
    Ok(combined(&cylinder, &slid, Op::Common, 0.)?)
}
