//! Helpers the boolean cases share: a block between two corners, a ball with its poles on the
//! vertical, and a rod along any axis, each one of springrep's primitives placed by a frame.
use springrep::math::{Frame, Vec3};
use springrep::primitive::{block, cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::{at, e};

/// The block between the corners `lo` and `hi`, its edges along the world's axes.
pub fn between(lo: [f64; 3], hi: [f64; 3], name: &str) -> Result<Brep, String> {
    let size = Vec3::from_array(hi) - Vec3::from_array(lo);
    block(&at(lo), size, name).map_err(e)
}

/// The ball of radius `r` about `center`, its poles on the vertical through it.
pub fn ball(center: [f64; 3], r: f64, name: &str) -> Result<Brep, String> {
    sphere(&at(center), r, name).map_err(e)
}

/// The rod of radius `r` from `base` along `axis` for `length`.
pub fn rod(
    base: [f64; 3],
    axis: [f64; 3],
    r: f64,
    length: f64,
    name: &str,
) -> Result<Brep, String> {
    let frame = Frame::from_axis(Vec3::from_array(base), Vec3::from_array(axis)).map_err(e)?;
    cylinder(&frame, r, length, name).map_err(e)
}
