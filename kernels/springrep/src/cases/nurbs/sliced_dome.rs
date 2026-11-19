//! The block under the bicubic Bézier dome, less everything above the plane z = 4 + x/2 through
//! its corner (0, 0, 4).
use springrep::boolean::split;
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::nurbs::bezier_dome;

pub fn build() -> Result<Brep, String> {
    let dome = bezier_dome()?;
    let (_, below) = split(&dome, Vec3::new(0., 0., 4.), Vec3::new(-0.5, 0., 1.)).map_err(e)?;
    Ok(below)
}
