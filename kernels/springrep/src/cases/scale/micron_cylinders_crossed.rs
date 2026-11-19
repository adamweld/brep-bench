//! The Steinmetz solid common to two cylinders of radius 0.003 and length 0.02, one along x and one
//! along y, both centred on the origin.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::boolean::rod;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let along_x = rod([-0.01, 0., 0.], [1., 0., 0.], 0.003, 0.02, "along_x")?;
    let along_y = rod([0., -0.01, 0.], [0., 1., 0.], 0.003, 0.02, "along_y")?;
    intersection(&along_x, &along_y).map_err(e)
}
