//! Helpers for the singular cases: a solid with everything on one side of a plane cut away.
use springrep::boolean::split;
use springrep::math::Vec3;
use springrep::topo::Brep;

use super::e;

/// `s` less everything on the side of the plane through `origin` that `normal` points to.
pub fn cut_away(s: &Brep, origin: [f64; 3], normal: [f64; 3]) -> Result<Brep, String> {
    let (_, kept) = split(s, Vec3::from_array(origin), Vec3::from_array(normal)).map_err(e)?;
    Ok(kept)
}
