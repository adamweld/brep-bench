//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) notched by a ball
//! of radius 0.8 about (10, 0, 0) united with a stem, the cylinder of radius 0.4 running 3 from the
//! ball's centre along (0, sin 20°, cos 20°), turned rigidly about z, right-handed, from −140° to
//! +30°: it starts clear and stops halfway across.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, revolve, sphere};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::FRAC_PI_3;

const UP: V = [0., 0., 1.];
const CENTRE: V = [10., 0., 0.];
const BALL: f64 = 0.8;
const STEM: f64 = 0.4;
const STEM_LENGTH: f64 = 3.;
/// How far the stem leans from upright toward +y, in degrees.
const LEAN: f64 = 20.;
/// The turn's start and stop, in degrees.
const FROM: f64 = -140.;
const TO: f64 = 30.;
/// How closely the tool is meshed for the sweep's membership reads: a two-hundredth of the stem's
/// radius.
const BAR: f64 = 0.002;

/// The ring sector: its section in the half-plane at 0° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0.; 3], UP, FRAC_PI_3)
}

pub fn build() -> Result<Brep, String> {
    let blank = sector()?;
    let lean = LEAN.to_radians();
    let along = [0., lean.sin(), lean.cos()];
    // the ball's poles on the stem's axis, so the one it leaves by is buried in the stem
    let ball = sphere(CENTRE, along, BALL)?;
    let stem = prism(&disc(CENTRE, along, STEM), 0., STEM_LENGTH)?;
    let tool = combined(&ball, &stem, Op::Union, 0.)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the sector on one arrangement: the ball's great
    // circle through the axis turned into a torus, the stem's side (about an axis leaning to the
    // turn's) touching along a curve turned into a surface of revolution, its flat end's rim a
    // sharp circle leaning to the axis
    let size = blank.size().max(tool.size());
    cut_from(&blank, &tool, &path, [FROM.to_radians(), TO.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the tool's sweep not built: {why:?}"))
}
