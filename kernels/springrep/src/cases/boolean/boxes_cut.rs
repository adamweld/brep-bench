//! A block 10 high over [0, 10]² with a block over [5, 15]² cut from it: an L-shaped prism.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let first = between([0., 0., 0.], [10., 10., 10.], "first")?;
    let second = between([5., 5., 0.], [15., 15., 10.], "second")?;
    difference(&first, &second).map_err(e)
}
