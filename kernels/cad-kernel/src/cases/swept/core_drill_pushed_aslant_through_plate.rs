//! A plate, the box (−10, −10, 0)–(10, 10, 4), cut right through by a core drill: the tube between
//! 1.5 and 2.5 from the line through (0, 0, 2) along u = (sin 30°, 0, cos 30°), running along it
//! from 18 to 6 before (0, 0, 2), its ends square to it, slid along u by 24. The core inside it is
//! cut free and stays: two solids.
use crate::sketch::block;
use cad_kernel::build::{disc, prism, Profile};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, scale};
use std::f64::consts::TAU;

const PLATE_LO: V = [-10., -10., 0.];
const PLATE_HI: V = [10., 10., 4.];
/// The point of the drill's axis it is placed about, and how far the axis leans toward +x.
const CENTRE: V = [0., 0., 2.];
const LEAN: f64 = 30.;
const RADII: [f64; 2] = [1.5, 2.5];
/// Where the tube starts along its axis from the centre, its length and the push.
const START: f64 = -18.;
const LENGTH: f64 = 12.;
const PUSH: f64 = 24.;
/// How closely the tube is meshed for the sweep's membership reads.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let plate = block(PLATE_LO, PLATE_HI)?;
    let lean = LEAN.to_radians();
    let u = [lean.sin(), 0., lean.cos()];
    // the tube: the annulus between its radii square to u, its outer circle and its hole, swept 12
    // along u
    let foot = add(CENTRE, scale(u, START));
    let [inner, outer] = RADII;
    let loops = vec![disc(foot, u, outer).loops.remove(0), disc(foot, u, inner).loops.remove(0)];
    let tube = prism(&Profile { origin: foot, normal: u, loops, names: Vec::new() }, 0., LENGTH)?;
    // the slide along u: a screw that does not turn, advancing its whole length over a roll of τ
    let path = Trajectory::Twist(Screw { origin: CENTRE, axis: u, ratio: 0., advance: PUSH });
    // the kernel's closed swept solid cut from the plate on one arrangement: each of the tube's
    // cylinders along the slide swept along itself, its flat ends at both end poses
    let size = plate.size().max(tube.size());
    cut_from(&plate, &tube, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the drill's sweep not built: {why:?}"))
}
