//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a sealed helical tunnel: what the
//! box 2.5 ≤ x ≤ 3.5, −0.5 ≤ y ≤ 0.5, 2 ≤ z ≤ 3 passes through as it is screwed about z,
//! right-handed and rising 4 a turn, from a turn of 0° to 720°, wholly inside the rod.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const BOX_LO: V = [2.5, -0.5, 2.];
const BOX_HI: V = [3.5, 0.5, 3.];
/// How far the screw rises a whole turn, and how many turns it makes.
const RISE: f64 = 4.;
const TURNS: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0.; 3], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let tool = block(BOX_LO, BOX_HI)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: up, ratio: 1., advance: RISE });
    // the kernel's closed swept solid cut from the rod on one arrangement: the box's edges along
    // the axis and square to it, its level edges screwed into helicoidal sheets and its upright
    // faces' contact lines into coaxial cylinders (a box's flat faces mesh exactly, at any bar)
    let size = rod.size();
    cut_from(&rod, &tool, &path, [0., TURNS * TAU], size, tolerance(size))
        .map_err(|why| format!("the box's sweep not built: {why:?}"))
}
