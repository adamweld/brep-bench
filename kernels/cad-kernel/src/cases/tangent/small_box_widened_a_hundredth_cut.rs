//! A small square prism, 0.0015 square about the z axis and 0.0015 tall from z = 0, and a copy a
//! hundredth wider, 0.001515 square, about the same axis between the same two planes: the copy less
//! the prism, a square tube whose walls are 0.0000075 thick, its ends in the prism's end planes.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

const WIDTH: f64 = 0.0015;
const WIDER: f64 = WIDTH * 1.01;
const HEIGHT: f64 = 0.0015;

pub fn build() -> Result<Brep, String> {
    let (h, w) = (WIDTH / 2., WIDER / 2.);
    let prism = block([-h, -h, 0.], [h, h, HEIGHT])?;
    let copy = block([-w, -w, 0.], [w, w, HEIGHT])?;
    // the two share their end planes: the tube's ends lie in both
    Ok(combined(&copy, &prism, Op::Cut, 0.)?)
}
