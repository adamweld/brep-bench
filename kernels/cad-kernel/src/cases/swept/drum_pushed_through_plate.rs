//! A plate, the box (0, −5, 0)–(20, 5, 4), with a slot punched through it by a drum — the cylinder
//! of radius 2 about the line y = 0, z = −5 from x = 6 to 14, its ends flat — slid without turning
//! straight up by 16, from wholly below the plate to wholly above it.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const PLATE_LO: V = [0., -5., 0.];
const PLATE_HI: V = [20., 5., 4.];
/// The middle of the drum's near end, its radius and its length along x.
const END: V = [6., 0., -5.];
const RADIUS: f64 = 2.;
const LENGTH: f64 = 8.;
const PUSH: f64 = 16.;
/// How closely the drum is meshed for the sweep's membership reads: a thousandth of its radius.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let plate = block(PLATE_LO, PLATE_HI)?;
    let drum = prism(&disc(END, [1., 0., 0.], RADIUS), 0., LENGTH)?;
    // the slide up z: a screw that does not turn, advancing its whole length over a roll of τ
    let up = [0., 0., 1.];
    let path = Trajectory::Twist(Screw { origin: END, axis: up, ratio: 0., advance: PUSH });
    // the kernel's closed swept solid cut from the plate on one arrangement: the side's rulings
    // whose normal is square to the push swept into the planes tangent along them, the flat ends
    // kept in their planes, the rims swept along their own planes
    let size = plate.size().max(drum.size());
    cut_from(&plate, &drum, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the drum's sweep not built: {why:?}"))
}
