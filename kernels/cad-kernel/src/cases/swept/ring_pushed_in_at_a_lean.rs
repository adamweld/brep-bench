//! A block, the box (−8, −8, −8)–(8, 8, 0), with a hole bored into its top by a ring — the torus
//! centred at (0, 0, 4) about (sin 35°, 0, cos 35°), of radii 3 and 1 — slid straight down 8
//! without turning, stopping wholly inside the block. The ring leans 35° from upright,
//! below tan² β = (R − r)/r, its inner outline's fold.
//!
//! Built by the kernel's sweep of a ring cut from a blank (`swept::closed::ring_cut_from`): a torus
//! is a tool of revolution whose meridian is a loop clear of its axis, so its contact under the
//! push is read meridian by meridian in closed form — two curves, the outer and the inner — and
//! each sweeps a tube, fitted. The block is split by the tubes and the ring where it starts and
//! stops at once, and kept where the sweep is not: no closed solid is built and then subtracted.
use crate::sketch::block;
use cad_kernel::build::torus;
use cad_kernel::geom::V;
use cad_kernel::swept::closed::ring_cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const CENTRE: V = [0., 0., 4.];
const LEAN: f64 = 35f64.to_radians();
const MAJOR: f64 = 3.;
const MINOR: f64 = 1.;
const PUSH: f64 = 8.;
/// How closely the ring is meshed for the sweep's membership reads: a hundredth of its tube's
/// radius.
const BAR: f64 = 0.01;
/// The tolerance the tubes are fitted and met to: a hundred-thousandth of the tube's radius.
const TOL: f64 = 1e-5;

pub fn build() -> Result<Brep, String> {
    let stock = block([-8., -8., -8.], [8., 8., 0.])?;
    let ring = torus(CENTRE, [LEAN.sin(), 0., LEAN.cos()], MAJOR, MINOR)?;
    // the push down: a screw that does not turn, advancing its whole length over a roll of τ
    let path = Trajectory::Twist(Screw {
        origin: CENTRE, axis: [0., 0., -1.], ratio: 0., advance: PUSH,
    });
    ring_cut_from(&stock, &ring, &path, [0., TAU], BAR, TOL)
        .map_err(|why| format!("the ring's sweep not built: {why:?}"))
}
