//! The cube [0, 10]³ with a saw cut 10⁻⁶ wide down into it from the top: the slot between x = 5 and
//! 5.000001, through the cube in y, from z = 5 up.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::between;

pub fn build() -> Result<Brep, String> {
    let cube = between([0., 0., 0.], [10., 10., 10.], "cube")?;
    // the saw runs past the cube's front, back and top
    let saw = between([5., -1., 5.], [5.000001, 11., 11.], "slot")?;
    difference(&cube, &saw).map_err(e)
}
