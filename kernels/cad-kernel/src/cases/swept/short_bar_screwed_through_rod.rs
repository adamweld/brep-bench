//! A rod, the cylinder of radius 5 about z from z = 0 to 20, threaded by a short bar, the box
//! 4 ≤ x ≤ 12, −1.5 ≤ y ≤ 1.5, −1 ≤ z ≤ 1, screwed about z — right-handed, rising 6 a turn — from
//! a turn of −90° to 1350°: its corners inside the rod carry the groove's walls too.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const LEAD: f64 = 6.;
const ROLL: [f64; 2] = [-90f64.to_radians(), 1350f64.to_radians()];

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let bar = block([4., -1.5, -1.], [12., 1.5, 1.])?;
    // the screw about z: a radian a radian of the roll, rising the lead a turn
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: up, ratio: 1., advance: LEAD,
    });
    // the kernel's swept solid cut from the rod on one arrangement: the bar's edges along the axis
    // and square to it, screwed into cylinders and helicoidal sheets (a box's flat faces mesh
    // exactly, at any bar)
    let size = rod.size().max(bar.size());
    cut_from(&rod, &bar, &path, ROLL, bar.size(), tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
