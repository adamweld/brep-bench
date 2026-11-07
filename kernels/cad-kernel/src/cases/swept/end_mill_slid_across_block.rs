//! A block, the box (0, −5, 0)–(20, 5, 10), with a slot cut across its top by an end mill, the
//! solid cylinder of radius 2 about the vertical line through (−5, 0) from z = 7 to 15, slid
//! without turning 30 along x.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const RADIUS: f64 = 2.;
const FLOOR: f64 = 7.;
const LENGTH: f64 = 8.;
const SLIDE: f64 = 30.;
/// How closely the mill is meshed for the sweep's membership reads: a hundredth of its radius.
const BAR: f64 = 0.02;

pub fn build() -> Result<Brep, String> {
    let stock = block([0., -5., 0.], [20., 5., 10.])?;
    let up = [0., 0., 1.];
    let mill = prism(&disc([-5., 0., FLOOR], up, RADIUS), 0., LENGTH)?;
    // the slide along x: a screw that does not turn, advancing its whole length over a roll of τ
    let path = Trajectory::Twist(Screw {
        origin: [-5., 0., FLOOR], axis: [1., 0., 0.], ratio: 0., advance: SLIDE,
    });
    // the kernel's swept solid cut from the block on one arrangement: the mill's flat end kept in
    // its plane, its side's two rulings square to the slide swept into the slot's walls
    let size = stock.size().max(mill.size());
    cut_from(&stock, &mill, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
