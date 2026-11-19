//! A pipe of radius 20 swept along the planar cubic Bézier (0, 0, 0), (12, −16, 0), (32, −16, 0),
//! (44, 0, 0), its section square to the curve.
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::along_curve;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::spline::{bezier_path, start_frame};

pub fn build() -> Result<Brep, String> {
    let path = bezier_path([[0., 0., 0.], [12., -16., 0.], [32., -16., 0.], [44., 0., 0.]])?;
    let sketch = start_frame(&path, [0., 0., 1.])?;
    let section = Profile::circle(Vec2::ZERO, 40.);
    along_curve(&section, &sketch, &path, (0., 1.), 0., "pipe").map_err(e)
}
