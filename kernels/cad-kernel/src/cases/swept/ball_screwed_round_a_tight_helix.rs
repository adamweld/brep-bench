//! The solid a ball of radius 2 fills as its centre runs two right-handed turns of the helix of
//! radius 1.5 about z rising 3 a turn, from (1.5, 0, 0) to (1.5, 0, 6): the ball screwed about z.
//! The helix's radius of curvature, about 1.65, is less than the ball's radius, so the tube about
//! it folds through itself, and the turns, 3 apart, overlap.
//!
//! Refused by the kernel: under a screw its closed sweep enters a ball's pipe about its centre's
//! helix, and refuses one whose radius reaches the helix's radius of curvature, where the pipe
//! folds (`Unbuilt::Folded`): the fold is not yet cut where it lies.
use cad_kernel::build::sphere;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const UP: V = [0., 0., 1.];
const START: V = [1.5, 0., 0.];
const RADIUS: f64 = 2.;
/// How far the screw rises a whole turn, and how many turns it makes.
const RISE: f64 = 3.;
const TURNS: f64 = 2.;
/// How closely the ball is meshed for the sweep's membership reads: a thousandth of its radius.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let ball = sphere(START, UP, RADIUS)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: RISE });
    // the kernel's closed swept solid: the ball's great circle square to its centre's velocity
    // screwed into the pipe about the centre's helix, a pipe coming back onto itself entered in
    // stretches
    // (the sweep's reach: the ball's and its climb)
    let size = ball.size() + RISE * TURNS;
    closed(&ball, &path, [0., TURNS * TAU], BAR, tolerance(size))
        .map_err(|why| format!("the ball's sweep not built: {why:?}"))
}
