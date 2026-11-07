//! A block, the box (−10, −10, −10)–(40, 40, 0), with a groove cut into its top by a bull-nose
//! mill — radius 2 from 1.5 below the top to 5 above it, its bottom corner rounded to 0.5 — whose
//! upright axis runs from (0, 0) along x to (20, 0), a quarter turn about the vertical line through
//! (20, 10) to (30, 10), carried rigidly, and along +y to (30, 30).
//!
//! Refused by the kernel, after about five minutes: the solid its arrangement selected disagrees
//! with the sweep's definition at a witness point inside the bend, at the height of the mill's
//! flat bottom (`Unbuilt::Wrong`), and it never returns a solid built wrong.
use crate::sketch::block;
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Piece, Pose, Screw, Trajectory};
use std::f64::consts::{FRAC_PI_2, TAU};

const STOCK_LO: V = [-10., -10., -10.];
const STOCK_HI: V = [40., 40., 0.];
/// The mill: the middle of its flat bottom, its radius, its corner's and its top.
const FOOT: V = [0., 0., -1.5];
const RADIUS: f64 = 2.;
const CORNER: f64 = 0.5;
const TOP: f64 = 5.;
/// The path: the first run's length, the arc's centre and radius, the last run's length.
const FIRST: f64 = 20.;
const ARC_CENTRE: V = [20., 10., 0.];
const ARC_RADIUS: f64 = 10.;
const LAST: f64 = 20.;
/// How closely the mill is meshed for the sweep's membership reads: a two-hundred-and-fiftieth of
/// its corner's radius.
const BAR: f64 = 0.002;

/// The mill about the vertical line through the origin: its meridian in the half-plane y = 0
/// beyond the axis — out along its flat bottom, round its corner, up its side, back across its top
/// and down its axis — turned once about the axis.
fn mill() -> Result<Brep, String> {
    let at = |x: f64, z: f64| [x, 0., z];
    let [_, _, z0] = FOOT;
    let corner = Frame::new(at(RADIUS - CORNER, z0 + CORNER), [0., -1., 0.], [1., 0., 0.]);
    let edges = vec![
        ProfileEdge::Line { a: at(0., z0), b: at(RADIUS - CORNER, z0) },
        ProfileEdge::Arc { frame: corner, r: CORNER, span: Some([-FRAC_PI_2, 0.]) },
        ProfileEdge::Line { a: at(RADIUS, z0 + CORNER), b: at(RADIUS, TOP) },
        ProfileEdge::Line { a: at(RADIUS, TOP), b: at(0., TOP) },
        ProfileEdge::Line { a: at(0., TOP), b: at(0., z0) },
    ];
    let meridian = Profile { origin: FOOT, normal: [0., 1., 0.], loops: vec![edges],
                             names: Vec::new() };
    revolve(&meridian, FOOT, [0., 0., 1.], TAU)
}

/// The constant twist `twist` taken up over the roll from `start` where the piece before it left
/// the tool, `left`: placed so that at `start` it stands at `left` (`Placed` stands `at`, then
/// `twist(t)`).
fn taken_up(twist: Screw, left: Pose, start: f64) -> Trajectory {
    Trajectory::Placed { twist, at: left.then(twist.pose(-start)) }
}

pub fn build() -> Result<Brep, String> {
    let stock = block(STOCK_LO, STOCK_HI)?;
    let tool = mill()?;
    let up = [0., 0., 1.];
    // the roll is the length along the axis's path, each piece a constant twist at unit speed:
    // a slide along x, a turn about the arc's centre of a radian per `ARC_RADIUS`, a slide along y
    let along_x = Screw { origin: [0.; 3], axis: [1., 0., 0.], ratio: 0., advance: TAU };
    let turn = Screw { origin: ARC_CENTRE, axis: up, ratio: 1. / ARC_RADIUS, advance: 0. };
    let along_y = Screw { origin: [0.; 3], axis: [0., 1., 0.], ratio: 0., advance: TAU };
    let bends = [FIRST, FIRST + FRAC_PI_2 * ARC_RADIUS];
    let end = bends[1] + LAST;
    let first = Trajectory::Twist(along_x);
    let second = taken_up(turn, along_x.pose(bends[0]), bends[0]);
    let left = second.pose(bends[1])?;
    let third = taken_up(along_y, left, bends[1]);
    let pieces = vec![Piece { over: [0., bends[0]], path: first },
                      Piece { over: bends, path: second },
                      Piece { over: [bends[1], end], path: third }];
    // the kernel's path of pieces, checked to meet itself at each join
    let size = stock.size().max(tool.size());
    let path = Trajectory::pieces(pieces, tolerance(size), size)?;
    // the kernel's closed swept solid cut from the block on one arrangement, its pieces' sweeps
    // united: under each slide square to the mill's axis its round's top and bottom rings and its
    // meridians square to the slide, under the turn about an axis along its own its meridians in
    // the plane through both axes
    cut_from(&stock, &tool, &path, [0., end], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
