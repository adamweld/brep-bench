//! Two tori about z, each of middle radius 5 and tube radius 2, their middle circles in z = −1
//! and z = 1: their common part, a ring whose section is the lens of the two tubes' disks.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::torus;
use cad_kernel::topo::Brep;

const MAJOR: f64 = 5.;
const MINOR: f64 = 2.;
/// How far each middle circle stands off the plane z = 0.
const OFF: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let lower = torus([0., 0., -OFF], [0., 0., 1.], MAJOR, MINOR)?;
    let upper = torus([0., 0., OFF], [0., 0., 1.], MAJOR, MINOR)?;
    Ok(combined(&lower, &upper, Op::Common, 0.)?)
}
