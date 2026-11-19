//! A square spire from 18 × 18 at z = 0 swelling to 24 × 24 and closing to a point at
//! (0, 0, 24), its corners on four cubic Bézier ridges.
use springrep::topo::Brep;

use crate::shared::loft::on_guides;

pub fn build() -> Result<Brep, String> {
    // no loft with guide curves (nor to a point): the walls are the ridge's plane curve carried
    // straight across, the common part of two extrusions
    on_guides([[9., 9., 0.], [15., 15., 8.], [12., 12., 19.], [0., 0., 24.]], "spire")
}
