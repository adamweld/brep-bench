//! The solid a ball of radius 2 about (1, 0, 0) fills as it turns rigidly about the z axis,
//! right-handed, by 120°: the axis passes through the ball, so the tube about its centre's arc is a
//! piece of a spindle torus.
//!
//! Refused by the kernel: under a turn its closed sweep takes a ball whose great circle through the
//! axis turns into a ring torus, and refuses a ball reaching across the axis (`Unbuilt::Curved`),
//! whose torus would be a spindle.
use cad_kernel::build::sphere;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const UP: V = [0., 0., 1.];
const CENTRE: V = [1., 0., 0.];
const RADIUS: f64 = 2.;
/// The turn, in degrees.
const TURN: f64 = 120.;
/// How closely the ball is meshed for the sweep's membership reads: a thousandth of its radius.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let ball = sphere(CENTRE, UP, RADIUS)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid: a ball's great circle in the plane through the axis and its
    // centre turned into a torus — here one reaching across the axis
    closed(&ball, &path, [0., TURN.to_radians()], BAR, tolerance(ball.size()))
        .map_err(|why| format!("the ball's sweep not built: {why:?}"))
}
