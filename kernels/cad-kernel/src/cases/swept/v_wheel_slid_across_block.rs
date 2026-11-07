//! A block, the box (0, −5, 0)–(20, 5, 10), with a V groove cut across its top by a wheel — the
//! triangle (0, 8), (∓3 tan 30°, 11) in (y, z) in the plane x = −8, turned about the line through
//! (−8, 0, 14) along y: a sharp rim of radius 6, flanks leaning 30° either side, a bore of radius
//! 3 — slid without turning 36 along x.
use crate::sketch::{block, polygon};
use cad_kernel::build::revolve;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const CENTRE: V = [-8., 0., 14.];
const SLIDE: f64 = 36.;
/// How closely the wheel is meshed for the sweep's membership reads.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let stock = block([0., -5., 0.], [20., 5., 10.])?;
    let half = 3. * 30f64.to_radians().tan();
    let triangle = polygon(&[[-8., 0., 8.], [-8., half, 11.], [-8., -half, 11.]]);
    // the wheel: the triangle turned once about its axis, two cones meeting in the sharp rim and
    // the bore's cylinder
    let wheel = revolve(&triangle, CENTRE, [0., 1., 0.], TAU)?;
    // the slide along x: a screw that does not turn, advancing its whole length over a roll of τ
    let path = Trajectory::Twist(Screw {
        origin: CENTRE, axis: [1., 0., 0.], ratio: 0., advance: SLIDE,
    });
    // the kernel's swept solid cut from the block on one arrangement: each flank's ruling square
    // to the slide swept into a plane, the rim's circle swept into an oblique cylinder
    let size = stock.size().max(wheel.size());
    cut_from(&stock, &wheel, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the wheel's sweep not built: {why:?}"))
}
