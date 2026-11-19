//! The torus about z (middle radius 10, tube radius 3) united with the slab [-13, 13]²×[-1, 1],
//! whose four sides its outer equator touches at their middles.
use springrep::boolean::union;
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::tangent::between;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the seam turned 45° about z, away from the points of contact
    let ring = torus(&frame([0., 0., 0.], [0., 0., 1.], [1., 1., 0.])?, 10., 3., "torus").map_err(e)?;
    let slab = between([-13., -13., -1.], [13., 13., 1.], "slab")?;
    union(&ring, &slab).map_err(e)
}
