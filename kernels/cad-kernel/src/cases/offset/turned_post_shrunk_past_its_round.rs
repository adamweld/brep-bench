//! A turned post on the z axis, the cylinder of radius 5 from z = 0 to z = 6 topped by the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, the convex edge between them rounded to
//! radius 1, shrunk by 1.5: every point of it at least 1.5 inside it.
//!
//! The kernel's offset sweeps a round's torus inward only as far as its tube's radius: past it
//! the torus would turn inside out, and the offset refuses it rather than drop it. A solid shrunk
//! by a and then by b is the solid shrunk by a + b (the points a ball of radius a + b about them
//! fits in), so it is shrunk first by the round's radius, which takes the round down to a sharp
//! edge, and then by the rest.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const SHOULDER: f64 = 6.;
const TOP_RADIUS: f64 = 2.;
const TOP: f64 = 10.;
const ROUND: f64 = 1.;
const SHRINK: f64 = 1.5;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let body = prism(&disc([0., 0., 0.], up, RADIUS), 0., SHOULDER)?;
    let head = cone([0., 0., SHOULDER], up, RADIUS, TOP_RADIUS, TOP - SHOULDER)?;
    let post = combined(&body, &head, Op::Union, 0.)?;
    // the shoulder is picked by a point on it; the kernel rolls the ball between the cylinder and
    // the cone round the loop and builds the ring of material outside the round
    let round = roll(&post, [0., RADIUS, SHOULDER], ROUND, tolerance(post.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    let turned = combined(&post, &round.piece, op, 0.)?;
    shrunk(&shrunk(&turned, ROUND, Join::Round)?, SHRINK - ROUND, Join::Round)
}
