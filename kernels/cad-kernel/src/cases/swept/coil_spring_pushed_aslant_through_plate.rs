//! A plate, the box (−15, −15, 0)–(15, 15, 4), with a hole punched through it by a coil spring slid
//! without turning along (3, 1, 18), from wholly below it to wholly above it. The spring is what a
//! ball of radius 0.4 fills as its centre runs three right-handed turns of the helix of radius 2
//! about z rising 1.5 a turn, from (2, 0, −12) to (2, 0, −7.5).
//!
//! The spring is the kernel's own sweep of the ball (`build::canal`: its disc screwed along the
//! helix, a half ball at each end), and the push the kernel's closed sweep of that solid. Refused
//! by the kernel: under a slide its closed sweep takes faces that are planes, cylinders, cones,
//! balls and tori, and the spring's coil is a screwed sheet (`Unbuilt::Curved`).
use crate::sketch::block;
use cad_kernel::build::canal;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{norm, unit};
use std::f64::consts::TAU;

const PLATE_LO: V = [-15., -15., 0.];
const PLATE_HI: V = [15., 15., 4.];
const WIRE: f64 = 0.4;
const START: V = [2., 0., -12.];
/// How far the coil rises a whole turn, and how many turns it makes.
const RISE: f64 = 1.5;
const TURNS: f64 = 3.;
const PUSH: V = [3., 1., 18.];
/// How closely the spring is meshed for the sweep's membership reads: a two-hundredth of its
/// wire's radius.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let plate = block(PLATE_LO, PLATE_HI)?;
    let up = [0., 0., 1.];
    let spring = canal(START, WIRE, [0.; 3], up, TURNS * TAU, TURNS * RISE, "spring")?;
    // the slide along (3, 1, 18): a screw that does not turn, advancing its whole length over a
    // roll of τ
    let path = Trajectory::Twist(Screw { origin: START, axis: unit(PUSH), ratio: 0.,
                                         advance: norm(PUSH) });
    let size = plate.size().max(spring.size());
    cut_from(&plate, &spring, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the spring's sweep not built: {why:?}"))
}
