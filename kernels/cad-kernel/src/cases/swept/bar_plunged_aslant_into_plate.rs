//! A plate, the box (0, −5, 0)–(20, 5, 4), with a pocket cut into its underside by a bar — every
//! point with 6 ≤ x ≤ 14 and |y| + |z + 6| ≤ 1, a square section stood on a corner — slid without
//! turning by 8 along (0.3, 0, √(1 − 0.3²)), stopping inside the plate.
use crate::sketch::{block, polygon};
use cad_kernel::build::prism;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const PLATE_LO: V = [0., -5., 0.];
const PLATE_HI: V = [20., 5., 4.];
/// The bar's section's corners (y, z) in the plane x = 6, and its length along x.
const SECTION: [[f64; 2]; 4] = [[0., -7.], [1., -6.], [0., -5.], [-1., -6.]];
const START: f64 = 6.;
const LENGTH: f64 = 8.;
/// The push's lean toward +x from straight up (its direction's x), and its length.
const LEAN: f64 = 0.3;
const PUSH: f64 = 8.;

pub fn build() -> Result<Brep, String> {
    let plate = block(PLATE_LO, PLATE_HI)?;
    // the section walked counter-clockwise in (y, z), so that its normal is +x, swept 8 along it
    let section = polygon(&SECTION.map(|[y, z]| [START, y, z]));
    let bar = prism(&section, 0., LENGTH)?;
    // the slide: a screw that does not turn, advancing its whole length over a roll of τ
    let along = [LEAN, 0., (1. - LEAN * LEAN).sqrt()];
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: along, ratio: 0., advance: PUSH });
    // the kernel's closed swept solid cut from the plate on one arrangement (a bar's flat faces
    // mesh exactly, at any bar)
    let size = plate.size().max(bar.size());
    cut_from(&plate, &bar, &path, [0., TAU], size, tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
