//! A block, the box (0, −10, −10)–(30, 10, 0), engraved along x by a V bit — the solid cone about
//! the vertical line through (5, 0), its point at (5, 0, −2), opening at 90° to a flat top of
//! radius 5 at z = 3 — slid without turning 15 along x, its point inside the block at both ends.
use crate::sketch::block;
use cad_kernel::build::cone;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const POINT: V = [5., 0., -2.];
const TOP_RADIUS: f64 = 5.;
const HEIGHT: f64 = 5.;
const SLIDE: f64 = 15.;
/// How closely the bit is meshed for the sweep's membership reads.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let stock = block([0., -10., -10.], [30., 10., 0.])?;
    // the cone from its point (radius nothing) up to its flat top
    let bit = cone(POINT, [0., 0., 1.], 0., TOP_RADIUS, HEIGHT)?;
    // the slide along x: a screw that does not turn, advancing its whole length over a roll of τ
    let path = Trajectory::Twist(Screw {
        origin: POINT, axis: [1., 0., 0.], ratio: 0., advance: SLIDE,
    });
    // the kernel's swept solid cut from the block on one arrangement: the cone at both end poses
    // and its two rulings square to the slide swept into the groove's flanks
    let size = stock.size().max(bit.size());
    cut_from(&stock, &bit, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the bit's sweep not built: {why:?}"))
}
