//! The T through (0, 0), (20, 0), (20, 4), (13, 4), (13, 16), (7, 16), (7, 4) and (0, 4), 8 high,
//! its eight sides drafted 45° inward about z = 0: the bar's long sides meet in a ridge at height
//! 2 and the stem's at 3, both under the top, and nothing of the top is left.
use crate::sketch::{block, drafted_face};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

const HEIGHT: f64 = 8.;
const DRAFT: f64 = 45.;

/// The block over the rectangle from `lo` to `hi` with its four sides drafted inward about the
/// floor: each turned about its bottom edge and the block kept inside it.
fn roof(lo: [f64; 2], hi: [f64; 2]) -> Result<Brep, String> {
    let draft = DRAFT.to_radians();
    let mut b = block([lo[0], lo[1], 0.], [hi[0], hi[1], HEIGHT])?;
    for (on, outward) in [([lo[0], lo[1], 0.], [0., -1., 0.]), ([hi[0], lo[1], 0.], [1., 0., 0.]),
                          ([hi[0], hi[1], 0.], [0., 1., 0.]), ([lo[0], hi[1], 0.], [-1., 0., 0.])] {
        b = drafted_face(&b, on, outward, draft)?;
    }
    Ok(b)
}

pub fn build() -> Result<Brep, String> {
    // cad-kernel drafts a profile as it extrudes it (`build::drafted`), but its offset section
    // refuses to vanish, and it has no draft of a solid's faces. A drafted side is a plane, and
    // the T's section at each height, moved into its material side by side, is its bar's
    // rectangle and its stem's (run down through the bar to y = 0), each moved into its own: so
    // the drafted T is the union of the two, each a block drafted on all four sides, which
    // `drafted_face` trims by the turned planes until the sides of each close in its ridge.
    let bar = roof([0., 0.], [20., 4.])?;
    let stem = roof([7., 0.], [13., 16.])?;
    Ok(combined(&bar, &stem, Op::Union, 0.)?)
}
