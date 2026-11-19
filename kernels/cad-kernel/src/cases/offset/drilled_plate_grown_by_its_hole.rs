//! The plate from (−10, −10, 0) to (10, 10, 4) drilled through along z with a hole of radius 3,
//! grown by 3, the hole's radius: the hole fills, and a dimple stays over each of its ends.
//!
//! Refused, by the kernel: "the join of `far` (plane) and face 6 (cylinder) united: a shell of the
//! arrangement is kept on one side of a plane (`near`) and dropped on another: two of its cells
//! joined round an edge". Grown by 3 the hole's wall offsets onto its own axis and the round over
//! its rim is a horn torus meeting itself there, and the union of the grown faces is refused. By 1
//! (`offset/drilled-plate-grown`) or by 4 (`offset/drilled-plate-grown-past-its-hole`) it builds.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const GROWTH: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let plate = block([-10., -10., 0.], [10., 10., 4.])?;
    // the drill runs 1 past both faces of the plate
    let drill = prism(&disc([0., 0., -1.], [0., 0., 1.], 3.), 0., 6.)?;
    grown(&combined(&plate, &drill, Op::Cut, 0.)?, GROWTH, Join::Round)
}
