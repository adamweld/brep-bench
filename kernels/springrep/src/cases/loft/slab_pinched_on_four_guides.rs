//! A slab lofted through rectangles centred on the z axis from z = 0 to 24, its corners on four
//! cubic Bézier guides, pinched to 0.4 thick at z = 26/3 as its long sides turn from x to y.
use springrep::topo::Brep;

use crate::shared::loft::on_guides;

pub fn build() -> Result<Brep, String> {
    // springrep lofts through sections only, with no guide curves; the rectangles' walls are each
    // a guide's plane curve carried straight across, the common part of two extrusions
    on_guides([[12., 3.2, 0.], [18., -2.8, 8.], [15., 0.2, 19.], [3., 12.2, 24.]], "slab")
}
