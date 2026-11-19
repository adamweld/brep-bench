//! Two balls of radius 5 about the origin and (4.8, 0, 3.6), poles upright, united: they meet in
//! a circle through a pole of each.
use springrep::boolean::union;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let a = sphere(&at([0., 0., 0.]), 5., "a").map_err(e)?;
    let b = sphere(&at([4.8, 0., 3.6]), 5., "b").map_err(e)?;
    union(&a, &b).map_err(e)
}
