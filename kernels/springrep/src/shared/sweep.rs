//! Helpers for the sweep cases: a solid cut back to one side of a plane.
use springrep::boolean::split;
use springrep::math::Vec3;
use springrep::topo::Brep;

use super::e;

/// The part of `s` on the side of the plane through `origin` away from `normal`.
pub fn behind(s: &Brep, origin: [f64; 3], normal: [f64; 3]) -> Result<Brep, String> {
    let (_, kept) = split(s, Vec3::from_array(origin), Vec3::from_array(normal)).map_err(e)?;
    Ok(kept)
}
