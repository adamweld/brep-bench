//! A groin vault: the common part of the block on −12 ≤ x, y ≤ 12 under a weighted rational
//! barrel vault arching across y, and the same block turned a quarter turn about z.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::nurbs::vault;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // sketched on the plane x = −12 (sketch x along y, sketch y up) and extruded along +x
    let along_x = vault(&frame([-12., 0., 0.], [1., 0., 0.], [0., 1., 0.])?)?;
    // the same turned: sketched on y = −12 (sketch x along −x) and extruded along +y
    let along_y = vault(&frame([0., -12., 0.], [0., 1., 0.], [-1., 0., 0.])?)?;
    intersection(&along_x, &along_y).map_err(e)
}
