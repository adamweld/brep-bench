//! The part common to two blocks 10 high on the xy plane, one over [0, 10]² and one over [5, 15]².
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let first = between([0., 0., 0.], [10., 10., 10.], "first")?;
    let second = between([5., 5., 0.], [15., 15., 10.], "second")?;
    intersection(&first, &second).map_err(e)
}
