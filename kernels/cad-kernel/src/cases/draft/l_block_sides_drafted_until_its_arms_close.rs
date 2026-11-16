//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), 8 high, its six sides
//! drafted 30° inward about z = 0: drafted that steeply, the sides of each 5-wide arm meet in a
//! ridge at 2.5 / tan 30° ≈ 4.33, under the top, and nothing of the top is left.
use crate::sketch::{block, drafted_face};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

const HEIGHT: f64 = 8.;
const DRAFT: f64 = 30.;

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
    // the L's section at each height, moved into its material side by side, is the two arms'
    // rectangles, each moved into its own: so the drafted L is the union of its two arms, each a
    // block drafted on all four sides, which `drafted_face` trims by the turned planes until the
    // sides of each close in its ridge. The sides the arms share a line with lie in one plane.
    let foot = roof([0., 0.], [20., 5.])?;
    let upright = roof([0., 0.], [5., 15.])?;
    Ok(combined(&foot, &upright, Op::Union, 0.)?)
}
