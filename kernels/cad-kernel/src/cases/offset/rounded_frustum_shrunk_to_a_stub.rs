//! The cone frustum on the z axis from the disk of radius 6 at z = 0 to the disk of radius 3 at
//! z = 6, both its rims rounded to radius 1.5, shrunk by 2.5: its points at least 2.5 inside it.
//!
//! The kernel's offset sweeps a round's torus inward only as far as its tube's radius: past it
//! the torus would turn inside out, and the offset refuses it rather than drop it. A solid shrunk
//! by a and then by b is the solid shrunk by a + b (the points a ball of radius a + b about them
//! fits in), so it is shrunk first by the round's radius, which takes the round down to a sharp
//! edge, and then by the rest.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

const FOOT: f64 = 6.;
const TOP: f64 = 3.;
const HEIGHT: f64 = 6.;
const ROUND: f64 = 1.5;
const SHRINK: f64 = 2.5;

/// `part` with the edge through `at` rounded to `ROUND`: the kernel's piece taken off a convex
/// edge, filled in at a concave one.
fn rounded(part: &Brep, at: V) -> Result<Brep, String> {
    let round = roll(part, at, ROUND, tolerance(part.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(part, &round.piece, op, 0.)?)
}

pub fn build() -> Result<Brep, String> {
    let frustum = cone([0., 0., 0.], [0., 0., 1.], FOOT, TOP, HEIGHT)?;
    // each rim is picked by a point on it, the top's and then the bottom's
    let frustum = rounded(&rounded(&frustum, [0., TOP, HEIGHT])?, [0., FOOT, 0.])?;
    shrunk(&shrunk(&frustum, ROUND, Join::Round)?, SHRINK - ROUND, Join::Round)
}
