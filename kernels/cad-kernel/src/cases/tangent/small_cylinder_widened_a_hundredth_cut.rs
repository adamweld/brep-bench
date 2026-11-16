//! A small cylinder, of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy a
//! hundredth wider, of radius 0.0007575, on the same axis between the same two planes: the copy less
//! the cylinder, a tube whose wall is 0.0000075 thick, its ends in the cylinder's end planes.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const UP: [f64; 3] = [0., 0., 1.];
const RADIUS: f64 = 0.00075;
const WIDER: f64 = RADIUS * 1.01;
const HEIGHT: f64 = 0.0015;

pub fn build() -> Result<Brep, String> {
    let cylinder = prism(&disc([0., 0., 0.], UP, RADIUS), 0., HEIGHT)?;
    let copy = prism(&disc([0., 0., 0.], UP, WIDER), 0., HEIGHT)?;
    Ok(combined(&copy, &cylinder, Op::Cut, 0.)?)
}
