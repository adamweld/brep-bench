//! The cube of side 0.001 from the origin and a copy of it turned 1.2° about the z axis, the line
//! of one of its edges: the part common to both.
use springrep::boolean::intersection;
use springrep::math::{Transform, Vec3};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::between;

pub fn build() -> Result<Brep, String> {
    let cube = between([0., 0., 0.], [0.001, 0.001, 0.001], "cube")?;
    let mut turned = cube.clone();
    turned.transform(&Transform::rotation(Vec3::ZERO, Vec3::Z, 1.2f64.to_radians()));
    intersection(&cube, &turned).map_err(e)
}
