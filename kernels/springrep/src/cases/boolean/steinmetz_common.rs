//! The Steinmetz solid: the part common to two cylinders of radius 5 and length 30, one along x and
//! one along y, both centred on the origin.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::boolean::rod;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let along_x = rod([-15., 0., 0.], [1., 0., 0.], 5., 30., "along_x")?;
    let along_y = rod([0., -15., 0.], [0., 1., 0.], 5., 30., "along_y")?;
    intersection(&along_x, &along_y).map_err(e)
}
