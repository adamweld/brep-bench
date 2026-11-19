//! An L-shaped block, its inside corner chamfered 7 on both faces, adding material: the chamfer
//! runs from the far edge of one face to the far edge of the other, consuming both.
use springrep::blend::chamfer;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the section in (x, z) on the plane y = 0, extruded along +y
    let xz = frame([0., 0., 0.], [0., 1., 0.], [1., 0., 0.])?;
    let p = |x: f64, z: f64| Vec2::new(x, -z); // the frame's y is −z
    let section = Profile::polygon(
        &[p(0., 0.), p(10., 0.), p(10., 3.), p(3., 3.), p(3., 10.), p(0., 10.)],
        &["bottom", "end", "ledge", "wall", "top", "back"],
    );
    let ell = extrude(&section, &xz, 10., "l").map_err(e)?;
    chamfer(&ell, &[("l.ledge", "l.wall")], 7., "chamfer").map_err(e)
}
