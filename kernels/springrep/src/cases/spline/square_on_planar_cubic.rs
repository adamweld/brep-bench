//! A 20 × 20 bar swept along the planar cubic Bézier (0, 0, 0), (16, 12, 0), (18, 26, 0),
//! (12, 34, 0), two of its sides horizontal.
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::along_curve;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::spline::{bezier_path, start_frame};

pub fn build() -> Result<Brep, String> {
    let path = bezier_path([[0., 0., 0.], [16., 12., 0.], [18., 26., 0.], [12., 34., 0.]])?;
    // the section's first axis upright: a planar path's rotation-minimizing frame keeps it so
    let sketch = start_frame(&path, [0., 0., 1.])?;
    let section = Profile::rect(Vec2::ZERO, 20., 20.);
    along_curve(&section, &sketch, &path, (0., 1.), 0., "bar").map_err(e)
}
