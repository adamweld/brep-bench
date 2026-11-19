//! The solid common to two cylinders of radius 0.003 and length 0.02, along x and along y, crossing
//! at the origin.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 0.003;
const LENGTH: f64 = 0.02;

pub fn build() -> Result<Brep, String> {
    // each a disc at one end, square to its axis, swept along it
    let along_x = prism(&disc([-LENGTH / 2., 0., 0.], [1., 0., 0.], RADIUS), 0., LENGTH)?;
    let along_y = prism(&disc([0., -LENGTH / 2., 0.], [0., 1., 0.], RADIUS), 0., LENGTH)?;
    Ok(combined(&along_x, &along_y, Op::Common, 0.)?)
}
