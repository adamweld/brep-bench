//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) notched by a mill
//! about the vertical line through (10, 0): a cylinder of radius 1 up to z = 3, ended below by the
//! cap of the sphere of radius 2 about (10, 0, 1.2), meeting it in a sharp circle at
//! z = 1.2 − √3. It turns rigidly about z, right-handed, from −140° to +30°: it starts clear and
//! stops halfway across.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, revolve, sphere};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::FRAC_PI_3;

const UP: V = [0., 0., 1.];
/// The dome's sphere's centre, on the mill's axis.
const DOME_CENTRE: V = [10., 0., 1.2];
const DOME: f64 = 2.;
const RADIUS: f64 = 1.;
const TOP: f64 = 3.;
/// The turn's start and stop, in degrees.
const FROM: f64 = -140.;
const TO: f64 = 30.;
/// How closely the mill is meshed for the sweep's membership reads: a five-hundredth of its radius.
const BAR: f64 = 0.002;

/// The ring sector: its section in the half-plane at 0° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0.; 3], UP, FRAC_PI_3)
}

/// The mill: the cylinder of radius 1 about its axis from below the dome's lowest point up to
/// z = 3, kept where it is in the ball of radius 2 about the dome's centre or above that centre.
/// The ball's poles are laid square to the axis, 2 off it, so that both lie outside the cylinder
/// and are cut away: the cap the mill keeps of the sphere has none (a pole is a point the
/// kernel's Booleans do not split a sphere at).
fn mill() -> Result<Brep, String> {
    let [x0, y0, z0] = DOME_CENTRE;
    let below = z0 - DOME - 1.;
    let side = prism(&disc([x0, y0, below], UP, RADIUS), 0., TOP - below)?;
    let ball = sphere(DOME_CENTRE, [0., 1., 0.], DOME)?;
    let above = prism(&disc(DOME_CENTRE, UP, 2. * DOME), 0., TOP + 1. - z0)?;
    let cap = combined(&ball, &above, Op::Union, 0.)?;
    Ok(combined(&side, &cap, Op::Common, 0.)?)
}

pub fn build() -> Result<Brep, String> {
    let blank = sector()?;
    let tool = mill()?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the sector on one arrangement: the cap's meridian
    // in the plane through both axes turned into tori about z, the side's generators into
    // cylinders, the sharp rim a circle square to the axis kept in its plane
    let size = blank.size().max(tool.size());
    cut_from(&blank, &tool, &path, [FROM.to_radians(), TO.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
