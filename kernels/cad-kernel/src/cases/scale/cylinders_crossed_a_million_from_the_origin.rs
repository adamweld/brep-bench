//! The solid common to two cylinders of radius 3 and length 20 crossing square at
//! c = (600000, −800000, 0), one along (2, 3, 6)/7 and one along (3, −6, 2)/7, each running 10 either
//! way from c.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{scale, sub, unit};

const CENTRE: V = [600000., -800000., 0.];
const RADIUS: f64 = 3.;
const HALF_LENGTH: f64 = 10.;

/// The cylinder along `direction` from `CENTRE` − 10u to `CENTRE` + 10u: a disc at its near end,
/// square to its axis, swept along it.
fn cylinder(direction: V) -> Result<Brep, String> {
    let u = unit(direction);
    prism(&disc(sub(CENTRE, scale(u, HALF_LENGTH)), u, RADIUS), 0., 2. * HALF_LENGTH)
}

pub fn build() -> Result<Brep, String> {
    let (first, second) = (cylinder([2., 3., 6.])?, cylinder([3., -6., 2.])?);
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
