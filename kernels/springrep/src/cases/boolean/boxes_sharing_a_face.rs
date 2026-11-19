//! Two cubes of side 10 side by side along x, [0, 10]³ and [10, 20]×[0, 10]², joined into one
//! block: the face they share goes.
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let first = between([0., 0., 0.], [10., 10., 10.], "first")?;
    let second = between([10., 0., 0.], [20., 10., 10.], "second")?;
    union(&first, &second).map_err(e)
}
