//! The solid a rounded block fills as it slides without turning along (8, 5, 3): the box
//! (0, 0, 0)–(10, 6, 4) with every edge rounded to radius 1 and every corner to a ball of radius 1,
//! everything within 1 of the box (1, 1, 1)–(9, 5, 3).
//!
//! cad-kernel's sweep of a solid (`swept::closed::closed`) takes the rounded block under the slide
//! (planes, cylinders and balls) but fails building it (`Unbuilt::Failed`, "a sphere has a hole in
//! no piece"). So the sweep is built in the other order: the block is everything within 1 of the
//! inner box, so what it passes through is everything within 1 of what the inner box passes
//! through — the inner box swept by the kernel's own sweep of a solid, then grown by 1 by its
//! offset.
use crate::sketch::block;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::offset::{grown, Join};
use cad_kernel::swept::closed::slid;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{norm, unit};
use std::f64::consts::TAU;

const ROUND: f64 = 1.;
const SLIDE: V = [8., 5., 3.];

pub fn build() -> Result<Brep, String> {
    let inner = block([1., 1., 1.], [9., 5., 3.])?;
    // the slide along (8, 5, 3): a screw that does not turn, advancing its whole length over a
    // roll of τ
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: unit(SLIDE), ratio: 0., advance: norm(SLIDE),
    });
    // the kernel's closed swept solid of the inner box (its flat faces mesh exactly, at any bar)
    let size = inner.size();
    let swept = slid(&inner, &path, [0., TAU], size, tolerance(size))
        .map_err(|why| format!("the inner box's sweep not built: {why:?}"))?;
    // everything within 1 of it: the kernel's offset with round joins
    grown(&swept, ROUND, Join::Round)
}
