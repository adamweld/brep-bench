//! A Reuleaux tetrahedron: the part common to four balls of radius 10 centred on the vertices of a
//! regular tetrahedron of side 10.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let s3 = 3f64.sqrt();
    let centres = [
        [0., 0., 0.],
        [10., 0., 0.],
        [5., 5. * s3, 0.],
        [5., 5. * s3 / 3., 10. * (2f64 / 3.).sqrt()],
    ];
    let mut common = ball(centres[0], 10., "a")?;
    for (c, name) in centres[1..].iter().zip(["b", "c", "d"]) {
        common = intersection(&common, &ball(*c, 10., name)?).map_err(e)?;
    }
    Ok(common)
}
