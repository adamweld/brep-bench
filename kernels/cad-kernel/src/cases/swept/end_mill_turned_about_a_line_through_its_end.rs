//! A block, the box (−10, −10, −5)–(10, 10, 0), with a pocket cut into its top by an end mill, the
//! solid cylinder of radius 3 about the vertical line through (1.5, 0) from z = −2 to 4, turned
//! rigidly 120° about the z axis, which passes through its flat end.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const RADIUS: f64 = 3.;
const OFF_AXIS: f64 = 1.5;
const FLOOR: f64 = -2.;
const LENGTH: f64 = 6.;
const TURN: f64 = 120f64.to_radians();
/// How closely the mill is meshed for the sweep's membership reads: a hundred-and-fiftieth of its
/// radius.
const BAR: f64 = 0.02;

pub fn build() -> Result<Brep, String> {
    let stock = block([-10., -10., -5.], [10., 10., 0.])?;
    let up = [0., 0., 1.];
    let mill = prism(&disc([OFF_AXIS, 0., FLOOR], up, RADIUS), 0., LENGTH)?;
    // the turn about z: a screw that does not advance, turning a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0., 0., 0.], axis: up, ratio: 1., advance: 0. });
    // the kernel's swept solid cut from the block on one arrangement: the mill's side about an
    // axis along the turn's, its end square to it kept in its own plane
    let size = stock.size().max(mill.size());
    cut_from(&stock, &mill, &path, [0., TURN], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
