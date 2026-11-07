//! A block, the box (−8, −8, −8)–(8, 8, 0), with a hole bored into its top by a ring — the torus
//! centred at (0, 0, 4) about (sin 35°, 0, cos 35°), of radii 3 and 1 — slid straight down 8
//! without turning, stopping wholly inside the block. The ring leans 35° from upright,
//! below tan² β = (R − r)/r, its inner outline's fold.
//!
//! Refused by the kernel. `cut_from` takes the ring (a whole torus slid at a lean short of its
//! fold, #805), but the arrangement fails meeting one of its circles with the cylinder over the
//! contour (`Unbuilt::Failed`: where a circle meets an extrusion "was not proved: its search read
//! more boxes than any search may").
use crate::sketch::block;
use cad_kernel::build::torus;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
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

pub fn build() -> Result<Brep, String> {
    let stock = block([-8., -8., -8.], [8., 8., 0.])?;
    let ring = torus(CENTRE, [LEAN.sin(), 0., LEAN.cos()], MAJOR, MINOR)?;
    // the push down: a screw that does not turn, advancing its whole length over a roll of τ
    let path = Trajectory::Twist(Screw {
        origin: CENTRE, axis: [0., 0., -1.], ratio: 0., advance: PUSH,
    });
    // the kernel's swept solid cut from the block on one arrangement: a whole ring slid at a lean
    // to its axis, each of its two contacts swept along the push into the cylinder over its
    // apparent contour
    let size = stock.size().max(ring.size());
    cut_from(&stock, &ring, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the ring's sweep not built: {why:?}"))
}
