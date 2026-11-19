//! The cube of side 0.0015 from the origin and a copy of it raised 0.000015 along z, a hundredth of
//! its side: the cube less the copy.
use springrep::boolean::difference;
use springrep::math::{Transform, Vec3};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::between;

pub fn build() -> Result<Brep, String> {
    let cube = between([0., 0., 0.], [0.0015, 0.0015, 0.0015], "cube")?;
    let mut raised = cube.clone();
    raised.transform(&Transform::translation(Vec3::new(0., 0., 0.000015)));
    difference(&cube, &raised).map_err(e)
}
