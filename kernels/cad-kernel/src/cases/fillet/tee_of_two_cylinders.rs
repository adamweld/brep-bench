//! A tee: the cylinder of radius 6 about the x axis from x = −20 to 20, joined by the cylinder of
//! radius 3 about the z axis from z = 0 to 15; the saddle-shaped junction where their sides meet
//! rounded to radius 1, concave.
//!
//! Unsettled. The kernel traces the ball's centre where the two cylinders' offsets meet and fits
//! the canal as a B-spline net within its tolerance.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const BAR_RADIUS: f64 = 6.;
const BAR_HALF: f64 = 20.;
const STEM_RADIUS: f64 = 3.;
const STEM_TOP: f64 = 15.;
const ROUND: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let bar = prism(&disc([-BAR_HALF, 0., 0.], [1., 0., 0.], BAR_RADIUS), 0., 2. * BAR_HALF)?;
    let stem = prism(&disc([0., 0., 0.], [0., 0., 1.], STEM_RADIUS), 0., STEM_TOP)?;
    let tee = combined(&bar, &stem, Op::Union, 0.)?;
    // the junction picked where it crosses y = 0: the stem's side at x = 3 meets the bar's at
    // z = √(6² − 0²) = 6; the kernel traces the ball's centre round the closed loop
    let round = roll(&tee, [STEM_RADIUS, 0., BAR_RADIUS], ROUND, tolerance(tee.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&tee, &round.piece, op, 0.)?)
}
