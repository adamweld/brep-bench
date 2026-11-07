//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) notched by a drill
//! about the vertical line through (10, 0): a cylinder of radius 1 up to z = 3, ended below by a
//! cone of 118° whose point is (10, 0, −0.8). It turns rigidly about z, right-handed, from −140° to
//! +30°: it starts clear and stops halfway across.
//!
//! Refused by the kernel: the closed sweep's arrangement fails to close the pieces of a cone face
//! (`Unbuilt::Failed`: "a cone's pieces do not close").
use crate::sketch::polygon;
use cad_kernel::build::revolve;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::{FRAC_PI_3, TAU};

const UP: V = [0., 0., 1.];
const POINT: V = [10., 0., -0.8];
const RADIUS: f64 = 1.;
const TOP: f64 = 3.;
/// Each side of the point's cone from its axis, in degrees.
const HALF_ANGLE: f64 = 59.;
/// The turn's start and stop, in degrees.
const FROM: f64 = -140.;
const TO: f64 = 30.;
/// How closely the drill is meshed for the sweep's membership reads: a five-hundredth of its
/// radius.
const BAR: f64 = 0.002;

/// The ring sector: its section in the half-plane at 0° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0.; 3], UP, FRAC_PI_3)
}

pub fn build() -> Result<Brep, String> {
    let blank = sector()?;
    // the drill: its meridian in the half-plane y = 0 beyond its axis — from the point up the
    // cone to the rim, up the side, back across the top and down the axis — turned once
    let [x0, _, z0] = POINT;
    let rim = z0 + RADIUS / HALF_ANGLE.to_radians().tan();
    let meridian = polygon(&[POINT, [x0 + RADIUS, 0., rim], [x0 + RADIUS, 0., TOP], [x0, 0., TOP]]);
    let drill = revolve(&meridian, POINT, UP, TAU)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the sector on one arrangement: the cone's and the
    // side's generators in the plane through both axes turned into a cone and cylinders about z,
    // the sharp rim a circle square to the axis kept in its plane
    let size = blank.size().max(drill.size());
    cut_from(&blank, &drill, &path, [FROM.to_radians(), TO.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the drill's sweep not built: {why:?}"))
}
