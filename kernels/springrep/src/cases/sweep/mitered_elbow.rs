//! A 10 × 10 bar along (0, 0, 0) → (40, 0, 0) → (20, 20√3, 0), mitered at the corner in the
//! plane x + √3y = 40 that bisects it.
use springrep::boolean::union;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::sweep::behind;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // springrep's sweep along legs fills a sharp turn by turning the section about the corner (a
    // round joint), so each run is extruded on past the corner and cut back to the miter plane
    let square = Profile::rect(Vec2::ZERO, 10., 10.);
    let s3 = 3f64.sqrt();
    let first = extrude(&square, &frame([0., 0., 0.], [1., 0., 0.], [0., 0., 1.])?, 60., "a")
        .map_err(e)?;
    let first = behind(&first, [40., 0., 0.], [1., s3, 0.])?;
    // the second run, from its far end back toward the corner
    let end = [20., 20. * s3, 0.];
    let back = [0.5, -s3 / 2., 0.];
    let second = extrude(&square, &frame(end, back, [0., 0., 1.])?, 60., "b").map_err(e)?;
    let second = behind(&second, [40., 0., 0.], [-1., -s3, 0.])?;
    union(&first, &second).map_err(e)
}
