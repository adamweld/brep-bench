//! A plate, the box (0, −5, 0)–(20, 5, 4), with a hole bored through it by a drill about the
//! vertical line through (10, 0) — a cylinder of radius 2 up to z = 14, ended below by a cone of
//! 118° whose point is (10, 0, 6) — slid without turning straight down along its axis by 20.
use crate::sketch::{block, polygon};
use cad_kernel::build::revolve;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const PLATE_LO: V = [0., -5., 0.];
const PLATE_HI: V = [20., 5., 4.];
const POINT: V = [10., 0., 6.];
const RADIUS: f64 = 2.;
const TOP: f64 = 14.;
/// Each side of the point's cone from its axis, in degrees.
const HALF_ANGLE: f64 = 59.;
const PUSH: f64 = 20.;
/// How closely the drill is meshed for the sweep's membership reads: a thousandth of its radius.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let plate = block(PLATE_LO, PLATE_HI)?;
    // the drill: its meridian in the half-plane y = 0 beyond its axis — from the point up the
    // cone to the rim, up the side, back across the top and down the axis — turned once
    let [x0, _, z0] = POINT;
    let rim = z0 + RADIUS / HALF_ANGLE.to_radians().tan();
    let meridian = polygon(&[POINT, [x0 + RADIUS, 0., rim], [x0 + RADIUS, 0., TOP], [x0, 0., TOP]]);
    let up = [0., 0., 1.];
    let drill = revolve(&meridian, POINT, up, TAU)?;
    // the slide down its axis: a screw that does not turn, advancing its whole length over a roll
    // of τ
    let down = [0., 0., -1.];
    let path = Trajectory::Twist(Screw { origin: POINT, axis: down, ratio: 0., advance: PUSH });
    // the kernel's closed swept solid cut from the plate on one arrangement: the side, a cylinder
    // along the slide, swept along itself, and the cone's face at both end poses
    let size = plate.size().max(drill.size());
    cut_from(&plate, &drill, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the drill's sweep not built: {why:?}"))
}
