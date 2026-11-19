//! A truncated cone on the z axis from z = 0 (radius 5) to z = 6 (radius 2).
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // a cone is placed by the frame at the middle of its base, its radii at the base and the top
    cone(&at([0., 0., 0.]), 5., 2., 6., "frustum").map_err(e)
}
