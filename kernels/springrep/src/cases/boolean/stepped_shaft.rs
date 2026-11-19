//! A stepped shaft on the z axis: a cylinder of radius 5 from z = 0 to 10 with one of radius 3 from
//! z = 10 to 20 standing on it, made one solid.
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let lower = cylinder(&at([0., 0., 0.]), 5., 10., "lower").map_err(e)?;
    let upper = cylinder(&at([0., 0., 10.]), 3., 10., "upper").map_err(e)?;
    union(&lower, &upper).map_err(e)
}
