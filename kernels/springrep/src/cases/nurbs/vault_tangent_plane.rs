//! The block on −12 ≤ x, y ≤ 12 under a weighted rational barrel vault, cut by the plane z = 16
//! that touches the vault along its crown.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::nurbs::vault;
use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let vaulted = vault(&frame([-12., 0., 0.], [1., 0., 0.], [0., 1., 0.])?)?;
    let above = block(&at([-20., -20., 16.]), Vec3::new(40., 40., 10.), "cut").map_err(e)?;
    difference(&vaulted, &above).map_err(e)
}
