//! A plate, the box (−15, −15, 0)–(15, 15, 4), with a hole punched through it by a tilted box —
//! everything (−2, −1, −8) + a e₁ + b e₂ + c e₃, |a| ≤ 2, |b| ≤ 1.5, |c| ≤ 1, along
//! e₁ = (2, 1, 2)/3, e₂ = (−2, 2, 1)/3, e₃ = (1, 2, −2)/3 — slid without
//! turning along (4, 2, 18), wholly through it.
use crate::sketch::{block, polygon};
use cad_kernel::build::prism;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, norm, scale, unit};
use std::f64::consts::TAU;

const CENTRE: V = [-2., -1., -8.];
const E1: V = [2. / 3., 1. / 3., 2. / 3.];
const E2: V = [-2. / 3., 2. / 3., 1. / 3.];
/// The box's half-extents along e₁, e₂ and e₃.
const HALF: [f64; 3] = [2., 1.5, 1.];
const PUSH: V = [4., 2., 18.];

pub fn build() -> Result<Brep, String> {
    let plate = block([-15., -15., 0.], [15., 15., 4.])?;
    // the box's middle section, the rectangle across e₁ and e₂ through its centre, swept 1 each
    // way along its normal (±e₃)
    let at = |a: f64, b: f64| add(CENTRE, add(scale(E1, a * HALF[0]), scale(E2, b * HALF[1])));
    let section = polygon(&[at(-1., -1.), at(1., -1.), at(1., 1.), at(-1., 1.)]);
    let tool = prism(&section, -HALF[2], HALF[2])?;
    // the push along (4, 2, 18): a screw that does not turn, advancing its whole length over a
    // roll of τ
    let path = Trajectory::Twist(Screw {
        origin: CENTRE, axis: unit(PUSH), ratio: 0., advance: norm(PUSH),
    });
    // the kernel's swept solid cut from the plate on one arrangement (a box's flat faces mesh
    // exactly, at any bar)
    let size = plate.size().max(tool.size());
    cut_from(&plate, &tool, &path, [0., TAU], tool.size(), tolerance(size))
        .map_err(|why| format!("the box's sweep not built: {why:?}"))
}
