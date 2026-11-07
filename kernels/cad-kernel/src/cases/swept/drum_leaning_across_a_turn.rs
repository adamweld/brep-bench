//! A ring — everything 3 to 8 from the z axis with z from −2.5 to 2.5 — with a sealed channel cut
//! inside it by a drum, the cylinder of radius 1 and length 3 whose middle is (5, 1.2, 0) and whose
//! axis leans 25° from upright along (sin 25°, 0, cos 25°), turned rigidly about z, right-handed,
//! by 100°.
use cad_kernel::build::{disc, prism, Profile};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{scale, sub};

const UP: V = [0., 0., 1.];
/// The ring: its radii and its half-height.
const RADII: [f64; 2] = [3., 8.];
const HALF_HEIGHT: f64 = 2.5;
const MIDDLE: V = [5., 1.2, 0.];
const RADIUS: f64 = 1.;
const LENGTH: f64 = 3.;
/// How far the drum's axis leans from upright toward +x, in degrees.
const LEAN: f64 = 25.;
/// The turn, in degrees.
const TURN: f64 = 100.;
/// How closely the drum is meshed for the sweep's membership reads: a five-hundredth of its radius.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    // the ring: the annulus between its radii, its outer circle and its hole, swept up
    let foot = [0., 0., -HALF_HEIGHT];
    let [inner, outer] = RADII;
    let loops = vec![disc(foot, UP, outer).loops.remove(0), disc(foot, UP, inner).loops.remove(0)];
    let annulus = Profile { origin: foot, normal: UP, loops, names: Vec::new() };
    let ring = prism(&annulus, 0., 2. * HALF_HEIGHT)?;
    let lean = LEAN.to_radians();
    let along = [lean.sin(), 0., lean.cos()];
    let drum = prism(&disc(sub(MIDDLE, scale(along, LENGTH / 2.)), along, RADIUS), 0., LENGTH)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the ring on one arrangement: the side, a cylinder
    // about an axis leaning to the turn's, touching along two curves each turned into a surface of
    // revolution, its rims sharp circles leaning to the axis each turned into its ring, the flat
    // ends' contact lines turned
    let size = ring.size().max(drum.size());
    cut_from(&ring, &drum, &path, [0., TURN.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the drum's sweep not built: {why:?}"))
}
