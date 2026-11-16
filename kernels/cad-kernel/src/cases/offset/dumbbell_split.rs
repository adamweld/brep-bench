//! The dumbbell-shaped plate, two 10 × 10 squares joined by a neck 4 long and 2 wide, 6 thick,
//! shrunk by 1.5: the neck vanishes and both halves are kept, two lumps of one solid.
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

const THICKNESS: f64 = 6.;
const SHRINK: f64 = 1.5;

pub fn build() -> Result<Brep, String> {
    // the outline in the xy plane: the left square, the neck, the right square
    let outline = polygon(&[[0., 0., 0.], [10., 0., 0.], [10., 4., 0.], [14., 4., 0.],
        [14., 0., 0.], [24., 0., 0.], [24., 10., 0.], [14., 10., 0.], [14., 6., 0.],
        [10., 6., 0.], [10., 10., 0.], [0., 10., 0.]]);
    shrunk(&prism(&outline, 0., THICKNESS)?, SHRINK, Join::Round)
}
