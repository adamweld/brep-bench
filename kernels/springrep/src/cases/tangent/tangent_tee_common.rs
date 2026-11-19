//! Two cylinders crossing at right angles, of radius 5 along x and of radius 3 parallel to y about
//! the line x = 0, z = 2, both 30 long: their tops touch at (0, 0, 5). Here, the part common to both.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::rod;

pub fn build() -> Result<Brep, String> {
    // the seams on the bottoms, away from the point of contact
    let main = rod([-15., 0., 0.], [1., 0., 0.], [0., 0., -1.], 5., 30., "main")?;
    let branch = rod([0., -15., 2.], [0., 1., 0.], [0., 0., -1.], 3., 30., "branch")?;
    intersection(&main, &branch).map_err(e)
}
