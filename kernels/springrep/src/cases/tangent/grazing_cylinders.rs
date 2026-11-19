//! The part common to two cylinders of radius 5 and length 30 that cross at right angles and
//! overlap only 0.01 deep: one along x, one parallel to y about the line x = 0, z = 9.99.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::rod;

pub fn build() -> Result<Brep, String> {
    let along_x = rod([-15., 0., 0.], [1., 0., 0.], [0., 0., -1.], 5., 30., "along_x")?;
    let along_y = rod([0., -15., 9.99], [0., 1., 0.], [0., 0., 1.], 5., 30., "along_y")?;
    intersection(&along_x, &along_y).map_err(e)
}
