//! The frustum from radius 6 at z = 0 to 3 at z = 6, its top rim beveled by a steeper cone from
//! radius 3.5 at z = 5 to 2 at z = 6; the bevel then deleted, the side and the top meeting again.
use springrep::boolean::union;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the bevel's two distances differ, so it is drawn as the two frustums it makes
    let frustum = cone(&at([0., 0., 0.]), 6., 3.5, 5., "frustum").map_err(e)?;
    let bevel = cone(&at([0., 0., 5.]), 3.5, 2., 1., "bevel").map_err(e)?;
    let part = union(&frustum, &bevel).map_err(e)?;
    delete_named(&part, &["bevel.side"])
}
