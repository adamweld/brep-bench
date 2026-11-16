//! A small cylinder, of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
//! it raised by 0.000015 along z, a hundredth of its height, the copy's side lying in the
//! cylinder's own: their common part, the cylinder from z = 0.000015 to 0.0015.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;

const UP: [f64; 3] = [0., 0., 1.];
const RADIUS: f64 = 0.00075;
const HEIGHT: f64 = 0.0015;
const RISE: f64 = HEIGHT / 100.;

pub fn build() -> Result<Brep, String> {
    let cylinder = prism(&disc([0., 0., 0.], UP, RADIUS), 0., HEIGHT)?;
    let raised = cylinder.moved(&Rigid { t: [0., 0., RISE], ..Rigid::identity() });
    Ok(combined(&cylinder, &raised, Op::Common, 0.)?)
}
