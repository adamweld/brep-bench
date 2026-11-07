//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) notched by a
//! bull-nose mill about the vertical line through (10, 0), from z = −0.5 to 0.5 — radius 1, its
//! bottom corner rounded to 0.3 — turned rigidly about z, right-handed, from −140° to +30°: it
//! starts clear and stops halfway across.
//!
//! Refused by the kernel: the closed sweep's arrangement fails to meet a traced curve with a plane
//! (`Unbuilt::Failed`: the meeting was not proved, the search reading more boxes than any may).
use crate::sketch::polygon;
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, TAU};

const UP: V = [0., 0., 1.];
/// The middle of the mill's flat bottom.
const FOOT: V = [10., 0., -0.5];
const RADIUS: f64 = 1.;
const CORNER: f64 = 0.3;
const HEIGHT: f64 = 1.;
/// The turn's start and stop, in degrees.
const FROM: f64 = -140.;
const TO: f64 = 30.;
/// How closely the mill is meshed for the sweep's membership reads: a hundred-and-fiftieth of its
/// corner's radius.
const BAR: f64 = 0.002;

/// The ring sector: its section in the half-plane at 0° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0.; 3], UP, FRAC_PI_3)
}

/// The mill: its meridian in the half-plane y = 0 beyond its axis — out along its flat bottom,
/// round the quarter circle of its corner, up its side, back across its top and down its axis —
/// turned once about the axis.
fn mill() -> Result<Brep, String> {
    let at = |x: f64, z: f64| [FOOT[0] + x, 0., FOOT[2] + z];
    let corner = Frame::new(at(RADIUS - CORNER, CORNER), [0., -1., 0.], [1., 0., 0.]);
    let edges = vec![
        ProfileEdge::Line { a: at(0., 0.), b: at(RADIUS - CORNER, 0.) },
        ProfileEdge::Arc { frame: corner, r: CORNER, span: Some([-FRAC_PI_2, 0.]) },
        ProfileEdge::Line { a: at(RADIUS, CORNER), b: at(RADIUS, HEIGHT) },
        ProfileEdge::Line { a: at(RADIUS, HEIGHT), b: at(0., HEIGHT) },
        ProfileEdge::Line { a: at(0., HEIGHT), b: at(0., 0.) },
    ];
    let meridian = Profile { origin: FOOT, normal: [0., 1., 0.], loops: vec![edges],
                             names: Vec::new() };
    revolve(&meridian, FOOT, UP, TAU)
}

pub fn build() -> Result<Brep, String> {
    let blank = sector()?;
    let tool = mill()?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the sector on one arrangement: a mill about an axis
    // along the turn's, its side's generators and its round's meridians in the plane through both
    // axes turned into cylinders and tori about z, its flat faces kept in their planes
    let size = blank.size().max(tool.size());
    cut_from(&blank, &tool, &path, [FROM.to_radians(), TO.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
