//! The L-block, the union of the boxes from the origin to (20, 10, 6) and to (6, 10, 20), its
//! concave edge x = 6, z = 6 rounded to radius 2, then the chain that leaves on the end face y = 0
//! (up the upright, round the concave round's quarter circle, along the foot) rounded to radius 2,
//! convex: a quarter torus where the convex round turns about the concave one.
//!
//! Refused, by the kernel, at the second step: the concave round is rolled and added, but the
//! chain's first stretch, up the upright, is not — "the fillet of `face 9` with `face 10` runs on
//! smoothly into `face 15`, but its contact crosses their seam in another section than its edge
//! does (2.00e0 off it there): no one section joins the run to the piece going on". The ball
//! rolling down the upright's edge reaches the concave round's tangent seam in one section and
//! touches it in another, so no run of `fillet::roll` ends where the torus about the concave
//! round would begin.
use crate::sketch::{block, named};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const FOOT: V = [20., 10., 6.];
const UPRIGHT: V = [6., 10., 20.];
const ROUND: f64 = 2.;

/// The round of the edge of `b` nearest `at` taken off (convex) or added (concave). Every face is
/// named apart first: a roll reads faces of one name as pieces of one face, and the faces the
/// Booleans leave share theirs.
fn rounded(b: &Brep, at: V) -> Result<Brep, String> {
    let b = &named(b.clone());
    let round = roll(b, at, ROUND, tolerance(b.size()))
        .map_err(|e| format!("the edge at {at:?}: {e}"))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    combined(b, &round.piece, op, 0.).map_err(|e| format!("the round at {at:?} applied: {e}"))
}

pub fn build() -> Result<Brep, String> {
    let l = combined(&block([0.; 3], FOOT)?, &block([0.; 3], UPRIGHT)?, Op::Union, 0.)?;
    // 1. the concave edge, picked by its middle
    let (x, z) = (UPRIGHT[0], FOOT[2]);
    let l = rounded(&l, [x, FOOT[1] / 2., z])?;
    // 2. the chain on y = 0 left by it: fillet::roll rolls the ball along the run between two
    // faces, which here is one edge at a time — up the upright, round the quarter circle about
    // (8, 0, 8), along the foot — so each is rolled in turn, picked by its middle
    let c = [x + ROUND, 0., z + ROUND];
    let chain = [
        [x, 0., 0.5 * (c[2] + UPRIGHT[2])],
        [c[0] - ROUND * FRAC_1_SQRT_2, 0., c[2] - ROUND * FRAC_1_SQRT_2],
        [0.5 * (c[0] + FOOT[0]), 0., z],
    ];
    chain.into_iter().try_fold(l, |b, at| rounded(&b, at))
}
