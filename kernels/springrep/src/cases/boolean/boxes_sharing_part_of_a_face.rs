//! The cube [0, 10]³ with the cube [5, 15]×[0, 10]×[10, 20] standing on it, half overhanging,
//! joined into one solid.
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let first = between([0., 0., 0.], [10., 10., 10.], "first")?;
    let second = between([5., 0., 10.], [15., 10., 20.], "second")?;
    union(&first, &second).map_err(e)
}
