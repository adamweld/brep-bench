//! A ring sector (radii 8 to 12 about z, z from −2 to 2, polar angle −30° to 30°) with a helical
//! groove milled across it by a ball-end mill about the vertical line through (10, 0) — the lower
//! half of the ball of radius 1 about (10, 0, 0) under the cylinder of radius 1 from z = 0 to 3,
//! flat on top — screwed about z, right-handed and rising 3 a turn, from a turn of −60° to +60°.
//!
//! Refused by the kernel: the closed sweep's arrangement fails to close the pieces of a sphere face
//! (`Unbuilt::Failed`: "a sphere's pieces do not close").
use crate::sketch::polygon;
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::{FRAC_PI_2, TAU};

const UP: V = [0., 0., 1.];
/// The sector: its radii, its half-height and its half-angle (degrees) about +x.
const RADII: [f64; 2] = [8., 12.];
const HALF_HEIGHT: f64 = 2.;
const HALF_ANGLE: f64 = 30.;
/// The ball's centre, on the mill's axis, the cylinder's radius and its top.
const CENTRE: V = [10., 0., 0.];
const RADIUS: f64 = 1.;
const TOP: f64 = 3.;
/// How far the screw rises a whole turn.
const RISE: f64 = 3.;
/// How far the screw turns each way from where the mill is drawn, in degrees.
const TURN: f64 = 60.;
/// How closely the mill is meshed for the sweep's membership reads: a five-hundredth of its radius.
const BAR: f64 = 0.002;

/// The ring sector: its section in the half-plane at −30° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let (s, c) = (-HALF_ANGLE).to_radians().sin_cos();
    let at = |r: f64, z: f64| [r * c, r * s, z];
    let [inner, outer] = RADII;
    let section = polygon(&[at(inner, -HALF_HEIGHT), at(outer, -HALF_HEIGHT),
                            at(outer, HALF_HEIGHT), at(inner, HALF_HEIGHT)]);
    revolve(&section, [0.; 3], UP, 2. * HALF_ANGLE.to_radians())
}

/// The mill: its meridian in the half-plane y = 0 beyond its axis — from the ball's lowest point
/// round a quarter circle to its widest, up the side, back across the top and down the axis —
/// turned once about the axis.
fn mill() -> Result<Brep, String> {
    let at = |x: f64, z: f64| [CENTRE[0] + x, 0., z];
    let ball = Frame::new(CENTRE, [0., -1., 0.], [1., 0., 0.]);
    let edges = vec![
        ProfileEdge::Arc { frame: ball, r: RADIUS, span: Some([-FRAC_PI_2, 0.]) },
        ProfileEdge::Line { a: at(RADIUS, CENTRE[2]), b: at(RADIUS, TOP) },
        ProfileEdge::Line { a: at(RADIUS, TOP), b: at(0., TOP) },
        ProfileEdge::Line { a: at(0., TOP), b: at(0., CENTRE[2] - RADIUS) },
    ];
    let meridian = Profile { origin: CENTRE, normal: [0., 1., 0.], loops: vec![edges],
                             names: Vec::new() };
    revolve(&meridian, CENTRE, UP, TAU)
}

pub fn build() -> Result<Brep, String> {
    let blank = sector()?;
    let tool = mill()?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: RISE });
    // the kernel's closed swept solid cut from the sector on one arrangement: the ball's great
    // circle square to its centre's velocity screwed into the pipe about the centre's helix, the
    // side's generators in the plane through both axes into coaxial cylinders, the top's rim's
    // disc into its ramp
    let size = blank.size().max(tool.size());
    let turn = TURN.to_radians();
    cut_from(&blank, &tool, &path, [-turn, turn], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
