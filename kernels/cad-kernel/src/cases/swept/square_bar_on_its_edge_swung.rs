//! The solid a square bar fills swung a quarter turn about the z axis: the prism from z = 0 to 3 on
//! the square with corners (4, 0), (6, 2), (8, 0) and (6, −2), stood on a corner so that its
//! upright edge through (4, 0) faces the axis.
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::FRAC_PI_2;

const HEIGHT: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    // the square walked counter-clockwise seen from above, so its prism rises along z
    let square = polygon(&[[4., 0., 0.], [6., -2., 0.], [8., 0., 0.], [6., 2., 0.]]);
    let bar = prism(&square, 0., HEIGHT)?;
    // the turn about z: a screw that does not advance, turning a radian a radian of the roll
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: [0., 0., 1.], ratio: 1., advance: 0.,
    });
    // the kernel's closed swept solid: a polyhedral tool under a turn (its flat faces mesh
    // exactly, at any bar)
    closed(&bar, &path, [0., FRAC_PI_2], bar.size(), tolerance(bar.size()))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
