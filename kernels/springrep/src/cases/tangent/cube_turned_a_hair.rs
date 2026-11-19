//! The cube [-5, 5]²×[0, 10] united with a copy of itself turned 0.001° about the z axis: a prism
//! over a sixteen-sided polygon.
use springrep::boolean::union;
use springrep::math::{Transform, Vec3};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::between;

pub fn build() -> Result<Brep, String> {
    let cube = between([-5., -5., 0.], [5., 5., 10.], "cube")?;
    let mut turned = cube.clone();
    turned.transform(&Transform::rotation(Vec3::ZERO, Vec3::Z, 0.001f64.to_radians()));
    union(&cube, &turned).map_err(e)
}
